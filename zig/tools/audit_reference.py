"""Compare reference inventories with the standalone SDK. Used by maintainers only."""
import argparse
import ast
import inspect
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]


def signature(method):
    args = method.args
    parameter = inspect.Parameter
    positional = [*args.posonlyargs, *args.args]
    required = len(positional) - len(args.defaults)
    parsed = []
    for index, arg in enumerate(positional):
        if index == 0 and arg.arg in {"self", "cls"}:
            continue
        kind = parameter.POSITIONAL_ONLY if index < len(args.posonlyargs) else parameter.POSITIONAL_OR_KEYWORD
        parsed.append(parameter(arg.arg, kind, default=parameter.empty if index < required else None))
    if args.vararg:
        parsed.append(parameter(args.vararg.arg, parameter.VAR_POSITIONAL))
    for arg, default in zip(args.kwonlyargs, args.kw_defaults):
        parsed.append(parameter(arg.arg, parameter.KEYWORD_ONLY, default=parameter.empty if default is None else None))
    if args.kwarg:
        parsed.append(parameter(args.kwarg.arg, parameter.VAR_KEYWORD))
    return inspect.Signature(parsed)


def incompatible_call(reference, candidate):
    """Check public positional and keyword call forms without importing either SDK."""
    def property_getter(method):
        return any(isinstance(d, ast.Name) and d.id == "property" for d in method.decorator_list)

    if property_getter(reference) != property_getter(candidate):
        return True
    original = signature(reference)
    if any(arg.kind in (arg.VAR_POSITIONAL, arg.VAR_KEYWORD) for arg in original.parameters.values()) and all(arg.kind in (arg.VAR_POSITIONAL, arg.VAR_KEYWORD) for arg in original.parameters.values()):
        # The native stubs use (*args, **kwargs) for some PyO3 methods. They do
        # not establish whether an empty call is valid or identify keyword names.
        return False
    target = signature(candidate)
    for use_keywords in (True, False):
        for use_defaults in (True, False):
            positional, keywords = [], {}
            for p in signature(reference).parameters.values():
                if use_defaults and p.default != p.empty:
                    continue
                if p.kind == p.POSITIONAL_ONLY or (p.kind == p.POSITIONAL_OR_KEYWORD and not use_keywords):
                    positional.append(None)
                elif p.kind in (p.POSITIONAL_OR_KEYWORD, p.KEYWORD_ONLY):
                    keywords[p.name] = None
            try:
                target.bind(*positional, **keywords)
            except TypeError:
                return True
    return False


def client_surfaces(directory, *, constructors=False):
    classes, aliases = {}, {}
    delegates = set()
    for path in sorted(directory.glob("*.py")):
        for item in ast.parse(path.read_text(), filename=str(path)).body:
            if isinstance(item, ast.ClassDef):
                methods = {f.name: f for f in item.body if isinstance(f,(ast.FunctionDef,ast.AsyncFunctionDef)) and (not f.name.startswith("_") or constructors and f.name == "__init__")}
                if any(isinstance(f, ast.FunctionDef) and f.name == "__getattr__" for f in item.body):
                    delegates.add((path.stem, item.name))
                bases = [base.id for base in item.bases if isinstance(base,ast.Name)]
                # Several reference modules use the private name _Client for different bases.
                classes[path.stem, item.name] = (methods,bases)
            if isinstance(item,ast.Assign) and isinstance(item.value,ast.Name):
                for name in item.targets:
                    if isinstance(name,ast.Name):
                        aliases[name.id] = item.value.id

    # These three chain facades explicitly forward unknown members to PyO3.
    native = directory / "_native.pyi"
    if delegates and native.exists():
        declarations = {c.name: c for c in ast.parse(native.read_text()).body if isinstance(c, ast.ClassDef)}
        for module, name in delegates:
            if name in declarations:
                methods, _ = classes[module, name]
                for method in declarations[name].body:
                    if isinstance(method, (ast.FunctionDef, ast.AsyncFunctionDef)) and not method.name.startswith("_"):
                        methods.setdefault(method.name, method)

    def inherited(module,name,seen):
        key = (module,name)
        if key not in classes:
            key = next((k for k in classes if k[1]==name),None)
        if key is None or key in seen:
            return {}
        methods,bases = classes[key]
        surface = {}
        for base in reversed(bases):
            surface.update(inherited(key[0], base, seen | {key}))
        surface.update(methods)
        return surface

    result = {name:inherited(module,name,set()) for module,name in classes if not name.startswith("_") and name.endswith("Client")}
    for name,target in aliases.items():
        if target in result and name.endswith("Client"):
            result[name] = result[target]
    return result


def inventory(crates):
    phoenix = crates / "exchanges/phoenix"
    dispatch = set(json.loads((phoenix / "tests/fixtures/instruction_dispatch.json").read_text()))
    constructors = {name for path in (phoenix/"src/instructions").rglob("*.rs")
                    for name in re.findall(r"^pub fn (create_\w+_ixs?)\(",path.read_text(),re.M)}
    phoenix_cases = set(json.loads((ROOT/"tests/phoenix-instructions.json").read_text()))
    metric = {(row["contract"],row["signature"]) for row in json.loads((crates/"exchanges/metric/contracts.json").read_text()) if row["kind"]=="function"}
    named = {tuple(line.split("\t")[:2]) for line in (ROOT/"src/exchanges/metric/schema/metric-contract-methods.tsv").read_text().splitlines()}
    abi = {(r["contract"],r["signature"]) for r in json.loads((ROOT/"tests/metric-abi.json").read_text()) if r["kind"]=="function"}
    expected = client_surfaces(crates/"python/python/aux_py")
    actual = client_surfaces(ROOT/"sdk/python/python/aux_zig")
    absent, signatures = {}, {}
    for name in sorted(actual.keys() & expected.keys()):
        reference, port = expected[name], actual[name]
        missing = sorted(reference.keys() - port.keys())
        if missing:
            absent[name] = missing
        mismatched = sorted(m for m in reference.keys() & port.keys() if incompatible_call(reference[m], port[m]))
        if mismatched:
            signatures[name] = mismatched
    ref_initializers = client_surfaces(crates/"python/python/aux_py", constructors=True)
    zig_initializers = client_surfaces(ROOT/"sdk/python/python/aux_zig", constructors=True)
    constructors_missing = []
    opaque = []
    for name, methods in sorted(ref_initializers.items()):
        init = methods.get("__init__")
        if init is None:
            continue
        parameters = signature(init).parameters.values()
        if any(p.kind == p.VAR_KEYWORD for p in parameters):
            opaque.append(name)
        target = zig_initializers.get(name, {}).get("__init__")
        if target is None or incompatible_call(init, target):
            constructors_missing.append(name)
    return {
        "python_reference_client_names":len(expected),
        "python_reference_client_members":sum(map(len, expected.values())),
        "python_constructors_with_incompatible_call_forms":constructors_missing,
        "python_constructors_accepting_arbitrary_reference_options":opaque,
        "phoenix_dispatch_count":len(dispatch),"phoenix_constructor_count":len(constructors),
        "phoenix_missing_fixtures":sorted((dispatch|constructors)-phoenix_cases),
        "metric_reference_functions":len(metric),"metric_missing_bindings":sorted(metric-named),
        "metric_missing_vectors":sorted(metric-abi),
        "python_missing_client_names":sorted(expected.keys()-actual.keys()),
        "python_methods_without_matching_name": absent,
        "python_methods_with_incompatible_call_forms": signatures,
    }


def runtime_exports(crates):
    import aux_zig

    source = crates / "python/python/aux_py/__init__.py"
    names = set()
    for statement in ast.parse(source.read_text(), filename=str(source)).body:
        if isinstance(statement, ast.Assign):
            targets = statement.targets
        elif isinstance(statement, ast.AugAssign) and isinstance(statement.op, ast.Add):
            targets = [statement.target]
        else:
            continue
        if any(isinstance(t, ast.Name) and t.id == "__all__" for t in targets):
            names.update(ast.literal_eval(statement.value))
    return {
        "python_reference_exports": len(names),
        "python_missing_runtime_exports": sorted(name for name in names if not hasattr(aux_zig, name)),
    }


if __name__ == "__main__":
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--crates",type=Path,default=ROOT.parent/"crates")
    parser.add_argument("--runtime-exports", action="store_true", help="compare reference __all__ with the installed aux_zig wheel")
    args=parser.parse_args()
    report = inventory(args.crates)
    if args.runtime_exports:
        report.update(runtime_exports(args.crates))
    print(json.dumps(report,indent=2))

// Compare the public client declarations and optionally the exports of both built native packages.
const { resolve } = require("node:path");
const ts = require(require.resolve("typescript", {paths:[resolve(__dirname, "../sdk/typescript")]}));

function clientMembers(filename) {
  const file = resolve(filename);
  const program = ts.createProgram([file], {
    target: ts.ScriptTarget.ES2022,
    module: ts.ModuleKind.Node16,
    moduleResolution: ts.ModuleResolutionKind.Node16,
    skipLibCheck: true,
  });
  const checker = program.getTypeChecker();
  const source = program.getSourceFile(file);
  if (!source) throw new Error("Could not load " + file);
  const clients = new Map();
  for (const item of checker.getExportsOfModule(checker.getSymbolAtLocation(source))) {
    if (!item.name.endsWith("Client")) continue;
    const symbol = item.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(item) : item;
    const members = checker.getDeclaredTypeOfSymbol(symbol).getProperties();
    clients.set(item.name, new Set(members.map(member => member.name)));
  }
  return clients;
}

const args = process.argv.slice(2);
const crates = args.find(arg => arg !== "--runtime-exports") ?? resolve(__dirname, "../../crates");
const reference = clientMembers(resolve(crates, "node/js/index.d.ts"));
const zig = clientMembers(resolve(__dirname, "../sdk/typescript/js/index.d.ts"));
const missing = {};
for (const [client, members] of reference) {
  const absent = [...members].filter(member => !zig.get(client)?.has(member));
  if (absent.length) missing[client] = absent.sort();
}
const unmatched = [...reference.keys()].filter(client => !zig.has(client));
const report = { clients: reference.size, unmatched, missing };
if (args.includes("--runtime-exports")) {
  const names = Object.keys(require(resolve(crates, "node")));
  const sdk = require(resolve(__dirname, "../sdk/typescript"));
  report.referenceExports = names.length;
  report.missingRuntimeExports = names.filter(name => !Object.hasOwn(sdk, name)).sort();
}
console.log(JSON.stringify(report, null, 2));
process.exitCode = Number(Boolean(unmatched.length || Object.keys(missing).length || report.missingRuntimeExports?.length));

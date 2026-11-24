"""Generate SDK methods that pass a named command to its dedicated Zig session."""
from pathlib import Path


def session_commands(root: Path):
    derive = [line.split()[0] for line in (root / "src/exchanges/derive/rpc-routes.txt").read_text().splitlines()]
    perps = ("create_orders modify_orders modify_orders_coid cancel_orders cancel_orders_coid "
             "cancel_all auto_cancel update_leverage update_leverages update_margin subscribe unsubscribe ping").split()
    return {"Derive": derive, "PolymarketPerps": perps, "PolymarketRfq": ["quote", "cancel_quote", "last_look"]}


def emit_sessions(root: Path, emit, camel):
    sessions = session_commands(root)
    sdk = root / "sdk"
    rust, js, ts = [], ['"use strict";', 'exports.commands = {'], []
    py = ['"""Generated from the native RPC command inventory."""', 'from . import _RpcSession', '']
    for label, commands in sessions.items():
        session = label + "RpcSession"
        rust.append(f'rpc_session!({session}, ' + ', '.join(m for m in commands if m != 'ping') + ');')
        py.append(f'class {session}(_RpcSession):')
        ts.append(f'export interface {session}Commands {{')
        venue = {"Derive":"derive", "PolymarketPerps":"polymarket_perps", "PolymarketRfq":"polymarket"}[label]
        js.append(f'  {venue}: send => ({{')
        for method in commands:
            channels = label == "PolymarketPerps" and method in ("subscribe","unsubscribe")
            param = "channels" if channels else "request"
            optional = label == "Derive"
            if method == "ping":
                rust.append(f'impl {session} {{ pub fn ping(&self) -> Result<Value> {{ self.send("ping", &Value::Null) }} }}')
                py.extend(['    async def ping(self):', '        return await self._command("ping")', ''])
                js.append('    ping: () => send("ping", {}),')
                ts.append('  ping(): Promise<import("./index").HttpValue>;')
                continue
            py.extend([f'    async def {method}(self, {param}' + ('=None' if optional else '') + '):',
                       f'        return await self._command("{method}", ' + ('{} if request is None else request' if optional else param) + ')', ''])
            js.append(f'    {camel(method)}: ({param}' + (' = {}' if optional else '') + f') => send("{method}", {param}),')
            ts.append(f'  {camel(method)}({param}' + ('?' if optional else '') + ': ' +
                      ('string[]' if channels else 'import("./index").HttpValue') + '): Promise<import("./index").HttpValue>;')
        js.append('  }),')
        ts.append('}')
    js.append('};')
    for path, lines in ((sdk/'rust/src/rpc_generated.rs', rust),(sdk/'python/python/aux_py/rpc_generated.py', py),
                        (sdk/'typescript/js/rpc.cjs', js),(sdk/'typescript/js/rpc.d.ts', ts)):
        emit(path, '\n'.join(lines)+'\n')

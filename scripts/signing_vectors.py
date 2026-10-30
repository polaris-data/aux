#!/usr/bin/env python3
"""Generate offline fixtures with a pinned official Hyperliquid signing oracle.

Run in an isolated development venv with eth-account==0.13.7 and msgpack==1.1.2.
This script fetches reference code; it is never part of normal CI or runtime.
The private key below is public test data from the reference SDK, not a secret.
"""
import hashlib
import importlib.metadata
import json
import sys
import tempfile
import urllib.request
from pathlib import Path

REVISION = "2fdb18f9517675ea03695a0962bd19eece9c83f0"
REPOSITORY = "https://github.com/hyperliquid-dex/hyperliquid-python-sdk"
TEST_KEY = "0x0123456789012345678901234567890123456789012345678901234567890123"
ROOT = Path(__file__).resolve().parent.parent


def main():
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        hashes = {}
        for name in ["hyperliquid/utils/signing.py", "hyperliquid/utils/types.py"]:
            content = urllib.request.urlopen(
                f"https://raw.githubusercontent.com/hyperliquid-dex/hyperliquid-python-sdk/{REVISION}/{name}",
                timeout=20,
            ).read()
            hashes[name] = hashlib.sha256(content).hexdigest()
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(content)
        for package in [root / "hyperliquid", root / "hyperliquid/utils"]:
            (package / "__init__.py").write_text("")
        sys.path.insert(0, directory)
        from hyperliquid.utils import signing as sdk
        import msgpack
        from eth_account import Account
        from eth_account.messages import encode_typed_data
        from eth_utils import keccak

        wallet = Account.from_key(TEST_KEY)

        def digest(payload):
            message = encode_typed_data(full_message=payload)
            return "0x" + keccak(b"\x19" + message.version + message.header + message.body).hex()

        def wire(kind="limit", cloid=None):
            order_type = {"limit": {"tif": "Ioc"}}
            if kind == "trigger":
                order_type = {"trigger": {"isMarket": True, "triggerPx": "1700", "tpsl": "sl"}}
            order = {"a": 4, "b": True, "p": "1670.1", "s": "0.0147", "r": False, "t": order_type}
            if cloid is not None:
                order["c"] = cloid
            return order

        cloid = "0x00000000000000000000000000000001"
        builder = {"b": "0x0000000000000000000000000000000000000003", "f": 1,
                   "s": [{"b": "0x0000000000000000000000000000000000000004", "s": 50000}]}
        actions = [
            ("limit", sdk.order_wires_to_order_action([wire()])),
            ("cloid", sdk.order_wires_to_order_action([wire(cloid=cloid)])),
            ("trigger", sdk.order_wires_to_order_action([wire("trigger")])),
            ("builder_priority", sdk.order_wires_to_order_action([wire()], builder, {"p": 1})),
            ("cancel", {"type": "cancel", "cancels": [{"a": 4, "o": 18446744073709551610}]}),
            ("cancel_by_cloid", {"type": "cancelByCloid", "cancels": [{"asset": 4, "cloid": cloid}]}),
        ]
        nonce = 1677777606040
        contexts = [
            ("plain", None, None),
            ("vault", "0x1719884eB866cb12b2287399B15f7DB5e7d775Ea", None),
            ("vault_expiry", "0x1719884eb866cb12b2287399b15f7db5e7d775ea", nonce + 30000),
        ]
        l1 = []
        for name, action in actions:
            for network in ["Mainnet", "Testnet"]:
                for context, vault, expiry in contexts:
                    connection_id = sdk.action_hash(action, vault, nonce, expiry)
                    payload = sdk.l1_payload(sdk.construct_phantom_agent(connection_id, network == "Mainnet"))
                    l1.append({
                        "name": f"{name}_{network}_{context}", "action": action, "network": network,
                        "vault_address": vault, "nonce": nonce, "expires_after": expiry,
                        "msgpack": msgpack.packb(action).hex(), "action_hash": "0x" + connection_id.hex(),
                        "digest": digest(payload),
                        "signature": sdk.sign_l1_action(wallet, action, vault, nonce, expiry, network == "Mainnet"),
                    })
        user_signed = []
        for network in ["Mainnet", "Testnet"]:
            for chain_id in [421614, 1]:
                for name in ["usd_send", "approve_agent"]:
                    if name == "usd_send":
                        action = {"destination": "0x5e9ee1089755c3435139848e47e6635505d5a13a", "amount": "1", "time": 1687816341423}
                        fields = sdk.USD_SEND_SIGN_TYPES
                        primary_type = "HyperliquidTransaction:UsdSend"
                    else:
                        action = {"agentAddress": "0x0000000000000000000000000000000000000005", "agentName": "desk-test", "nonce": nonce}
                        fields = [{"name": "hyperliquidChain", "type": "string"},
                                  {"name": "agentAddress", "type": "address"},
                                  {"name": "agentName", "type": "string"},
                                  {"name": "nonce", "type": "uint64"}]
                        primary_type = "HyperliquidTransaction:ApproveAgent"
                    action["signatureChainId"] = hex(chain_id)
                    action["hyperliquidChain"] = network
                    payload = sdk.user_signed_payload(primary_type, fields, action)
                    user_signed.append({
                        "name": name, "action": action, "network": network, "signature_chain_id": chain_id,
                        "digest": digest(payload), "signature": sdk.sign_inner(wallet, payload),
                    })
        result = {
            "reference": {"repository": REPOSITORY, "revision": REVISION, "sha256": hashes,
                          "packages": {name: importlib.metadata.version(name) for name in ["eth-account", "eth-utils", "msgpack"]}},
            "public_test_private_key": TEST_KEY, "signer_address": wallet.address.lower(),
            "l1": l1, "user_signed": user_signed,
        }
        target = ROOT / "crates/exchanges/hyperliquid/tests/fixtures/signing_vectors.json"
        target.write_text(json.dumps(result, indent=2) + "\n")
        print(f"Generated {len(l1)} L1 and {len(user_signed)} user-signed reference vectors.")


if __name__ == "__main__":
    main()

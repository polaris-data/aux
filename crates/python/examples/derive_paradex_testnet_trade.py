"""Opt-in testnet submit/cancel. Uses caller-selected size and price from env."""
import argparse
import asyncio
import os
from aux_py import DeriveClient, DeriveSigner, ParadexClient, ParadexSigner


async def trade(venue):
    if venue == "derive":
        async with DeriveClient(network="testnet", signer=DeriveSigner(os.environ["DERIVE_PRIVATE_KEY"]),
                                wallet=os.environ["DERIVE_WALLET"]) as client:
            subaccount = int(os.environ["DERIVE_SUBACCOUNT_ID"])
            result = await client.private_order({
                "subaccount_id": subaccount, "instrument_name": "ETH-PERP", "direction": "buy",
                "amount": os.environ["DERIVE_SIZE"], "limit_price": os.environ["DERIVE_PRICE"],
                "max_fee": "1", "order_type": "limit", "time_in_force": "post_only",
            })
            order_id = result["order"]["order_id"]
            print("Testnet order:", order_id)
            print(await client.private_cancel({"subaccount_id": subaccount, "instrument_name": "ETH-PERP", "order_id": order_id}))
    else:
        async with ParadexClient(network="testnet", signer=ParadexSigner(os.environ["PARADEX_PRIVATE_KEY"]),
                                 account=os.environ["PARADEX_ACCOUNT"], subkey=os.environ.get("PARADEX_SUBKEY")) as client:
            await client.authenticate()
            result = await client.orders_new({"body": {
                "market": "ETH-USD-PERP", "instruction": "POST_ONLY", "side": "BUY", "type": "LIMIT",
                "size": os.environ["PARADEX_SIZE"], "price": os.environ["PARADEX_PRICE"],
            }})
            order_id = result["id"]
            print("Testnet order:", order_id)
            print(await client.orders_cancel({"order_id": order_id}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("venue", choices=("derive", "paradex"))
    parser.add_argument("--execute", action="store_true", help="submit and cancel an actual testnet order")
    args = parser.parse_args()
    if args.execute:
        asyncio.run(trade(args.venue))
    else:
        print("No order submitted. Supply --execute and the documented testnet environment variables.")

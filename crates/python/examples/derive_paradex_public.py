"""Read-only testnet checks. Run independently of offline tests."""
import asyncio
from aux_py import DeriveClient, ParadexClient


async def first_event(stream):
    async for event in stream:
        if "params" in event:
            return event["params"]["channel"]
    raise RuntimeError("stream ended before its first event")


async def main():
    async with DeriveClient() as derive:
        market = await derive.public_get_instrument({"instrument_name": "ETH-PERP"})
        print("Derive:", market["instrument_name"], market["tick_size"])
        async with await derive.subscribe({"type": "orderbook", "instrument_name": "ETH-PERP", "group": 10, "depth": 20}) as stream:
            print("Derive book:", await asyncio.wait_for(first_event(stream), 15))
    async with ParadexClient() as paradex:
        config = await paradex.get_system_config()
        print("Paradex:", config["starknet_chain_id"])
        async with await paradex.subscribe({"type": "order_book", "market": "ETH-USD-PERP", "feed_type": "snapshot", "refresh_rate": "100ms", "price_tick": None}) as stream:
            print("Paradex SBE book:", await asyncio.wait_for(first_event(stream), 15))


asyncio.run(main())

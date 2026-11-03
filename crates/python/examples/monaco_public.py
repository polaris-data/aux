"""Explicit public testnet network check; no credentials or orders."""
import asyncio
from aux_py import MonacoClient


async def main():
    async with MonacoClient() as client:
        pairs = (await client.trading_pairs({"pageSize": 100, "isActive": True}))["tradingPairs"]
        pair = next((p for p in pairs if p["symbol"] == "BTC/USDC"), pairs[0])
        print(pair["symbol"], pair["marketType"])
        print(await client.order_book({"tradingPairId": pair["id"], "tradingMode": pair["marketType"], "levels": 20}))
        stream = await client.subscribe({"type": "orderbook", "tradingPairId": pair["id"], "mode": pair["marketType"]})
        async with stream:
            while True:
                event = await asyncio.wait_for(anext(stream), 15)
                if event["type"] == "Snapshot":
                    print("Orderbook snapshot received")
                    break


if __name__ == "__main__":
    asyncio.run(main())

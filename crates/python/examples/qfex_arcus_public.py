"""Read-only SDK smoke checks for QFEX UAT and Arcus testnet."""
import asyncio
from aux_py import QfexClient, ArcusClient


async def main():
    async with QfexClient(network="testnet") as qfex:
        symbols = await qfex.refdata(ticker="AAPL-USD")
        book = await qfex.order_book("AAPL-USD")
        async with await qfex.subscribe("level2", "AAPL-USD") as stream:
            while True:
                event = await asyncio.wait_for(anext(stream), 15)
                if event["type"] == "level2":
                    break
        print(f'QFEX UAT: {len(symbols["data"])} symbols; {len(book["bids"])} bids; native stream received')
    async with ArcusClient(network="testnet") as arcus:
        markets = await arcus.markets(market="BTC-USD")
        book = await arcus.order_book("BTC-USD")
        async with await arcus.subscribe({"channel": "l2Orderbook", "id": "BTC-USD", "nLevels": 2}) as stream:
            while True:
                event = await asyncio.wait_for(anext(stream), 15)
                if event.get("contents") is not None:
                    break
        print(f'Arcus testnet: {len(markets["markets"])} markets; {len(book["bids"])} bids; native stream received')


asyncio.run(main())

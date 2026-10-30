"""Explicit, read-only public testnet check; never run by the offline suite."""
import asyncio
from aux_py import HyperliquidClient, LighterClient


async def main():
    async with HyperliquidClient(network="testnet") as client:
        meta = await client.meta()
        print(f"Testnet perpetual markets: {len(meta['universe'])}")
        book = await client.l2_book("ETH")
        print(f"HTTP best bid (native string): {book['levels'][0][0]['px']}")
        async with await client.subscribe_l2_book("ETH") as books:
            snapshot = await asyncio.wait_for(anext(books), timeout=20)
            print(f"WebSocket snapshot time: {snapshot['time']}")
        assert books.closed
        account = "0x0000000000000000000000000000000000000001"
        state = await client.clearinghouse_state(account)
        orders = await client.open_orders(account)
        print(f"Hyperliquid public account: {len(state['assetPositions'])} positions, {len(orders)} orders")
        async with await client.subscribe_trades("ETH") as trades:
            # Generic streams retain the native acknowledgement envelope.
            acknowledgement = await asyncio.wait_for(anext(trades), timeout=20)
            assert acknowledgement["channel"] == "subscriptionResponse"
    assert client.closed
    print("Python client and subscription closed.")

    async with LighterClient(network="testnet") as client:
        markets = await client.order_books()
        market = next(m for m in markets["order_books"] if m["status"] == "active")
        market_id = market["market_id"]
        await client.order_book_orders(market_id, limit=2)
        await client.recent_trades(market_id, limit=2)
        await client.account(account_index=1)
        print(f"Lighter testnet markets: {len(markets['order_books'])}; selected {market['symbol']}")
        for subscribe in [lambda: client.subscribe_order_book(market_id),
                          lambda: client.subscribe_trades(market_id),
                          lambda: client.subscribe_account(1)]:
            async with await subscribe() as stream:
                event = await asyncio.wait_for(anext(stream), timeout=20)
                assert event["type"].startswith("subscribed/")
                print(f"Lighter native snapshot: {event['type']}")
    assert client.closed


if __name__ == "__main__":
    asyncio.run(main())

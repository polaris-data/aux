"""Explicitly run read-only public smoke requests. No credentials or writes."""
import asyncio
from aux_py import KalshiClient, PolymarketClient, PascalClient

async def main():
    async with KalshiClient(network='demo') as kalshi:
        print('Kalshi', await kalshi.get_exchange_status())
    async with PolymarketClient(network='mainnet') as polymarket:
        print('Polymarket', await polymarket.clob_get_time())
    async with PascalClient(network='canary') as pascal:
        print('Pascal', await pascal.server_time())

if __name__ == '__main__':
    asyncio.run(main())

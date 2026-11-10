"""Explicit public market reads; no credentials, subscriptions or writes."""
import asyncio
from aux_py import PhoenixClient, ExtendedClient

async def main():
    async with PhoenixClient() as phoenix:
        print('Phoenix mainnet markets:', len(await phoenix.list_markets()))
    async with ExtendedClient(network='testnet') as extended:
        print('Extended testnet markets:', len((await extended.get_markets())['data']))

if __name__ == '__main__':
    asyncio.run(main())

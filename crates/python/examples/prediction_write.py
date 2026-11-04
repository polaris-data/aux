"""Explicit single native write. Provide a reviewed request JSON file; no retries.

Credentials: KALSHI_KEY_ID + KALSHI_PEM_FILE, PASCAL_OWNER + PASCAL_SEED,
or POLYMARKET_KEY + WALLET + API_KEY + API_SECRET + API_PASSPHRASE.
Read timeout/disconnect means reconcile externally before issuing another write.
"""
import argparse
import asyncio
import json
import os
from decimal import Decimal
from pathlib import Path
from aux_py import (KalshiClient, KalshiSigner, PascalClient, PascalSigner,
                    PolymarketClient, PolymarketSigner)

async def main(args):
    request = json.loads(Path(args.request).read_text(), parse_float=Decimal)
    if args.venue == 'kalshi':
        client = KalshiClient(network=args.network, key_id=os.environ['KALSHI_KEY_ID'],
                             signer=KalshiSigner(Path(os.environ['KALSHI_PEM_FILE']).read_text()))
        method = 'create_order_v2'
    elif args.venue == 'pascal':
        client = PascalClient(network=args.network, owner=os.environ['PASCAL_OWNER'],
                              signer=PascalSigner(os.environ['PASCAL_SEED']))
        method = 'place_and_replace_orders'
    else:
        client = PolymarketClient(network=args.network,
            signer=PolymarketSigner(os.environ['POLYMARKET_KEY']), wallet=os.environ['POLYMARKET_WALLET'],
            api_key=os.environ['POLYMARKET_API_KEY'], api_secret=os.environ['POLYMARKET_API_SECRET'],
            api_passphrase=os.environ['POLYMARKET_API_PASSPHRASE'],
            order_version=os.environ['POLYMARKET_ORDER_VERSION'])
        method = 'clob_post_order'
    async with client:
        print(await getattr(client, method)(request))

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('venue', choices=['kalshi', 'pascal', 'polymarket'])
    parser.add_argument('--network', required=True)
    parser.add_argument('--request', required=True, help='reviewed native Params mapping, including body')
    parser.add_argument('--execute', action='store_true', help='submit one actual order')
    args = parser.parse_args()
    if not args.execute:
        parser.exit(message='No order sent. Use --execute with a reviewed native request.\n')
    if args.network in ('mainnet', 'production') and os.environ.get('AUX_ALLOW_MAINNET') != '1':
        parser.error('set AUX_ALLOW_MAINNET=1 explicitly for a mainnet order')
    asyncio.run(main(args))

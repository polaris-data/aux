"""Offline prediction connector parity, signing and lifecycle contracts."""
from __future__ import annotations
import asyncio
import base64
import contextvars
import json
import unittest
from decimal import Decimal
from pathlib import Path
from urllib.parse import urlsplit, parse_qs
import aux_py as sdk

ROOT = Path(__file__).resolve().parents[2] / 'exchanges'
VENUES = ('kalshi', 'polymarket', 'pascal')

def data(venue, name):
    return json.loads((ROOT / venue / name).read_text())

class PredictionTests(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.clients = []
        self.tasks = set()
        self.requests = []
        self.payload = {}
        self.status = 200
        self.started = asyncio.Event()
        self.release = asyncio.Event()
        self.hang = False
        self.server = await asyncio.start_server(self.http, '127.0.0.1', 0)
        self.url = f'http://127.0.0.1:{self.server.sockets[0].getsockname()[1]}'

    async def asyncTearDown(self):
        self.release.set()
        for client in self.clients:
            await asyncio.wait_for(client.aclose(), 5)
        self.server.close()
        await self.server.wait_closed()
        for task in list(self.tasks):
            task.cancel()
        await asyncio.gather(*self.tasks, return_exceptions=True)

    async def http(self, reader, writer):
        self.tasks.add(asyncio.current_task())
        try:
            raw = await reader.readuntil(b'\r\n\r\n')
            lines = raw.decode().split('\r\n')
            headers = dict((k.lower(), v.strip()) for line in lines[1:] if ':' in line for k, v in [line.split(':', 1)])
            body = await reader.readexactly(int(headers.get('content-length', '0')))
            self.requests.append((lines[0].split(), headers, json.loads(body) if body else None))
            self.started.set()
            if self.hang:
                await self.release.wait()
            payload = json.dumps(self.payload, separators=(',', ':')).encode()
            writer.write(f'HTTP/1.1 {self.status} Test\r\nContent-Type: application/json\r\nContent-Length: {len(payload)}\r\nConnection: close\r\n\r\n'.encode() + payload)
            await writer.drain()
        except (ConnectionError, asyncio.IncompleteReadError):
            pass
        finally:
            writer.close()
            await writer.wait_closed()
            self.tasks.discard(asyncio.current_task())

    def client(self, cls, venue, **kwargs):
        services = data(venue, 'schema/services.json')
        c = getattr(sdk, cls)(endpoints={s['name']: self.url for s in services}, **kwargs)
        self.clients.append(c)
        return c

    async def test_every_published_method_and_public_fixture(self):
        """Exercise every public binding with the same pinned native fixture as Rust."""
        count = 0
        for venue in VENUES:
            rows = data(venue, 'coverage.json')['rest']
            fixtures = data(venue, 'tests/fixtures/contracts.json')
            clients = {cls: self.client(cls, venue) for cls in dict.fromkeys(r['client'] for r in rows)}
            for row in rows:
                with self.subTest(venue=venue, method=row['method']):
                    method = getattr(clients[row['client']], row['method'])
                    self.assertTrue(callable(method))
                    if row['auth'] != 'none':
                        continue
                    fixture = fixtures[row['service'] + '.' + row['method']]
                    self.payload = fixture['response']
                    await method(fixture['request'])
                    self.assertEqual(self.requests[-1][0][0], row['verb'])
                    self.assertNotIn('authorization', self.requests[-1][1])
                    count += 1
        self.assertGreater(count, 200)

    async def test_exact_numbers_nulls_unknown_fields_and_query_encoding(self):
        c = self.client('PascalClient', 'pascal')
        self.payload = data('pascal', 'tests/fixtures/contracts.json')['read.server_time']['response']
        self.payload.update(precise=123456789012345678901234567890, nullable=None)
        # Numeric strings remain strings; native JSON integers remain integers.
        result = await c.server_time()
        self.assertEqual(result['precise'], self.payload['precise'])
        self.assertIsNone(result['nullable'])
        self.assertIsInstance(result['data']['server_time_ms'], str)
        self.payload = data('kalshi', 'tests/fixtures/contracts.json')['predictions.get_markets']['response']
        self.payload['exact_unknown'] = {'a': None, 'b': 123456789012345678901234567890}
        k = self.client('KalshiClient', 'kalshi')
        result = await k.get_markets({'cursor': 'a/b +?&', 'limit': 2})
        self.assertEqual(result['exact_unknown'], self.payload['exact_unknown'])
        self.assertEqual(parse_qs(urlsplit(self.requests[-1][0][1]).query)['cursor'], ['a/b +?&'])
        for value in [0.1, float('nan'), {'nested': [0.2]}]:
            with self.assertRaises((sdk.ValidationError, TypeError, ValueError)):
                await k.get_markets({'limit': value})

    async def test_kalshi_auth_error_and_single_attempt_write(self):
        vector = data('kalshi', 'tests/fixtures/signing.json')[0]
        signer = sdk.KalshiSigner(vector['private_pem'])
        c = self.client('KalshiClient', 'kalshi', key_id='offline-key', signer=signer, timeout_ms=250)
        self.payload = data('kalshi', 'tests/fixtures/contracts.json')['predictions.get_balance']['response']
        await c.get_balance()
        headers = self.requests[-1][1]
        self.assertEqual(headers['kalshi-access-key'], 'offline-key')
        self.assertEqual(len(base64.b64decode(headers['kalshi-access-signature'])), 64)
        self.status, self.payload = 403, {'error': {'code': 'permission_denied', 'message': 'offline role test'}}
        with self.assertRaises(sdk.ExchangeError) as error:
            await c.get_balance()
        self.assertEqual(error.exception.code, 'permission_denied')
        self.assertEqual(error.exception.http_status, 403)
        self.status, self.hang = 200, True
        request = data('kalshi', 'tests/fixtures/contracts.json')['predictions.create_order_v2']['request']
        before = len(self.requests)
        with self.assertRaises(sdk.TransportError) as timeout:
            await c.create_order_v2(request)
        self.assertEqual(timeout.exception.outcome, 'unknown')
        self.assertEqual(len(self.requests), before + 1)

    async def test_polymarket_typed_data_and_pascal_permit_reference(self):
        from aux_py import polymarket
        signer = sdk.PolymarketSigner('0x' + '01'.zfill(64))
        rows = data('polymarket', 'tests/fixtures/signing.json')
        # Fixtures identify their private key explicitly; use the matching local signer.
        for row in rows:
            if 'typed_data' in row:
                local = sdk.PolymarketSigner('07' * 32)
                signature = await local.sign_typed_data(row['typed_data'])
                self.assertEqual(signature.lower(), row['signature'].lower())
                if 'order_signature' in row:
                    td = row['typed_data']
                    order = td['message']['contents'] if td['primaryType'] == 'TypedDataSign' else td['message']
                    signed_order = await local.sign_order(order, system='ctf' if td['domain']['version'] == '2' else 'v2')
                    self.assertEqual(signed_order, row['order_signature'])
        owner = 'GmaDrppBC7P5ARKV8g3djiwP89vz1jLK23V2GBjuAEGB'
        row = data('pascal', 'tests/fixtures/signing.json')[1]
        p = sdk.PascalSigner(row['private_key_hex'])
        signed = await p.sign_request(owner, 3, 'cancel_orders', {'client_order_id': '42'}, client_ts_ms=1731536000000)
        self.assertEqual(signed['auth']['signature'], row['signature'])
        self.assertEqual(polymarket.market_asset_id({'version': 'v2', 'outcomes': ['Y', 'N'], 'positionIds': ['123', '456']}, 'N'), '456')
        self.assertTrue(signer.address.startswith('0x'))

    async def test_remote_signer_context_cancellation_and_cleanup(self):
        marker = contextvars.ContextVar('signer_marker', default=None)
        entered = asyncio.Event()
        finished = asyncio.Event()
        row = data('kalshi', 'tests/fixtures/signing.json')[0]
        async def sign(payload):
            self.assertEqual(marker.get(), 'originating-context')
            self.assertIn('message_base64', payload)
            entered.set()
            try:
                await asyncio.Event().wait()
            finally:
                finished.set()
        signer = sdk.KalshiSigner.remote(row['public_pem'], sign)
        c = self.client('KalshiClient', 'kalshi', key_id='offline', signer=signer)
        marker.set('originating-context')
        task = asyncio.create_task(c.get_balance())
        await asyncio.wait_for(entered.wait(), 3)
        task.cancel()
        with self.assertRaises(asyncio.CancelledError):
            await task
        await asyncio.wait_for(finished.wait(), 3)
        self.assertEqual(self.requests, [])
        await c.aclose()
        self.assertTrue(c.closed)
        with self.assertRaises(sdk.ClosedError):
            await c.get_exchange_status()

    async def test_contract_builders_and_solana_bytes_match_independent_fixtures(self):
        from aux_py import polymarket
        import re
        for row in data('polymarket', 'tests/fixtures/contracts-abi.json'):
            name = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', row['function']).lower()
            builder = getattr(getattr(polymarket, row['interface']), name)
            self.assertEqual(builder(*row['args']), row['calldata'])
        row = data('pascal', 'tests/fixtures/solana.json')
        p = sdk.PascalChainClient(rpc_url=self.url, mint=row['mint'], signer=sdk.PascalSigner('07' * 32))
        self.clients.append(p)
        prepared = dict(message=list(bytes.fromhex(row['message_hex'])), owner=row['owner'], source=row['source'],
                        destination=row['destination'], mint=row['mint'], recent_blockhash=row['blockhash'],
                        last_valid_block_height=999, amount=25_000_000, decimals=6)
        signed = await p.sign_deposit(prepared)
        self.assertEqual(signed['signature'], row['signature'])
        self.assertEqual(signed['transaction_base64'], row['transaction_base64'])
        prepared['amount'] += 1
        with self.assertRaises(sdk.SigningError):
            await p.sign_deposit(prepared)

    async def test_native_streams_exact_values_and_parent_cleanup(self):
        from websockets.asyncio.server import serve
        vector = data('kalshi', 'tests/fixtures/signing.json')[0]
        configurations = [
            ('KalshiClient', 'kalshi', 'predictions', {'channel': 'ticker'},
             {'key_id': 'offline', 'signer': sdk.KalshiSigner(vector['private_pem'])}, 'ticker'),
            ('PolymarketClient', 'polymarket', 'rtds', {'topic': 'crypto_prices', 'type': 'update'}, {}, 'update'),
            ('PascalClient', 'pascal', 'read', {'channel': 'trades', 'symbol': 'TEST'}, {}, 'update'),
        ]
        for cls, venue, service, params, credentials, event_type in configurations:
            closed = asyncio.Event()
            async def handler(socket):
                request = json.loads(await socket.recv())
                if venue == 'kalshi':
                    self.assertIn('kalshi-access-signature', socket.request.headers)
                    await socket.send(json.dumps({'type': 'subscribed', 'id': request['id'], 'msg': {'sid': 1}}))
                await socket.send('{"type":"' + event_type + '","sid":1,"seq":1,"exact":0.12345678901234567890123456789,"large":123456789012345678901234567890}')
                await socket.wait_closed()
                closed.set()
            async with serve(handler, '127.0.0.1', 0) as server:
                url = f'ws://127.0.0.1:{server.sockets[0].getsockname()[1]}/stream'
                c = self.client(cls, venue, websocket={service: url}, **credentials)
                sub = await c.subscribe({'service': service, 'params': params})
                while True:
                    event = await asyncio.wait_for(anext(sub), 3)
                    if 'exact' in event:
                        break
                self.assertEqual(event['exact'], Decimal('0.12345678901234567890123456789'))
                self.assertEqual(event['large'], 123456789012345678901234567890)
                waiting = asyncio.create_task(anext(sub))
                await asyncio.sleep(0)
                await c.aclose()
                with self.assertRaises((sdk.ClosedError, StopAsyncIteration)):
                    await asyncio.wait_for(waiting, 3)
                await asyncio.wait_for(closed.wait(), 3)
                self.assertTrue(sub.closed)

    async def test_cancelled_pascal_command_retains_pending_without_replay(self):
        from websockets.asyncio.server import serve
        entered, release = asyncio.Event(), asyncio.Event()
        received = []
        async def handler(socket):
            request = json.loads(await socket.recv())
            received.append(request)
            entered.set()
            await asyncio.wait_for(release.wait(), 5)
            await socket.send(json.dumps({'id': request['id'], 'type': 'place', 'payload': [
                {'status': 'success', 'data': {'order': {'id': '42'}}},
                {'status': 'error', 'error': {'code': 'insufficient_collateral'}}]}))
            await socket.wait_closed()
        async with serve(handler, '127.0.0.1', 0) as server:
            url = f'ws://127.0.0.1:{server.sockets[0].getsockname()[1]}'
            c = self.client('PascalClient', 'pascal', websocket={'write': url},
                            owner='GmaDrppBC7P5ARKV8g3djiwP89vz1jLK23V2GBjuAEGB', signer=sdk.PascalSigner('08' * 32))
            session = await c.open_rpc()
            orders = [{'client_order_id': '42', 'symbol': 'TEST', 'side': 'BID', 'price': '0.550000', 'size': '10'}]
            try:
                task = asyncio.create_task(session.place_orders(orders))
                await asyncio.wait_for(entered.wait(), 3)
                task.cancel()
                with self.assertRaises(asyncio.CancelledError):
                    await task
                for _ in range(100):
                    try:
                        pending = session.pending
                        break
                    except sdk.BusyError:
                        await asyncio.sleep(0.001)
                else:
                    self.fail('cancelled command did not release the session')
                self.assertIsNotNone(pending)
                with self.assertRaises(sdk.OutcomeUncertainError):
                    await session.place_orders(orders)
                self.assertEqual(len(received), 1)
                release.set()
                result = await session.reconcile()
                self.assertEqual(result['payload'][1]['status'], 'error')
                self.assertIsNone(session.pending)
            finally:
                release.set()
                await session.aclose()

    async def test_rfq_safe_wallet_identity_and_quote_acknowledgements(self):
        from websockets.asyncio.server import serve
        received = []
        async def handler(socket):
            auth = json.loads(await socket.recv())
            received.append(auth)
            self.assertEqual(auth['identity']['signature_type'], 2)
            await socket.send(json.dumps({'type': 'auth', 'success': True}))
            request = json.loads(await socket.recv())
            received.append(request)
            self.assertNotIn('id', request)
            for quote_id in ['unrelated', request['quote_id']]:
                await socket.send(json.dumps({'type': 'ACK_RFQ_QUOTE_CANCEL', 'rfq_id': request['rfq_id'], 'quote_id': quote_id}))
            await socket.wait_closed()
        async with serve(handler, '127.0.0.1', 0) as server:
            url = f'ws://127.0.0.1:{server.sockets[0].getsockname()[1]}'
            c = self.client('PolymarketClient', 'polymarket', websocket={'rfq': url},
                wallet='0x1111111111111111111111111111111111111111', signer=sdk.PolymarketSigner('07' * 32),
                api_key='public-key', api_secret='cHVibGljLXNlY3JldA==', api_passphrase='public-pass', rfq_signature_type=2)
            async with await c.open_rpc() as session:
                result = await asyncio.wait_for(session.cancel_quote({'rfq_id': 'r1', 'quote_id': 'q1'}), 3)
                self.assertEqual(result['quote_id'], 'q1')
                self.assertEqual((await session.next_event())['quote_id'], 'unrelated')
                self.assertIsNone(session.pending)
            self.assertEqual(len(received), 2)

if __name__ == '__main__':
    unittest.main()

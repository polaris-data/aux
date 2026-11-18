"""Oracle SDK contracts against local servers and checked-in protocol fixtures."""
import asyncio
from decimal import Decimal
import hashlib
import hmac
import json
from pathlib import Path
import unittest
from urllib.parse import parse_qs

import aux_py
import test_data
import websockets

DATA = Path(__file__).resolve().parents[2] / 'data'


def fixtures(vendor, name='operations'):
    return json.loads((DATA / vendor / 'tests' / 'fixtures' / f'{name}.json').read_text())


class OracleTests(unittest.IsolatedAsyncioTestCase):
    asyncSetUp = test_data.DataTests.asyncSetUp
    asyncTearDown = test_data.DataTests.asyncTearDown
    http = test_data.DataTests.http

    async def test_oracle_http_inventory(self):
        configs = {
            'PythProClient': dict(api_url=self.base+'/v1', router_url=self.base, api_key='fixture-key', publisher_token='publisher-key'),
            'PythHermesClient': dict(hermes_url=self.base+'/hermes', api_key='fixture-key'),
            'ChainlinkDataStreamsClient': dict(network='testnet', api_url=self.base, api_key='fixture-key', api_secret='fixture-secret'),
            'ChainlinkCandlestickClient': dict(network='testnet', candles_url=self.base, token='fixture-token'),
        }
        for row in fixtures('pyth') + fixtures('chainlink'):
            if row['kind'].endswith('_ws'):
                continue
            with self.subTest(client=row['client'], method=row['method']):
                self.routes[row['verb'], row['path']] = row['status'], row['headers'], row['response'].encode()
                async with getattr(aux_py, row['client'])(configs[row['client']]) as client:
                    method = getattr(client, row['method'])
                    args = [row['request']] if row['request'] else []
                    if row['status'] == 400:
                        with self.assertRaises(aux_py.DataVendorError):
                            await method(*args)
                        continue
                    result = await method(*args)
                    if row['kind'] in ('objects', 'sse'):
                        async with result:
                            event = await anext(result)
                            if row['kind'] == 'sse':
                                self.assertEqual(event['binary']['data'], ['abcd'])
                            else:
                                self.assertEqual(event['heartbeat'], 1748525528)
                                self.assertEqual((await anext(result))['p'], Decimal('1.0000000000000000001e+21'))
                    else:
                        expected = row['response'] if row['kind'] == 'text' else json.loads(row['response'], parse_float=Decimal)
                        self.assertEqual(result['data'], expected)
                        self.assertEqual(result['status'], row['status'])
                    method, url, headers, body = self.requests[-1]
                    self.assertEqual(method, row['verb'])
                    self.assertEqual(url.path, row['path'])
                    self.assertEqual(parse_qs(url.query), row['query'])
                    if row['kind'] == 'post':
                        self.assertEqual(json.loads(body), row['body'])
                    if row['kind'] == 'form':
                        self.assertEqual(parse_qs(body.decode()), {k:[v] for k,v in row['body'].items()})
                        self.assertNotIn('authorization', headers)
                    if row['method'] == 'publisher_updates':
                        self.assertEqual(headers['authorization'], 'Bearer publisher-key')
                    if row['client'] == 'ChainlinkDataStreamsClient':
                        signed = f'{method} {url.geturl()} {hashlib.sha256(body).hexdigest()} fixture-key {headers["x-authorization-timestamp"]}'
                        self.assertEqual(headers['x-authorization-signature-sha256'], hmac.new(b'fixture-secret', signed.encode(), 'sha256').hexdigest())

    async def test_pyth_redundant_websockets(self):
        connections = []
        fixture = next(row for row in fixtures('pyth') if row['kind'] == 'pyth_ws')

        async def serve(ws):
            self.assertEqual(ws.request.headers['Authorization'], 'Bearer fixture-key')
            request = json.loads(await ws.recv())
            self.assertEqual(request, {'type': 'subscribe', **fixture['request']})
            connections.append(ws)
            base = dict(type='streamUpdated', subscriptionId=1, parsed={'timestampUs':'1700000000000001'})
            await ws.send(json.dumps({**base,'price': 1}))
            await ws.send(json.dumps({**base,'price': 2}))
            self.assertEqual(json.loads(await ws.recv()), {'type':'unsubscribe','subscriptionId':1})

        servers = [await websockets.serve(serve, '127.0.0.1', 0) for _ in range(3)]
        try:
            urls = ['ws://127.0.0.1:'+str(server.sockets[0].getsockname()[1]) for server in servers]
            async with aux_py.PythProClient(dict(api_key='fixture-key', ws_urls=urls)) as client:
                async with await client.subscribe(fixture['request']) as stream:
                    self.assertEqual((await asyncio.wait_for(anext(stream), 2))['price'], 1)
                    self.assertEqual((await asyncio.wait_for(anext(stream), 2))['price'], 2)
                    with self.assertRaises(asyncio.TimeoutError):
                        await asyncio.wait_for(anext(stream), 0.05)
                self.assertEqual(len(connections), 3)
        finally:
            for server in servers:
                server.close()
            await asyncio.gather(*(server.wait_closed() for server in servers))

    async def test_report_decoding(self):
        async with aux_py.ChainlinkDataStreamsClient(dict(network='testnet')) as client:
            for row in fixtures('chainlink', 'reports'):
                decoded = client.decode_report(row['full_report'])
                self.assertEqual(decoded['version'], row['version'])
                self.assertEqual(decoded['report_body'], row['report_body'])
                if row['key_field']:
                    self.assertEqual(decoded['fields'][row['key_field']], row['key_value'])
                else:
                    self.assertEqual(decoded['kind'], 'unknown')

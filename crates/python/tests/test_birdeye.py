"""Birdeye integration contracts; run after building an extension with Birdeye."""
import asyncio
from decimal import Decimal
import json
from pathlib import Path
import unittest
from urllib.parse import parse_qs

import aux_py
import test_data

FIXTURES = Path(__file__).resolve().parents[2] / 'data/birdeye/tests/fixtures/http.json'


@unittest.skipUnless(hasattr(aux_py._native, 'BirdeyeClient'), 'requires a native extension containing Birdeye')
class BirdeyeTests(unittest.IsolatedAsyncioTestCase):
    asyncSetUp = test_data.DataTests.asyncSetUp
    http = test_data.DataTests.http

    async def asyncTearDown(self):
        self.server.close()
        pending = list(self.tasks)
        for task in pending:
            task.cancel()
        await asyncio.gather(*pending, return_exceptions=True)
        await self.server.wait_closed()

    async def test_birdeye_http_inventory_and_precision(self):
        async with aux_py.BirdeyeClient('fixture-secret', chain='base', api_url=self.base) as client:
            for row in json.loads(FIXTURES.read_text()):
                self.routes[row['verb'], row['path']] = (200, {}, row['response_body'].encode())
                result = await getattr(client, row['operation'])(json.loads(row['request_json'], parse_float=Decimal))
                self.assertEqual(result, json.loads(row['response_body'], parse_float=Decimal))
                self.assertEqual(result['data']['value'], Decimal('0.1000000000000000001'))
                self.assertEqual(result['data']['rawAmount'], 123456789012345678901234567890)
                method, url, headers, body = self.requests[-1]
                self.assertEqual(method, row['verb'])
                self.assertEqual(url.path, row['path'])
                self.assertEqual(parse_qs(url.query), {key: [value] for key, value in row['query'].items()})
                self.assertEqual(headers['x-api-key'], 'fixture-secret')
                self.assertEqual(headers['x-chain'], 'base')
                self.assertNotIn('authorization', headers)
                if row['body_json'] is not None:
                    self.assertEqual(headers['content-type'], 'application/json')
                    self.assertEqual(json.loads(body, parse_float=Decimal), json.loads(row['body_json'], parse_float=Decimal))
                else:
                    self.assertEqual(body, b'')
            self.assertEqual(len(self.requests), 46)
        self.assertTrue(client.closed)
        with self.assertRaises(aux_py.ClosedError):
            await client.price({'address': 'TokenA'})

    async def test_birdeye_errors_defaults_and_validation(self):
        route = '/defi/price'
        async with aux_py.BirdeyeClient('fixture-secret', api_url=self.base) as client:
            for status in (200, 400, 401, 403, 422, 429, 500):
                self.routes['GET', route] = (status, {}, b'{"success":false,"message":"fixture-secret rejected","code":"native"}')
                before = len(self.requests)
                with self.assertRaises(aux_py.DataVendorError) as caught:
                    await client.price({'address': 'TokenA'})
                self.assertEqual(caught.exception.http_status, status)
                self.assertEqual(caught.exception.native['code'], 'native')
                self.assertNotIn('fixture-secret', str(caught.exception.native))
                self.assertEqual(len(self.requests), before + 1)
            for data in ('null', '[]', '{}'):
                self.routes['GET', route] = (200, {}, ('{"success":true,"data":'+data+'}').encode())
                await client.price({'address': 'TokenA'})
                self.assertEqual(parse_qs(self.requests[-1][1].query), {'address': ['TokenA']})
            for request in ({'address': 'TokenA', 'check_liquidity': 0.1}, {}, {'address': 'TokenA', 'typo': True}):
                with self.assertRaises(aux_py.ValidationError):
                    await client.price(request)
            self.routes['GET', route] = (200, {}, b'{}')
            with self.assertRaises(aux_py.DecodeError):
                await client.price({'address': 'TokenA'})

    async def test_birdeye_timeout_and_close(self):
        started = asyncio.Event()
        async def pending(_):
            started.set()
            await asyncio.Future()
        self.routes['GET', '/defi/price'] = pending
        async with aux_py.BirdeyeClient('key', api_url=self.base, timeout_ms=50) as client:
            with self.assertRaises(aux_py.TransportError):
                await asyncio.wait_for(client.price({'address': 'TokenA'}), 2)
        client = aux_py.BirdeyeClient('key', api_url=self.base)
        started.clear()
        task = asyncio.create_task(client.price({'address': 'TokenA'}))
        await asyncio.wait_for(started.wait(), 2)
        await client.aclose()
        with self.assertRaises(aux_py.ClosedError):
            await asyncio.wait_for(task, 2)

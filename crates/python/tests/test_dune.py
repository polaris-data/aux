"""Dune contracts through the built Python extension; no real API requests."""
import asyncio
from decimal import Decimal
import json
from pathlib import Path
import unittest
from urllib.parse import parse_qs

import aux_py
import test_data

FIXTURES = Path(__file__).resolve().parents[2] / 'data/dune/tests/fixtures/http.json'


class DuneTests(unittest.IsolatedAsyncioTestCase):
    # Reuse the data-client local HTTP harness without inheriting its test cases.
    asyncSetUp = test_data.DataTests.asyncSetUp
    async def asyncTearDown(self):
        self.server.close()
        pending = list(self.tasks)
        for task in pending:
            task.cancel()
        await asyncio.gather(*pending, return_exceptions=True)
        await self.server.wait_closed()

    http = test_data.DataTests.http

    async def test_dune_http_inventory_and_precision(self):
        fixtures = json.loads(FIXTURES.read_text())
        async with aux_py.DuneClient('fixture-secret', api_url=self.base + '/api/v1') as client:
            for row in fixtures:
                self.routes[row['verb'], row['path']] = (200, row['response_headers'], row['response_body'].encode())
                args = []
                if '/query/' in row['path']:
                    args.append(123)
                elif '/execution/' in row['path']:
                    args.append('exec-1')
                if row['request_json'] is not None:
                    args.append(json.loads(row['request_json'], parse_float=Decimal))
                output = await getattr(client, row['operation'])(*args)
                if row['operation'].endswith('_csv'):
                    self.assertEqual(output['data'], row['response_body'])
                    self.assertEqual(output['next_offset'], 1)
                    self.assertEqual(output['next_uri'], row['response_headers']['X-Dune-Next-Uri'])
                else:
                    self.assertEqual(output, json.loads(row['response_body'], parse_float=Decimal))
                if row['operation'] in ('get_execution_results', 'get_latest_query_results'):
                    result_row = output['result']['rows'][0]
                    self.assertEqual(result_row['amount'], Decimal('0.1000000000000000001'))
                    self.assertEqual(result_row['id'], 9007199254740993)
                    self.assertEqual(result_row['nested']['x'], 123456789012345678901234567890)
                method, url, headers, body = self.requests[-1]
                self.assertEqual(method, row['verb'])
                self.assertEqual(url.path, row['path'])
                self.assertEqual(headers['x-dune-api-key'], 'fixture-secret')
                self.assertNotIn('authorization', headers)
                self.assertNotIn('fixture-secret', url.query)
                expected_query = {k: [str(v).lower() if isinstance(v, bool) else str(v)] for k, v in row['query'].items()}
                self.assertEqual(parse_qs(url.query), expected_query)
                if row['verb'] == 'POST' and row['request_json'] is not None:
                    self.assertEqual(headers['content-type'], 'application/json')
                    self.assertEqual(json.loads(body, parse_float=Decimal), json.loads(row['request_json'], parse_float=Decimal))
                else:
                    self.assertEqual(body, b'')
            self.assertEqual(len(self.requests), 9)
        self.assertTrue(client.closed)
        with self.assertRaises(aux_py.ClosedError):
            await client.get_execution_status('exec-1')

    async def test_dune_errors_defaults_and_validation(self):
        route = '/api/v1/query/123/execute'
        self.routes['POST', route] = (200, {}, b'{"execution_id":"x","state":"QUERY_STATE_PENDING"}')
        async with aux_py.DuneClient('fixture-secret', api_url=self.base + '/api/v1') as client:
            await client.execute_query(123)
            self.assertEqual(self.requests[-1][3], b'{}')
            for invalid in ({'query_parameters': {'value': 0.1}}, {'query_parameters': {'value': Decimal('NaN')}}, {'query_parameters': {'value': Decimal('Infinity')}}, {'performance': 'invalid'}):
                with self.assertRaises(aux_py.ValidationError):
                    await client.execute_query(123, invalid)
            with self.assertRaises(aux_py.ValidationError):
                await client.get_latest_query_results(123, {'sample_count': 1, 'offset': 0})
            for status, kind in [(401, 'authentication'), (402, 'vendor'), (403, 'entitlement'), (429, 'rate_limit'), (500, 'vendor')]:
                self.routes['POST', route] = (status, {}, b'{"error":"fixture-secret rejected","nested":["fixture-secret"]}')
                before = len(self.requests)
                with self.assertRaises(aux_py.DataVendorError) as caught:
                    await client.execute_query(123)
                self.assertEqual(caught.exception.http_status, status)
                self.assertEqual(caught.exception.kind, kind)
                self.assertNotIn('fixture-secret', str(caught.exception))
                self.assertNotIn('fixture-secret', str(caught.exception.native))
                self.assertEqual(len(self.requests), before + 1)
            self.routes['GET', '/api/v1/execution/x/status'] = (200, {}, b'{}')
            with self.assertRaises(aux_py.DecodeError):
                await client.get_execution_status('x')
        for config in ({'api_key': ''}, {'api_key': 'key', 'timeout_ms': 0}):
            with self.assertRaises(aux_py.ValidationError):
                aux_py.DuneClient(**config)

    async def test_dune_close_and_cancellation(self):
        started = asyncio.Event()
        async def pending(_):
            started.set()
            await asyncio.Future()
        self.routes['GET', '/api/v1/execution/x/status'] = pending
        client = aux_py.DuneClient('key', api_url=self.base + '/api/v1')
        task = asyncio.create_task(client.get_execution_status('x'))
        await asyncio.wait_for(started.wait(), 2)
        await client.aclose()
        with self.assertRaises(aux_py.ClosedError):
            await asyncio.wait_for(task, 2)
        await client.aclose()
        started.clear()
        async with aux_py.DuneClient('key', api_url=self.base + '/api/v1') as client:
            task = asyncio.create_task(client.get_execution_status('x'))
            await asyncio.wait_for(started.wait(), 2)
            task.cancel()
            with self.assertRaises(asyncio.CancelledError):
                await task

    async def test_dune_timeout_is_not_retried(self):
        async def pending(_):
            await asyncio.Future()
        self.routes['GET', '/api/v1/execution/x/status'] = pending
        async with aux_py.DuneClient('key', api_url=self.base + '/api/v1', timeout_ms=50) as client:
            with self.assertRaises(aux_py.TransportError) as caught:
                await asyncio.wait_for(client.get_execution_status('x'), 2)
            self.assertEqual(caught.exception.kind, 'timeout')
            self.assertEqual(len(self.requests), 1)

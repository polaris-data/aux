"""Metric HTTP, EVM and exact codec parity against offline references and local peers."""
from __future__ import annotations
import asyncio
import contextvars
import inspect
import json
import unittest
from decimal import Decimal
from pathlib import Path
from urllib.parse import urlsplit
from aux_py import MetricClient, MetricChainClient, MetricSigner, AuthenticationError, ClosedError, ExchangeError, OutcomeUncertainError, SigningError, ValidationError
from aux_py import metric

BASE = Path(__file__).resolve().parents[2] / 'exchanges/metric'
ADDRESS = '0x1111111111111111111111111111111111111111'
OTHER = '0x2222222222222222222222222222222222222222'

def fixture(name):
    return json.loads((BASE / 'tests/fixtures' / name).read_text())

def normalized(value):
    if isinstance(value, bool):
        return value
    if isinstance(value, int):
        return str(value)
    if isinstance(value, list):
        return [normalized(x) for x in value]
    if isinstance(value, dict):
        return {k: normalized(v) for k, v in value.items()}
    return value


class MetricTests(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.clients = []
        self.tasks = set()
        self.requests = []
        self.errors = []
        self.chain_id = 1
        self.rpc_result = '0x'
        self.rpc_status_row = None
        self.http_status = 200
        self.hang_send = False
        self.send_started = asyncio.Event()
        self.release_send = asyncio.Event()
        self.server = await asyncio.start_server(self.http, '127.0.0.1', 0)
        self.url = f'http://127.0.0.1:{self.server.sockets[0].getsockname()[1]}'

    async def asyncTearDown(self):
        self.release_send.set()
        for client in self.clients:
            await asyncio.wait_for(client.aclose(), 4)
        self.server.close()
        await self.server.wait_closed()
        for task in list(self.tasks):
            task.cancel()
        await asyncio.gather(*list(self.tasks), return_exceptions=True)
        self.assertEqual(self.errors, [])

    async def http(self, reader, writer):
        task = asyncio.current_task()
        self.tasks.add(task)
        try:
            head = (await reader.readuntil(b'\r\n\r\n')).decode()
            lines = head.split('\r\n')
            headers = dict((k.lower(), v.strip()) for line in lines[1:] if ':' in line for k, v in [line.split(':', 1)])
            raw = await reader.readexactly(int(headers.get('content-length', 0)))
            body = json.loads(raw) if raw else None
            path = urlsplit(lines[0].split()[1]).path
            self.requests.append((path, headers, body))
            if body and 'jsonrpc' in body:
                method = body['method']
                if method == 'eth_chainId':
                    result = hex(self.chain_id)
                elif method == 'eth_getCode':
                    result = '0x6000'
                elif method == 'eth_getTransactionCount':
                    result = '0x3'
                elif method == 'eth_blockNumber':
                    result = '0x21'
                elif method == 'eth_call' and self.rpc_status_row is not None:
                    self.assertEqual(body['params'][1], self.rpc_status_row['blockNumber'])
                    call = body['params'][0]
                    match = next(r for r in self.rpc_status_row['calls'] if r['data'] == call['data'] and r['to'].lower() == call['to'].lower())
                    result = match['result']
                elif method == 'eth_sendRawTransaction':
                    self.send_started.set()
                    if self.hang_send:
                        await self.release_send.wait()
                    result = '0x' + '00' * 32
                elif method in ('eth_getTransactionReceipt', 'eth_getTransactionByHash'):
                    result = None
                else:
                    result = self.rpc_result
                payload = {'jsonrpc': '2.0', 'id': body['id'], 'result': result}
            elif path.endswith('/chains'):
                payload = fixture('chains.json')
            elif path.endswith('/factory-settings'):
                payload = fixture('factory_settings.json')
            elif path.endswith('/metadata'):
                payload = fixture('metadata.json')[0]
            elif path.endswith('/bid_ask/batch'):
                payload = fixture('batch_bid_ask.json')[0]
            elif path.endswith('/bid_ask'):
                payload = fixture('bid_ask.json')[0]
            elif path.endswith('/route'):
                payload = fixture('route.json')[0]
            elif path.endswith('/pamm-quote'):
                payload = fixture('pamm_quote.json')[0]
            else:
                raise AssertionError(path)
            if self.http_status != 200:
                payload = {'code': 'denied', 'error': 'test-secret-echo'}
            data = json.dumps(payload).encode()
            writer.write(f'HTTP/1.1 {self.http_status} Response\r\nContent-Length: {len(data)}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n'.encode() + data)
            await writer.drain()
        except (asyncio.IncompleteReadError, ConnectionError):
            pass
        except Exception as error:
            self.errors.append(error)
        finally:
            writer.close()
            try:
                await writer.wait_closed()
            except ConnectionError:
                pass
            self.tasks.discard(task)

    def api(self, **kwargs):
        c = MetricClient(chain_id=1, api_url=self.url, **kwargs)
        self.clients.append(c)
        return c

    def chain(self, **kwargs):
        c = MetricChainClient(chain_id=self.chain_id, read_rpc=self.url, **kwargs)
        self.clients.append(c)
        return c

    async def test_all_eight_http_methods_native_values_and_auth(self):
        c = self.api()
        self.assertTrue((await c.chains())['chains'])
        self.assertNotIn('authorization', self.requests[-1][1])
        self.assertIn('chains', await c.factory_settings())
        metadata = await c.metadata({'count': 1, 'minTvlFiat': 0})
        self.assertIsInstance(metadata['data'][0]['token0FiatPrice'], Decimal)
        self.assertTrue((await c.cross_chain_metadata())['data'])
        self.assertEqual((await c.bid_ask(ADDRESS))['bidAdj'], '49940417631045696435290')
        batch = await c.batch_bid_ask({'pools': [ADDRESS, OTHER]})
        self.assertEqual(batch['results'][1]['error']['status'], 404)
        self.assertEqual((await c.route({'tokenIn': ADDRESS, 'tokenOut': OTHER, 'amountIn': '1'}))['amountOut'], '136120897')
        request = {'tx': '0x', 'blockNumber': 1, 'replacementUuid': '0x' + '00' * 16, 'replacementSeqNumber': 1}
        with self.assertRaises(AuthenticationError):
            await c.pamm_quote(request)
        private = self.api(trading_api_key='public-test-key')
        self.assertIn('providers', await private.pamm_quote(request))
        self.assertEqual(self.requests[-1][1]['authorization'], 'Bearer public-test-key')
        self.http_status = 401
        with self.assertRaises(ExchangeError) as caught:
            await private.chains()
        self.assertNotIn('test-secret-echo', str(caught.exception))
        self.assertEqual(caught.exception.http_status, 401)
        with self.assertRaises(ValidationError):
            await c.route({'tokenIn': ADDRESS, 'tokenOut': OTHER, 'amountIn': 1.1})
        await c.aclose()
        with self.assertRaises(ClosedError):
            await c.chains()

    async def test_all_abi_vectors_and_named_contract_interfaces(self):
        c = self.chain()
        inventory = {(r['contract'], r['kind'], r['signature']): r for r in json.loads((BASE / 'contracts.json').read_text())}
        for row in fixture('abi.json'):
            with self.subTest(contract=row['contract'], signature=row['signature']):
                name = row['contract']
                sig = row['signature']
                if row['kind'] == 'function':
                    self.assertEqual(c.encode_call(name, sig, row['args']), row['input'])
                    self.assertEqual(normalized(c.decode_input(name, sig, row['input'])), row['args'])
                    self.assertEqual(normalized(c.decode_output(name, sig, row['output'])), row['outputArgs'])
                    method = inventory[(name, 'function', sig)]['python'].split('.')[1]
                    operation = getattr(c.contract(name, ADDRESS), method)
                    if inspect.iscoroutinefunction(operation):
                        self.rpc_result = row['output']
                        self.assertEqual(normalized(await operation(row['args'])), row['outputArgs'])
                    else:
                        self.assertEqual(operation(row['args'])['data'], row['input'])
                elif row['kind'] == 'event':
                    decoded = c.decode_event(name, sig, row['topics'], row['data'])
                    self.assertEqual(normalized(decoded['indexed']), row['indexed'])
                    self.assertEqual(normalized(decoded['body']), row['body'])
                else:
                    self.assertEqual(normalized(c.decode_error(name, row['input'])['arguments']), row['args'])

    async def test_local_and_remote_signers_use_same_reference_transactions(self):
        rows = fixture('signing.json')
        context = contextvars.ContextVar('metric-context')
        for row in rows:
            local = MetricSigner(row['key'])
            self.assertEqual(local.address.lower(), row['address'])
            signed = await local.sign_transaction(row['prepared'])
            self.assertEqual(signed['bytes'], row['signed'])
            self.assertEqual(signed['hash'], row['hash'])
            context.set('originating-task')
            async def callback(payload):
                self.assertEqual(context.get(), 'originating-task')
                return await local.sign_digest(payload['digest'])
            remote = MetricSigner.remote(local.address, callback)
            self.assertEqual((await remote.sign_transaction(row['prepared']))['bytes'], row['signed'])
            self.assertNotIn(row['key'], repr(local))
        request = {**rows[0]['prepared'], 'value': 2**200, 'maxFeePerGas': 2**80}
        integer_signed = await local.sign_transaction(request)
        self.assertEqual(integer_signed, await local.sign_transaction({**request, 'value': hex(2**200)}))
        other = MetricSigner('0x' + '0' * 63 + '2')
        async def wrong_signature(payload):
            return await other.sign_digest(payload['digest'])
        wrong = MetricSigner.remote(rows[0]['address'], wrong_signature)
        with self.assertRaises(SigningError):
            await wrong.sign_transaction(rows[0]['prepared'])

    async def test_signer_cancellation_reaches_callback(self):
        started, cancelled = asyncio.Event(), asyncio.Event()
        async def callback(_):
            started.set()
            try:
                await asyncio.Event().wait()
            finally:
                cancelled.set()
        row = fixture('signing.json')[0]
        signer = MetricSigner.remote(row['address'], callback)
        future = asyncio.ensure_future(signer.sign_transaction(row['prepared']))
        await asyncio.wait_for(started.wait(), 3)
        future.cancel()
        with self.assertRaises(asyncio.CancelledError):
            await future
        await asyncio.wait_for(cancelled.wait(), 3)

    async def test_cancelled_submission_keeps_pending_identity(self):
        self.chain_id = 778001
        self.hang_send = True
        row = fixture('signing.json')[0]
        signer = MetricSigner(row['key'])
        c = self.chain(signer=signer)
        request = {**row['prepared'], 'chainId': self.chain_id}
        future = asyncio.ensure_future(c.send_transaction(request))
        await asyncio.wait_for(self.send_started.wait(), 3)
        future.cancel()
        with self.assertRaises(asyncio.CancelledError):
            await future
        for _ in range(100):
            pending = c.pending(signer.address)
            if pending:
                break
            await asyncio.sleep(0.01)
        self.assertIsNotNone(pending)
        with self.assertRaises(OutcomeUncertainError):
            await c.send_transaction(request)
        sends = [r for r in self.requests if r[2] and r[2].get('method') == 'eth_sendRawTransaction']
        self.assertEqual(len(sends), 1)
        self.assertFalse(await c.reconcile(signer.address))

    async def test_invalid_submission_hash_is_explicitly_uncertain(self):
        self.chain_id = 778002
        row = fixture('signing.json')[0]
        signer = MetricSigner(row['key'])
        c = self.chain(signer=signer)
        with self.assertRaises(OutcomeUncertainError) as error:
            await c.send_transaction({**row['prepared'], 'chainId': self.chain_id})
        self.assertEqual(error.exception.outcome, 'unknown')
        self.assertEqual(c.pending(signer.address)['hash'], error.exception.hash)
        self.assertIsInstance(error.exception.cause, ValidationError)

    async def test_helpers_precision_and_native_payment(self):
        data = fixture('math.json')
        self.assertEqual(metric.pack_bins(data['bins']), data['packed'])
        self.assertEqual(metric.decimal_to_raw({'value': '1.230000', 'decimals': 6}), 1230000)
        with self.assertRaises(ValidationError):
            metric.decimal_to_raw({'value': '1.0000001', 'decimals': 6})
        with self.assertRaises(ValidationError):
            metric.price_to_q64({'numerator': 1, 'denominator': 10})
        self.assertEqual(metric.price_to_q64({'numerator': 1, 'denominator': 10, 'rounding': 'down'}), (1 << 64) // 10)
        c = self.chain()
        wrapped = '0x3333333333333333333333333333333333333333'
        router = '0x4444444444444444444444444444444444444444'
        params = [ADDRESS, wrapped, OTHER, True, 100, 90, ADDRESS, 100000, 0, '0x']
        call = c.prepare_call('MetricOmmSimpleRouter', 'exactInputSingle', router, [params])
        native = metric.native_payment({'call': call, 'value': 50, 'wrappedNative': wrapped})
        self.assertEqual(native['value'], '0x32')
        calls = c.decode_input('MetricOmmSimpleRouter', 'multicall', native['data'])[0]
        self.assertEqual(calls[0], call['data'])
        self.assertEqual(len(calls), 2)
        with self.assertRaises(ValidationError):
            metric.native_payment({'call': call, 'value': 50, 'wrappedNative': OTHER})
        oracle = fixture('oracle.json')
        self.assertEqual(metric.pack_oracle_slot({'lanes': oracle['lanes'], 'timestampMs': oracle['timestampMs'], 'slotId': oracle['slotId'], 'rounding': 'down'}), int(oracle['word'], 16))
        self.assertEqual(metric.oracle_push_calldata({'words': [int(oracle['word'], 16)]}), '0x00' + oracle['word'][2:].zfill(64))
        self.assertEqual(metric.slot_update_digest({'chainId': 1, 'oracle': oracle['oracle'], 'creator': oracle['creator'], 'word': oracle['word']}), oracle['updateDigest'])
        self.assertEqual(await MetricSigner(fixture('signing.json')[0]['key']).sign_personal_digest(oracle['updateDigest']), oracle['signature'])

    async def test_liquidity_plans_match_pinned_sdk(self):
        for row in fixture('liquidity.json'):
            self.assertEqual(metric.addition_plan(row['request']), row['expected'])
            self.assertEqual(metric.weighted_plan(row['weightedRequest']), row['weightedExpected'])
        with self.assertRaises(ValidationError):
            row = fixture('liquidity.json')[0]['request']
            metric.addition_plan({**row, 'targets': row['targets'] * 2})

    async def test_sdk_workflow_helpers_match_reference_and_do_not_submit(self):
        data = fixture('helpers.json')
        helpers = {
            'poolCreation': metric.prepare_create_pool,
            'uniformBins': metric.create_uniform_bins,
            'liquidity': metric.prepare_liquidity_addition,
            'fallbackSwaps': metric.prepare_swap_with_fallback,
            'externalSwaps': metric.prepare_external_swap,
            'stopLoss': metric.evaluate_oracle_stop_loss,
        }
        for group, helper in helpers.items():
            for i, row in enumerate(data[group]):
                with self.subTest(group=group, case=i):
                    self.assertEqual(helper(row['request']), row['expected'])
        for row in data['poolCreation']:
            self.assertEqual(metric.prepare_factory_create_pool_params(row['request']['params']), row['expected']['params'])
        for row in data['liquidity']:
            if 'nativeValue' in row['request']:
                self.assertEqual(metric.liquidity_native_payment({'call': row['baseCall'], 'value': 50}), row['expected'])
        self.assertEqual(self.requests, [])
        request = data['poolCreation'][0]['request']
        for percentage in ['838.8608', '-838.8609', '0.00001']:
            with self.assertRaises(ValidationError):
                metric.prepare_create_pool({**request, 'params': {**request['params'], 'curBinDistFromProvidedPricePercent': percentage}})
        with self.assertRaises(ValidationError):
            metric.prepare_create_pool({**request, 'params': {**request['params'], 'curBinDistFromProvidedPricePercent': 0.01}})
        # The digest/ABI path accepts large Python integers with no float conversion.
        row = data['stopLoss'][5]
        converted = {**row['request'], 'bidPriceX64': int(row['request']['bidPriceX64'], 16), 'askPriceX64': int(row['request']['askPriceX64'], 16)}
        self.assertEqual(metric.evaluate_oracle_stop_loss(converted), row['expected'])

    async def test_aggregate_stop_loss_reads_native_state_at_one_block(self):
        row = fixture('helpers.json')['stopLossStatus'][0]
        self.rpc_status_row = row
        c = self.chain()
        request = {**row['request'], 'bidPriceX64': int(row['request']['bidPriceX64'], 16), 'askPriceX64': int(row['request']['askPriceX64'], 16)}
        self.assertEqual(await c.oracle_stop_loss_status(request), row['expected'])
        self.assertEqual(len(self.requests), 8)
        self.assertTrue(all(r[2]['method'] in ('eth_blockNumber', 'eth_call') for r in self.requests))
        with self.assertRaises(ValidationError):
            await c.oracle_stop_loss_status({**request, 'maxBins': 2})

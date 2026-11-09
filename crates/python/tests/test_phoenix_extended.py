import asyncio
from copy import deepcopy
from decimal import Decimal
import json
from pathlib import Path
import unittest

import aux_py as aux
from test_derive_paradex import HttpPeer

ROOT=Path(__file__).resolve().parents[3]

def fixtures(venue):
    return json.loads((ROOT/'crates/exchanges'/venue/'tests/fixtures/contracts.json').read_text())

class PhoenixExtendedTests(unittest.IsolatedAsyncioTestCase):
    async def test_all_native_rest_operations_and_cleanup(self):
        for venue,cls in [('phoenix',aux.PhoenixClient),('extended',aux.ExtendedClient)]:
            peer=HttpPeer()
            peer.server.RequestHandlerClass.do_PATCH=peer.server.RequestHandlerClass.do_POST
            client=cls(http_url=peer.url,chain_id='SN_SEPOLIA' if venue=='extended' else 'mainnet-beta',api_key='fixture',access_token='fixture')
            try:
                inventory=json.loads((ROOT/'crates/exchanges'/venue/'coverage.json').read_text())['rest']
                contracts=fixtures(venue)
                async with client:
                    for op in inventory:
                        with self.subTest(venue=venue,method=op['method']):
                            contract=deepcopy(contracts[op['method']])
                            peer.expected.append({**contract,'verb':op['verb'],'private':op['private']})
                            result=await getattr(client,op['method'])(contract['request'])
                            if isinstance(contract['response'],dict) and 'future_field' in contract['response']:
                                self.assertEqual(result['future_field'],contract['response']['future_field'])
                self.assertTrue(client.closed)
            finally:
                await client.aclose()
                await asyncio.to_thread(peer.close)

    async def test_exact_values_and_request_float_rejection(self):
        peer=HttpPeer()
        client=aux.PhoenixClient(http_url=peer.url,chain_id='mainnet-beta')
        try:
            contract=fixtures('phoenix')['get_exchange']
            raw=json.dumps(contract['response'])[:-1]+',"native_decimal":0.123456789012345678901,"large_id":184467440737095516170}'
            peer.expected.append({**contract,'verb':'get','raw':raw.encode()})
            result=await client.get_exchange()
            self.assertEqual(result['native_decimal'],Decimal('0.123456789012345678901'))
            self.assertEqual(result['large_id'],184467440737095516170)
            with self.assertRaises(aux.ValidationError):
                await client.get_exchange({'body':{'price':0.1}})
        finally:
            await client.aclose()
            await asyncio.to_thread(peer.close)

    async def test_local_and_remote_message_signers(self):
        signer=aux.PhoenixSigner('01'*32)
        async def callback(payload):
            self.assertEqual(payload['scheme'],'phoenix_ed25519')
            return await signer.sign_message(payload['message_base64'])
        remote=aux.PhoenixSigner.remote(signer.public_key,callback)
        self.assertEqual(await signer.sign_message('aGVsbG8='),await remote.sign_message('aGVsbG8='))
        async def invalid(_):
            return '1'*64
        bad=aux.PhoenixSigner.remote(signer.public_key,invalid)
        with self.assertRaises(aux.SigningError):
            await bad.sign_message('aGVsbG8=')

    async def test_extended_preparation_uses_native_rust_hash(self):
        async with aux.ExtendedClient() as client:
            signer=aux.ExtendedSigner('0x1')
            order={'market':'BTC-USD','side':'BUY','qty':'0.001','price':'100','fee':'0.0005','nonce':'7','expiryEpochMillis':1705626537000}
            context={'synthetic_asset_id':'0x2','collateral_asset_id':'0x1','synthetic_resolution':1000000,'collateral_resolution':1000000,'collateral_position':10002}
            prepared=client.prepare_order(signer.public_key,order,context)
            signed=await client.sign_order(signer,order,context)
            self.assertEqual(signed['id'],str(int(prepared['hash'],16)))
            self.assertTrue(signed['settlement']['signature']['r'].startswith('0x'))

    async def test_wallet_derivation_and_versioned_transaction_helpers(self):
        signature='0x9ef64d5936681edf44b4a7ad713f3bc24065d4039562af03fccf6a08d6996eab367df11439169b417b6a6d8ce81d409edb022597ce193916757c7d5d9cbf97301c'
        derived=aux.ExtendedSigner.from_wallet_signature(signature)
        self.assertEqual(derived.public_key,'0x78298687996aff29a0bbcb994e1305db082d084f85ec38bb78c41e6787740ec')
        self.assertTrue((await derived.sign_hash('0x1'))['r'].startswith('0x'))
        async with aux.PhoenixClient() as client:
            signer=aux.PhoenixSigner('01'*32)
            prepared=client.prepare_transaction({'version':'v0','instructions':[{'program_id':'11111111111111111111111111111111','accounts':[],'data':[1]}],'fee_payer':signer.public_key,'recent_blockhash':'11111111111111111111111111111111','last_valid_block_height':500,'lookup_tables':[]})
            signatures=await signer.sign_transaction(prepared)
            signed=client.assemble_transaction(prepared,signatures)
            client.validate_server_transaction(prepared,signed['transaction_base64'])
            self.assertEqual(signed['last_valid_block_height'],500)

    async def test_captured_public_payloads_preserve_string_encoded_numbers(self):
        for venue,cls,method,route in [('phoenix',aux.PhoenixClient,'list_markets','/v1/view/exchange/markets'),('extended',aux.ExtendedClient,'get_markets','/api/v1/info/markets')]:
            peer=HttpPeer()
            client=cls(http_url=peer.url,chain_id='SN_SEPOLIA' if venue=='extended' else 'mainnet-beta')
            try:
                raw=(ROOT/'crates/exchanges'/venue/'tests/fixtures/public_markets.json').read_bytes()
                peer.expected.append({'verb':'get','route':route,'body':None,'raw':raw})
                result=await getattr(client,method)()
                if venue=='phoenix':self.assertEqual(result[0]['maxLiquidationSizeBaseLots'],'1820000')
                else:self.assertEqual(result['data'][0]['tradingConfig']['maxNumOrders'],'200')
            finally:
                await client.aclose()
                await asyncio.to_thread(peer.close)

    async def test_every_phoenix_instruction_builder_matches_accounts_and_bytes(self):
        contracts=json.loads((ROOT/'crates/exchanges/phoenix/tests/fixtures/instruction_dispatch.json').read_text())
        async with aux.PhoenixClient() as client:
            for name,contract in contracts.items():
                with self.subTest(builder=name):
                    self.assertEqual(client.build_instruction(name,contract['request']),contract['instruction'])

    async def test_native_stream_bindings_and_parent_cleanup(self):
        import websockets.asyncio.server
        for venue,cls in [('phoenix',aux.PhoenixClient),('extended',aux.ExtendedClient)]:
            async def handler(socket):
                request=json.loads(await socket.recv())
                if venue=='extended':
                    await socket.send(json.dumps({'id':'unrelated','result':{'wrong':True}}))
                    await socket.send(json.dumps({'id':request['id'],'result':{}}))
                    frame={'subscription':'orderbooks.BTC-USD','type':'ORDERBOOKS.SNAPSHOT','seq':1,'data':{'m':'BTC-USD','b':[],'a':[]}}
                else:
                    frame={'channel':'orderbook','symbol':'SOL','sequenceNumber':1,'data':{'bids':[],'asks':[]}}
                frame['exact']='0.123456789012345678901'
                await socket.send(json.dumps(frame))
                await socket.wait_closed()
            async with websockets.asyncio.server.serve(handler,'127.0.0.1',0) as server:
                url=f'ws://127.0.0.1:{server.sockets[0].getsockname()[1]}'
                client=cls(ws_url=url,chain_id='SN_SEPOLIA' if venue=='extended' else 'mainnet-beta')
                async with client:
                    stream=await client.subscribe({'channel':'orderbooks','market':'BTC-USD','rpc':True} if venue=='extended' else {'channel':'orderbook','symbol':'SOL'})
                    if venue=='extended':
                        ack=await asyncio.wait_for(anext(stream),3)
                        self.assertEqual(ack['result'],{})
                    event=await asyncio.wait_for(anext(stream),3)
                    self.assertEqual(event['exact'],'0.123456789012345678901')
                with self.assertRaises(StopAsyncIteration):
                    await anext(stream)

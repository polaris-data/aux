#!/usr/bin/env node
// Explicit external SDK input; normal CI consumes the checked-in output offline.
import fs from 'node:fs';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
import {createRequire} from 'node:module';
const root = process.argv[2];
if (!root) throw Error('usage: node scripts/generate-metric-helper-fixtures.mjs /path/to/pinned/sdk/package');
const sdk = await import(pathToFileURL(path.resolve(root,'dist/index.js')).href);
const A='0x1111111111111111111111111111111111111111', B='0x2222222222222222222222222222222222222222';
const R='0x3333333333333333333333333333333333333333', W='0x4444444444444444444444444444444444444444';
const E='0x5555555555555555555555555555555555555555', X='0x6666666666666666666666666666666666666666';
const MAX=(1n<<104n)-1n, WAD=10n**18n, Q64=1n<<64n;
const json = o => JSON.parse(JSON.stringify(o,(_,v)=>typeof v==='bigint'?'0x'+v.toString(16):v));
const call = (data,value=0n,to=R) => json({to,data,value});
const out={poolCreation:[],uniformBins:[],liquidity:[],fallbackSwaps:[],externalSwaps:[],stopLoss:[]};
for (const count of [0,1,5,6,11]) {
 const request={count,lengthE6:1000,addFeeBuyE6:2,addFeeSellE6:3};
 out.uniformBins.push({request,expected:json(sdk.createUniformBins(count,1000,2,3))});
}
for (const distance of ['0','-0.0125','838.8607','-838.8608']) {
 const params={token0:A,token1:B,priceProvider:X,admin:A,initialScaledAmount0PerE18Shares:2n*WAD,
  initialScaledAmount1PerE18Shares:6n*WAD,minimalOperationalLiquidity:1000000n,
  protocolSpreadFeeE6:200,adminSpreadFeeE6:10,protocolNotionalFeeE8:100,adminNotionalFeeE8:2,
  adminFeeDestination:B,curBinDistFromProvidedPricePercent:Number(distance),bidAskRefAsymmetryX8:65535,
  nonNegativeBins:sdk.createUniformBins(6,10000),negativeBins:sdk.createUniformBins(5,10000),salt:'0x'+'12'.repeat(32)};
 if (distance==='-0.0125') Object.assign(params,{extensions:[E],extensionInitData:['0x1234'],extensionOrders:{...sdk.EMPTY_EXTENSION_ORDERS,beforeSwap:1n},priceProviderTimelock:3600n});
 const requestParams=json({...params,curBinDistFromProvidedPricePercent:distance});
 const prepared=json(sdk.prepareFactoryCreatePoolParams(params));
 out.poolCreation.push({request:{target:R,params:requestParams},expected:{params:prepared,call:call(sdk.encodeCreatePoolCalldata(params))}});
 const override=123;
 out.poolCreation.push({request:{target:R,params:requestParams,helperAsymmetryOverride:override},expected:{params:prepared,call:call(sdk.encodeCreatePoolViaImmutableAppPoolHelperCalldata(params,override))}});
}
for (const weighted of [false,true]) for (const owner of [false,true]) for (const native of [false,true]) {
 const params={pool:A,salt:7n,maxAmountToken0:100n,maxAmountToken1:200n,extensionData:'0x1234'};
 const deltas={binIdxs:[-1n,0n,1n],shares:[1n,2n,3n]};
 params[weighted?'weightDeltas':'deltas']=deltas;
 if (owner) params.owner=B;
 const bounds={minimalCurBin:-1,minimalPosition:0n,maximalCurBin:1,maximalPosition:MAX};
 if (weighted || owner) Object.assign(params,bounds);
 const part=weighted?'Weighted':'ExactShares', who=owner?'WithOwner':'Self';
 const encode=sdk[`encodeAddLiquidity${part}${who}Calldata`];
 const prepared=native?sdk[`prepareAddLiquidity${part}${who}SupportingNativePaymentCalldata`](params):encode(params);
 const value=native?50n:0n;
 const request={liquidityAdder:R,pool:A,salt:7n,deltas:{binIdxs:[-1,0,1],shares:[1n,2n,3n]},
  maxAmountToken0:100n,maxAmountToken1:200n,extensionData:'0x1234',kind:weighted?'weighted':'exactShares'};
 if (owner) request.owner=B;
 if (weighted || owner) request.binPositionBounds=bounds;
 if (native) request.nativeValue=value;
 out.liquidity.push({request:json(request),baseCall:call(encode(params)),expected:call(prepared,value)});
}
const fallback={executor:E,externalRouter:X,externalRouterCalldata:'0x1234'};
for (const exactOutput of [false,true]) for (const single of [false,true]) for (const mode of ['wrapped','nativeInput','unwrapOutput']) {
 const method=`Exact${exactOutput?'Output':'Input'}${single?'Single':''}`;
 const tokenIn=mode==='unwrapOutput'?B:W, tokenOut=mode==='unwrapOutput'?W:B;
 const params={recipient:A,deadline:1700000000n};
 if (exactOutput) Object.assign(params,{amountOut:80n,amountInMaximum:100n});
 else Object.assign(params,{amountIn:100n,amountOutMinimum:80n});
 if (single) Object.assign(params,{pool:A,tokenIn,tokenOut,zeroForOne:true,priceLimitX64:0n,extensionData:'0x1234'});
 else Object.assign(params,{tokens:[tokenIn,A,tokenOut],pools:[A,B],extensionDatas:['0x','0x1234'],zeroForOneBitMap:1n});
 const payment=mode==='wrapped'?{type:mode}:{type:mode,wrappedNative:W};
 const value=mode==='nativeInput'?50n:0n;
 const inputCall=call(sdk[`encode${method}Calldata`](params),value);
 let data;
 if (mode==='wrapped') data=sdk[`prepare${method}WithExternalSwapFallbackCalldata`](params,fallback);
 else if (mode==='nativeInput') data=sdk[`prepare${method}SupportingNativePaymentWithExternalSwapFallbackCalldata`](params,fallback,null,W);
 else data=sdk[`prepare${method}AndUnwrapWithExternalSwapFallbackCalldata`](params,fallback,R,null,W);
 out.fallbackSwaps.push({method,mode,request:{call:inputCall,fallback,payment},expected:call(data,value)});
 const external={tokenIn,tokenOut,amountInMaximum:100n,amountOutMinimum:80n,externalRouter:X,externalRouterCalldata:'0x1234',recipient:A,deadline:1700000000n};
 let externalData;
 if (mode==='wrapped') externalData=sdk.prepareExternalSwapCalldata(E,external);
 else if (mode==='nativeInput') externalData=sdk.prepareExternalSwapSupportingNativePaymentCalldata(E,external,null,W);
 else externalData=sdk.prepareExternalSwapAndUnwrapCalldata(E,external,R,null,W);
 out.externalSwaps.push({request:json({router:R,executor:E,params:external,payment,value}),expected:call(externalData,value)});
}
const baseBins=[
 {binIdx:-1,token0BalanceScaled:10n,token1BalanceScaled:50n,totalShares:10n,highWatermark0:100n*WAD,highWatermark1:80n*WAD},
 {binIdx:0,token0BalanceScaled:100n,token1BalanceScaled:10n,totalShares:10n,highWatermark0:10n*WAD,highWatermark1:1n*WAD},
 {binIdx:1,token0BalanceScaled:1n,token1BalanceScaled:0n,totalShares:0n,highWatermark0:10n*WAD,highWatermark1:10n*WAD},
];
for (const drawdown of [0,100000,1000000]) for (const rate of [0n,10n**15n]) for (const skipEmptyBins of [false,true]) {
 const request={bins:baseBins,bidPriceX64:2n*Q64,askPriceX64:3n*Q64,drawdownE6:drawdown,decayPerSecondE18:rate,minimalOperationalLiquidity:1n,skipEmptyBins};
 const result=sdk.evaluateOracleStopLoss(request);
 const durations = o => ({...o,secondsUntilZeroForOneResumes:o.secondsUntilZeroForOneResumes===null?null:Number(o.secondsUntilZeroForOneResumes),secondsUntilOneForZeroResumes:o.secondsUntilOneForZeroResumes===null?null:Number(o.secondsUntilOneForZeroResumes)});
 out.stopLoss.push({request:json(request),expected:json({...durations(result),bins:result.bins.map(durations),breachedBins:result.breachedBins.map(durations)})});
}
const {encodeFunctionData,encodeFunctionResult}=createRequire(path.resolve(root,'package.json'))('viem');
const {encodeBinStateBytes32}=await import(pathToFileURL(path.resolve(root,'dist/utils/poolState.js')).href);
const immAbi=sdk.MetricOmmPoolAbi.find(f=>f.name==='getImmutables');
const imm=Object.fromEntries(immAbi.outputs[0].components.map(c=>[c.name,c.type==='address'?A:0n]));
Object.assign(imm,{token0:A,token1:B,token0ScaleMultiplier:1n,token1ScaleMultiplier:1n,
 initialScaledToken0PerShareE18:2n*WAD,initialScaledToken1PerShareE18:6n*WAD,
 minimalOperationalLiquidity:1n,lowestBin:-1n,highestBin:1n});
const config=[100000,10n**15n,3600,true];
const slot=[0,-1,MAX/2n,0n,0n,0n,65535n];
const packed=baseBins.map(b=>encodeBinStateBytes32({...b,lengthE6:1000,addFeeBuyE6:0,addFeeSellE6:0}));
const shares=baseBins.map(b=>'0x'+b.totalShares.toString(16).padStart(64,'0'));
const readCalls=new Map();
const referenceRead=({abi,functionName,address,args=[]})=>{
 let result;
 if(functionName==='getImmutables') result=imm;
 else if(functionName==='oracleStopLossConfig') result=config;
 else if(functionName==='slot0') result=slot;
 else if(functionName==='binStates') result=packed;
 else if(functionName==='binTotalShares') result=shares;
 else if(functionName==='currentHighWatermarks') {
  const b=baseBins.find(b=>b.binIdx===Number(args[1]));result=[b.highWatermark0,b.highWatermark1];
 } else throw Error('unhandled status reference '+functionName);
 const entries=abi.filter(f=>f.type==='function'&&f.name===functionName&&
  (functionName!=='binTotalShares'||f.inputs[1].type==='int16[]'));
 const data=encodeFunctionData({abi:entries,functionName,args});
 const encoded=encodeFunctionResult({abi:entries,functionName,result});
 readCalls.set(address.toLowerCase()+data,{to:address,data,result:encoded,functionName});
 return result;
};
const status=await sdk.getOracleStopLossAnyBinStatus({publicClient:{multicall:async({contracts})=>contracts.map(referenceRead)},
 extension:E,dataProvider:X,pool:A,bidPriceX64:2n*Q64,askPriceX64:3n*Q64});
const durations = o => ({...o,secondsUntilZeroForOneResumes:o.secondsUntilZeroForOneResumes===null?null:Number(o.secondsUntilZeroForOneResumes),secondsUntilOneForZeroResumes:o.secondsUntilOneForZeroResumes===null?null:Number(o.secondsUntilOneForZeroResumes)});
out.stopLossStatus=[{request:json({pool:A,extension:E,dataProvider:X,bidPriceX64:2n*Q64,askPriceX64:3n*Q64}),blockNumber:'0x21',
 calls:[...readCalls.values()],expected:json({...durations(status),bins:status.bins.map(durations),breachedBins:status.breachedBins.map(durations),block:'0x21',
 config:{...status.config,timelock:Number(status.config.timelock)},curBinIdx:Number(status.curBinIdx),lowestBin:Number(status.lowestBin),highestBin:Number(status.highestBin)})}];
const output='crates/exchanges/metric/tests/fixtures/helpers.json';
const encoded=JSON.stringify(out,null,2)+'\n';
if(process.argv.includes('--check')) {
 if(fs.readFileSync(output,'utf8')!==encoded) throw Error('Metric helper fixtures differ from pinned SDK results');
} else fs.writeFileSync(output,encoded);
console.log(Object.fromEntries(Object.entries(out).map(([key,values])=>[key,values.length])));

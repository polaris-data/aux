import type { MetricChainClient, HttpValue } from "./index";
type PreparedCall = {to:string;data:string;value:string};

export class AnchoredPriceProvider {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  bpsBase(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  bpsBaseU(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  confidenceCooldown(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  confidenceMax(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxRefStaleness(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  mutableParams(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  sourceGasLimit(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  baseFeedId(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  baseMaxStaleness(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  baseSpreadMult(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  baseToken(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  confidenceParam(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  factory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareGetQuote(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  lastConfidenceUpdate(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  lastSpreadMultUpdate(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  lastStalenessUpdate(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  marginStep(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxSpreadMult(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  minMargin(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  offchainOracle(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteFeedId(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteMaxStaleness(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteSpreadMult(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteToken(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareSetConfidenceParam(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetMaxStaleness(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetSource(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetSpreadMultipliers(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  source(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  token0(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  token1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  version(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
}

export class AnchoredProviderFactory {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  adminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  defaultAdminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  defaultClass(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareAddOracle(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCreateAnchoredProvider(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  envelopes(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  feedClass(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getAllProviders(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getOracles(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getProviders(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getRoleAdmin(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareGrantRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareGrantUpdater(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  hasRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  isOracle(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  isProvider(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  isUpdater(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareMulticall(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  oracleCount(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  providerAt(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  providerCount(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  providerCountByCreator(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  providerOwner(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareRemoveEnvelope(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRemoveOracle(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRenounceRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRevokeRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRevokeUpdater(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetConfidence(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetEnvelope(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetFeedClass(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetMaxStaleness(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetSource(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetSpreadMultipliers(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  supportsInterface(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareTransferProviderOwnership(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  version(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
}

export class B20AssetParams {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  registry(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  smallChangeBps(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getOracleParams(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  version(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
}

export class ChainlinkOracle {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  adminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  bpsBase(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  defaultAdminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  stallSpread(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  tagV050(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  tagV051(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareAcceptStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAddApprovedFactory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAddIntegrator(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  approvedFactoryCount(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  blacklisted(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  decimals(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  feeToken(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getApprovedFactories(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getIntegrators(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getOracleData(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getOracleDataBulk(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getRoleAdmin(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareGrantRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  hasRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  integratorCount(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareIntegratorPrice(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  isApprovedFactory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  isIntegrator(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  kind(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareMulticall(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  pendingStateGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  preparePrice(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  priceGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  preparePurgePendingStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  preparePurgeStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRegister(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  registeredPool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  registrationFee(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareRemoveApprovedFactory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRemoveIntegrator(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRemoveVerifier(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRenounceRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRevokeRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetBlacklist(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetIntegrators(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPriceGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetRegistrationFee(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetVerifier(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  stateGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  supportsInterface(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareUpdateReport(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareUpdateReports(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  verifierByTag(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  verifierTags(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  version(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareWithdrawEth(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class CompressedOracleV1 {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  adminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  bpsBase(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  defaultAdminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  stallSpread(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareAcceptStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAllowContractPushers(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAllowPushers(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  decimals(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  extsloadOverload_1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  extsloadOverload_2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  extsloadOverload_3(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  feedIdOf(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getCompressedOracle(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getOracleData(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getOracleDataBulk(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getRoleAdmin(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getSlotLayout(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareGrantRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  hasRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  kind(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  namespaceRemapping(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  pendingStateGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  preparePrice(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  priceGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  preparePurgePendingStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  preparePurgeStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRemovePushers(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRenounceRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRevokePusher(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRevokeRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPendingStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPriceGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  stateGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  supportsInterface(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareUpdateBySignature(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  usedPusherDeadline(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  version(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
}

export class DepositAllowlistExtension {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  factory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareAfterAddLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAfterRemoveLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAfterSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  allowAllDepositors(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  allowedDepositor(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  beforeAddLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareBeforeRemoveLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareBeforeSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareInitialize(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  isAllowedToDeposit(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareSetAllowAllDepositors(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetAllowedToDeposit(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class ExternalSwapExecutor {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  router(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class IERC20 {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  allowance(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareApprove(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  balanceOf(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  totalSupply(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareTransfer(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareTransferFrom(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class ImmutableAppPoolHelper {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  appFactory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  poolFactory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareCreatePool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPoolBidAskRefAsymmetryX8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class MetricOmmPool {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  prepareAddLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCollectFees(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  extsloadOverload_1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  extsloadOverload_2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  extsloadOverload_3(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getImmutables(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareGetSellAndBuyPrices(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  inSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareRemoveLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetBidAskRefAsymmetryX8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetBinAdditionalFees(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPause(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPoolFees(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPriceProvider(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSimulateSwapAndRevert(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class MetricOmmPoolDataProvider {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  binState(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  binStates(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  binTotalSharesOverload_1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  binTotalSharesOverload_2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  distanceFromProvidedPriceX64(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareGetLiquidityDepthHypothetical(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareGetLiquidityDepthLive(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  positionBinSharesOverload_1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  positionBinSharesOverload_2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  priceProvider(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  slot0(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  slot1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  slot2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
}

export class MetricOmmPoolDeployer {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  factory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareDeploy(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class MetricOmmPoolFactory {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  prepareAcceptOwnership(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAcceptPoolAdmin(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  authority(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareCancelPoolAdminTransfer(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCollectEth(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCollectPoolFees(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCollectTokens(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCreatePool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareExecutePoolPriceProviderUpdate(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  feeReceiver(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getFeeCaps(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  hardMaxBinDistanceLimitE6(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  hardMinBinDistanceLimitE6(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  idxToPool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  isPool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxAdminNotionalFeeE8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxAdminSpreadFeeE6(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxBinDistanceE6(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxOwnerAdminNotionalCapE8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxOwnerAdminSpreadCapE6(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxOwnerProtocolNotionalCapE8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxOwnerProtocolSpreadCapE6(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxProtocolNotionalFeeE8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxProtocolSpreadFeeE6(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  minBinDistanceE6(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  nextPoolIdx(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  onCreationMaxBidAskRefAsymmetryX8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  owner(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  preparePausePool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  pendingOwner(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  pendingPoolAdmin(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  pendingPriceProvider(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  pendingPriceProviderExecuteAfter(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  poolAdmin(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  poolAdminFeeDestination(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  poolDeployer(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  poolFeeConfig(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  poolToIdx(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  priceProviderTimelock(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareProposePoolAdminTransfer(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareProposePoolPriceProvider(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  protocolNotionalFeeE8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareProtocolPausePool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareProtocolUnpausePool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetAuthority(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetBinDistanceLimits(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetDefaultProtocolNotionalFeeE8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetDefaultSpreadProtocolFeeE6(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetFeeCaps(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetFeeReceiver(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetOnCreationMaxBidAskRefAsymmetryX8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPoolAdminFeeDestination(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPoolAdminFees(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPoolBidAskRefAsymmetryX8(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPoolBidAskRefAsymmetryX8Batch(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPoolBinAdditionalFees(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPoolDeployer(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPoolProtocolFee(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPoolProtocolFeeBatch(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  spreadProtocolFeeE6(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareTransferOwnership(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareUnpausePool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class MetricOmmPoolLiquidityAdder {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  prepareAddLiquidityExactSharesOverload_1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAddLiquidityExactSharesOverload_2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAddLiquidityWeightedOverload_1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAddLiquidityWeightedOverload_2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareMetricOmmModifyLiquidityCallback(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareMulticall(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRefundEth(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSweepToken(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareUnwrapWeth9(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class MetricOmmSimpleRouter {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  prepareExactInput(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareExactInputSingle(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareExactOutput(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareExactOutputSingle(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareExternalSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareMetricOmmSwapCallback(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareMulticall(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRefundEth(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSelfPermit(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSelfPermitAllowed(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSelfPermitAllowedIfNecessary(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSelfPermitIfNecessary(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSequence(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSweepToken(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareUnwrapWeth9(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class MetricOmmSwapQuoter {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  quoteHypotheticalExactInput(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteHypotheticalExactInputSingleOverload_1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteHypotheticalExactInputSingleOverload_2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteHypotheticalExactOutput(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteHypotheticalExactOutputSingleOverload_1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteHypotheticalExactOutputSingleOverload_2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteLiveExactIn(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteLiveExactInSingleOverload_1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteLiveExactInSingleOverload_2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteLiveExactOut(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteLiveExactOutSingleOverload_1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteLiveExactOutSingleOverload_2(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
}

export class Multicall3 {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  prepareAggregate(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAggregate3(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAggregate3Value(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareBlockAndAggregate(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  getBasefee(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getBlockHash(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getBlockNumber(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getChainId(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getCurrentBlockCoinbase(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getCurrentBlockDifficulty(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getCurrentBlockGasLimit(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getCurrentBlockTimestamp(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getEthBalance(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getLastBlockHash(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareTryAggregate(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareTryBlockAndAggregate(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class MultiplierOracle {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  adapterGasLimit(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  adminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  defaultAdminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  underlying(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareAddParamsAdapter(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCreateFeed(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  decimals(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  feedDisabled(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  feedIdOf(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  feeds(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getParamsAdapters(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getRoleAdmin(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareGrantRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  hasRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  isParamsAdapter(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  kind(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  paramsAdapterCount(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  preparePrice(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  priceGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareRegister(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  registeredPool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareRemoveParamsAdapter(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRenounceRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRevokeRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetFeedDisabled(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  supportsInterface(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  version(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
}

export class OracleProvider {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  blacklisted(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  registeredPool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  registrationFee(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareRegister(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class OracleValueStopLossExtension {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  factory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareAfterAddLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAfterRemoveLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAfterSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareBeforeAddLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareBeforeRemoveLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareBeforeSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCancelOracleStopLossDecay(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCancelOracleStopLossDrawdown(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCancelOracleStopLossHighWatermarks(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCancelOracleStopLossTimelock(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  currentHighWatermarks(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareExecuteOracleStopLossDecay(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareExecuteOracleStopLossDrawdown(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareExecuteOracleStopLossHighWatermarks(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareExecuteOracleStopLossTimelock(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  highWatermarks(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareInitialize(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  oracleStopLossConfig(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  pendingHighWatermark(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  poolStopLossSchedule(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareProposeOracleStopLossDecay(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareProposeOracleStopLossDrawdown(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareProposeOracleStopLossHighWatermarks(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareProposeOracleStopLossTimelock(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class PriceProvider {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  bpsBase(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  bpsBaseU(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  confidenceCooldown(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  confidenceMax(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  maxTimeDelta(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  baseToken(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  confidenceParam(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  factory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareGetQuote(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  lastConfidenceUpdate(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  marginStep(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  offchainFeedId(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  offchainOracle(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  quoteToken(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareSetConfidenceParam(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  token0(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  token1(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  version(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
}

export class PriceProviderFactory {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  adminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  defaultAdminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareAddProvider(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareCreatePriceProvider(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  getAllProviders(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getProviders(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getRoleAdmin(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareGrantRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareGrantUpdater(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  hasRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  isProvider(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  isUpdater(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareMulticall(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  providerAt(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  providerCount(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  providerCountByCreator(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  providerOwner(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareRemoveProvider(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRenounceRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRevokeRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRevokeUpdater(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetConfidence(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  supportsInterface(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareTransferProviderOwnership(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  version(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
}

export class PriceVelocityGuardExtension {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  factory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareAfterAddLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAfterRemoveLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAfterSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareBeforeAddLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareBeforeRemoveLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareBeforeSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareInitialize(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  priceVelocityState(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareSetAnchorMidPrice(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetMaxChangePerBlock(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class PythOracle {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  adminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  bpsBase(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  defaultAdminRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  stallSpread(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareAcceptStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAddApprovedFactory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAddIntegrator(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  approvedFactoryCount(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  blacklisted(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  decimals(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getApprovedFactories(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getIntegrators(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getOracleData(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getOracleDataBulk(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getRoleAdmin(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareGrantRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  hasRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  integratorCount(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareIntegratorPrice(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  isApprovedFactory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  isIntegrator(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  kind(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareMulticall(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  pendingStateGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  preparePrice(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  priceGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  preparePurgePendingStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  preparePurgeStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRegister(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  registeredPool(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  registrationFee(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareRemoveApprovedFactory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRemoveIntegrator(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRenounceRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareRevokeRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetBlacklist(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetIntegrators(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetPriceGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetRegistrationFee(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetStateGuardRole(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  stateGuard(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  supportsInterface(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  version(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareWithdrawEth(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}

export class RobinhoodStockTokenParams {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  smallChangeBps(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  getOracleParams(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  version(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
}

export class SwapAllowlistExtension {
  constructor(client: MetricChainClient, address: string);
  readonly address: string;
  factory(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareAfterAddLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAfterRemoveLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareAfterSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  allowAllSwappers(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  allowedSwapper(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareBeforeAddLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareBeforeRemoveLiquidity(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  beforeSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareInitialize(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  isAllowedToSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; block?:string; fromAddress?:string;}): Promise<HttpValue>;
  prepareSetAllowAllSwappers(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
  prepareSetAllowedToSwap(options?:{args?:HttpValue[] | Record<string,HttpValue>; value?:string; }): PreparedCall;
}


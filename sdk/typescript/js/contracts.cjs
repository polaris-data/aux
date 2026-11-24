"use strict";

class AnchoredPriceProvider {
  constructor(client, address) { this.client = client; this.address = address; }
  bpsBase(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"BPS_BASE()", to:this.address}); }
  bpsBaseU(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"BPS_BASE_U()", to:this.address}); }
  confidenceCooldown(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"CONFIDENCE_COOLDOWN()", to:this.address}); }
  confidenceMax(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"CONFIDENCE_MAX()", to:this.address}); }
  maxRefStaleness(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"MAX_REF_STALENESS()", to:this.address}); }
  mutableParams(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"MUTABLE_PARAMS()", to:this.address}); }
  sourceGasLimit(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"SOURCE_GAS_LIMIT()", to:this.address}); }
  baseFeedId(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"baseFeedId()", to:this.address}); }
  baseMaxStaleness(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"baseMaxStaleness()", to:this.address}); }
  baseSpreadMult(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"baseSpreadMult()", to:this.address}); }
  baseToken(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"baseToken()", to:this.address}); }
  confidenceParam(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"confidenceParam()", to:this.address}); }
  factory(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"factory()", to:this.address}); }
  prepareGetQuote(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredPriceProvider", function:"getQuote()", to:this.address}); }
  lastConfidenceUpdate(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"lastConfidenceUpdate()", to:this.address}); }
  lastSpreadMultUpdate(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"lastSpreadMultUpdate()", to:this.address}); }
  lastStalenessUpdate(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"lastStalenessUpdate()", to:this.address}); }
  marginStep(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"marginStep()", to:this.address}); }
  maxSpreadMult(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"maxSpreadMult()", to:this.address}); }
  minMargin(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"minMargin()", to:this.address}); }
  offchainOracle(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"offchainOracle()", to:this.address}); }
  quoteFeedId(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"quoteFeedId()", to:this.address}); }
  quoteMaxStaleness(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"quoteMaxStaleness()", to:this.address}); }
  quoteSpreadMult(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"quoteSpreadMult()", to:this.address}); }
  quoteToken(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"quoteToken()", to:this.address}); }
  prepareSetConfidenceParam(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredPriceProvider", function:"setConfidenceParam(uint256)", to:this.address}); }
  prepareSetMaxStaleness(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredPriceProvider", function:"setMaxStaleness(uint256,uint256)", to:this.address}); }
  prepareSetSource(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredPriceProvider", function:"setSource(address)", to:this.address}); }
  prepareSetSpreadMultipliers(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredPriceProvider", function:"setSpreadMultipliers(uint256,uint256)", to:this.address}); }
  source(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"source()", to:this.address}); }
  token0(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"token0()", to:this.address}); }
  token1(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"token1()", to:this.address}); }
  version(options = {}) { return this.client.call({...options, contract:"AnchoredPriceProvider", function:"version()", to:this.address}); }
}
exports.AnchoredPriceProvider = AnchoredPriceProvider;

class AnchoredProviderFactory {
  constructor(client, address) { this.client = client; this.address = address; }
  adminRole(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"ADMIN_ROLE()", to:this.address}); }
  defaultAdminRole(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"DEFAULT_ADMIN_ROLE()", to:this.address}); }
  defaultClass(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"DEFAULT_CLASS()", to:this.address}); }
  prepareAddOracle(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"addOracle(address)", to:this.address}); }
  prepareCreateAnchoredProvider(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"createAnchoredProvider(address,bytes32,bytes32,uint256,uint256,uint256,bool,int256,address,address,(address,uint256,uint256,uint256,uint256))", to:this.address}); }
  envelopes(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"envelopes(bytes32)", to:this.address}); }
  feedClass(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"feedClass(bytes32)", to:this.address}); }
  getAllProviders(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"getAllProviders(uint256,uint256)", to:this.address}); }
  getOracles(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"getOracles(uint256,uint256)", to:this.address}); }
  getProviders(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"getProviders(address,uint256,uint256)", to:this.address}); }
  getRoleAdmin(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"getRoleAdmin(bytes32)", to:this.address}); }
  prepareGrantRole(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"grantRole(bytes32,address)", to:this.address}); }
  prepareGrantUpdater(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"grantUpdater(address,address)", to:this.address}); }
  hasRole(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"hasRole(bytes32,address)", to:this.address}); }
  isOracle(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"isOracle(address)", to:this.address}); }
  isProvider(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"isProvider(address)", to:this.address}); }
  isUpdater(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"isUpdater(address,address)", to:this.address}); }
  prepareMulticall(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"multicall(bytes[])", to:this.address}); }
  oracleCount(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"oracleCount()", to:this.address}); }
  providerAt(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"providerAt(uint256)", to:this.address}); }
  providerCount(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"providerCount()", to:this.address}); }
  providerCountByCreator(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"providerCountByCreator(address)", to:this.address}); }
  providerOwner(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"providerOwner(address)", to:this.address}); }
  prepareRemoveEnvelope(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"removeEnvelope(bytes32)", to:this.address}); }
  prepareRemoveOracle(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"removeOracle(address)", to:this.address}); }
  prepareRenounceRole(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"renounceRole(bytes32,address)", to:this.address}); }
  prepareRevokeRole(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"revokeRole(bytes32,address)", to:this.address}); }
  prepareRevokeUpdater(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"revokeUpdater(address,address)", to:this.address}); }
  prepareSetConfidence(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"setConfidence(address[],uint256[])", to:this.address}); }
  prepareSetEnvelope(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"setEnvelope(bytes32,(uint256,uint256,uint256,uint256,uint256,bool))", to:this.address}); }
  prepareSetFeedClass(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"setFeedClass(bytes32,bytes32)", to:this.address}); }
  prepareSetMaxStaleness(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"setMaxStaleness(address[],uint256[],uint256[])", to:this.address}); }
  prepareSetSource(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"setSource(address,address)", to:this.address}); }
  prepareSetSpreadMultipliers(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"setSpreadMultipliers(address[],uint256[],uint256[])", to:this.address}); }
  supportsInterface(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"supportsInterface(bytes4)", to:this.address}); }
  prepareTransferProviderOwnership(options = {}) { return this.client.prepareCall({...options, contract:"AnchoredProviderFactory", function:"transferProviderOwnership(address,address)", to:this.address}); }
  version(options = {}) { return this.client.call({...options, contract:"AnchoredProviderFactory", function:"version()", to:this.address}); }
}
exports.AnchoredProviderFactory = AnchoredProviderFactory;

class B20AssetParams {
  constructor(client, address) { this.client = client; this.address = address; }
  registry(options = {}) { return this.client.call({...options, contract:"B20AssetParams", function:"REGISTRY()", to:this.address}); }
  smallChangeBps(options = {}) { return this.client.call({...options, contract:"B20AssetParams", function:"SMALL_CHANGE_BPS()", to:this.address}); }
  getOracleParams(options = {}) { return this.client.call({...options, contract:"B20AssetParams", function:"getOracleParams(address)", to:this.address}); }
  version(options = {}) { return this.client.call({...options, contract:"B20AssetParams", function:"version()", to:this.address}); }
}
exports.B20AssetParams = B20AssetParams;

class ChainlinkOracle {
  constructor(client, address) { this.client = client; this.address = address; }
  adminRole(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"ADMIN_ROLE()", to:this.address}); }
  bpsBase(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"BPS_BASE()", to:this.address}); }
  defaultAdminRole(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"DEFAULT_ADMIN_ROLE()", to:this.address}); }
  stallSpread(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"STALL_SPREAD()", to:this.address}); }
  tagV050(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"TAG_V050()", to:this.address}); }
  tagV051(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"TAG_V051()", to:this.address}); }
  prepareAcceptStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"acceptStateGuardRole(bytes32)", to:this.address}); }
  prepareAddApprovedFactory(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"addApprovedFactory(address)", to:this.address}); }
  prepareAddIntegrator(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"addIntegrator(address)", to:this.address}); }
  approvedFactoryCount(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"approvedFactoryCount()", to:this.address}); }
  blacklisted(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"blacklisted(address)", to:this.address}); }
  decimals(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"decimals()", to:this.address}); }
  feeToken(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"feeToken()", to:this.address}); }
  getApprovedFactories(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"getApprovedFactories(uint256,uint256)", to:this.address}); }
  getIntegrators(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"getIntegrators(uint256,uint256)", to:this.address}); }
  getOracleData(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"getOracleData(bytes32)", to:this.address}); }
  getOracleDataBulk(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"getOracleDataBulk(bytes32[])", to:this.address}); }
  getRoleAdmin(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"getRoleAdmin(bytes32)", to:this.address}); }
  prepareGrantRole(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"grantRole(bytes32,address)", to:this.address}); }
  hasRole(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"hasRole(bytes32,address)", to:this.address}); }
  integratorCount(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"integratorCount()", to:this.address}); }
  prepareIntegratorPrice(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"integratorPrice(bytes32)", to:this.address}); }
  isApprovedFactory(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"isApprovedFactory(address)", to:this.address}); }
  isIntegrator(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"isIntegrator(address)", to:this.address}); }
  kind(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"kind()", to:this.address}); }
  prepareMulticall(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"multicall(bytes[])", to:this.address}); }
  pendingStateGuard(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"pendingStateGuard(bytes32)", to:this.address}); }
  preparePrice(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"price(bytes32,address)", to:this.address}); }
  priceGuard(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"priceGuard(bytes32)", to:this.address}); }
  preparePurgePendingStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"purgePendingStateGuardRole(bytes32)", to:this.address}); }
  preparePurgeStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"purgeStateGuardRole(bytes32)", to:this.address}); }
  prepareRegister(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"register(bytes32,address,address)", to:this.address}); }
  registeredPool(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"registeredPool(bytes32,address)", to:this.address}); }
  registrationFee(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"registrationFee()", to:this.address}); }
  prepareRemoveApprovedFactory(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"removeApprovedFactory(address)", to:this.address}); }
  prepareRemoveIntegrator(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"removeIntegrator(address)", to:this.address}); }
  prepareRemoveVerifier(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"removeVerifier(uint8)", to:this.address}); }
  prepareRenounceRole(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"renounceRole(bytes32,address)", to:this.address}); }
  prepareRevokeRole(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"revokeRole(bytes32,address)", to:this.address}); }
  prepareSetBlacklist(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"setBlacklist(address,bool)", to:this.address}); }
  prepareSetIntegrators(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"setIntegrators(address[],bool)", to:this.address}); }
  prepareSetPriceGuard(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"setPriceGuard(bytes32,uint128,uint128)", to:this.address}); }
  prepareSetRegistrationFee(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"setRegistrationFee(uint256)", to:this.address}); }
  prepareSetStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"setStateGuardRole(bytes32,address)", to:this.address}); }
  prepareSetVerifier(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"setVerifier(uint8,address,bool)", to:this.address}); }
  stateGuard(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"stateGuard(bytes32)", to:this.address}); }
  supportsInterface(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"supportsInterface(bytes4)", to:this.address}); }
  prepareUpdateReport(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"updateReport(bytes)", to:this.address}); }
  prepareUpdateReports(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"updateReports(bytes[])", to:this.address}); }
  verifierByTag(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"verifierByTag(uint8)", to:this.address}); }
  verifierTags(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"verifierTags()", to:this.address}); }
  version(options = {}) { return this.client.call({...options, contract:"ChainlinkOracle", function:"version()", to:this.address}); }
  prepareWithdrawEth(options = {}) { return this.client.prepareCall({...options, contract:"ChainlinkOracle", function:"withdrawEth()", to:this.address}); }
}
exports.ChainlinkOracle = ChainlinkOracle;

class CompressedOracleV1 {
  constructor(client, address) { this.client = client; this.address = address; }
  adminRole(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"ADMIN_ROLE()", to:this.address}); }
  bpsBase(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"BPS_BASE()", to:this.address}); }
  defaultAdminRole(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"DEFAULT_ADMIN_ROLE()", to:this.address}); }
  stallSpread(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"STALL_SPREAD()", to:this.address}); }
  prepareAcceptStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"acceptStateGuardRole(bytes32)", to:this.address}); }
  prepareAllowContractPushers(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"allowContractPushers(address[])", to:this.address}); }
  prepareAllowPushers(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"allowPushers(uint256,address[],bytes[])", to:this.address}); }
  decimals(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"decimals()", to:this.address}); }
  extsloadOverload_1(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"extsload(bytes32)", to:this.address}); }
  extsloadOverload_2(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"extsload(bytes32,uint256)", to:this.address}); }
  extsloadOverload_3(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"extsload(bytes32[])", to:this.address}); }
  feedIdOf(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"feedIdOf(address,uint8,uint8)", to:this.address}); }
  getCompressedOracle(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"getCompressedOracle(bytes32)", to:this.address}); }
  getOracleData(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"getOracleData(bytes32)", to:this.address}); }
  getOracleDataBulk(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"getOracleDataBulk(bytes32[])", to:this.address}); }
  getRoleAdmin(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"getRoleAdmin(bytes32)", to:this.address}); }
  getSlotLayout(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"getSlotLayout(bytes32)", to:this.address}); }
  prepareGrantRole(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"grantRole(bytes32,address)", to:this.address}); }
  hasRole(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"hasRole(bytes32,address)", to:this.address}); }
  kind(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"kind()", to:this.address}); }
  namespaceRemapping(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"namespaceRemapping(address)", to:this.address}); }
  pendingStateGuard(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"pendingStateGuard(bytes32)", to:this.address}); }
  preparePrice(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"price(bytes32,address)", to:this.address}); }
  priceGuard(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"priceGuard(bytes32)", to:this.address}); }
  preparePurgePendingStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"purgePendingStateGuardRole(bytes32)", to:this.address}); }
  preparePurgeStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"purgeStateGuardRole(bytes32)", to:this.address}); }
  prepareRemovePushers(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"removePushers(address[])", to:this.address}); }
  prepareRenounceRole(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"renounceRole(bytes32,address)", to:this.address}); }
  prepareRevokePusher(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"revokePusher()", to:this.address}); }
  prepareRevokeRole(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"revokeRole(bytes32,address)", to:this.address}); }
  prepareSetPendingStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"setPendingStateGuardRole(bytes32,address)", to:this.address}); }
  prepareSetPriceGuard(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"setPriceGuard(bytes32,uint128,uint128)", to:this.address}); }
  stateGuard(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"stateGuard(bytes32)", to:this.address}); }
  supportsInterface(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"supportsInterface(bytes4)", to:this.address}); }
  prepareUpdateBySignature(options = {}) { return this.client.prepareCall({...options, contract:"CompressedOracleV1", function:"updateBySignature(address,uint256,bytes)", to:this.address}); }
  usedPusherDeadline(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"usedPusherDeadline(address,address)", to:this.address}); }
  version(options = {}) { return this.client.call({...options, contract:"CompressedOracleV1", function:"version()", to:this.address}); }
}
exports.CompressedOracleV1 = CompressedOracleV1;

class DepositAllowlistExtension {
  constructor(client, address) { this.client = client; this.address = address; }
  factory(options = {}) { return this.client.call({...options, contract:"DepositAllowlistExtension", function:"FACTORY()", to:this.address}); }
  prepareAfterAddLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"DepositAllowlistExtension", function:"afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", to:this.address}); }
  prepareAfterRemoveLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"DepositAllowlistExtension", function:"afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", to:this.address}); }
  prepareAfterSwap(options = {}) { return this.client.prepareCall({...options, contract:"DepositAllowlistExtension", function:"afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)", to:this.address}); }
  allowAllDepositors(options = {}) { return this.client.call({...options, contract:"DepositAllowlistExtension", function:"allowAllDepositors(address)", to:this.address}); }
  allowedDepositor(options = {}) { return this.client.call({...options, contract:"DepositAllowlistExtension", function:"allowedDepositor(address,address)", to:this.address}); }
  beforeAddLiquidity(options = {}) { return this.client.call({...options, contract:"DepositAllowlistExtension", function:"beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", to:this.address}); }
  prepareBeforeRemoveLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"DepositAllowlistExtension", function:"beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", to:this.address}); }
  prepareBeforeSwap(options = {}) { return this.client.prepareCall({...options, contract:"DepositAllowlistExtension", function:"beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)", to:this.address}); }
  prepareInitialize(options = {}) { return this.client.prepareCall({...options, contract:"DepositAllowlistExtension", function:"initialize(address,bytes)", to:this.address}); }
  isAllowedToDeposit(options = {}) { return this.client.call({...options, contract:"DepositAllowlistExtension", function:"isAllowedToDeposit(address,address)", to:this.address}); }
  prepareSetAllowAllDepositors(options = {}) { return this.client.prepareCall({...options, contract:"DepositAllowlistExtension", function:"setAllowAllDepositors(address,bool)", to:this.address}); }
  prepareSetAllowedToDeposit(options = {}) { return this.client.prepareCall({...options, contract:"DepositAllowlistExtension", function:"setAllowedToDeposit(address,address,bool)", to:this.address}); }
}
exports.DepositAllowlistExtension = DepositAllowlistExtension;

class ExternalSwapExecutor {
  constructor(client, address) { this.client = client; this.address = address; }
  router(options = {}) { return this.client.call({...options, contract:"ExternalSwapExecutor", function:"router()", to:this.address}); }
  prepareSwap(options = {}) { return this.client.prepareCall({...options, contract:"ExternalSwapExecutor", function:"swap((address,address,uint128,uint128,address,bytes,address,uint256))", to:this.address}); }
}
exports.ExternalSwapExecutor = ExternalSwapExecutor;

class IERC20 {
  constructor(client, address) { this.client = client; this.address = address; }
  allowance(options = {}) { return this.client.call({...options, contract:"IERC20", function:"allowance(address,address)", to:this.address}); }
  prepareApprove(options = {}) { return this.client.prepareCall({...options, contract:"IERC20", function:"approve(address,uint256)", to:this.address}); }
  balanceOf(options = {}) { return this.client.call({...options, contract:"IERC20", function:"balanceOf(address)", to:this.address}); }
  totalSupply(options = {}) { return this.client.call({...options, contract:"IERC20", function:"totalSupply()", to:this.address}); }
  prepareTransfer(options = {}) { return this.client.prepareCall({...options, contract:"IERC20", function:"transfer(address,uint256)", to:this.address}); }
  prepareTransferFrom(options = {}) { return this.client.prepareCall({...options, contract:"IERC20", function:"transferFrom(address,address,uint256)", to:this.address}); }
}
exports.IERC20 = IERC20;

class ImmutableAppPoolHelper {
  constructor(client, address) { this.client = client; this.address = address; }
  appFactory(options = {}) { return this.client.call({...options, contract:"ImmutableAppPoolHelper", function:"APP_FACTORY()", to:this.address}); }
  poolFactory(options = {}) { return this.client.call({...options, contract:"ImmutableAppPoolHelper", function:"POOL_FACTORY()", to:this.address}); }
  prepareCreatePool(options = {}) { return this.client.prepareCall({...options, contract:"ImmutableAppPoolHelper", function:"createPool((address,address,address,address[],(uint256,uint256,uint256,uint256,uint256,uint256),bytes[],uint256,address,uint256,uint256,uint256,uint24,uint24,uint24,uint24,address,int24,uint256[],uint256[],bytes32,uint16),uint16)", to:this.address}); }
  prepareSetPoolBidAskRefAsymmetryX8(options = {}) { return this.client.prepareCall({...options, contract:"ImmutableAppPoolHelper", function:"setPoolBidAskRefAsymmetryX8(address,uint16)", to:this.address}); }
}
exports.ImmutableAppPoolHelper = ImmutableAppPoolHelper;

class MetricOmmPool {
  constructor(client, address) { this.client = client; this.address = address; }
  prepareAddLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"addLiquidity(address,uint80,(int256[],uint256[]),bytes,bytes)", to:this.address}); }
  prepareCollectFees(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"collectFees(uint256,uint256,uint256,uint256,address,address)", to:this.address}); }
  extsloadOverload_1(options = {}) { return this.client.call({...options, contract:"MetricOmmPool", function:"extsload(bytes32)", to:this.address}); }
  extsloadOverload_2(options = {}) { return this.client.call({...options, contract:"MetricOmmPool", function:"extsload(bytes32,uint256)", to:this.address}); }
  extsloadOverload_3(options = {}) { return this.client.call({...options, contract:"MetricOmmPool", function:"extsload(bytes32[])", to:this.address}); }
  getImmutables(options = {}) { return this.client.call({...options, contract:"MetricOmmPool", function:"getImmutables()", to:this.address}); }
  prepareGetSellAndBuyPrices(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"getSellAndBuyPrices()", to:this.address}); }
  inSwap(options = {}) { return this.client.call({...options, contract:"MetricOmmPool", function:"inSwap()", to:this.address}); }
  prepareRemoveLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"removeLiquidity(address,uint80,(int256[],uint256[]),bytes)", to:this.address}); }
  prepareSetBidAskRefAsymmetryX8(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"setBidAskRefAsymmetryX8(uint16)", to:this.address}); }
  prepareSetBinAdditionalFees(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"setBinAdditionalFees(int16,uint16,uint16)", to:this.address}); }
  prepareSetPause(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"setPause(uint8)", to:this.address}); }
  prepareSetPoolFees(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"setPoolFees(uint24,uint24)", to:this.address}); }
  prepareSetPriceProvider(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"setPriceProvider(address)", to:this.address}); }
  prepareSimulateSwapAndRevert(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"simulateSwapAndRevert(address,address,bool,int128,uint128,uint128,uint128,uint128,bytes)", to:this.address}); }
  prepareSwap(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPool", function:"swap(address,bool,int128,uint128,bytes,bytes)", to:this.address}); }
}
exports.MetricOmmPool = MetricOmmPool;

class MetricOmmPoolDataProvider {
  constructor(client, address) { this.client = client; this.address = address; }
  binState(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"binState(address,int16)", to:this.address}); }
  binStates(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"binStates(address,int16[])", to:this.address}); }
  binTotalSharesOverload_1(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"binTotalShares(address,int16[])", to:this.address}); }
  binTotalSharesOverload_2(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"binTotalShares(address,int16)", to:this.address}); }
  distanceFromProvidedPriceX64(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"distanceFromProvidedPriceX64(address)", to:this.address}); }
  prepareGetLiquidityDepthHypothetical(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolDataProvider", function:"getLiquidityDepthHypothetical(address,uint8,uint128,uint128,uint128)", to:this.address}); }
  prepareGetLiquidityDepthLive(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolDataProvider", function:"getLiquidityDepthLive(address,uint8)", to:this.address}); }
  positionBinSharesOverload_1(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"positionBinShares(address,bytes32)", to:this.address}); }
  positionBinSharesOverload_2(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"positionBinShares(address,address,uint80,int16)", to:this.address}); }
  priceProvider(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"priceProvider(address)", to:this.address}); }
  slot0(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"slot0(address)", to:this.address}); }
  slot1(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"slot1(address)", to:this.address}); }
  slot2(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDataProvider", function:"slot2(address)", to:this.address}); }
}
exports.MetricOmmPoolDataProvider = MetricOmmPoolDataProvider;

class MetricOmmPoolDeployer {
  constructor(client, address) { this.client = client; this.address = address; }
  factory(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolDeployer", function:"FACTORY()", to:this.address}); }
  prepareDeploy(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolDeployer", function:"deploy((bytes32,address,address,address,address,(address,address,address,address,address,address,address),(uint256,uint256,uint256,uint256,uint256,uint256),bool,uint256,uint256,uint256,uint256,uint256,uint24,int24,(uint104,uint104,uint16,uint16,uint16)[],(uint104,uint104,uint16,uint16,uint16)[],uint24,uint16))", to:this.address}); }
}
exports.MetricOmmPoolDeployer = MetricOmmPoolDeployer;

class MetricOmmPoolFactory {
  constructor(client, address) { this.client = client; this.address = address; }
  prepareAcceptOwnership(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"acceptOwnership()", to:this.address}); }
  prepareAcceptPoolAdmin(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"acceptPoolAdmin(address)", to:this.address}); }
  authority(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"authority()", to:this.address}); }
  prepareCancelPoolAdminTransfer(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"cancelPoolAdminTransfer(address)", to:this.address}); }
  prepareCollectEth(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"collectEth(address,uint256)", to:this.address}); }
  prepareCollectPoolFees(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"collectPoolFees(address)", to:this.address}); }
  prepareCollectTokens(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"collectTokens(address,address,uint256)", to:this.address}); }
  prepareCreatePool(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"createPool((address,address,address,address[],(uint256,uint256,uint256,uint256,uint256,uint256),bytes[],uint256,address,uint256,uint256,uint256,uint24,uint24,uint24,uint24,address,int24,uint256[],uint256[],bytes32,uint16))", to:this.address}); }
  prepareExecutePoolPriceProviderUpdate(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"executePoolPriceProviderUpdate(address)", to:this.address}); }
  feeReceiver(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"feeReceiver()", to:this.address}); }
  getFeeCaps(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"getFeeCaps()", to:this.address}); }
  hardMaxBinDistanceLimitE6(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"hardMaxBinDistanceLimitE6()", to:this.address}); }
  hardMinBinDistanceLimitE6(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"hardMinBinDistanceLimitE6()", to:this.address}); }
  idxToPool(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"idxToPool(uint256)", to:this.address}); }
  isPool(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"isPool(address)", to:this.address}); }
  maxAdminNotionalFeeE8(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"maxAdminNotionalFeeE8()", to:this.address}); }
  maxAdminSpreadFeeE6(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"maxAdminSpreadFeeE6()", to:this.address}); }
  maxBinDistanceE6(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"maxBinDistanceE6()", to:this.address}); }
  maxOwnerAdminNotionalCapE8(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"maxOwnerAdminNotionalCapE8()", to:this.address}); }
  maxOwnerAdminSpreadCapE6(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"maxOwnerAdminSpreadCapE6()", to:this.address}); }
  maxOwnerProtocolNotionalCapE8(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"maxOwnerProtocolNotionalCapE8()", to:this.address}); }
  maxOwnerProtocolSpreadCapE6(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"maxOwnerProtocolSpreadCapE6()", to:this.address}); }
  maxProtocolNotionalFeeE8(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"maxProtocolNotionalFeeE8()", to:this.address}); }
  maxProtocolSpreadFeeE6(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"maxProtocolSpreadFeeE6()", to:this.address}); }
  minBinDistanceE6(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"minBinDistanceE6()", to:this.address}); }
  nextPoolIdx(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"nextPoolIdx()", to:this.address}); }
  onCreationMaxBidAskRefAsymmetryX8(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"onCreationMaxBidAskRefAsymmetryX8()", to:this.address}); }
  owner(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"owner()", to:this.address}); }
  preparePausePool(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"pausePool(address)", to:this.address}); }
  pendingOwner(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"pendingOwner()", to:this.address}); }
  pendingPoolAdmin(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"pendingPoolAdmin(address)", to:this.address}); }
  pendingPriceProvider(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"pendingPriceProvider(address)", to:this.address}); }
  pendingPriceProviderExecuteAfter(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"pendingPriceProviderExecuteAfter(address)", to:this.address}); }
  poolAdmin(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"poolAdmin(address)", to:this.address}); }
  poolAdminFeeDestination(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"poolAdminFeeDestination(address)", to:this.address}); }
  poolDeployer(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"poolDeployer()", to:this.address}); }
  poolFeeConfig(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"poolFeeConfig(address)", to:this.address}); }
  poolToIdx(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"poolToIdx(address)", to:this.address}); }
  priceProviderTimelock(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"priceProviderTimelock(address)", to:this.address}); }
  prepareProposePoolAdminTransfer(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"proposePoolAdminTransfer(address,address)", to:this.address}); }
  prepareProposePoolPriceProvider(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"proposePoolPriceProvider(address,address)", to:this.address}); }
  protocolNotionalFeeE8(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"protocolNotionalFeeE8()", to:this.address}); }
  prepareProtocolPausePool(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"protocolPausePool(address)", to:this.address}); }
  prepareProtocolUnpausePool(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"protocolUnpausePool(address)", to:this.address}); }
  prepareSetAuthority(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setAuthority(address)", to:this.address}); }
  prepareSetBinDistanceLimits(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setBinDistanceLimits(int128,int128)", to:this.address}); }
  prepareSetDefaultProtocolNotionalFeeE8(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setDefaultProtocolNotionalFeeE8(uint24)", to:this.address}); }
  prepareSetDefaultSpreadProtocolFeeE6(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setDefaultSpreadProtocolFeeE6(uint24)", to:this.address}); }
  prepareSetFeeCaps(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setFeeCaps(uint24,uint24,uint24,uint24)", to:this.address}); }
  prepareSetFeeReceiver(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setFeeReceiver(address)", to:this.address}); }
  prepareSetOnCreationMaxBidAskRefAsymmetryX8(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setOnCreationMaxBidAskRefAsymmetryX8(uint16)", to:this.address}); }
  prepareSetPoolAdminFeeDestination(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setPoolAdminFeeDestination(address,address)", to:this.address}); }
  prepareSetPoolAdminFees(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setPoolAdminFees(address,uint24,uint24)", to:this.address}); }
  prepareSetPoolBidAskRefAsymmetryX8(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setPoolBidAskRefAsymmetryX8(address,uint16)", to:this.address}); }
  prepareSetPoolBidAskRefAsymmetryX8Batch(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setPoolBidAskRefAsymmetryX8Batch(address[],uint16[])", to:this.address}); }
  prepareSetPoolBinAdditionalFees(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setPoolBinAdditionalFees(address,int16,uint16,uint16)", to:this.address}); }
  prepareSetPoolDeployer(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setPoolDeployer(address)", to:this.address}); }
  prepareSetPoolProtocolFee(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setPoolProtocolFee(address,uint24,uint24)", to:this.address}); }
  prepareSetPoolProtocolFeeBatch(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"setPoolProtocolFeeBatch(address[],uint24[],uint24[])", to:this.address}); }
  spreadProtocolFeeE6(options = {}) { return this.client.call({...options, contract:"MetricOmmPoolFactory", function:"spreadProtocolFeeE6()", to:this.address}); }
  prepareTransferOwnership(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"transferOwnership(address)", to:this.address}); }
  prepareUnpausePool(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolFactory", function:"unpausePool(address)", to:this.address}); }
}
exports.MetricOmmPoolFactory = MetricOmmPoolFactory;

class MetricOmmPoolLiquidityAdder {
  constructor(client, address) { this.client = client; this.address = address; }
  prepareAddLiquidityExactSharesOverload_1(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolLiquidityAdder", function:"addLiquidityExactShares(address,address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)", to:this.address}); }
  prepareAddLiquidityExactSharesOverload_2(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolLiquidityAdder", function:"addLiquidityExactShares(address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)", to:this.address}); }
  prepareAddLiquidityWeightedOverload_1(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolLiquidityAdder", function:"addLiquidityWeighted(address,address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)", to:this.address}); }
  prepareAddLiquidityWeightedOverload_2(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolLiquidityAdder", function:"addLiquidityWeighted(address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)", to:this.address}); }
  prepareMetricOmmModifyLiquidityCallback(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolLiquidityAdder", function:"metricOmmModifyLiquidityCallback(uint256,uint256,bytes)", to:this.address}); }
  prepareMulticall(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolLiquidityAdder", function:"multicall(bytes[])", to:this.address}); }
  prepareRefundEth(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolLiquidityAdder", function:"refundETH()", to:this.address}); }
  prepareSweepToken(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolLiquidityAdder", function:"sweepToken(address,uint256,address)", to:this.address}); }
  prepareUnwrapWeth9(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmPoolLiquidityAdder", function:"unwrapWETH9(uint256,address)", to:this.address}); }
}
exports.MetricOmmPoolLiquidityAdder = MetricOmmPoolLiquidityAdder;

class MetricOmmSimpleRouter {
  constructor(client, address) { this.client = client; this.address = address; }
  prepareExactInput(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"exactInput((address[],address[],bytes[],uint256,uint128,uint128,address,uint256))", to:this.address}); }
  prepareExactInputSingle(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"exactInputSingle((address,address,address,bool,uint128,uint128,address,uint256,uint128,bytes))", to:this.address}); }
  prepareExactOutput(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"exactOutput((address[],address[],bytes[],uint256,uint128,uint128,address,uint256))", to:this.address}); }
  prepareExactOutputSingle(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"exactOutputSingle((address,address,address,bool,uint128,uint128,address,uint256,uint128,bytes))", to:this.address}); }
  prepareExternalSwap(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"externalSwap(address,bool,bool,(address,address,uint128,uint128,address,bytes,address,uint256))", to:this.address}); }
  prepareMetricOmmSwapCallback(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"metricOmmSwapCallback(int256,int256,bytes)", to:this.address}); }
  prepareMulticall(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"multicall(bytes[])", to:this.address}); }
  prepareRefundEth(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"refundETH()", to:this.address}); }
  prepareSelfPermit(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)", to:this.address}); }
  prepareSelfPermitAllowed(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)", to:this.address}); }
  prepareSelfPermitAllowedIfNecessary(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)", to:this.address}); }
  prepareSelfPermitIfNecessary(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)", to:this.address}); }
  prepareSequence(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"sequence((bytes,uint8,uint8)[])", to:this.address}); }
  prepareSweepToken(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"sweepToken(address,uint256,address)", to:this.address}); }
  prepareUnwrapWeth9(options = {}) { return this.client.prepareCall({...options, contract:"MetricOmmSimpleRouter", function:"unwrapWETH9(uint256,address)", to:this.address}); }
}
exports.MetricOmmSimpleRouter = MetricOmmSimpleRouter;

class MetricOmmSwapQuoter {
  constructor(client, address) { this.client = client; this.address = address; }
  quoteHypotheticalExactInput(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteHypotheticalExactInput(address,(address[],bytes[],uint256,uint128,uint128[],uint128[],uint128[]))", to:this.address}); }
  quoteHypotheticalExactInputSingleOverload_1(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteHypotheticalExactInputSingle(address,address,bool,uint128,uint128,uint128,uint128,uint128)", to:this.address}); }
  quoteHypotheticalExactInputSingleOverload_2(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteHypotheticalExactInputSingle(address,address,address,bool,uint128,uint128,uint128,uint128,uint128,bytes)", to:this.address}); }
  quoteHypotheticalExactOutput(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteHypotheticalExactOutput(address,(address[],bytes[],uint256,uint128,uint128[],uint128[],uint128[]))", to:this.address}); }
  quoteHypotheticalExactOutputSingleOverload_1(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteHypotheticalExactOutputSingle(address,address,bool,uint128,uint128,uint128,uint128,uint128)", to:this.address}); }
  quoteHypotheticalExactOutputSingleOverload_2(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteHypotheticalExactOutputSingle(address,address,address,bool,uint128,uint128,uint128,uint128,uint128,bytes)", to:this.address}); }
  quoteLiveExactIn(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteLiveExactIn(address,(address[],bytes[],uint256,uint128))", to:this.address}); }
  quoteLiveExactInSingleOverload_1(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteLiveExactInSingle(address,address,address,bool,uint128,uint128,bytes)", to:this.address}); }
  quoteLiveExactInSingleOverload_2(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteLiveExactInSingle(address,address,bool,uint128,uint128)", to:this.address}); }
  quoteLiveExactOut(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteLiveExactOut(address,(address[],bytes[],uint256,uint128))", to:this.address}); }
  quoteLiveExactOutSingleOverload_1(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteLiveExactOutSingle(address,address,address,bool,uint128,uint128,bytes)", to:this.address}); }
  quoteLiveExactOutSingleOverload_2(options = {}) { return this.client.call({...options, contract:"MetricOmmSwapQuoter", function:"quoteLiveExactOutSingle(address,address,bool,uint128,uint128)", to:this.address}); }
}
exports.MetricOmmSwapQuoter = MetricOmmSwapQuoter;

class Multicall3 {
  constructor(client, address) { this.client = client; this.address = address; }
  prepareAggregate(options = {}) { return this.client.prepareCall({...options, contract:"Multicall3", function:"aggregate((address,bytes)[])", to:this.address}); }
  prepareAggregate3(options = {}) { return this.client.prepareCall({...options, contract:"Multicall3", function:"aggregate3((address,bool,bytes)[])", to:this.address}); }
  prepareAggregate3Value(options = {}) { return this.client.prepareCall({...options, contract:"Multicall3", function:"aggregate3Value((address,bool,uint256,bytes)[])", to:this.address}); }
  prepareBlockAndAggregate(options = {}) { return this.client.prepareCall({...options, contract:"Multicall3", function:"blockAndAggregate((address,bytes)[])", to:this.address}); }
  getBasefee(options = {}) { return this.client.call({...options, contract:"Multicall3", function:"getBasefee()", to:this.address}); }
  getBlockHash(options = {}) { return this.client.call({...options, contract:"Multicall3", function:"getBlockHash(uint256)", to:this.address}); }
  getBlockNumber(options = {}) { return this.client.call({...options, contract:"Multicall3", function:"getBlockNumber()", to:this.address}); }
  getChainId(options = {}) { return this.client.call({...options, contract:"Multicall3", function:"getChainId()", to:this.address}); }
  getCurrentBlockCoinbase(options = {}) { return this.client.call({...options, contract:"Multicall3", function:"getCurrentBlockCoinbase()", to:this.address}); }
  getCurrentBlockDifficulty(options = {}) { return this.client.call({...options, contract:"Multicall3", function:"getCurrentBlockDifficulty()", to:this.address}); }
  getCurrentBlockGasLimit(options = {}) { return this.client.call({...options, contract:"Multicall3", function:"getCurrentBlockGasLimit()", to:this.address}); }
  getCurrentBlockTimestamp(options = {}) { return this.client.call({...options, contract:"Multicall3", function:"getCurrentBlockTimestamp()", to:this.address}); }
  getEthBalance(options = {}) { return this.client.call({...options, contract:"Multicall3", function:"getEthBalance(address)", to:this.address}); }
  getLastBlockHash(options = {}) { return this.client.call({...options, contract:"Multicall3", function:"getLastBlockHash()", to:this.address}); }
  prepareTryAggregate(options = {}) { return this.client.prepareCall({...options, contract:"Multicall3", function:"tryAggregate(bool,(address,bytes)[])", to:this.address}); }
  prepareTryBlockAndAggregate(options = {}) { return this.client.prepareCall({...options, contract:"Multicall3", function:"tryBlockAndAggregate(bool,(address,bytes)[])", to:this.address}); }
}
exports.Multicall3 = Multicall3;

class MultiplierOracle {
  constructor(client, address) { this.client = client; this.address = address; }
  adapterGasLimit(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"ADAPTER_GAS_LIMIT()", to:this.address}); }
  adminRole(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"ADMIN_ROLE()", to:this.address}); }
  defaultAdminRole(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"DEFAULT_ADMIN_ROLE()", to:this.address}); }
  underlying(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"UNDERLYING()", to:this.address}); }
  prepareAddParamsAdapter(options = {}) { return this.client.prepareCall({...options, contract:"MultiplierOracle", function:"addParamsAdapter(address)", to:this.address}); }
  prepareCreateFeed(options = {}) { return this.client.prepareCall({...options, contract:"MultiplierOracle", function:"createFeed(bytes32,address,address)", to:this.address}); }
  decimals(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"decimals()", to:this.address}); }
  feedDisabled(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"feedDisabled(bytes32)", to:this.address}); }
  feedIdOf(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"feedIdOf(bytes32,address,address)", to:this.address}); }
  feeds(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"feeds(bytes32)", to:this.address}); }
  getParamsAdapters(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"getParamsAdapters(uint256,uint256)", to:this.address}); }
  getRoleAdmin(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"getRoleAdmin(bytes32)", to:this.address}); }
  prepareGrantRole(options = {}) { return this.client.prepareCall({...options, contract:"MultiplierOracle", function:"grantRole(bytes32,address)", to:this.address}); }
  hasRole(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"hasRole(bytes32,address)", to:this.address}); }
  isParamsAdapter(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"isParamsAdapter(address)", to:this.address}); }
  kind(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"kind()", to:this.address}); }
  paramsAdapterCount(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"paramsAdapterCount()", to:this.address}); }
  preparePrice(options = {}) { return this.client.prepareCall({...options, contract:"MultiplierOracle", function:"price(bytes32,address)", to:this.address}); }
  priceGuard(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"priceGuard(bytes32)", to:this.address}); }
  prepareRegister(options = {}) { return this.client.prepareCall({...options, contract:"MultiplierOracle", function:"register(bytes32,address,address)", to:this.address}); }
  registeredPool(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"registeredPool(bytes32,address)", to:this.address}); }
  prepareRemoveParamsAdapter(options = {}) { return this.client.prepareCall({...options, contract:"MultiplierOracle", function:"removeParamsAdapter(address)", to:this.address}); }
  prepareRenounceRole(options = {}) { return this.client.prepareCall({...options, contract:"MultiplierOracle", function:"renounceRole(bytes32,address)", to:this.address}); }
  prepareRevokeRole(options = {}) { return this.client.prepareCall({...options, contract:"MultiplierOracle", function:"revokeRole(bytes32,address)", to:this.address}); }
  prepareSetFeedDisabled(options = {}) { return this.client.prepareCall({...options, contract:"MultiplierOracle", function:"setFeedDisabled(bytes32,bool)", to:this.address}); }
  supportsInterface(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"supportsInterface(bytes4)", to:this.address}); }
  version(options = {}) { return this.client.call({...options, contract:"MultiplierOracle", function:"version()", to:this.address}); }
}
exports.MultiplierOracle = MultiplierOracle;

class OracleProvider {
  constructor(client, address) { this.client = client; this.address = address; }
  blacklisted(options = {}) { return this.client.call({...options, contract:"OracleProvider", function:"blacklisted(address)", to:this.address}); }
  registeredPool(options = {}) { return this.client.call({...options, contract:"OracleProvider", function:"registeredPool(bytes32,address)", to:this.address}); }
  registrationFee(options = {}) { return this.client.call({...options, contract:"OracleProvider", function:"registrationFee()", to:this.address}); }
  prepareRegister(options = {}) { return this.client.prepareCall({...options, contract:"OracleProvider", function:"register(bytes32,address,address)", to:this.address}); }
}
exports.OracleProvider = OracleProvider;

class OracleValueStopLossExtension {
  constructor(client, address) { this.client = client; this.address = address; }
  factory(options = {}) { return this.client.call({...options, contract:"OracleValueStopLossExtension", function:"FACTORY()", to:this.address}); }
  prepareAfterAddLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", to:this.address}); }
  prepareAfterRemoveLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", to:this.address}); }
  prepareAfterSwap(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)", to:this.address}); }
  prepareBeforeAddLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", to:this.address}); }
  prepareBeforeRemoveLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", to:this.address}); }
  prepareBeforeSwap(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)", to:this.address}); }
  prepareCancelOracleStopLossDecay(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"cancelOracleStopLossDecay(address)", to:this.address}); }
  prepareCancelOracleStopLossDrawdown(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"cancelOracleStopLossDrawdown(address)", to:this.address}); }
  prepareCancelOracleStopLossHighWatermarks(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"cancelOracleStopLossHighWatermarks(address)", to:this.address}); }
  prepareCancelOracleStopLossTimelock(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"cancelOracleStopLossTimelock(address)", to:this.address}); }
  currentHighWatermarks(options = {}) { return this.client.call({...options, contract:"OracleValueStopLossExtension", function:"currentHighWatermarks(address,int16)", to:this.address}); }
  prepareExecuteOracleStopLossDecay(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"executeOracleStopLossDecay(address)", to:this.address}); }
  prepareExecuteOracleStopLossDrawdown(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"executeOracleStopLossDrawdown(address)", to:this.address}); }
  prepareExecuteOracleStopLossHighWatermarks(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"executeOracleStopLossHighWatermarks(address)", to:this.address}); }
  prepareExecuteOracleStopLossTimelock(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"executeOracleStopLossTimelock(address)", to:this.address}); }
  highWatermarks(options = {}) { return this.client.call({...options, contract:"OracleValueStopLossExtension", function:"highWatermarks(address,int16)", to:this.address}); }
  prepareInitialize(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"initialize(address,bytes)", to:this.address}); }
  oracleStopLossConfig(options = {}) { return this.client.call({...options, contract:"OracleValueStopLossExtension", function:"oracleStopLossConfig(address)", to:this.address}); }
  pendingHighWatermark(options = {}) { return this.client.call({...options, contract:"OracleValueStopLossExtension", function:"pendingHighWatermark(address)", to:this.address}); }
  poolStopLossSchedule(options = {}) { return this.client.call({...options, contract:"OracleValueStopLossExtension", function:"poolStopLossSchedule(address)", to:this.address}); }
  prepareProposeOracleStopLossDecay(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"proposeOracleStopLossDecay(address,uint256)", to:this.address}); }
  prepareProposeOracleStopLossDrawdown(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"proposeOracleStopLossDrawdown(address,uint256)", to:this.address}); }
  prepareProposeOracleStopLossHighWatermarks(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"proposeOracleStopLossHighWatermarks(address,int16,uint104,uint104)", to:this.address}); }
  prepareProposeOracleStopLossTimelock(options = {}) { return this.client.prepareCall({...options, contract:"OracleValueStopLossExtension", function:"proposeOracleStopLossTimelock(address,uint40)", to:this.address}); }
}
exports.OracleValueStopLossExtension = OracleValueStopLossExtension;

class PriceProvider {
  constructor(client, address) { this.client = client; this.address = address; }
  bpsBase(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"BPS_BASE()", to:this.address}); }
  bpsBaseU(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"BPS_BASE_U()", to:this.address}); }
  confidenceCooldown(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"CONFIDENCE_COOLDOWN()", to:this.address}); }
  confidenceMax(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"CONFIDENCE_MAX()", to:this.address}); }
  maxTimeDelta(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"MAX_TIME_DELTA()", to:this.address}); }
  baseToken(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"baseToken()", to:this.address}); }
  confidenceParam(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"confidenceParam()", to:this.address}); }
  factory(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"factory()", to:this.address}); }
  prepareGetQuote(options = {}) { return this.client.prepareCall({...options, contract:"PriceProvider", function:"getQuote()", to:this.address}); }
  lastConfidenceUpdate(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"lastConfidenceUpdate()", to:this.address}); }
  marginStep(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"marginStep()", to:this.address}); }
  offchainFeedId(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"offchainFeedId()", to:this.address}); }
  offchainOracle(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"offchainOracle()", to:this.address}); }
  quoteToken(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"quoteToken()", to:this.address}); }
  prepareSetConfidenceParam(options = {}) { return this.client.prepareCall({...options, contract:"PriceProvider", function:"setConfidenceParam(uint256)", to:this.address}); }
  token0(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"token0()", to:this.address}); }
  token1(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"token1()", to:this.address}); }
  version(options = {}) { return this.client.call({...options, contract:"PriceProvider", function:"version()", to:this.address}); }
}
exports.PriceProvider = PriceProvider;

class PriceProviderFactory {
  constructor(client, address) { this.client = client; this.address = address; }
  adminRole(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"ADMIN_ROLE()", to:this.address}); }
  defaultAdminRole(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"DEFAULT_ADMIN_ROLE()", to:this.address}); }
  prepareAddProvider(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"addProvider(address)", to:this.address}); }
  prepareCreatePriceProvider(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"createPriceProvider(address,bytes32,int256,uint256,address,address)", to:this.address}); }
  getAllProviders(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"getAllProviders(uint256,uint256)", to:this.address}); }
  getProviders(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"getProviders(address,uint256,uint256)", to:this.address}); }
  getRoleAdmin(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"getRoleAdmin(bytes32)", to:this.address}); }
  prepareGrantRole(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"grantRole(bytes32,address)", to:this.address}); }
  prepareGrantUpdater(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"grantUpdater(address,address)", to:this.address}); }
  hasRole(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"hasRole(bytes32,address)", to:this.address}); }
  isProvider(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"isProvider(address)", to:this.address}); }
  isUpdater(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"isUpdater(address,address)", to:this.address}); }
  prepareMulticall(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"multicall(bytes[])", to:this.address}); }
  providerAt(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"providerAt(uint256)", to:this.address}); }
  providerCount(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"providerCount()", to:this.address}); }
  providerCountByCreator(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"providerCountByCreator(address)", to:this.address}); }
  providerOwner(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"providerOwner(address)", to:this.address}); }
  prepareRemoveProvider(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"removeProvider(address)", to:this.address}); }
  prepareRenounceRole(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"renounceRole(bytes32,address)", to:this.address}); }
  prepareRevokeRole(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"revokeRole(bytes32,address)", to:this.address}); }
  prepareRevokeUpdater(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"revokeUpdater(address,address)", to:this.address}); }
  prepareSetConfidence(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"setConfidence(address[],uint256[])", to:this.address}); }
  supportsInterface(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"supportsInterface(bytes4)", to:this.address}); }
  prepareTransferProviderOwnership(options = {}) { return this.client.prepareCall({...options, contract:"PriceProviderFactory", function:"transferProviderOwnership(address,address)", to:this.address}); }
  version(options = {}) { return this.client.call({...options, contract:"PriceProviderFactory", function:"version()", to:this.address}); }
}
exports.PriceProviderFactory = PriceProviderFactory;

class PriceVelocityGuardExtension {
  constructor(client, address) { this.client = client; this.address = address; }
  factory(options = {}) { return this.client.call({...options, contract:"PriceVelocityGuardExtension", function:"FACTORY()", to:this.address}); }
  prepareAfterAddLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"PriceVelocityGuardExtension", function:"afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", to:this.address}); }
  prepareAfterRemoveLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"PriceVelocityGuardExtension", function:"afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", to:this.address}); }
  prepareAfterSwap(options = {}) { return this.client.prepareCall({...options, contract:"PriceVelocityGuardExtension", function:"afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)", to:this.address}); }
  prepareBeforeAddLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"PriceVelocityGuardExtension", function:"beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", to:this.address}); }
  prepareBeforeRemoveLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"PriceVelocityGuardExtension", function:"beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", to:this.address}); }
  prepareBeforeSwap(options = {}) { return this.client.prepareCall({...options, contract:"PriceVelocityGuardExtension", function:"beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)", to:this.address}); }
  prepareInitialize(options = {}) { return this.client.prepareCall({...options, contract:"PriceVelocityGuardExtension", function:"initialize(address,bytes)", to:this.address}); }
  priceVelocityState(options = {}) { return this.client.call({...options, contract:"PriceVelocityGuardExtension", function:"priceVelocityState(address)", to:this.address}); }
  prepareSetAnchorMidPrice(options = {}) { return this.client.prepareCall({...options, contract:"PriceVelocityGuardExtension", function:"setAnchorMidPrice(address,uint128)", to:this.address}); }
  prepareSetMaxChangePerBlock(options = {}) { return this.client.prepareCall({...options, contract:"PriceVelocityGuardExtension", function:"setMaxChangePerBlock(address,uint64)", to:this.address}); }
}
exports.PriceVelocityGuardExtension = PriceVelocityGuardExtension;

class PythOracle {
  constructor(client, address) { this.client = client; this.address = address; }
  adminRole(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"ADMIN_ROLE()", to:this.address}); }
  bpsBase(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"BPS_BASE()", to:this.address}); }
  defaultAdminRole(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"DEFAULT_ADMIN_ROLE()", to:this.address}); }
  stallSpread(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"STALL_SPREAD()", to:this.address}); }
  prepareAcceptStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"acceptStateGuardRole(bytes32)", to:this.address}); }
  prepareAddApprovedFactory(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"addApprovedFactory(address)", to:this.address}); }
  prepareAddIntegrator(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"addIntegrator(address)", to:this.address}); }
  approvedFactoryCount(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"approvedFactoryCount()", to:this.address}); }
  blacklisted(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"blacklisted(address)", to:this.address}); }
  decimals(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"decimals()", to:this.address}); }
  getApprovedFactories(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"getApprovedFactories(uint256,uint256)", to:this.address}); }
  getIntegrators(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"getIntegrators(uint256,uint256)", to:this.address}); }
  getOracleData(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"getOracleData(bytes32)", to:this.address}); }
  getOracleDataBulk(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"getOracleDataBulk(bytes32[])", to:this.address}); }
  getRoleAdmin(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"getRoleAdmin(bytes32)", to:this.address}); }
  prepareGrantRole(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"grantRole(bytes32,address)", to:this.address}); }
  hasRole(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"hasRole(bytes32,address)", to:this.address}); }
  integratorCount(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"integratorCount()", to:this.address}); }
  prepareIntegratorPrice(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"integratorPrice(bytes32)", to:this.address}); }
  isApprovedFactory(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"isApprovedFactory(address)", to:this.address}); }
  isIntegrator(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"isIntegrator(address)", to:this.address}); }
  kind(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"kind()", to:this.address}); }
  prepareMulticall(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"multicall(bytes[])", to:this.address}); }
  pendingStateGuard(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"pendingStateGuard(bytes32)", to:this.address}); }
  preparePrice(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"price(bytes32,address)", to:this.address}); }
  priceGuard(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"priceGuard(bytes32)", to:this.address}); }
  preparePurgePendingStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"purgePendingStateGuardRole(bytes32)", to:this.address}); }
  preparePurgeStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"purgeStateGuardRole(bytes32)", to:this.address}); }
  prepareRegister(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"register(bytes32,address,address)", to:this.address}); }
  registeredPool(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"registeredPool(bytes32,address)", to:this.address}); }
  registrationFee(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"registrationFee()", to:this.address}); }
  prepareRemoveApprovedFactory(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"removeApprovedFactory(address)", to:this.address}); }
  prepareRemoveIntegrator(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"removeIntegrator(address)", to:this.address}); }
  prepareRenounceRole(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"renounceRole(bytes32,address)", to:this.address}); }
  prepareRevokeRole(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"revokeRole(bytes32,address)", to:this.address}); }
  prepareSetBlacklist(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"setBlacklist(address,bool)", to:this.address}); }
  prepareSetIntegrators(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"setIntegrators(address[],bool)", to:this.address}); }
  prepareSetPriceGuard(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"setPriceGuard(bytes32,uint128,uint128)", to:this.address}); }
  prepareSetRegistrationFee(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"setRegistrationFee(uint256)", to:this.address}); }
  prepareSetStateGuardRole(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"setStateGuardRole(bytes32,address)", to:this.address}); }
  stateGuard(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"stateGuard(bytes32)", to:this.address}); }
  supportsInterface(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"supportsInterface(bytes4)", to:this.address}); }
  version(options = {}) { return this.client.call({...options, contract:"PythOracle", function:"version()", to:this.address}); }
  prepareWithdrawEth(options = {}) { return this.client.prepareCall({...options, contract:"PythOracle", function:"withdrawEth()", to:this.address}); }
}
exports.PythOracle = PythOracle;

class RobinhoodStockTokenParams {
  constructor(client, address) { this.client = client; this.address = address; }
  smallChangeBps(options = {}) { return this.client.call({...options, contract:"RobinhoodStockTokenParams", function:"SMALL_CHANGE_BPS()", to:this.address}); }
  getOracleParams(options = {}) { return this.client.call({...options, contract:"RobinhoodStockTokenParams", function:"getOracleParams(address)", to:this.address}); }
  version(options = {}) { return this.client.call({...options, contract:"RobinhoodStockTokenParams", function:"version()", to:this.address}); }
}
exports.RobinhoodStockTokenParams = RobinhoodStockTokenParams;

class SwapAllowlistExtension {
  constructor(client, address) { this.client = client; this.address = address; }
  factory(options = {}) { return this.client.call({...options, contract:"SwapAllowlistExtension", function:"FACTORY()", to:this.address}); }
  prepareAfterAddLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"SwapAllowlistExtension", function:"afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", to:this.address}); }
  prepareAfterRemoveLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"SwapAllowlistExtension", function:"afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", to:this.address}); }
  prepareAfterSwap(options = {}) { return this.client.prepareCall({...options, contract:"SwapAllowlistExtension", function:"afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)", to:this.address}); }
  allowAllSwappers(options = {}) { return this.client.call({...options, contract:"SwapAllowlistExtension", function:"allowAllSwappers(address)", to:this.address}); }
  allowedSwapper(options = {}) { return this.client.call({...options, contract:"SwapAllowlistExtension", function:"allowedSwapper(address,address)", to:this.address}); }
  prepareBeforeAddLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"SwapAllowlistExtension", function:"beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", to:this.address}); }
  prepareBeforeRemoveLiquidity(options = {}) { return this.client.prepareCall({...options, contract:"SwapAllowlistExtension", function:"beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", to:this.address}); }
  beforeSwap(options = {}) { return this.client.call({...options, contract:"SwapAllowlistExtension", function:"beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)", to:this.address}); }
  prepareInitialize(options = {}) { return this.client.prepareCall({...options, contract:"SwapAllowlistExtension", function:"initialize(address,bytes)", to:this.address}); }
  isAllowedToSwap(options = {}) { return this.client.call({...options, contract:"SwapAllowlistExtension", function:"isAllowedToSwap(address,address)", to:this.address}); }
  prepareSetAllowAllSwappers(options = {}) { return this.client.prepareCall({...options, contract:"SwapAllowlistExtension", function:"setAllowAllSwappers(address,bool)", to:this.address}); }
  prepareSetAllowedToSwap(options = {}) { return this.client.prepareCall({...options, contract:"SwapAllowlistExtension", function:"setAllowedToSwap(address,address,bool)", to:this.address}); }
}
exports.SwapAllowlistExtension = SwapAllowlistExtension;


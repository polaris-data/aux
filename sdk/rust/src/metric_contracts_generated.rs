// Named Metric contract calls backed by the Zig ABI registry.
use crate::{MetricChainClient, Result};
use serde_json::Value;

pub struct Contracts<'a>(pub(crate) &'a MetricChainClient);
pub struct ReadOptions<'a> { pub block: &'a str, pub sender: Option<&'a str>, pub value: &'a str }
impl Default for ReadOptions<'_> { fn default() -> Self { Self { block: "latest", sender: None, value: "0x0" } } }
impl MetricChainClient { pub fn contracts(&self) -> Contracts<'_> { Contracts(self) } }

impl<'a> Contracts<'a> { pub fn anchored_price_provider(&self, address: &'a str) -> AnchoredPriceProvider<'a> { AnchoredPriceProvider(self.0,address) } }
pub struct AnchoredPriceProvider<'a>(&'a MetricChainClient,&'a str);
impl AnchoredPriceProvider<'_> {
    pub fn bps_base(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","BPS_BASE()",self.1,args,options.block,options.sender,options.value) }
    pub fn bps_base_u(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","BPS_BASE_U()",self.1,args,options.block,options.sender,options.value) }
    pub fn confidence_cooldown(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","CONFIDENCE_COOLDOWN()",self.1,args,options.block,options.sender,options.value) }
    pub fn confidence_max(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","CONFIDENCE_MAX()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_ref_staleness(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","MAX_REF_STALENESS()",self.1,args,options.block,options.sender,options.value) }
    pub fn mutable_params(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","MUTABLE_PARAMS()",self.1,args,options.block,options.sender,options.value) }
    pub fn source_gas_limit(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","SOURCE_GAS_LIMIT()",self.1,args,options.block,options.sender,options.value) }
    pub fn base_feed_id(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","baseFeedId()",self.1,args,options.block,options.sender,options.value) }
    pub fn base_max_staleness(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","baseMaxStaleness()",self.1,args,options.block,options.sender,options.value) }
    pub fn base_spread_mult(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","baseSpreadMult()",self.1,args,options.block,options.sender,options.value) }
    pub fn base_token(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","baseToken()",self.1,args,options.block,options.sender,options.value) }
    pub fn confidence_param(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","confidenceParam()",self.1,args,options.block,options.sender,options.value) }
    pub fn factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","factory()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_get_quote(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredPriceProvider","getQuote()",self.1,args,value) }
    pub fn last_confidence_update(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","lastConfidenceUpdate()",self.1,args,options.block,options.sender,options.value) }
    pub fn last_spread_mult_update(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","lastSpreadMultUpdate()",self.1,args,options.block,options.sender,options.value) }
    pub fn last_staleness_update(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","lastStalenessUpdate()",self.1,args,options.block,options.sender,options.value) }
    pub fn margin_step(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","marginStep()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_spread_mult(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","maxSpreadMult()",self.1,args,options.block,options.sender,options.value) }
    pub fn min_margin(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","minMargin()",self.1,args,options.block,options.sender,options.value) }
    pub fn offchain_oracle(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","offchainOracle()",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_feed_id(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","quoteFeedId()",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_max_staleness(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","quoteMaxStaleness()",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_spread_mult(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","quoteSpreadMult()",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_token(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","quoteToken()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_set_confidence_param(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredPriceProvider","setConfidenceParam(uint256)",self.1,args,value) }
    pub fn prepare_set_max_staleness(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredPriceProvider","setMaxStaleness(uint256,uint256)",self.1,args,value) }
    pub fn prepare_set_source(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredPriceProvider","setSource(address)",self.1,args,value) }
    pub fn prepare_set_spread_multipliers(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredPriceProvider","setSpreadMultipliers(uint256,uint256)",self.1,args,value) }
    pub fn source(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","source()",self.1,args,options.block,options.sender,options.value) }
    pub fn token0(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","token0()",self.1,args,options.block,options.sender,options.value) }
    pub fn token1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","token1()",self.1,args,options.block,options.sender,options.value) }
    pub fn version(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredPriceProvider","version()",self.1,args,options.block,options.sender,options.value) }
}

impl<'a> Contracts<'a> { pub fn anchored_provider_factory(&self, address: &'a str) -> AnchoredProviderFactory<'a> { AnchoredProviderFactory(self.0,address) } }
pub struct AnchoredProviderFactory<'a>(&'a MetricChainClient,&'a str);
impl AnchoredProviderFactory<'_> {
    pub fn admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn default_admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","DEFAULT_ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn default_class(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","DEFAULT_CLASS()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_add_oracle(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","addOracle(address)",self.1,args,value) }
    pub fn prepare_create_anchored_provider(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","createAnchoredProvider(address,bytes32,bytes32,uint256,uint256,uint256,bool,int256,address,address,(address,uint256,uint256,uint256,uint256))",self.1,args,value) }
    pub fn envelopes(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","envelopes(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn feed_class(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","feedClass(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_all_providers(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","getAllProviders(uint256,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_oracles(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","getOracles(uint256,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_providers(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","getProviders(address,uint256,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_role_admin(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","getRoleAdmin(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_grant_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","grantRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_grant_updater(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","grantUpdater(address,address)",self.1,args,value) }
    pub fn has_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","hasRole(bytes32,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn is_oracle(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","isOracle(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn is_provider(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","isProvider(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn is_updater(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","isUpdater(address,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_multicall(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","multicall(bytes[])",self.1,args,value) }
    pub fn oracle_count(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","oracleCount()",self.1,args,options.block,options.sender,options.value) }
    pub fn provider_at(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","providerAt(uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn provider_count(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","providerCount()",self.1,args,options.block,options.sender,options.value) }
    pub fn provider_count_by_creator(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","providerCountByCreator(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn provider_owner(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","providerOwner(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_remove_envelope(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","removeEnvelope(bytes32)",self.1,args,value) }
    pub fn prepare_remove_oracle(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","removeOracle(address)",self.1,args,value) }
    pub fn prepare_renounce_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","renounceRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_revoke_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","revokeRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_revoke_updater(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","revokeUpdater(address,address)",self.1,args,value) }
    pub fn prepare_set_confidence(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","setConfidence(address[],uint256[])",self.1,args,value) }
    pub fn prepare_set_envelope(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","setEnvelope(bytes32,(uint256,uint256,uint256,uint256,uint256,bool))",self.1,args,value) }
    pub fn prepare_set_feed_class(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","setFeedClass(bytes32,bytes32)",self.1,args,value) }
    pub fn prepare_set_max_staleness(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","setMaxStaleness(address[],uint256[],uint256[])",self.1,args,value) }
    pub fn prepare_set_source(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","setSource(address,address)",self.1,args,value) }
    pub fn prepare_set_spread_multipliers(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","setSpreadMultipliers(address[],uint256[],uint256[])",self.1,args,value) }
    pub fn supports_interface(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","supportsInterface(bytes4)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_transfer_provider_ownership(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("AnchoredProviderFactory","transferProviderOwnership(address,address)",self.1,args,value) }
    pub fn version(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("AnchoredProviderFactory","version()",self.1,args,options.block,options.sender,options.value) }
}

impl<'a> Contracts<'a> { pub fn b20_asset_params(&self, address: &'a str) -> B20AssetParams<'a> { B20AssetParams(self.0,address) } }
pub struct B20AssetParams<'a>(&'a MetricChainClient,&'a str);
impl B20AssetParams<'_> {
    pub fn registry(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("B20AssetParams","REGISTRY()",self.1,args,options.block,options.sender,options.value) }
    pub fn small_change_bps(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("B20AssetParams","SMALL_CHANGE_BPS()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_oracle_params(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("B20AssetParams","getOracleParams(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn version(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("B20AssetParams","version()",self.1,args,options.block,options.sender,options.value) }
}

impl<'a> Contracts<'a> { pub fn chainlink_oracle(&self, address: &'a str) -> ChainlinkOracle<'a> { ChainlinkOracle(self.0,address) } }
pub struct ChainlinkOracle<'a>(&'a MetricChainClient,&'a str);
impl ChainlinkOracle<'_> {
    pub fn admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn bps_base(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","BPS_BASE()",self.1,args,options.block,options.sender,options.value) }
    pub fn default_admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","DEFAULT_ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn stall_spread(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","STALL_SPREAD()",self.1,args,options.block,options.sender,options.value) }
    pub fn tag_v050(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","TAG_V050()",self.1,args,options.block,options.sender,options.value) }
    pub fn tag_v051(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","TAG_V051()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_accept_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","acceptStateGuardRole(bytes32)",self.1,args,value) }
    pub fn prepare_add_approved_factory(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","addApprovedFactory(address)",self.1,args,value) }
    pub fn prepare_add_integrator(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","addIntegrator(address)",self.1,args,value) }
    pub fn approved_factory_count(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","approvedFactoryCount()",self.1,args,options.block,options.sender,options.value) }
    pub fn blacklisted(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","blacklisted(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn decimals(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","decimals()",self.1,args,options.block,options.sender,options.value) }
    pub fn fee_token(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","feeToken()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_approved_factories(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","getApprovedFactories(uint256,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_integrators(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","getIntegrators(uint256,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_oracle_data(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","getOracleData(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_oracle_data_bulk(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","getOracleDataBulk(bytes32[])",self.1,args,options.block,options.sender,options.value) }
    pub fn get_role_admin(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","getRoleAdmin(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_grant_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","grantRole(bytes32,address)",self.1,args,value) }
    pub fn has_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","hasRole(bytes32,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn integrator_count(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","integratorCount()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_integrator_price(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","integratorPrice(bytes32)",self.1,args,value) }
    pub fn is_approved_factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","isApprovedFactory(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn is_integrator(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","isIntegrator(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn kind(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","kind()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_multicall(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","multicall(bytes[])",self.1,args,value) }
    pub fn pending_state_guard(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","pendingStateGuard(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_price(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","price(bytes32,address)",self.1,args,value) }
    pub fn price_guard(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","priceGuard(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_purge_pending_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","purgePendingStateGuardRole(bytes32)",self.1,args,value) }
    pub fn prepare_purge_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","purgeStateGuardRole(bytes32)",self.1,args,value) }
    pub fn prepare_register(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","register(bytes32,address,address)",self.1,args,value) }
    pub fn registered_pool(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","registeredPool(bytes32,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn registration_fee(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","registrationFee()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_remove_approved_factory(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","removeApprovedFactory(address)",self.1,args,value) }
    pub fn prepare_remove_integrator(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","removeIntegrator(address)",self.1,args,value) }
    pub fn prepare_remove_verifier(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","removeVerifier(uint8)",self.1,args,value) }
    pub fn prepare_renounce_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","renounceRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_revoke_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","revokeRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_set_blacklist(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","setBlacklist(address,bool)",self.1,args,value) }
    pub fn prepare_set_integrators(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","setIntegrators(address[],bool)",self.1,args,value) }
    pub fn prepare_set_price_guard(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","setPriceGuard(bytes32,uint128,uint128)",self.1,args,value) }
    pub fn prepare_set_registration_fee(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","setRegistrationFee(uint256)",self.1,args,value) }
    pub fn prepare_set_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","setStateGuardRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_set_verifier(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","setVerifier(uint8,address,bool)",self.1,args,value) }
    pub fn state_guard(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","stateGuard(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn supports_interface(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","supportsInterface(bytes4)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_update_report(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","updateReport(bytes)",self.1,args,value) }
    pub fn prepare_update_reports(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","updateReports(bytes[])",self.1,args,value) }
    pub fn verifier_by_tag(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","verifierByTag(uint8)",self.1,args,options.block,options.sender,options.value) }
    pub fn verifier_tags(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","verifierTags()",self.1,args,options.block,options.sender,options.value) }
    pub fn version(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ChainlinkOracle","version()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_withdraw_eth(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ChainlinkOracle","withdrawEth()",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn compressed_oracle_v1(&self, address: &'a str) -> CompressedOracleV1<'a> { CompressedOracleV1(self.0,address) } }
pub struct CompressedOracleV1<'a>(&'a MetricChainClient,&'a str);
impl CompressedOracleV1<'_> {
    pub fn admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn bps_base(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","BPS_BASE()",self.1,args,options.block,options.sender,options.value) }
    pub fn default_admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","DEFAULT_ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn stall_spread(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","STALL_SPREAD()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_accept_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","acceptStateGuardRole(bytes32)",self.1,args,value) }
    pub fn prepare_allow_contract_pushers(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","allowContractPushers(address[])",self.1,args,value) }
    pub fn prepare_allow_pushers(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","allowPushers(uint256,address[],bytes[])",self.1,args,value) }
    pub fn decimals(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","decimals()",self.1,args,options.block,options.sender,options.value) }
    pub fn extsload_overload_1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","extsload(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn extsload_overload_2(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","extsload(bytes32,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn extsload_overload_3(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","extsload(bytes32[])",self.1,args,options.block,options.sender,options.value) }
    pub fn feed_id_of(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","feedIdOf(address,uint8,uint8)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_compressed_oracle(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","getCompressedOracle(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_oracle_data(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","getOracleData(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_oracle_data_bulk(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","getOracleDataBulk(bytes32[])",self.1,args,options.block,options.sender,options.value) }
    pub fn get_role_admin(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","getRoleAdmin(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_slot_layout(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","getSlotLayout(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_grant_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","grantRole(bytes32,address)",self.1,args,value) }
    pub fn has_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","hasRole(bytes32,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn kind(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","kind()",self.1,args,options.block,options.sender,options.value) }
    pub fn namespace_remapping(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","namespaceRemapping(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn pending_state_guard(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","pendingStateGuard(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_price(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","price(bytes32,address)",self.1,args,value) }
    pub fn price_guard(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","priceGuard(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_purge_pending_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","purgePendingStateGuardRole(bytes32)",self.1,args,value) }
    pub fn prepare_purge_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","purgeStateGuardRole(bytes32)",self.1,args,value) }
    pub fn prepare_remove_pushers(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","removePushers(address[])",self.1,args,value) }
    pub fn prepare_renounce_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","renounceRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_revoke_pusher(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","revokePusher()",self.1,args,value) }
    pub fn prepare_revoke_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","revokeRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_set_pending_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","setPendingStateGuardRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_set_price_guard(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","setPriceGuard(bytes32,uint128,uint128)",self.1,args,value) }
    pub fn state_guard(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","stateGuard(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn supports_interface(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","supportsInterface(bytes4)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_update_by_signature(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("CompressedOracleV1","updateBySignature(address,uint256,bytes)",self.1,args,value) }
    pub fn used_pusher_deadline(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","usedPusherDeadline(address,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn version(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("CompressedOracleV1","version()",self.1,args,options.block,options.sender,options.value) }
}

impl<'a> Contracts<'a> { pub fn deposit_allowlist_extension(&self, address: &'a str) -> DepositAllowlistExtension<'a> { DepositAllowlistExtension(self.0,address) } }
pub struct DepositAllowlistExtension<'a>(&'a MetricChainClient,&'a str);
impl DepositAllowlistExtension<'_> {
    pub fn factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("DepositAllowlistExtension","FACTORY()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_after_add_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("DepositAllowlistExtension","afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)",self.1,args,value) }
    pub fn prepare_after_remove_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("DepositAllowlistExtension","afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)",self.1,args,value) }
    pub fn prepare_after_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("DepositAllowlistExtension","afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)",self.1,args,value) }
    pub fn allow_all_depositors(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("DepositAllowlistExtension","allowAllDepositors(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn allowed_depositor(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("DepositAllowlistExtension","allowedDepositor(address,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn before_add_liquidity(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("DepositAllowlistExtension","beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_before_remove_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("DepositAllowlistExtension","beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)",self.1,args,value) }
    pub fn prepare_before_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("DepositAllowlistExtension","beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)",self.1,args,value) }
    pub fn prepare_initialize(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("DepositAllowlistExtension","initialize(address,bytes)",self.1,args,value) }
    pub fn is_allowed_to_deposit(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("DepositAllowlistExtension","isAllowedToDeposit(address,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_set_allow_all_depositors(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("DepositAllowlistExtension","setAllowAllDepositors(address,bool)",self.1,args,value) }
    pub fn prepare_set_allowed_to_deposit(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("DepositAllowlistExtension","setAllowedToDeposit(address,address,bool)",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn external_swap_executor(&self, address: &'a str) -> ExternalSwapExecutor<'a> { ExternalSwapExecutor(self.0,address) } }
pub struct ExternalSwapExecutor<'a>(&'a MetricChainClient,&'a str);
impl ExternalSwapExecutor<'_> {
    pub fn router(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ExternalSwapExecutor","router()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ExternalSwapExecutor","swap((address,address,uint128,uint128,address,bytes,address,uint256))",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn ierc20(&self, address: &'a str) -> IERC20<'a> { IERC20(self.0,address) } }
pub struct IERC20<'a>(&'a MetricChainClient,&'a str);
impl IERC20<'_> {
    pub fn allowance(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("IERC20","allowance(address,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_approve(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("IERC20","approve(address,uint256)",self.1,args,value) }
    pub fn balance_of(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("IERC20","balanceOf(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn total_supply(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("IERC20","totalSupply()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_transfer(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("IERC20","transfer(address,uint256)",self.1,args,value) }
    pub fn prepare_transfer_from(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("IERC20","transferFrom(address,address,uint256)",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn immutable_app_pool_helper(&self, address: &'a str) -> ImmutableAppPoolHelper<'a> { ImmutableAppPoolHelper(self.0,address) } }
pub struct ImmutableAppPoolHelper<'a>(&'a MetricChainClient,&'a str);
impl ImmutableAppPoolHelper<'_> {
    pub fn app_factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ImmutableAppPoolHelper","APP_FACTORY()",self.1,args,options.block,options.sender,options.value) }
    pub fn pool_factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("ImmutableAppPoolHelper","POOL_FACTORY()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_create_pool(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ImmutableAppPoolHelper","createPool((address,address,address,address[],(uint256,uint256,uint256,uint256,uint256,uint256),bytes[],uint256,address,uint256,uint256,uint256,uint24,uint24,uint24,uint24,address,int24,uint256[],uint256[],bytes32,uint16),uint16)",self.1,args,value) }
    pub fn prepare_set_pool_bid_ask_ref_asymmetry_x8(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("ImmutableAppPoolHelper","setPoolBidAskRefAsymmetryX8(address,uint16)",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn metric_omm_pool(&self, address: &'a str) -> MetricOmmPool<'a> { MetricOmmPool(self.0,address) } }
pub struct MetricOmmPool<'a>(&'a MetricChainClient,&'a str);
impl MetricOmmPool<'_> {
    pub fn prepare_add_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","addLiquidity(address,uint80,(int256[],uint256[]),bytes,bytes)",self.1,args,value) }
    pub fn prepare_collect_fees(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","collectFees(uint256,uint256,uint256,uint256,address,address)",self.1,args,value) }
    pub fn extsload_overload_1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPool","extsload(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn extsload_overload_2(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPool","extsload(bytes32,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn extsload_overload_3(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPool","extsload(bytes32[])",self.1,args,options.block,options.sender,options.value) }
    pub fn get_immutables(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPool","getImmutables()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_get_sell_and_buy_prices(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","getSellAndBuyPrices()",self.1,args,value) }
    pub fn in_swap(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPool","inSwap()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_remove_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","removeLiquidity(address,uint80,(int256[],uint256[]),bytes)",self.1,args,value) }
    pub fn prepare_set_bid_ask_ref_asymmetry_x8(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","setBidAskRefAsymmetryX8(uint16)",self.1,args,value) }
    pub fn prepare_set_bin_additional_fees(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","setBinAdditionalFees(int16,uint16,uint16)",self.1,args,value) }
    pub fn prepare_set_pause(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","setPause(uint8)",self.1,args,value) }
    pub fn prepare_set_pool_fees(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","setPoolFees(uint24,uint24)",self.1,args,value) }
    pub fn prepare_set_price_provider(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","setPriceProvider(address)",self.1,args,value) }
    pub fn prepare_simulate_swap_and_revert(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","simulateSwapAndRevert(address,address,bool,int128,uint128,uint128,uint128,uint128,bytes)",self.1,args,value) }
    pub fn prepare_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPool","swap(address,bool,int128,uint128,bytes,bytes)",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn metric_omm_pool_data_provider(&self, address: &'a str) -> MetricOmmPoolDataProvider<'a> { MetricOmmPoolDataProvider(self.0,address) } }
pub struct MetricOmmPoolDataProvider<'a>(&'a MetricChainClient,&'a str);
impl MetricOmmPoolDataProvider<'_> {
    pub fn bin_state(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","binState(address,int16)",self.1,args,options.block,options.sender,options.value) }
    pub fn bin_states(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","binStates(address,int16[])",self.1,args,options.block,options.sender,options.value) }
    pub fn bin_total_shares_overload_1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","binTotalShares(address,int16[])",self.1,args,options.block,options.sender,options.value) }
    pub fn bin_total_shares_overload_2(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","binTotalShares(address,int16)",self.1,args,options.block,options.sender,options.value) }
    pub fn distance_from_provided_price_x64(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","distanceFromProvidedPriceX64(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_get_liquidity_depth_hypothetical(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolDataProvider","getLiquidityDepthHypothetical(address,uint8,uint128,uint128,uint128)",self.1,args,value) }
    pub fn prepare_get_liquidity_depth_live(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolDataProvider","getLiquidityDepthLive(address,uint8)",self.1,args,value) }
    pub fn position_bin_shares_overload_1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","positionBinShares(address,bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn position_bin_shares_overload_2(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","positionBinShares(address,address,uint80,int16)",self.1,args,options.block,options.sender,options.value) }
    pub fn price_provider(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","priceProvider(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn slot0(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","slot0(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn slot1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","slot1(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn slot2(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDataProvider","slot2(address)",self.1,args,options.block,options.sender,options.value) }
}

impl<'a> Contracts<'a> { pub fn metric_omm_pool_deployer(&self, address: &'a str) -> MetricOmmPoolDeployer<'a> { MetricOmmPoolDeployer(self.0,address) } }
pub struct MetricOmmPoolDeployer<'a>(&'a MetricChainClient,&'a str);
impl MetricOmmPoolDeployer<'_> {
    pub fn factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolDeployer","FACTORY()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_deploy(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolDeployer","deploy((bytes32,address,address,address,address,(address,address,address,address,address,address,address),(uint256,uint256,uint256,uint256,uint256,uint256),bool,uint256,uint256,uint256,uint256,uint256,uint24,int24,(uint104,uint104,uint16,uint16,uint16)[],(uint104,uint104,uint16,uint16,uint16)[],uint24,uint16))",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn metric_omm_pool_factory(&self, address: &'a str) -> MetricOmmPoolFactory<'a> { MetricOmmPoolFactory(self.0,address) } }
pub struct MetricOmmPoolFactory<'a>(&'a MetricChainClient,&'a str);
impl MetricOmmPoolFactory<'_> {
    pub fn prepare_accept_ownership(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","acceptOwnership()",self.1,args,value) }
    pub fn prepare_accept_pool_admin(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","acceptPoolAdmin(address)",self.1,args,value) }
    pub fn authority(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","authority()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_cancel_pool_admin_transfer(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","cancelPoolAdminTransfer(address)",self.1,args,value) }
    pub fn prepare_collect_eth(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","collectEth(address,uint256)",self.1,args,value) }
    pub fn prepare_collect_pool_fees(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","collectPoolFees(address)",self.1,args,value) }
    pub fn prepare_collect_tokens(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","collectTokens(address,address,uint256)",self.1,args,value) }
    pub fn prepare_create_pool(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","createPool((address,address,address,address[],(uint256,uint256,uint256,uint256,uint256,uint256),bytes[],uint256,address,uint256,uint256,uint256,uint24,uint24,uint24,uint24,address,int24,uint256[],uint256[],bytes32,uint16))",self.1,args,value) }
    pub fn prepare_execute_pool_price_provider_update(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","executePoolPriceProviderUpdate(address)",self.1,args,value) }
    pub fn fee_receiver(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","feeReceiver()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_fee_caps(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","getFeeCaps()",self.1,args,options.block,options.sender,options.value) }
    pub fn hard_max_bin_distance_limit_e6(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","hardMaxBinDistanceLimitE6()",self.1,args,options.block,options.sender,options.value) }
    pub fn hard_min_bin_distance_limit_e6(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","hardMinBinDistanceLimitE6()",self.1,args,options.block,options.sender,options.value) }
    pub fn idx_to_pool(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","idxToPool(uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn is_pool(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","isPool(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn max_admin_notional_fee_e8(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","maxAdminNotionalFeeE8()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_admin_spread_fee_e6(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","maxAdminSpreadFeeE6()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_bin_distance_e6(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","maxBinDistanceE6()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_owner_admin_notional_cap_e8(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","maxOwnerAdminNotionalCapE8()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_owner_admin_spread_cap_e6(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","maxOwnerAdminSpreadCapE6()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_owner_protocol_notional_cap_e8(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","maxOwnerProtocolNotionalCapE8()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_owner_protocol_spread_cap_e6(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","maxOwnerProtocolSpreadCapE6()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_protocol_notional_fee_e8(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","maxProtocolNotionalFeeE8()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_protocol_spread_fee_e6(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","maxProtocolSpreadFeeE6()",self.1,args,options.block,options.sender,options.value) }
    pub fn min_bin_distance_e6(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","minBinDistanceE6()",self.1,args,options.block,options.sender,options.value) }
    pub fn next_pool_idx(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","nextPoolIdx()",self.1,args,options.block,options.sender,options.value) }
    pub fn on_creation_max_bid_ask_ref_asymmetry_x8(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","onCreationMaxBidAskRefAsymmetryX8()",self.1,args,options.block,options.sender,options.value) }
    pub fn owner(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","owner()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_pause_pool(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","pausePool(address)",self.1,args,value) }
    pub fn pending_owner(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","pendingOwner()",self.1,args,options.block,options.sender,options.value) }
    pub fn pending_pool_admin(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","pendingPoolAdmin(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn pending_price_provider(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","pendingPriceProvider(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn pending_price_provider_execute_after(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","pendingPriceProviderExecuteAfter(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn pool_admin(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","poolAdmin(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn pool_admin_fee_destination(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","poolAdminFeeDestination(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn pool_deployer(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","poolDeployer()",self.1,args,options.block,options.sender,options.value) }
    pub fn pool_fee_config(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","poolFeeConfig(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn pool_to_idx(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","poolToIdx(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn price_provider_timelock(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","priceProviderTimelock(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_propose_pool_admin_transfer(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","proposePoolAdminTransfer(address,address)",self.1,args,value) }
    pub fn prepare_propose_pool_price_provider(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","proposePoolPriceProvider(address,address)",self.1,args,value) }
    pub fn protocol_notional_fee_e8(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","protocolNotionalFeeE8()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_protocol_pause_pool(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","protocolPausePool(address)",self.1,args,value) }
    pub fn prepare_protocol_unpause_pool(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","protocolUnpausePool(address)",self.1,args,value) }
    pub fn prepare_set_authority(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setAuthority(address)",self.1,args,value) }
    pub fn prepare_set_bin_distance_limits(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setBinDistanceLimits(int128,int128)",self.1,args,value) }
    pub fn prepare_set_default_protocol_notional_fee_e8(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setDefaultProtocolNotionalFeeE8(uint24)",self.1,args,value) }
    pub fn prepare_set_default_spread_protocol_fee_e6(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setDefaultSpreadProtocolFeeE6(uint24)",self.1,args,value) }
    pub fn prepare_set_fee_caps(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setFeeCaps(uint24,uint24,uint24,uint24)",self.1,args,value) }
    pub fn prepare_set_fee_receiver(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setFeeReceiver(address)",self.1,args,value) }
    pub fn prepare_set_on_creation_max_bid_ask_ref_asymmetry_x8(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setOnCreationMaxBidAskRefAsymmetryX8(uint16)",self.1,args,value) }
    pub fn prepare_set_pool_admin_fee_destination(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setPoolAdminFeeDestination(address,address)",self.1,args,value) }
    pub fn prepare_set_pool_admin_fees(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setPoolAdminFees(address,uint24,uint24)",self.1,args,value) }
    pub fn prepare_set_pool_bid_ask_ref_asymmetry_x8(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setPoolBidAskRefAsymmetryX8(address,uint16)",self.1,args,value) }
    pub fn prepare_set_pool_bid_ask_ref_asymmetry_x8_batch(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setPoolBidAskRefAsymmetryX8Batch(address[],uint16[])",self.1,args,value) }
    pub fn prepare_set_pool_bin_additional_fees(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setPoolBinAdditionalFees(address,int16,uint16,uint16)",self.1,args,value) }
    pub fn prepare_set_pool_deployer(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setPoolDeployer(address)",self.1,args,value) }
    pub fn prepare_set_pool_protocol_fee(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setPoolProtocolFee(address,uint24,uint24)",self.1,args,value) }
    pub fn prepare_set_pool_protocol_fee_batch(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","setPoolProtocolFeeBatch(address[],uint24[],uint24[])",self.1,args,value) }
    pub fn spread_protocol_fee_e6(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmPoolFactory","spreadProtocolFeeE6()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_transfer_ownership(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","transferOwnership(address)",self.1,args,value) }
    pub fn prepare_unpause_pool(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolFactory","unpausePool(address)",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn metric_omm_pool_liquidity_adder(&self, address: &'a str) -> MetricOmmPoolLiquidityAdder<'a> { MetricOmmPoolLiquidityAdder(self.0,address) } }
pub struct MetricOmmPoolLiquidityAdder<'a>(&'a MetricChainClient,&'a str);
impl MetricOmmPoolLiquidityAdder<'_> {
    pub fn prepare_add_liquidity_exact_shares_overload_1(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolLiquidityAdder","addLiquidityExactShares(address,address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)",self.1,args,value) }
    pub fn prepare_add_liquidity_exact_shares_overload_2(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolLiquidityAdder","addLiquidityExactShares(address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)",self.1,args,value) }
    pub fn prepare_add_liquidity_weighted_overload_1(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolLiquidityAdder","addLiquidityWeighted(address,address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)",self.1,args,value) }
    pub fn prepare_add_liquidity_weighted_overload_2(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolLiquidityAdder","addLiquidityWeighted(address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)",self.1,args,value) }
    pub fn prepare_metric_omm_modify_liquidity_callback(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolLiquidityAdder","metricOmmModifyLiquidityCallback(uint256,uint256,bytes)",self.1,args,value) }
    pub fn prepare_multicall(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolLiquidityAdder","multicall(bytes[])",self.1,args,value) }
    pub fn prepare_refund_eth(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolLiquidityAdder","refundETH()",self.1,args,value) }
    pub fn prepare_sweep_token(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolLiquidityAdder","sweepToken(address,uint256,address)",self.1,args,value) }
    pub fn prepare_unwrap_weth9(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmPoolLiquidityAdder","unwrapWETH9(uint256,address)",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn metric_omm_simple_router(&self, address: &'a str) -> MetricOmmSimpleRouter<'a> { MetricOmmSimpleRouter(self.0,address) } }
pub struct MetricOmmSimpleRouter<'a>(&'a MetricChainClient,&'a str);
impl MetricOmmSimpleRouter<'_> {
    pub fn prepare_exact_input(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","exactInput((address[],address[],bytes[],uint256,uint128,uint128,address,uint256))",self.1,args,value) }
    pub fn prepare_exact_input_single(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","exactInputSingle((address,address,address,bool,uint128,uint128,address,uint256,uint128,bytes))",self.1,args,value) }
    pub fn prepare_exact_output(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","exactOutput((address[],address[],bytes[],uint256,uint128,uint128,address,uint256))",self.1,args,value) }
    pub fn prepare_exact_output_single(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","exactOutputSingle((address,address,address,bool,uint128,uint128,address,uint256,uint128,bytes))",self.1,args,value) }
    pub fn prepare_external_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","externalSwap(address,bool,bool,(address,address,uint128,uint128,address,bytes,address,uint256))",self.1,args,value) }
    pub fn prepare_metric_omm_swap_callback(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","metricOmmSwapCallback(int256,int256,bytes)",self.1,args,value) }
    pub fn prepare_multicall(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","multicall(bytes[])",self.1,args,value) }
    pub fn prepare_refund_eth(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","refundETH()",self.1,args,value) }
    pub fn prepare_self_permit(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)",self.1,args,value) }
    pub fn prepare_self_permit_allowed(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)",self.1,args,value) }
    pub fn prepare_self_permit_allowed_if_necessary(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)",self.1,args,value) }
    pub fn prepare_self_permit_if_necessary(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)",self.1,args,value) }
    pub fn prepare_sequence(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","sequence((bytes,uint8,uint8)[])",self.1,args,value) }
    pub fn prepare_sweep_token(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","sweepToken(address,uint256,address)",self.1,args,value) }
    pub fn prepare_unwrap_weth9(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MetricOmmSimpleRouter","unwrapWETH9(uint256,address)",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn metric_omm_swap_quoter(&self, address: &'a str) -> MetricOmmSwapQuoter<'a> { MetricOmmSwapQuoter(self.0,address) } }
pub struct MetricOmmSwapQuoter<'a>(&'a MetricChainClient,&'a str);
impl MetricOmmSwapQuoter<'_> {
    pub fn quote_hypothetical_exact_input(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteHypotheticalExactInput(address,(address[],bytes[],uint256,uint128,uint128[],uint128[],uint128[]))",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_hypothetical_exact_input_single_overload_1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteHypotheticalExactInputSingle(address,address,bool,uint128,uint128,uint128,uint128,uint128)",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_hypothetical_exact_input_single_overload_2(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteHypotheticalExactInputSingle(address,address,address,bool,uint128,uint128,uint128,uint128,uint128,bytes)",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_hypothetical_exact_output(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteHypotheticalExactOutput(address,(address[],bytes[],uint256,uint128,uint128[],uint128[],uint128[]))",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_hypothetical_exact_output_single_overload_1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteHypotheticalExactOutputSingle(address,address,bool,uint128,uint128,uint128,uint128,uint128)",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_hypothetical_exact_output_single_overload_2(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteHypotheticalExactOutputSingle(address,address,address,bool,uint128,uint128,uint128,uint128,uint128,bytes)",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_live_exact_in(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteLiveExactIn(address,(address[],bytes[],uint256,uint128))",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_live_exact_in_single_overload_1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteLiveExactInSingle(address,address,address,bool,uint128,uint128,bytes)",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_live_exact_in_single_overload_2(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteLiveExactInSingle(address,address,bool,uint128,uint128)",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_live_exact_out(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteLiveExactOut(address,(address[],bytes[],uint256,uint128))",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_live_exact_out_single_overload_1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteLiveExactOutSingle(address,address,address,bool,uint128,uint128,bytes)",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_live_exact_out_single_overload_2(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MetricOmmSwapQuoter","quoteLiveExactOutSingle(address,address,bool,uint128,uint128)",self.1,args,options.block,options.sender,options.value) }
}

impl<'a> Contracts<'a> { pub fn multicall3(&self, address: &'a str) -> Multicall3<'a> { Multicall3(self.0,address) } }
pub struct Multicall3<'a>(&'a MetricChainClient,&'a str);
impl Multicall3<'_> {
    pub fn prepare_aggregate(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("Multicall3","aggregate((address,bytes)[])",self.1,args,value) }
    pub fn prepare_aggregate3(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("Multicall3","aggregate3((address,bool,bytes)[])",self.1,args,value) }
    pub fn prepare_aggregate3_value(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("Multicall3","aggregate3Value((address,bool,uint256,bytes)[])",self.1,args,value) }
    pub fn prepare_block_and_aggregate(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("Multicall3","blockAndAggregate((address,bytes)[])",self.1,args,value) }
    pub fn get_basefee(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("Multicall3","getBasefee()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_block_hash(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("Multicall3","getBlockHash(uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_block_number(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("Multicall3","getBlockNumber()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_chain_id(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("Multicall3","getChainId()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_current_block_coinbase(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("Multicall3","getCurrentBlockCoinbase()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_current_block_difficulty(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("Multicall3","getCurrentBlockDifficulty()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_current_block_gas_limit(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("Multicall3","getCurrentBlockGasLimit()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_current_block_timestamp(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("Multicall3","getCurrentBlockTimestamp()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_eth_balance(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("Multicall3","getEthBalance(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_last_block_hash(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("Multicall3","getLastBlockHash()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_try_aggregate(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("Multicall3","tryAggregate(bool,(address,bytes)[])",self.1,args,value) }
    pub fn prepare_try_block_and_aggregate(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("Multicall3","tryBlockAndAggregate(bool,(address,bytes)[])",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn multiplier_oracle(&self, address: &'a str) -> MultiplierOracle<'a> { MultiplierOracle(self.0,address) } }
pub struct MultiplierOracle<'a>(&'a MetricChainClient,&'a str);
impl MultiplierOracle<'_> {
    pub fn adapter_gas_limit(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","ADAPTER_GAS_LIMIT()",self.1,args,options.block,options.sender,options.value) }
    pub fn admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn default_admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","DEFAULT_ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn underlying(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","UNDERLYING()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_add_params_adapter(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MultiplierOracle","addParamsAdapter(address)",self.1,args,value) }
    pub fn prepare_create_feed(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MultiplierOracle","createFeed(bytes32,address,address)",self.1,args,value) }
    pub fn decimals(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","decimals()",self.1,args,options.block,options.sender,options.value) }
    pub fn feed_disabled(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","feedDisabled(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn feed_id_of(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","feedIdOf(bytes32,address,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn feeds(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","feeds(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_params_adapters(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","getParamsAdapters(uint256,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_role_admin(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","getRoleAdmin(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_grant_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MultiplierOracle","grantRole(bytes32,address)",self.1,args,value) }
    pub fn has_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","hasRole(bytes32,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn is_params_adapter(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","isParamsAdapter(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn kind(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","kind()",self.1,args,options.block,options.sender,options.value) }
    pub fn params_adapter_count(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","paramsAdapterCount()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_price(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MultiplierOracle","price(bytes32,address)",self.1,args,value) }
    pub fn price_guard(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","priceGuard(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_register(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MultiplierOracle","register(bytes32,address,address)",self.1,args,value) }
    pub fn registered_pool(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","registeredPool(bytes32,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_remove_params_adapter(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MultiplierOracle","removeParamsAdapter(address)",self.1,args,value) }
    pub fn prepare_renounce_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MultiplierOracle","renounceRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_revoke_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MultiplierOracle","revokeRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_set_feed_disabled(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("MultiplierOracle","setFeedDisabled(bytes32,bool)",self.1,args,value) }
    pub fn supports_interface(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","supportsInterface(bytes4)",self.1,args,options.block,options.sender,options.value) }
    pub fn version(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("MultiplierOracle","version()",self.1,args,options.block,options.sender,options.value) }
}

impl<'a> Contracts<'a> { pub fn oracle_provider(&self, address: &'a str) -> OracleProvider<'a> { OracleProvider(self.0,address) } }
pub struct OracleProvider<'a>(&'a MetricChainClient,&'a str);
impl OracleProvider<'_> {
    pub fn blacklisted(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("OracleProvider","blacklisted(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn registered_pool(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("OracleProvider","registeredPool(bytes32,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn registration_fee(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("OracleProvider","registrationFee()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_register(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleProvider","register(bytes32,address,address)",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn oracle_value_stop_loss_extension(&self, address: &'a str) -> OracleValueStopLossExtension<'a> { OracleValueStopLossExtension(self.0,address) } }
pub struct OracleValueStopLossExtension<'a>(&'a MetricChainClient,&'a str);
impl OracleValueStopLossExtension<'_> {
    pub fn factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("OracleValueStopLossExtension","FACTORY()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_after_add_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)",self.1,args,value) }
    pub fn prepare_after_remove_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)",self.1,args,value) }
    pub fn prepare_after_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)",self.1,args,value) }
    pub fn prepare_before_add_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)",self.1,args,value) }
    pub fn prepare_before_remove_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)",self.1,args,value) }
    pub fn prepare_before_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)",self.1,args,value) }
    pub fn prepare_cancel_oracle_stop_loss_decay(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","cancelOracleStopLossDecay(address)",self.1,args,value) }
    pub fn prepare_cancel_oracle_stop_loss_drawdown(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","cancelOracleStopLossDrawdown(address)",self.1,args,value) }
    pub fn prepare_cancel_oracle_stop_loss_high_watermarks(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","cancelOracleStopLossHighWatermarks(address)",self.1,args,value) }
    pub fn prepare_cancel_oracle_stop_loss_timelock(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","cancelOracleStopLossTimelock(address)",self.1,args,value) }
    pub fn current_high_watermarks(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("OracleValueStopLossExtension","currentHighWatermarks(address,int16)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_execute_oracle_stop_loss_decay(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","executeOracleStopLossDecay(address)",self.1,args,value) }
    pub fn prepare_execute_oracle_stop_loss_drawdown(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","executeOracleStopLossDrawdown(address)",self.1,args,value) }
    pub fn prepare_execute_oracle_stop_loss_high_watermarks(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","executeOracleStopLossHighWatermarks(address)",self.1,args,value) }
    pub fn prepare_execute_oracle_stop_loss_timelock(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","executeOracleStopLossTimelock(address)",self.1,args,value) }
    pub fn high_watermarks(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("OracleValueStopLossExtension","highWatermarks(address,int16)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_initialize(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","initialize(address,bytes)",self.1,args,value) }
    pub fn oracle_stop_loss_config(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("OracleValueStopLossExtension","oracleStopLossConfig(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn pending_high_watermark(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("OracleValueStopLossExtension","pendingHighWatermark(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn pool_stop_loss_schedule(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("OracleValueStopLossExtension","poolStopLossSchedule(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_propose_oracle_stop_loss_decay(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","proposeOracleStopLossDecay(address,uint256)",self.1,args,value) }
    pub fn prepare_propose_oracle_stop_loss_drawdown(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","proposeOracleStopLossDrawdown(address,uint256)",self.1,args,value) }
    pub fn prepare_propose_oracle_stop_loss_high_watermarks(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","proposeOracleStopLossHighWatermarks(address,int16,uint104,uint104)",self.1,args,value) }
    pub fn prepare_propose_oracle_stop_loss_timelock(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("OracleValueStopLossExtension","proposeOracleStopLossTimelock(address,uint40)",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn price_provider(&self, address: &'a str) -> PriceProvider<'a> { PriceProvider(self.0,address) } }
pub struct PriceProvider<'a>(&'a MetricChainClient,&'a str);
impl PriceProvider<'_> {
    pub fn bps_base(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","BPS_BASE()",self.1,args,options.block,options.sender,options.value) }
    pub fn bps_base_u(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","BPS_BASE_U()",self.1,args,options.block,options.sender,options.value) }
    pub fn confidence_cooldown(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","CONFIDENCE_COOLDOWN()",self.1,args,options.block,options.sender,options.value) }
    pub fn confidence_max(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","CONFIDENCE_MAX()",self.1,args,options.block,options.sender,options.value) }
    pub fn max_time_delta(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","MAX_TIME_DELTA()",self.1,args,options.block,options.sender,options.value) }
    pub fn base_token(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","baseToken()",self.1,args,options.block,options.sender,options.value) }
    pub fn confidence_param(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","confidenceParam()",self.1,args,options.block,options.sender,options.value) }
    pub fn factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","factory()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_get_quote(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProvider","getQuote()",self.1,args,value) }
    pub fn last_confidence_update(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","lastConfidenceUpdate()",self.1,args,options.block,options.sender,options.value) }
    pub fn margin_step(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","marginStep()",self.1,args,options.block,options.sender,options.value) }
    pub fn offchain_feed_id(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","offchainFeedId()",self.1,args,options.block,options.sender,options.value) }
    pub fn offchain_oracle(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","offchainOracle()",self.1,args,options.block,options.sender,options.value) }
    pub fn quote_token(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","quoteToken()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_set_confidence_param(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProvider","setConfidenceParam(uint256)",self.1,args,value) }
    pub fn token0(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","token0()",self.1,args,options.block,options.sender,options.value) }
    pub fn token1(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","token1()",self.1,args,options.block,options.sender,options.value) }
    pub fn version(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProvider","version()",self.1,args,options.block,options.sender,options.value) }
}

impl<'a> Contracts<'a> { pub fn price_provider_factory(&self, address: &'a str) -> PriceProviderFactory<'a> { PriceProviderFactory(self.0,address) } }
pub struct PriceProviderFactory<'a>(&'a MetricChainClient,&'a str);
impl PriceProviderFactory<'_> {
    pub fn admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn default_admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","DEFAULT_ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_add_provider(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","addProvider(address)",self.1,args,value) }
    pub fn prepare_create_price_provider(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","createPriceProvider(address,bytes32,int256,uint256,address,address)",self.1,args,value) }
    pub fn get_all_providers(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","getAllProviders(uint256,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_providers(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","getProviders(address,uint256,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_role_admin(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","getRoleAdmin(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_grant_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","grantRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_grant_updater(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","grantUpdater(address,address)",self.1,args,value) }
    pub fn has_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","hasRole(bytes32,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn is_provider(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","isProvider(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn is_updater(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","isUpdater(address,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_multicall(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","multicall(bytes[])",self.1,args,value) }
    pub fn provider_at(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","providerAt(uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn provider_count(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","providerCount()",self.1,args,options.block,options.sender,options.value) }
    pub fn provider_count_by_creator(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","providerCountByCreator(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn provider_owner(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","providerOwner(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_remove_provider(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","removeProvider(address)",self.1,args,value) }
    pub fn prepare_renounce_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","renounceRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_revoke_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","revokeRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_revoke_updater(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","revokeUpdater(address,address)",self.1,args,value) }
    pub fn prepare_set_confidence(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","setConfidence(address[],uint256[])",self.1,args,value) }
    pub fn supports_interface(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","supportsInterface(bytes4)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_transfer_provider_ownership(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceProviderFactory","transferProviderOwnership(address,address)",self.1,args,value) }
    pub fn version(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceProviderFactory","version()",self.1,args,options.block,options.sender,options.value) }
}

impl<'a> Contracts<'a> { pub fn price_velocity_guard_extension(&self, address: &'a str) -> PriceVelocityGuardExtension<'a> { PriceVelocityGuardExtension(self.0,address) } }
pub struct PriceVelocityGuardExtension<'a>(&'a MetricChainClient,&'a str);
impl PriceVelocityGuardExtension<'_> {
    pub fn factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceVelocityGuardExtension","FACTORY()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_after_add_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceVelocityGuardExtension","afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)",self.1,args,value) }
    pub fn prepare_after_remove_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceVelocityGuardExtension","afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)",self.1,args,value) }
    pub fn prepare_after_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceVelocityGuardExtension","afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)",self.1,args,value) }
    pub fn prepare_before_add_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceVelocityGuardExtension","beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)",self.1,args,value) }
    pub fn prepare_before_remove_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceVelocityGuardExtension","beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)",self.1,args,value) }
    pub fn prepare_before_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceVelocityGuardExtension","beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)",self.1,args,value) }
    pub fn prepare_initialize(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceVelocityGuardExtension","initialize(address,bytes)",self.1,args,value) }
    pub fn price_velocity_state(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PriceVelocityGuardExtension","priceVelocityState(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_set_anchor_mid_price(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceVelocityGuardExtension","setAnchorMidPrice(address,uint128)",self.1,args,value) }
    pub fn prepare_set_max_change_per_block(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PriceVelocityGuardExtension","setMaxChangePerBlock(address,uint64)",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn pyth_oracle(&self, address: &'a str) -> PythOracle<'a> { PythOracle(self.0,address) } }
pub struct PythOracle<'a>(&'a MetricChainClient,&'a str);
impl PythOracle<'_> {
    pub fn admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn bps_base(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","BPS_BASE()",self.1,args,options.block,options.sender,options.value) }
    pub fn default_admin_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","DEFAULT_ADMIN_ROLE()",self.1,args,options.block,options.sender,options.value) }
    pub fn stall_spread(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","STALL_SPREAD()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_accept_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","acceptStateGuardRole(bytes32)",self.1,args,value) }
    pub fn prepare_add_approved_factory(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","addApprovedFactory(address)",self.1,args,value) }
    pub fn prepare_add_integrator(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","addIntegrator(address)",self.1,args,value) }
    pub fn approved_factory_count(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","approvedFactoryCount()",self.1,args,options.block,options.sender,options.value) }
    pub fn blacklisted(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","blacklisted(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn decimals(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","decimals()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_approved_factories(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","getApprovedFactories(uint256,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_integrators(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","getIntegrators(uint256,uint256)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_oracle_data(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","getOracleData(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn get_oracle_data_bulk(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","getOracleDataBulk(bytes32[])",self.1,args,options.block,options.sender,options.value) }
    pub fn get_role_admin(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","getRoleAdmin(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_grant_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","grantRole(bytes32,address)",self.1,args,value) }
    pub fn has_role(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","hasRole(bytes32,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn integrator_count(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","integratorCount()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_integrator_price(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","integratorPrice(bytes32)",self.1,args,value) }
    pub fn is_approved_factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","isApprovedFactory(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn is_integrator(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","isIntegrator(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn kind(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","kind()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_multicall(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","multicall(bytes[])",self.1,args,value) }
    pub fn pending_state_guard(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","pendingStateGuard(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_price(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","price(bytes32,address)",self.1,args,value) }
    pub fn price_guard(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","priceGuard(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_purge_pending_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","purgePendingStateGuardRole(bytes32)",self.1,args,value) }
    pub fn prepare_purge_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","purgeStateGuardRole(bytes32)",self.1,args,value) }
    pub fn prepare_register(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","register(bytes32,address,address)",self.1,args,value) }
    pub fn registered_pool(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","registeredPool(bytes32,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn registration_fee(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","registrationFee()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_remove_approved_factory(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","removeApprovedFactory(address)",self.1,args,value) }
    pub fn prepare_remove_integrator(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","removeIntegrator(address)",self.1,args,value) }
    pub fn prepare_renounce_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","renounceRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_revoke_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","revokeRole(bytes32,address)",self.1,args,value) }
    pub fn prepare_set_blacklist(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","setBlacklist(address,bool)",self.1,args,value) }
    pub fn prepare_set_integrators(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","setIntegrators(address[],bool)",self.1,args,value) }
    pub fn prepare_set_price_guard(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","setPriceGuard(bytes32,uint128,uint128)",self.1,args,value) }
    pub fn prepare_set_registration_fee(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","setRegistrationFee(uint256)",self.1,args,value) }
    pub fn prepare_set_state_guard_role(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","setStateGuardRole(bytes32,address)",self.1,args,value) }
    pub fn state_guard(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","stateGuard(bytes32)",self.1,args,options.block,options.sender,options.value) }
    pub fn supports_interface(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","supportsInterface(bytes4)",self.1,args,options.block,options.sender,options.value) }
    pub fn version(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("PythOracle","version()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_withdraw_eth(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("PythOracle","withdrawEth()",self.1,args,value) }
}

impl<'a> Contracts<'a> { pub fn robinhood_stock_token_params(&self, address: &'a str) -> RobinhoodStockTokenParams<'a> { RobinhoodStockTokenParams(self.0,address) } }
pub struct RobinhoodStockTokenParams<'a>(&'a MetricChainClient,&'a str);
impl RobinhoodStockTokenParams<'_> {
    pub fn small_change_bps(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("RobinhoodStockTokenParams","SMALL_CHANGE_BPS()",self.1,args,options.block,options.sender,options.value) }
    pub fn get_oracle_params(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("RobinhoodStockTokenParams","getOracleParams(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn version(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("RobinhoodStockTokenParams","version()",self.1,args,options.block,options.sender,options.value) }
}

impl<'a> Contracts<'a> { pub fn swap_allowlist_extension(&self, address: &'a str) -> SwapAllowlistExtension<'a> { SwapAllowlistExtension(self.0,address) } }
pub struct SwapAllowlistExtension<'a>(&'a MetricChainClient,&'a str);
impl SwapAllowlistExtension<'_> {
    pub fn factory(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("SwapAllowlistExtension","FACTORY()",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_after_add_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("SwapAllowlistExtension","afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)",self.1,args,value) }
    pub fn prepare_after_remove_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("SwapAllowlistExtension","afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)",self.1,args,value) }
    pub fn prepare_after_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("SwapAllowlistExtension","afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)",self.1,args,value) }
    pub fn allow_all_swappers(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("SwapAllowlistExtension","allowAllSwappers(address)",self.1,args,options.block,options.sender,options.value) }
    pub fn allowed_swapper(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("SwapAllowlistExtension","allowedSwapper(address,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_before_add_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("SwapAllowlistExtension","beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)",self.1,args,value) }
    pub fn prepare_before_remove_liquidity(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("SwapAllowlistExtension","beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)",self.1,args,value) }
    pub fn before_swap(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("SwapAllowlistExtension","beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_initialize(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("SwapAllowlistExtension","initialize(address,bytes)",self.1,args,value) }
    pub fn is_allowed_to_swap(&self, args: &Value, options: ReadOptions) -> Result<Value> { self.0.call("SwapAllowlistExtension","isAllowedToSwap(address,address)",self.1,args,options.block,options.sender,options.value) }
    pub fn prepare_set_allow_all_swappers(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("SwapAllowlistExtension","setAllowAllSwappers(address,bool)",self.1,args,value) }
    pub fn prepare_set_allowed_to_swap(&self, args: &Value, value: &str) -> Result<Value> { self.0.prepare_call("SwapAllowlistExtension","setAllowedToSwap(address,address,bool)",self.1,args,value) }
}


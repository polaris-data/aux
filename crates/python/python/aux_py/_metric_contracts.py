"""Generated native contract interfaces; protocol work stays in Rust."""
from __future__ import annotations

class AnchoredPriceProvider:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def bps_base(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "BPS_BASE()", self.address, args, block=block, from_address=from_address, value=value)

    async def bps_base_u(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "BPS_BASE_U()", self.address, args, block=block, from_address=from_address, value=value)

    async def confidence_cooldown(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "CONFIDENCE_COOLDOWN()", self.address, args, block=block, from_address=from_address, value=value)

    async def confidence_max(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "CONFIDENCE_MAX()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_ref_staleness(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "MAX_REF_STALENESS()", self.address, args, block=block, from_address=from_address, value=value)

    async def mutable_params(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "MUTABLE_PARAMS()", self.address, args, block=block, from_address=from_address, value=value)

    async def source_gas_limit(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "SOURCE_GAS_LIMIT()", self.address, args, block=block, from_address=from_address, value=value)

    async def base_feed_id(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "baseFeedId()", self.address, args, block=block, from_address=from_address, value=value)

    async def base_max_staleness(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "baseMaxStaleness()", self.address, args, block=block, from_address=from_address, value=value)

    async def base_spread_mult(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "baseSpreadMult()", self.address, args, block=block, from_address=from_address, value=value)

    async def base_token(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "baseToken()", self.address, args, block=block, from_address=from_address, value=value)

    async def confidence_param(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "confidenceParam()", self.address, args, block=block, from_address=from_address, value=value)

    async def factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "factory()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_get_quote(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredPriceProvider", "getQuote()", self.address, args, value=value)

    async def last_confidence_update(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "lastConfidenceUpdate()", self.address, args, block=block, from_address=from_address, value=value)

    async def last_spread_mult_update(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "lastSpreadMultUpdate()", self.address, args, block=block, from_address=from_address, value=value)

    async def last_staleness_update(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "lastStalenessUpdate()", self.address, args, block=block, from_address=from_address, value=value)

    async def margin_step(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "marginStep()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_spread_mult(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "maxSpreadMult()", self.address, args, block=block, from_address=from_address, value=value)

    async def min_margin(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "minMargin()", self.address, args, block=block, from_address=from_address, value=value)

    async def offchain_oracle(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "offchainOracle()", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_feed_id(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "quoteFeedId()", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_max_staleness(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "quoteMaxStaleness()", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_spread_mult(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "quoteSpreadMult()", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_token(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "quoteToken()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_set_confidence_param(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredPriceProvider", "setConfidenceParam(uint256)", self.address, args, value=value)

    def prepare_set_max_staleness(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredPriceProvider", "setMaxStaleness(uint256,uint256)", self.address, args, value=value)

    def prepare_set_source(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredPriceProvider", "setSource(address)", self.address, args, value=value)

    def prepare_set_spread_multipliers(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredPriceProvider", "setSpreadMultipliers(uint256,uint256)", self.address, args, value=value)

    async def source(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "source()", self.address, args, block=block, from_address=from_address, value=value)

    async def token0(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "token0()", self.address, args, block=block, from_address=from_address, value=value)

    async def token1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "token1()", self.address, args, block=block, from_address=from_address, value=value)

    async def version(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredPriceProvider", "version()", self.address, args, block=block, from_address=from_address, value=value)


class AnchoredProviderFactory:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def default_admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "DEFAULT_ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def default_class(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "DEFAULT_CLASS()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_add_oracle(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "addOracle(address)", self.address, args, value=value)

    def prepare_create_anchored_provider(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "createAnchoredProvider(address,bytes32,bytes32,uint256,uint256,uint256,bool,int256,address,address,(address,uint256,uint256,uint256,uint256))", self.address, args, value=value)

    async def envelopes(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "envelopes(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def feed_class(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "feedClass(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_all_providers(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "getAllProviders(uint256,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_oracles(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "getOracles(uint256,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_providers(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "getProviders(address,uint256,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_role_admin(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "getRoleAdmin(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_grant_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "grantRole(bytes32,address)", self.address, args, value=value)

    def prepare_grant_updater(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "grantUpdater(address,address)", self.address, args, value=value)

    async def has_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "hasRole(bytes32,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def is_oracle(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "isOracle(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def is_provider(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "isProvider(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def is_updater(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "isUpdater(address,address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_multicall(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "multicall(bytes[])", self.address, args, value=value)

    async def oracle_count(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "oracleCount()", self.address, args, block=block, from_address=from_address, value=value)

    async def provider_at(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "providerAt(uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def provider_count(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "providerCount()", self.address, args, block=block, from_address=from_address, value=value)

    async def provider_count_by_creator(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "providerCountByCreator(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def provider_owner(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "providerOwner(address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_remove_envelope(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "removeEnvelope(bytes32)", self.address, args, value=value)

    def prepare_remove_oracle(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "removeOracle(address)", self.address, args, value=value)

    def prepare_renounce_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "renounceRole(bytes32,address)", self.address, args, value=value)

    def prepare_revoke_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "revokeRole(bytes32,address)", self.address, args, value=value)

    def prepare_revoke_updater(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "revokeUpdater(address,address)", self.address, args, value=value)

    def prepare_set_confidence(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "setConfidence(address[],uint256[])", self.address, args, value=value)

    def prepare_set_envelope(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "setEnvelope(bytes32,(uint256,uint256,uint256,uint256,uint256,bool))", self.address, args, value=value)

    def prepare_set_feed_class(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "setFeedClass(bytes32,bytes32)", self.address, args, value=value)

    def prepare_set_max_staleness(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "setMaxStaleness(address[],uint256[],uint256[])", self.address, args, value=value)

    def prepare_set_source(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "setSource(address,address)", self.address, args, value=value)

    def prepare_set_spread_multipliers(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "setSpreadMultipliers(address[],uint256[],uint256[])", self.address, args, value=value)

    async def supports_interface(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "supportsInterface(bytes4)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_transfer_provider_ownership(self, args=(), *, value="0x0"):
        return self._client.prepare_call("AnchoredProviderFactory", "transferProviderOwnership(address,address)", self.address, args, value=value)

    async def version(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("AnchoredProviderFactory", "version()", self.address, args, block=block, from_address=from_address, value=value)


class B20AssetParams:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def registry(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("B20AssetParams", "REGISTRY()", self.address, args, block=block, from_address=from_address, value=value)

    async def small_change_bps(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("B20AssetParams", "SMALL_CHANGE_BPS()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_oracle_params(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("B20AssetParams", "getOracleParams(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def version(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("B20AssetParams", "version()", self.address, args, block=block, from_address=from_address, value=value)


class ChainlinkOracle:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def bps_base(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "BPS_BASE()", self.address, args, block=block, from_address=from_address, value=value)

    async def default_admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "DEFAULT_ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def stall_spread(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "STALL_SPREAD()", self.address, args, block=block, from_address=from_address, value=value)

    async def tag_v050(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "TAG_V050()", self.address, args, block=block, from_address=from_address, value=value)

    async def tag_v051(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "TAG_V051()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_accept_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "acceptStateGuardRole(bytes32)", self.address, args, value=value)

    def prepare_add_approved_factory(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "addApprovedFactory(address)", self.address, args, value=value)

    def prepare_add_integrator(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "addIntegrator(address)", self.address, args, value=value)

    async def approved_factory_count(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "approvedFactoryCount()", self.address, args, block=block, from_address=from_address, value=value)

    async def blacklisted(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "blacklisted(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def decimals(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "decimals()", self.address, args, block=block, from_address=from_address, value=value)

    async def fee_token(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "feeToken()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_approved_factories(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "getApprovedFactories(uint256,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_integrators(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "getIntegrators(uint256,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_oracle_data(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "getOracleData(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_oracle_data_bulk(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "getOracleDataBulk(bytes32[])", self.address, args, block=block, from_address=from_address, value=value)

    async def get_role_admin(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "getRoleAdmin(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_grant_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "grantRole(bytes32,address)", self.address, args, value=value)

    async def has_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "hasRole(bytes32,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def integrator_count(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "integratorCount()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_integrator_price(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "integratorPrice(bytes32)", self.address, args, value=value)

    async def is_approved_factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "isApprovedFactory(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def is_integrator(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "isIntegrator(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def kind(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "kind()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_multicall(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "multicall(bytes[])", self.address, args, value=value)

    async def pending_state_guard(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "pendingStateGuard(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_price(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "price(bytes32,address)", self.address, args, value=value)

    async def price_guard(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "priceGuard(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_purge_pending_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "purgePendingStateGuardRole(bytes32)", self.address, args, value=value)

    def prepare_purge_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "purgeStateGuardRole(bytes32)", self.address, args, value=value)

    def prepare_register(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "register(bytes32,address,address)", self.address, args, value=value)

    async def registered_pool(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "registeredPool(bytes32,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def registration_fee(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "registrationFee()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_remove_approved_factory(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "removeApprovedFactory(address)", self.address, args, value=value)

    def prepare_remove_integrator(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "removeIntegrator(address)", self.address, args, value=value)

    def prepare_remove_verifier(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "removeVerifier(uint8)", self.address, args, value=value)

    def prepare_renounce_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "renounceRole(bytes32,address)", self.address, args, value=value)

    def prepare_revoke_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "revokeRole(bytes32,address)", self.address, args, value=value)

    def prepare_set_blacklist(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "setBlacklist(address,bool)", self.address, args, value=value)

    def prepare_set_integrators(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "setIntegrators(address[],bool)", self.address, args, value=value)

    def prepare_set_price_guard(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "setPriceGuard(bytes32,uint128,uint128)", self.address, args, value=value)

    def prepare_set_registration_fee(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "setRegistrationFee(uint256)", self.address, args, value=value)

    def prepare_set_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "setStateGuardRole(bytes32,address)", self.address, args, value=value)

    def prepare_set_verifier(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "setVerifier(uint8,address,bool)", self.address, args, value=value)

    async def state_guard(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "stateGuard(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def supports_interface(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "supportsInterface(bytes4)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_update_report(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "updateReport(bytes)", self.address, args, value=value)

    def prepare_update_reports(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "updateReports(bytes[])", self.address, args, value=value)

    async def verifier_by_tag(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "verifierByTag(uint8)", self.address, args, block=block, from_address=from_address, value=value)

    async def verifier_tags(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "verifierTags()", self.address, args, block=block, from_address=from_address, value=value)

    async def version(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ChainlinkOracle", "version()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_withdraw_eth(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ChainlinkOracle", "withdrawEth()", self.address, args, value=value)


class CompressedOracleV1:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def bps_base(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "BPS_BASE()", self.address, args, block=block, from_address=from_address, value=value)

    async def default_admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "DEFAULT_ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def stall_spread(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "STALL_SPREAD()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_accept_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "acceptStateGuardRole(bytes32)", self.address, args, value=value)

    def prepare_allow_contract_pushers(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "allowContractPushers(address[])", self.address, args, value=value)

    def prepare_allow_pushers(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "allowPushers(uint256,address[],bytes[])", self.address, args, value=value)

    async def decimals(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "decimals()", self.address, args, block=block, from_address=from_address, value=value)

    async def extsload_overload_1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "extsload(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def extsload_overload_2(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "extsload(bytes32,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def extsload_overload_3(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "extsload(bytes32[])", self.address, args, block=block, from_address=from_address, value=value)

    async def feed_id_of(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "feedIdOf(address,uint8,uint8)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_compressed_oracle(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "getCompressedOracle(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_oracle_data(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "getOracleData(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_oracle_data_bulk(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "getOracleDataBulk(bytes32[])", self.address, args, block=block, from_address=from_address, value=value)

    async def get_role_admin(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "getRoleAdmin(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_slot_layout(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "getSlotLayout(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_grant_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "grantRole(bytes32,address)", self.address, args, value=value)

    async def has_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "hasRole(bytes32,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def kind(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "kind()", self.address, args, block=block, from_address=from_address, value=value)

    async def namespace_remapping(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "namespaceRemapping(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def pending_state_guard(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "pendingStateGuard(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_price(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "price(bytes32,address)", self.address, args, value=value)

    async def price_guard(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "priceGuard(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_purge_pending_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "purgePendingStateGuardRole(bytes32)", self.address, args, value=value)

    def prepare_purge_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "purgeStateGuardRole(bytes32)", self.address, args, value=value)

    def prepare_remove_pushers(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "removePushers(address[])", self.address, args, value=value)

    def prepare_renounce_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "renounceRole(bytes32,address)", self.address, args, value=value)

    def prepare_revoke_pusher(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "revokePusher()", self.address, args, value=value)

    def prepare_revoke_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "revokeRole(bytes32,address)", self.address, args, value=value)

    def prepare_set_pending_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "setPendingStateGuardRole(bytes32,address)", self.address, args, value=value)

    def prepare_set_price_guard(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "setPriceGuard(bytes32,uint128,uint128)", self.address, args, value=value)

    async def state_guard(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "stateGuard(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def supports_interface(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "supportsInterface(bytes4)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_update_by_signature(self, args=(), *, value="0x0"):
        return self._client.prepare_call("CompressedOracleV1", "updateBySignature(address,uint256,bytes)", self.address, args, value=value)

    async def used_pusher_deadline(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "usedPusherDeadline(address,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def version(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("CompressedOracleV1", "version()", self.address, args, block=block, from_address=from_address, value=value)


class DepositAllowlistExtension:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("DepositAllowlistExtension", "FACTORY()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_after_add_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("DepositAllowlistExtension", "afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", self.address, args, value=value)

    def prepare_after_remove_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("DepositAllowlistExtension", "afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", self.address, args, value=value)

    def prepare_after_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("DepositAllowlistExtension", "afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)", self.address, args, value=value)

    async def allow_all_depositors(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("DepositAllowlistExtension", "allowAllDepositors(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def allowed_depositor(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("DepositAllowlistExtension", "allowedDepositor(address,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def before_add_liquidity(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("DepositAllowlistExtension", "beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_before_remove_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("DepositAllowlistExtension", "beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", self.address, args, value=value)

    def prepare_before_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("DepositAllowlistExtension", "beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)", self.address, args, value=value)

    def prepare_initialize(self, args=(), *, value="0x0"):
        return self._client.prepare_call("DepositAllowlistExtension", "initialize(address,bytes)", self.address, args, value=value)

    async def is_allowed_to_deposit(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("DepositAllowlistExtension", "isAllowedToDeposit(address,address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_set_allow_all_depositors(self, args=(), *, value="0x0"):
        return self._client.prepare_call("DepositAllowlistExtension", "setAllowAllDepositors(address,bool)", self.address, args, value=value)

    def prepare_set_allowed_to_deposit(self, args=(), *, value="0x0"):
        return self._client.prepare_call("DepositAllowlistExtension", "setAllowedToDeposit(address,address,bool)", self.address, args, value=value)


class ExternalSwapExecutor:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def router(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ExternalSwapExecutor", "router()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ExternalSwapExecutor", "swap((address,address,uint128,uint128,address,bytes,address,uint256))", self.address, args, value=value)


class IERC20:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def allowance(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("IERC20", "allowance(address,address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_approve(self, args=(), *, value="0x0"):
        return self._client.prepare_call("IERC20", "approve(address,uint256)", self.address, args, value=value)

    async def balance_of(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("IERC20", "balanceOf(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def total_supply(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("IERC20", "totalSupply()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_transfer(self, args=(), *, value="0x0"):
        return self._client.prepare_call("IERC20", "transfer(address,uint256)", self.address, args, value=value)

    def prepare_transfer_from(self, args=(), *, value="0x0"):
        return self._client.prepare_call("IERC20", "transferFrom(address,address,uint256)", self.address, args, value=value)


class ImmutableAppPoolHelper:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def app_factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ImmutableAppPoolHelper", "APP_FACTORY()", self.address, args, block=block, from_address=from_address, value=value)

    async def pool_factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("ImmutableAppPoolHelper", "POOL_FACTORY()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_create_pool(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ImmutableAppPoolHelper", "createPool((address,address,address,address[],(uint256,uint256,uint256,uint256,uint256,uint256),bytes[],uint256,address,uint256,uint256,uint256,uint24,uint24,uint24,uint24,address,int24,uint256[],uint256[],bytes32,uint16),uint16)", self.address, args, value=value)

    def prepare_set_pool_bid_ask_ref_asymmetry_x8(self, args=(), *, value="0x0"):
        return self._client.prepare_call("ImmutableAppPoolHelper", "setPoolBidAskRefAsymmetryX8(address,uint16)", self.address, args, value=value)


class MetricOmmPool:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    def prepare_add_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "addLiquidity(address,uint80,(int256[],uint256[]),bytes,bytes)", self.address, args, value=value)

    def prepare_collect_fees(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "collectFees(uint256,uint256,uint256,uint256,address,address)", self.address, args, value=value)

    async def extsload_overload_1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPool", "extsload(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def extsload_overload_2(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPool", "extsload(bytes32,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def extsload_overload_3(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPool", "extsload(bytes32[])", self.address, args, block=block, from_address=from_address, value=value)

    async def get_immutables(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPool", "getImmutables()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_get_sell_and_buy_prices(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "getSellAndBuyPrices()", self.address, args, value=value)

    async def in_swap(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPool", "inSwap()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_remove_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "removeLiquidity(address,uint80,(int256[],uint256[]),bytes)", self.address, args, value=value)

    def prepare_set_bid_ask_ref_asymmetry_x8(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "setBidAskRefAsymmetryX8(uint16)", self.address, args, value=value)

    def prepare_set_bin_additional_fees(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "setBinAdditionalFees(int16,uint16,uint16)", self.address, args, value=value)

    def prepare_set_pause(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "setPause(uint8)", self.address, args, value=value)

    def prepare_set_pool_fees(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "setPoolFees(uint24,uint24)", self.address, args, value=value)

    def prepare_set_price_provider(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "setPriceProvider(address)", self.address, args, value=value)

    def prepare_simulate_swap_and_revert(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "simulateSwapAndRevert(address,address,bool,int128,uint128,uint128,uint128,uint128,bytes)", self.address, args, value=value)

    def prepare_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPool", "swap(address,bool,int128,uint128,bytes,bytes)", self.address, args, value=value)


class MetricOmmPoolDataProvider:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def bin_state(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "binState(address,int16)", self.address, args, block=block, from_address=from_address, value=value)

    async def bin_states(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "binStates(address,int16[])", self.address, args, block=block, from_address=from_address, value=value)

    async def bin_total_shares_overload_1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "binTotalShares(address,int16[])", self.address, args, block=block, from_address=from_address, value=value)

    async def bin_total_shares_overload_2(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "binTotalShares(address,int16)", self.address, args, block=block, from_address=from_address, value=value)

    async def distance_from_provided_price_x64(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "distanceFromProvidedPriceX64(address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_get_liquidity_depth_hypothetical(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolDataProvider", "getLiquidityDepthHypothetical(address,uint8,uint128,uint128,uint128)", self.address, args, value=value)

    def prepare_get_liquidity_depth_live(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolDataProvider", "getLiquidityDepthLive(address,uint8)", self.address, args, value=value)

    async def position_bin_shares_overload_1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "positionBinShares(address,bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def position_bin_shares_overload_2(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "positionBinShares(address,address,uint80,int16)", self.address, args, block=block, from_address=from_address, value=value)

    async def price_provider(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "priceProvider(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def slot0(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "slot0(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def slot1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "slot1(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def slot2(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDataProvider", "slot2(address)", self.address, args, block=block, from_address=from_address, value=value)


class MetricOmmPoolDeployer:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolDeployer", "FACTORY()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_deploy(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolDeployer", "deploy((bytes32,address,address,address,address,(address,address,address,address,address,address,address),(uint256,uint256,uint256,uint256,uint256,uint256),bool,uint256,uint256,uint256,uint256,uint256,uint24,int24,(uint104,uint104,uint16,uint16,uint16)[],(uint104,uint104,uint16,uint16,uint16)[],uint24,uint16))", self.address, args, value=value)


class MetricOmmPoolFactory:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    def prepare_accept_ownership(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "acceptOwnership()", self.address, args, value=value)

    def prepare_accept_pool_admin(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "acceptPoolAdmin(address)", self.address, args, value=value)

    async def authority(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "authority()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_cancel_pool_admin_transfer(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "cancelPoolAdminTransfer(address)", self.address, args, value=value)

    def prepare_collect_eth(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "collectEth(address,uint256)", self.address, args, value=value)

    def prepare_collect_pool_fees(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "collectPoolFees(address)", self.address, args, value=value)

    def prepare_collect_tokens(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "collectTokens(address,address,uint256)", self.address, args, value=value)

    def prepare_create_pool(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "createPool((address,address,address,address[],(uint256,uint256,uint256,uint256,uint256,uint256),bytes[],uint256,address,uint256,uint256,uint256,uint24,uint24,uint24,uint24,address,int24,uint256[],uint256[],bytes32,uint16))", self.address, args, value=value)

    def prepare_execute_pool_price_provider_update(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "executePoolPriceProviderUpdate(address)", self.address, args, value=value)

    async def fee_receiver(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "feeReceiver()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_fee_caps(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "getFeeCaps()", self.address, args, block=block, from_address=from_address, value=value)

    async def hard_max_bin_distance_limit_e6(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "hardMaxBinDistanceLimitE6()", self.address, args, block=block, from_address=from_address, value=value)

    async def hard_min_bin_distance_limit_e6(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "hardMinBinDistanceLimitE6()", self.address, args, block=block, from_address=from_address, value=value)

    async def idx_to_pool(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "idxToPool(uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def is_pool(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "isPool(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def max_admin_notional_fee_e8(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "maxAdminNotionalFeeE8()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_admin_spread_fee_e6(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "maxAdminSpreadFeeE6()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_bin_distance_e6(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "maxBinDistanceE6()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_owner_admin_notional_cap_e8(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "maxOwnerAdminNotionalCapE8()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_owner_admin_spread_cap_e6(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "maxOwnerAdminSpreadCapE6()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_owner_protocol_notional_cap_e8(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "maxOwnerProtocolNotionalCapE8()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_owner_protocol_spread_cap_e6(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "maxOwnerProtocolSpreadCapE6()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_protocol_notional_fee_e8(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "maxProtocolNotionalFeeE8()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_protocol_spread_fee_e6(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "maxProtocolSpreadFeeE6()", self.address, args, block=block, from_address=from_address, value=value)

    async def min_bin_distance_e6(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "minBinDistanceE6()", self.address, args, block=block, from_address=from_address, value=value)

    async def next_pool_idx(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "nextPoolIdx()", self.address, args, block=block, from_address=from_address, value=value)

    async def on_creation_max_bid_ask_ref_asymmetry_x8(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "onCreationMaxBidAskRefAsymmetryX8()", self.address, args, block=block, from_address=from_address, value=value)

    async def owner(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "owner()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_pause_pool(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "pausePool(address)", self.address, args, value=value)

    async def pending_owner(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "pendingOwner()", self.address, args, block=block, from_address=from_address, value=value)

    async def pending_pool_admin(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "pendingPoolAdmin(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def pending_price_provider(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "pendingPriceProvider(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def pending_price_provider_execute_after(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "pendingPriceProviderExecuteAfter(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def pool_admin(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "poolAdmin(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def pool_admin_fee_destination(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "poolAdminFeeDestination(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def pool_deployer(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "poolDeployer()", self.address, args, block=block, from_address=from_address, value=value)

    async def pool_fee_config(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "poolFeeConfig(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def pool_to_idx(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "poolToIdx(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def price_provider_timelock(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "priceProviderTimelock(address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_propose_pool_admin_transfer(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "proposePoolAdminTransfer(address,address)", self.address, args, value=value)

    def prepare_propose_pool_price_provider(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "proposePoolPriceProvider(address,address)", self.address, args, value=value)

    async def protocol_notional_fee_e8(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "protocolNotionalFeeE8()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_protocol_pause_pool(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "protocolPausePool(address)", self.address, args, value=value)

    def prepare_protocol_unpause_pool(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "protocolUnpausePool(address)", self.address, args, value=value)

    def prepare_set_authority(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setAuthority(address)", self.address, args, value=value)

    def prepare_set_bin_distance_limits(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setBinDistanceLimits(int128,int128)", self.address, args, value=value)

    def prepare_set_default_protocol_notional_fee_e8(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setDefaultProtocolNotionalFeeE8(uint24)", self.address, args, value=value)

    def prepare_set_default_spread_protocol_fee_e6(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setDefaultSpreadProtocolFeeE6(uint24)", self.address, args, value=value)

    def prepare_set_fee_caps(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setFeeCaps(uint24,uint24,uint24,uint24)", self.address, args, value=value)

    def prepare_set_fee_receiver(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setFeeReceiver(address)", self.address, args, value=value)

    def prepare_set_on_creation_max_bid_ask_ref_asymmetry_x8(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setOnCreationMaxBidAskRefAsymmetryX8(uint16)", self.address, args, value=value)

    def prepare_set_pool_admin_fee_destination(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setPoolAdminFeeDestination(address,address)", self.address, args, value=value)

    def prepare_set_pool_admin_fees(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setPoolAdminFees(address,uint24,uint24)", self.address, args, value=value)

    def prepare_set_pool_bid_ask_ref_asymmetry_x8(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setPoolBidAskRefAsymmetryX8(address,uint16)", self.address, args, value=value)

    def prepare_set_pool_bid_ask_ref_asymmetry_x8_batch(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setPoolBidAskRefAsymmetryX8Batch(address[],uint16[])", self.address, args, value=value)

    def prepare_set_pool_bin_additional_fees(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setPoolBinAdditionalFees(address,int16,uint16,uint16)", self.address, args, value=value)

    def prepare_set_pool_deployer(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setPoolDeployer(address)", self.address, args, value=value)

    def prepare_set_pool_protocol_fee(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setPoolProtocolFee(address,uint24,uint24)", self.address, args, value=value)

    def prepare_set_pool_protocol_fee_batch(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "setPoolProtocolFeeBatch(address[],uint24[],uint24[])", self.address, args, value=value)

    async def spread_protocol_fee_e6(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmPoolFactory", "spreadProtocolFeeE6()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_transfer_ownership(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "transferOwnership(address)", self.address, args, value=value)

    def prepare_unpause_pool(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolFactory", "unpausePool(address)", self.address, args, value=value)


class MetricOmmPoolLiquidityAdder:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    def prepare_add_liquidity_exact_shares_overload_1(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolLiquidityAdder", "addLiquidityExactShares(address,address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)", self.address, args, value=value)

    def prepare_add_liquidity_exact_shares_overload_2(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolLiquidityAdder", "addLiquidityExactShares(address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)", self.address, args, value=value)

    def prepare_add_liquidity_weighted_overload_1(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolLiquidityAdder", "addLiquidityWeighted(address,address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)", self.address, args, value=value)

    def prepare_add_liquidity_weighted_overload_2(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolLiquidityAdder", "addLiquidityWeighted(address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)", self.address, args, value=value)

    def prepare_metric_omm_modify_liquidity_callback(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolLiquidityAdder", "metricOmmModifyLiquidityCallback(uint256,uint256,bytes)", self.address, args, value=value)

    def prepare_multicall(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolLiquidityAdder", "multicall(bytes[])", self.address, args, value=value)

    def prepare_refund_eth(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolLiquidityAdder", "refundETH()", self.address, args, value=value)

    def prepare_sweep_token(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolLiquidityAdder", "sweepToken(address,uint256,address)", self.address, args, value=value)

    def prepare_unwrap_weth9(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmPoolLiquidityAdder", "unwrapWETH9(uint256,address)", self.address, args, value=value)


class MetricOmmSimpleRouter:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    def prepare_exact_input(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "exactInput((address[],address[],bytes[],uint256,uint128,uint128,address,uint256))", self.address, args, value=value)

    def prepare_exact_input_single(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "exactInputSingle((address,address,address,bool,uint128,uint128,address,uint256,uint128,bytes))", self.address, args, value=value)

    def prepare_exact_output(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "exactOutput((address[],address[],bytes[],uint256,uint128,uint128,address,uint256))", self.address, args, value=value)

    def prepare_exact_output_single(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "exactOutputSingle((address,address,address,bool,uint128,uint128,address,uint256,uint128,bytes))", self.address, args, value=value)

    def prepare_external_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "externalSwap(address,bool,bool,(address,address,uint128,uint128,address,bytes,address,uint256))", self.address, args, value=value)

    def prepare_metric_omm_swap_callback(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "metricOmmSwapCallback(int256,int256,bytes)", self.address, args, value=value)

    def prepare_multicall(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "multicall(bytes[])", self.address, args, value=value)

    def prepare_refund_eth(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "refundETH()", self.address, args, value=value)

    def prepare_self_permit(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)", self.address, args, value=value)

    def prepare_self_permit_allowed(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)", self.address, args, value=value)

    def prepare_self_permit_allowed_if_necessary(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)", self.address, args, value=value)

    def prepare_self_permit_if_necessary(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)", self.address, args, value=value)

    def prepare_sequence(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "sequence((bytes,uint8,uint8)[])", self.address, args, value=value)

    def prepare_sweep_token(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "sweepToken(address,uint256,address)", self.address, args, value=value)

    def prepare_unwrap_weth9(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MetricOmmSimpleRouter", "unwrapWETH9(uint256,address)", self.address, args, value=value)


class MetricOmmSwapQuoter:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def quote_hypothetical_exact_input(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteHypotheticalExactInput(address,(address[],bytes[],uint256,uint128,uint128[],uint128[],uint128[]))", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_hypothetical_exact_input_single_overload_1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteHypotheticalExactInputSingle(address,address,bool,uint128,uint128,uint128,uint128,uint128)", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_hypothetical_exact_input_single_overload_2(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteHypotheticalExactInputSingle(address,address,address,bool,uint128,uint128,uint128,uint128,uint128,bytes)", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_hypothetical_exact_output(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteHypotheticalExactOutput(address,(address[],bytes[],uint256,uint128,uint128[],uint128[],uint128[]))", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_hypothetical_exact_output_single_overload_1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteHypotheticalExactOutputSingle(address,address,bool,uint128,uint128,uint128,uint128,uint128)", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_hypothetical_exact_output_single_overload_2(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteHypotheticalExactOutputSingle(address,address,address,bool,uint128,uint128,uint128,uint128,uint128,bytes)", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_live_exact_in(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteLiveExactIn(address,(address[],bytes[],uint256,uint128))", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_live_exact_in_single_overload_1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteLiveExactInSingle(address,address,address,bool,uint128,uint128,bytes)", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_live_exact_in_single_overload_2(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteLiveExactInSingle(address,address,bool,uint128,uint128)", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_live_exact_out(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteLiveExactOut(address,(address[],bytes[],uint256,uint128))", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_live_exact_out_single_overload_1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteLiveExactOutSingle(address,address,address,bool,uint128,uint128,bytes)", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_live_exact_out_single_overload_2(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MetricOmmSwapQuoter", "quoteLiveExactOutSingle(address,address,bool,uint128,uint128)", self.address, args, block=block, from_address=from_address, value=value)


class Multicall3:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    def prepare_aggregate(self, args=(), *, value="0x0"):
        return self._client.prepare_call("Multicall3", "aggregate((address,bytes)[])", self.address, args, value=value)

    def prepare_aggregate3(self, args=(), *, value="0x0"):
        return self._client.prepare_call("Multicall3", "aggregate3((address,bool,bytes)[])", self.address, args, value=value)

    def prepare_aggregate3_value(self, args=(), *, value="0x0"):
        return self._client.prepare_call("Multicall3", "aggregate3Value((address,bool,uint256,bytes)[])", self.address, args, value=value)

    def prepare_block_and_aggregate(self, args=(), *, value="0x0"):
        return self._client.prepare_call("Multicall3", "blockAndAggregate((address,bytes)[])", self.address, args, value=value)

    async def get_basefee(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("Multicall3", "getBasefee()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_block_hash(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("Multicall3", "getBlockHash(uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_block_number(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("Multicall3", "getBlockNumber()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_chain_id(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("Multicall3", "getChainId()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_current_block_coinbase(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("Multicall3", "getCurrentBlockCoinbase()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_current_block_difficulty(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("Multicall3", "getCurrentBlockDifficulty()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_current_block_gas_limit(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("Multicall3", "getCurrentBlockGasLimit()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_current_block_timestamp(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("Multicall3", "getCurrentBlockTimestamp()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_eth_balance(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("Multicall3", "getEthBalance(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_last_block_hash(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("Multicall3", "getLastBlockHash()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_try_aggregate(self, args=(), *, value="0x0"):
        return self._client.prepare_call("Multicall3", "tryAggregate(bool,(address,bytes)[])", self.address, args, value=value)

    def prepare_try_block_and_aggregate(self, args=(), *, value="0x0"):
        return self._client.prepare_call("Multicall3", "tryBlockAndAggregate(bool,(address,bytes)[])", self.address, args, value=value)


class MultiplierOracle:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def adapter_gas_limit(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "ADAPTER_GAS_LIMIT()", self.address, args, block=block, from_address=from_address, value=value)

    async def admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def default_admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "DEFAULT_ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def underlying(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "UNDERLYING()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_add_params_adapter(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MultiplierOracle", "addParamsAdapter(address)", self.address, args, value=value)

    def prepare_create_feed(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MultiplierOracle", "createFeed(bytes32,address,address)", self.address, args, value=value)

    async def decimals(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "decimals()", self.address, args, block=block, from_address=from_address, value=value)

    async def feed_disabled(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "feedDisabled(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def feed_id_of(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "feedIdOf(bytes32,address,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def feeds(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "feeds(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_params_adapters(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "getParamsAdapters(uint256,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_role_admin(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "getRoleAdmin(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_grant_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MultiplierOracle", "grantRole(bytes32,address)", self.address, args, value=value)

    async def has_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "hasRole(bytes32,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def is_params_adapter(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "isParamsAdapter(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def kind(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "kind()", self.address, args, block=block, from_address=from_address, value=value)

    async def params_adapter_count(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "paramsAdapterCount()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_price(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MultiplierOracle", "price(bytes32,address)", self.address, args, value=value)

    async def price_guard(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "priceGuard(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_register(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MultiplierOracle", "register(bytes32,address,address)", self.address, args, value=value)

    async def registered_pool(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "registeredPool(bytes32,address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_remove_params_adapter(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MultiplierOracle", "removeParamsAdapter(address)", self.address, args, value=value)

    def prepare_renounce_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MultiplierOracle", "renounceRole(bytes32,address)", self.address, args, value=value)

    def prepare_revoke_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MultiplierOracle", "revokeRole(bytes32,address)", self.address, args, value=value)

    def prepare_set_feed_disabled(self, args=(), *, value="0x0"):
        return self._client.prepare_call("MultiplierOracle", "setFeedDisabled(bytes32,bool)", self.address, args, value=value)

    async def supports_interface(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "supportsInterface(bytes4)", self.address, args, block=block, from_address=from_address, value=value)

    async def version(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("MultiplierOracle", "version()", self.address, args, block=block, from_address=from_address, value=value)


class OracleProvider:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def blacklisted(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("OracleProvider", "blacklisted(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def registered_pool(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("OracleProvider", "registeredPool(bytes32,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def registration_fee(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("OracleProvider", "registrationFee()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_register(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleProvider", "register(bytes32,address,address)", self.address, args, value=value)


class OracleValueStopLossExtension:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("OracleValueStopLossExtension", "FACTORY()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_after_add_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", self.address, args, value=value)

    def prepare_after_remove_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", self.address, args, value=value)

    def prepare_after_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)", self.address, args, value=value)

    def prepare_before_add_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", self.address, args, value=value)

    def prepare_before_remove_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", self.address, args, value=value)

    def prepare_before_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)", self.address, args, value=value)

    def prepare_cancel_oracle_stop_loss_decay(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "cancelOracleStopLossDecay(address)", self.address, args, value=value)

    def prepare_cancel_oracle_stop_loss_drawdown(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "cancelOracleStopLossDrawdown(address)", self.address, args, value=value)

    def prepare_cancel_oracle_stop_loss_high_watermarks(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "cancelOracleStopLossHighWatermarks(address)", self.address, args, value=value)

    def prepare_cancel_oracle_stop_loss_timelock(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "cancelOracleStopLossTimelock(address)", self.address, args, value=value)

    async def current_high_watermarks(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("OracleValueStopLossExtension", "currentHighWatermarks(address,int16)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_execute_oracle_stop_loss_decay(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "executeOracleStopLossDecay(address)", self.address, args, value=value)

    def prepare_execute_oracle_stop_loss_drawdown(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "executeOracleStopLossDrawdown(address)", self.address, args, value=value)

    def prepare_execute_oracle_stop_loss_high_watermarks(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "executeOracleStopLossHighWatermarks(address)", self.address, args, value=value)

    def prepare_execute_oracle_stop_loss_timelock(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "executeOracleStopLossTimelock(address)", self.address, args, value=value)

    async def high_watermarks(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("OracleValueStopLossExtension", "highWatermarks(address,int16)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_initialize(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "initialize(address,bytes)", self.address, args, value=value)

    async def oracle_stop_loss_config(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("OracleValueStopLossExtension", "oracleStopLossConfig(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def pending_high_watermark(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("OracleValueStopLossExtension", "pendingHighWatermark(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def pool_stop_loss_schedule(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("OracleValueStopLossExtension", "poolStopLossSchedule(address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_propose_oracle_stop_loss_decay(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "proposeOracleStopLossDecay(address,uint256)", self.address, args, value=value)

    def prepare_propose_oracle_stop_loss_drawdown(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "proposeOracleStopLossDrawdown(address,uint256)", self.address, args, value=value)

    def prepare_propose_oracle_stop_loss_high_watermarks(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "proposeOracleStopLossHighWatermarks(address,int16,uint104,uint104)", self.address, args, value=value)

    def prepare_propose_oracle_stop_loss_timelock(self, args=(), *, value="0x0"):
        return self._client.prepare_call("OracleValueStopLossExtension", "proposeOracleStopLossTimelock(address,uint40)", self.address, args, value=value)


class PriceProvider:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def bps_base(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "BPS_BASE()", self.address, args, block=block, from_address=from_address, value=value)

    async def bps_base_u(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "BPS_BASE_U()", self.address, args, block=block, from_address=from_address, value=value)

    async def confidence_cooldown(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "CONFIDENCE_COOLDOWN()", self.address, args, block=block, from_address=from_address, value=value)

    async def confidence_max(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "CONFIDENCE_MAX()", self.address, args, block=block, from_address=from_address, value=value)

    async def max_time_delta(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "MAX_TIME_DELTA()", self.address, args, block=block, from_address=from_address, value=value)

    async def base_token(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "baseToken()", self.address, args, block=block, from_address=from_address, value=value)

    async def confidence_param(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "confidenceParam()", self.address, args, block=block, from_address=from_address, value=value)

    async def factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "factory()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_get_quote(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProvider", "getQuote()", self.address, args, value=value)

    async def last_confidence_update(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "lastConfidenceUpdate()", self.address, args, block=block, from_address=from_address, value=value)

    async def margin_step(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "marginStep()", self.address, args, block=block, from_address=from_address, value=value)

    async def offchain_feed_id(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "offchainFeedId()", self.address, args, block=block, from_address=from_address, value=value)

    async def offchain_oracle(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "offchainOracle()", self.address, args, block=block, from_address=from_address, value=value)

    async def quote_token(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "quoteToken()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_set_confidence_param(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProvider", "setConfidenceParam(uint256)", self.address, args, value=value)

    async def token0(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "token0()", self.address, args, block=block, from_address=from_address, value=value)

    async def token1(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "token1()", self.address, args, block=block, from_address=from_address, value=value)

    async def version(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProvider", "version()", self.address, args, block=block, from_address=from_address, value=value)


class PriceProviderFactory:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def default_admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "DEFAULT_ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_add_provider(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "addProvider(address)", self.address, args, value=value)

    def prepare_create_price_provider(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "createPriceProvider(address,bytes32,int256,uint256,address,address)", self.address, args, value=value)

    async def get_all_providers(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "getAllProviders(uint256,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_providers(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "getProviders(address,uint256,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_role_admin(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "getRoleAdmin(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_grant_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "grantRole(bytes32,address)", self.address, args, value=value)

    def prepare_grant_updater(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "grantUpdater(address,address)", self.address, args, value=value)

    async def has_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "hasRole(bytes32,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def is_provider(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "isProvider(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def is_updater(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "isUpdater(address,address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_multicall(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "multicall(bytes[])", self.address, args, value=value)

    async def provider_at(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "providerAt(uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def provider_count(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "providerCount()", self.address, args, block=block, from_address=from_address, value=value)

    async def provider_count_by_creator(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "providerCountByCreator(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def provider_owner(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "providerOwner(address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_remove_provider(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "removeProvider(address)", self.address, args, value=value)

    def prepare_renounce_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "renounceRole(bytes32,address)", self.address, args, value=value)

    def prepare_revoke_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "revokeRole(bytes32,address)", self.address, args, value=value)

    def prepare_revoke_updater(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "revokeUpdater(address,address)", self.address, args, value=value)

    def prepare_set_confidence(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "setConfidence(address[],uint256[])", self.address, args, value=value)

    async def supports_interface(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "supportsInterface(bytes4)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_transfer_provider_ownership(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceProviderFactory", "transferProviderOwnership(address,address)", self.address, args, value=value)

    async def version(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceProviderFactory", "version()", self.address, args, block=block, from_address=from_address, value=value)


class PriceVelocityGuardExtension:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceVelocityGuardExtension", "FACTORY()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_after_add_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceVelocityGuardExtension", "afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", self.address, args, value=value)

    def prepare_after_remove_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceVelocityGuardExtension", "afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", self.address, args, value=value)

    def prepare_after_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceVelocityGuardExtension", "afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)", self.address, args, value=value)

    def prepare_before_add_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceVelocityGuardExtension", "beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", self.address, args, value=value)

    def prepare_before_remove_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceVelocityGuardExtension", "beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", self.address, args, value=value)

    def prepare_before_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceVelocityGuardExtension", "beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)", self.address, args, value=value)

    def prepare_initialize(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceVelocityGuardExtension", "initialize(address,bytes)", self.address, args, value=value)

    async def price_velocity_state(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PriceVelocityGuardExtension", "priceVelocityState(address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_set_anchor_mid_price(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceVelocityGuardExtension", "setAnchorMidPrice(address,uint128)", self.address, args, value=value)

    def prepare_set_max_change_per_block(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PriceVelocityGuardExtension", "setMaxChangePerBlock(address,uint64)", self.address, args, value=value)


class PythOracle:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def bps_base(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "BPS_BASE()", self.address, args, block=block, from_address=from_address, value=value)

    async def default_admin_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "DEFAULT_ADMIN_ROLE()", self.address, args, block=block, from_address=from_address, value=value)

    async def stall_spread(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "STALL_SPREAD()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_accept_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "acceptStateGuardRole(bytes32)", self.address, args, value=value)

    def prepare_add_approved_factory(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "addApprovedFactory(address)", self.address, args, value=value)

    def prepare_add_integrator(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "addIntegrator(address)", self.address, args, value=value)

    async def approved_factory_count(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "approvedFactoryCount()", self.address, args, block=block, from_address=from_address, value=value)

    async def blacklisted(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "blacklisted(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def decimals(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "decimals()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_approved_factories(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "getApprovedFactories(uint256,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_integrators(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "getIntegrators(uint256,uint256)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_oracle_data(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "getOracleData(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def get_oracle_data_bulk(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "getOracleDataBulk(bytes32[])", self.address, args, block=block, from_address=from_address, value=value)

    async def get_role_admin(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "getRoleAdmin(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_grant_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "grantRole(bytes32,address)", self.address, args, value=value)

    async def has_role(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "hasRole(bytes32,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def integrator_count(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "integratorCount()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_integrator_price(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "integratorPrice(bytes32)", self.address, args, value=value)

    async def is_approved_factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "isApprovedFactory(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def is_integrator(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "isIntegrator(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def kind(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "kind()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_multicall(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "multicall(bytes[])", self.address, args, value=value)

    async def pending_state_guard(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "pendingStateGuard(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_price(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "price(bytes32,address)", self.address, args, value=value)

    async def price_guard(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "priceGuard(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_purge_pending_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "purgePendingStateGuardRole(bytes32)", self.address, args, value=value)

    def prepare_purge_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "purgeStateGuardRole(bytes32)", self.address, args, value=value)

    def prepare_register(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "register(bytes32,address,address)", self.address, args, value=value)

    async def registered_pool(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "registeredPool(bytes32,address)", self.address, args, block=block, from_address=from_address, value=value)

    async def registration_fee(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "registrationFee()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_remove_approved_factory(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "removeApprovedFactory(address)", self.address, args, value=value)

    def prepare_remove_integrator(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "removeIntegrator(address)", self.address, args, value=value)

    def prepare_renounce_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "renounceRole(bytes32,address)", self.address, args, value=value)

    def prepare_revoke_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "revokeRole(bytes32,address)", self.address, args, value=value)

    def prepare_set_blacklist(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "setBlacklist(address,bool)", self.address, args, value=value)

    def prepare_set_integrators(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "setIntegrators(address[],bool)", self.address, args, value=value)

    def prepare_set_price_guard(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "setPriceGuard(bytes32,uint128,uint128)", self.address, args, value=value)

    def prepare_set_registration_fee(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "setRegistrationFee(uint256)", self.address, args, value=value)

    def prepare_set_state_guard_role(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "setStateGuardRole(bytes32,address)", self.address, args, value=value)

    async def state_guard(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "stateGuard(bytes32)", self.address, args, block=block, from_address=from_address, value=value)

    async def supports_interface(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "supportsInterface(bytes4)", self.address, args, block=block, from_address=from_address, value=value)

    async def version(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("PythOracle", "version()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_withdraw_eth(self, args=(), *, value="0x0"):
        return self._client.prepare_call("PythOracle", "withdrawEth()", self.address, args, value=value)


class RobinhoodStockTokenParams:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def small_change_bps(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("RobinhoodStockTokenParams", "SMALL_CHANGE_BPS()", self.address, args, block=block, from_address=from_address, value=value)

    async def get_oracle_params(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("RobinhoodStockTokenParams", "getOracleParams(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def version(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("RobinhoodStockTokenParams", "version()", self.address, args, block=block, from_address=from_address, value=value)


class SwapAllowlistExtension:
    def __init__(self, client, address):
        self._client = client
        self.address = address

    async def factory(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("SwapAllowlistExtension", "FACTORY()", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_after_add_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("SwapAllowlistExtension", "afterAddLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", self.address, args, value=value)

    def prepare_after_remove_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("SwapAllowlistExtension", "afterRemoveLiquidity(address,address,uint80,(int256[],uint256[]),uint256,uint256,bytes)", self.address, args, value=value)

    def prepare_after_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("SwapAllowlistExtension", "afterSwap(address,address,bool,int128,uint128,uint256,uint256,uint128,uint128,uint128,int128,int128,uint256,bytes)", self.address, args, value=value)

    async def allow_all_swappers(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("SwapAllowlistExtension", "allowAllSwappers(address)", self.address, args, block=block, from_address=from_address, value=value)

    async def allowed_swapper(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("SwapAllowlistExtension", "allowedSwapper(address,address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_before_add_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("SwapAllowlistExtension", "beforeAddLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", self.address, args, value=value)

    def prepare_before_remove_liquidity(self, args=(), *, value="0x0"):
        return self._client.prepare_call("SwapAllowlistExtension", "beforeRemoveLiquidity(address,address,uint80,(int256[],uint256[]),bytes)", self.address, args, value=value)

    async def before_swap(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("SwapAllowlistExtension", "beforeSwap(address,address,bool,int128,uint128,uint256,uint128,uint128,uint128,bytes)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_initialize(self, args=(), *, value="0x0"):
        return self._client.prepare_call("SwapAllowlistExtension", "initialize(address,bytes)", self.address, args, value=value)

    async def is_allowed_to_swap(self, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._client.call("SwapAllowlistExtension", "isAllowedToSwap(address,address)", self.address, args, block=block, from_address=from_address, value=value)

    def prepare_set_allow_all_swappers(self, args=(), *, value="0x0"):
        return self._client.prepare_call("SwapAllowlistExtension", "setAllowAllSwappers(address,bool)", self.address, args, value=value)

    def prepare_set_allowed_to_swap(self, args=(), *, value="0x0"):
        return self._client.prepare_call("SwapAllowlistExtension", "setAllowedToSwap(address,address,bool)", self.address, args, value=value)


CONTRACTS = {"AnchoredPriceProvider":AnchoredPriceProvider,"AnchoredProviderFactory":AnchoredProviderFactory,"B20AssetParams":B20AssetParams,"ChainlinkOracle":ChainlinkOracle,"CompressedOracleV1":CompressedOracleV1,"DepositAllowlistExtension":DepositAllowlistExtension,"ExternalSwapExecutor":ExternalSwapExecutor,"IERC20":IERC20,"ImmutableAppPoolHelper":ImmutableAppPoolHelper,"MetricOmmPool":MetricOmmPool,"MetricOmmPoolDataProvider":MetricOmmPoolDataProvider,"MetricOmmPoolDeployer":MetricOmmPoolDeployer,"MetricOmmPoolFactory":MetricOmmPoolFactory,"MetricOmmPoolLiquidityAdder":MetricOmmPoolLiquidityAdder,"MetricOmmSimpleRouter":MetricOmmSimpleRouter,"MetricOmmSwapQuoter":MetricOmmSwapQuoter,"Multicall3":Multicall3,"MultiplierOracle":MultiplierOracle,"OracleProvider":OracleProvider,"OracleValueStopLossExtension":OracleValueStopLossExtension,"PriceProvider":PriceProvider,"PriceProviderFactory":PriceProviderFactory,"PriceVelocityGuardExtension":PriceVelocityGuardExtension,"PythOracle":PythOracle,"RobinhoodStockTokenParams":RobinhoodStockTokenParams,"SwapAllowlistExtension":SwapAllowlistExtension}

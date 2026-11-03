//! Generated typed bindings from pinned official ABI snapshots.
pub mod anchored_price_provider {
    alloy_sol_types::sol!(AnchoredPriceProvider, "abi/AnchoredPriceProvider.json");
}
pub use anchored_price_provider::AnchoredPriceProvider;
pub mod anchored_provider_factory {
    alloy_sol_types::sol!(AnchoredProviderFactory, "abi/AnchoredProviderFactory.json");
}
pub use anchored_provider_factory::AnchoredProviderFactory;
pub mod b20_asset_params {
    alloy_sol_types::sol!(B20AssetParams, "abi/B20AssetParams.json");
}
pub use b20_asset_params::B20AssetParams;
pub mod chainlink_oracle {
    alloy_sol_types::sol!(ChainlinkOracle, "abi/ChainlinkOracle.json");
}
pub use chainlink_oracle::ChainlinkOracle;
pub mod compressed_oracle_v1 {
    alloy_sol_types::sol!(CompressedOracleV1, "abi/CompressedOracleV1.json");
}
pub use compressed_oracle_v1::CompressedOracleV1;
pub mod deposit_allowlist_extension {
    alloy_sol_types::sol!(
        DepositAllowlistExtension,
        "abi/DepositAllowlistExtension.json"
    );
}
pub use deposit_allowlist_extension::DepositAllowlistExtension;
pub mod external_swap_executor {
    alloy_sol_types::sol!(ExternalSwapExecutor, "abi/ExternalSwapExecutor.json");
}
pub use external_swap_executor::ExternalSwapExecutor;
pub mod ierc20 {
    alloy_sol_types::sol!(IERC20, "abi/IERC20.json");
}
pub use ierc20::IERC20;
pub mod immutable_app_pool_helper {
    alloy_sol_types::sol!(ImmutableAppPoolHelper, "abi/ImmutableAppPoolHelper.json");
}
pub use immutable_app_pool_helper::ImmutableAppPoolHelper;
pub mod metric_omm_pool {
    alloy_sol_types::sol!(MetricOmmPool, "abi/MetricOmmPool.json");
}
pub use metric_omm_pool::MetricOmmPool;
pub mod metric_omm_pool_data_provider {
    alloy_sol_types::sol!(
        MetricOmmPoolDataProvider,
        "abi/MetricOmmPoolDataProvider.json"
    );
}
pub use metric_omm_pool_data_provider::MetricOmmPoolDataProvider;
pub mod metric_omm_pool_deployer {
    alloy_sol_types::sol!(MetricOmmPoolDeployer, "abi/MetricOmmPoolDeployer.json");
}
pub use metric_omm_pool_deployer::MetricOmmPoolDeployer;
pub mod metric_omm_pool_factory {
    alloy_sol_types::sol!(MetricOmmPoolFactory, "abi/MetricOmmPoolFactory.json");
}
pub use metric_omm_pool_factory::MetricOmmPoolFactory;
pub mod metric_omm_pool_liquidity_adder {
    alloy_sol_types::sol!(
        MetricOmmPoolLiquidityAdder,
        "abi/MetricOmmPoolLiquidityAdder.json"
    );
}
pub use metric_omm_pool_liquidity_adder::MetricOmmPoolLiquidityAdder;
pub mod metric_omm_simple_router {
    alloy_sol_types::sol!(MetricOmmSimpleRouter, "abi/MetricOmmSimpleRouter.json");
}
pub use metric_omm_simple_router::MetricOmmSimpleRouter;
pub mod metric_omm_swap_quoter {
    alloy_sol_types::sol!(MetricOmmSwapQuoter, "abi/MetricOmmSwapQuoter.json");
}
pub use metric_omm_swap_quoter::MetricOmmSwapQuoter;
pub mod multicall3 {
    alloy_sol_types::sol!(Multicall3, "abi/Multicall3.json");
}
pub use multicall3::Multicall3;
pub mod multiplier_oracle {
    alloy_sol_types::sol!(MultiplierOracle, "abi/MultiplierOracle.json");
}
pub use multiplier_oracle::MultiplierOracle;
pub mod oracle_provider {
    alloy_sol_types::sol!(OracleProvider, "abi/OracleProvider.json");
}
pub use oracle_provider::OracleProvider;
pub mod oracle_value_stop_loss_extension {
    alloy_sol_types::sol!(
        OracleValueStopLossExtension,
        "abi/OracleValueStopLossExtension.json"
    );
}
pub use oracle_value_stop_loss_extension::OracleValueStopLossExtension;
pub mod price_provider {
    alloy_sol_types::sol!(PriceProvider, "abi/PriceProvider.json");
}
pub use price_provider::PriceProvider;
pub mod price_provider_factory {
    alloy_sol_types::sol!(PriceProviderFactory, "abi/PriceProviderFactory.json");
}
pub use price_provider_factory::PriceProviderFactory;
pub mod price_velocity_guard_extension {
    alloy_sol_types::sol!(
        PriceVelocityGuardExtension,
        "abi/PriceVelocityGuardExtension.json"
    );
}
pub use price_velocity_guard_extension::PriceVelocityGuardExtension;
pub mod pyth_oracle {
    alloy_sol_types::sol!(PythOracle, "abi/PythOracle.json");
}
pub use pyth_oracle::PythOracle;
pub mod robinhood_stock_token_params {
    alloy_sol_types::sol!(
        RobinhoodStockTokenParams,
        "abi/RobinhoodStockTokenParams.json"
    );
}
pub use robinhood_stock_token_params::RobinhoodStockTokenParams;
pub mod swap_allowlist_extension {
    alloy_sol_types::sol!(SwapAllowlistExtension, "abi/SwapAllowlistExtension.json");
}
pub use swap_allowlist_extension::SwapAllowlistExtension;
pub const ABI_NAMES: &[&str] = &[
    "AnchoredPriceProvider",
    "AnchoredProviderFactory",
    "B20AssetParams",
    "ChainlinkOracle",
    "CompressedOracleV1",
    "DepositAllowlistExtension",
    "ExternalSwapExecutor",
    "IERC20",
    "ImmutableAppPoolHelper",
    "MetricOmmPool",
    "MetricOmmPoolDataProvider",
    "MetricOmmPoolDeployer",
    "MetricOmmPoolFactory",
    "MetricOmmPoolLiquidityAdder",
    "MetricOmmSimpleRouter",
    "MetricOmmSwapQuoter",
    "Multicall3",
    "MultiplierOracle",
    "OracleProvider",
    "OracleValueStopLossExtension",
    "PriceProvider",
    "PriceProviderFactory",
    "PriceVelocityGuardExtension",
    "PythOracle",
    "RobinhoodStockTokenParams",
    "SwapAllowlistExtension",
];
pub fn abi_json(name: &str) -> Option<&'static str> {
    match name {
        "AnchoredPriceProvider" => Some(include_str!("../abi/AnchoredPriceProvider.json")),
        "AnchoredProviderFactory" => Some(include_str!("../abi/AnchoredProviderFactory.json")),
        "B20AssetParams" => Some(include_str!("../abi/B20AssetParams.json")),
        "ChainlinkOracle" => Some(include_str!("../abi/ChainlinkOracle.json")),
        "CompressedOracleV1" => Some(include_str!("../abi/CompressedOracleV1.json")),
        "DepositAllowlistExtension" => Some(include_str!("../abi/DepositAllowlistExtension.json")),
        "ExternalSwapExecutor" => Some(include_str!("../abi/ExternalSwapExecutor.json")),
        "IERC20" => Some(include_str!("../abi/IERC20.json")),
        "ImmutableAppPoolHelper" => Some(include_str!("../abi/ImmutableAppPoolHelper.json")),
        "MetricOmmPool" => Some(include_str!("../abi/MetricOmmPool.json")),
        "MetricOmmPoolDataProvider" => Some(include_str!("../abi/MetricOmmPoolDataProvider.json")),
        "MetricOmmPoolDeployer" => Some(include_str!("../abi/MetricOmmPoolDeployer.json")),
        "MetricOmmPoolFactory" => Some(include_str!("../abi/MetricOmmPoolFactory.json")),
        "MetricOmmPoolLiquidityAdder" => {
            Some(include_str!("../abi/MetricOmmPoolLiquidityAdder.json"))
        }
        "MetricOmmSimpleRouter" => Some(include_str!("../abi/MetricOmmSimpleRouter.json")),
        "MetricOmmSwapQuoter" => Some(include_str!("../abi/MetricOmmSwapQuoter.json")),
        "Multicall3" => Some(include_str!("../abi/Multicall3.json")),
        "MultiplierOracle" => Some(include_str!("../abi/MultiplierOracle.json")),
        "OracleProvider" => Some(include_str!("../abi/OracleProvider.json")),
        "OracleValueStopLossExtension" => {
            Some(include_str!("../abi/OracleValueStopLossExtension.json"))
        }
        "PriceProvider" => Some(include_str!("../abi/PriceProvider.json")),
        "PriceProviderFactory" => Some(include_str!("../abi/PriceProviderFactory.json")),
        "PriceVelocityGuardExtension" => {
            Some(include_str!("../abi/PriceVelocityGuardExtension.json"))
        }
        "PythOracle" => Some(include_str!("../abi/PythOracle.json")),
        "RobinhoodStockTokenParams" => Some(include_str!("../abi/RobinhoodStockTokenParams.json")),
        "SwapAllowlistExtension" => Some(include_str!("../abi/SwapAllowlistExtension.json")),
        _ => None,
    }
}

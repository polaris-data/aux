//! Metric compatibility wrapper around the shared EVM execution client.
pub(crate) use aux_rs_evm::validate_signed_transaction;
pub use aux_rs_evm::{
    ChainConfig, EvmSigner, LocalEvmSigner, PendingTransaction, PreparedTransaction,
    SignedTransaction,
};
#[derive(Clone, Debug)]
pub struct MetricChainClient<T = aux_rs_core::DefaultTransport>(aux_rs_evm::EvmChainClient<T>);
impl<T> std::ops::Deref for MetricChainClient<T> {
    type Target = aux_rs_evm::EvmChainClient<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> MetricChainClient<T> {
    pub fn with_transport(config: ChainConfig, transport: T) -> Self {
        Self(aux_rs_evm::EvmChainClient::with_transport(
            config, transport,
        ))
    }
}
#[cfg(feature = "http")]
impl MetricChainClient<aux_rs_core::StandardTransport> {
    pub fn new(config: ChainConfig) -> Result<Self, aux_rs_core::TransportError> {
        Ok(Self(aux_rs_evm::EvmChainClient::new(config)?))
    }
    pub fn with_config(
        config: ChainConfig,
        transport_config: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        Ok(Self(aux_rs_evm::EvmChainClient::with_config(
            config,
            transport_config,
        )?))
    }
}

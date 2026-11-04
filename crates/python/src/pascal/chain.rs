use super::*;
#[derive(Clone)]
struct Resources {
    client: v::PascalChainClient<StandardTransport>,
    signer: Option<MessageSigner>,
}
impl ClientResources for Resources {
    fn transport(&self) -> &StandardTransport {
        self.client.transport()
    }
}
#[pyclass(module = "aux_py._native")]
pub struct PascalChainClient {
    state: Arc<ClientState<Resources>>,
}
#[pymethods]
impl PascalChainClient {
    #[new]
    #[pyo3(signature=(*,rpc_url,mint,decimals=6,signer=None,timeout_ms=10000))]
    fn new(
        py: Python<'_>,
        rpc_url: String,
        mint: String,
        decimals: u8,
        signer: Option<PyRef<'_, PascalSigner>>,
        timeout_ms: u64,
    ) -> PyResult<Self> {
        let config = v::SolanaConfig::new(rpc_url, mint, decimals).map_err(local)?;
        let transport = state::build(py, || {
            StandardTransport::new(aux_rs::TransportConfig {
                timeout: std::time::Duration::from_millis(timeout_ms),
                ..Default::default()
            })
        })?;
        Ok(Self {
            state: ClientState::new(Resources {
                client: v::PascalChainClient::with_transport(config, transport),
                signer: signer.map(|s| s.inner.clone()),
            }),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    #[getter]
    fn pending(&self) -> PyResult<Vec<String>> {
        Ok(self.state.client()?.client.pending())
    }
    fn associated_token_address(&self, owner: &str, mint: &str) -> PyResult<String> {
        self.state.client()?;
        v::associated_token_address(owner, mint).map_err(local)
    }
    fn clear_reconciled_submission(&self, signature: &str) -> PyResult<()> {
        self.state
            .client()?
            .client
            .clear_reconciled_submission(signature)
            .map_err(local)
    }
    fn reconcile_deposit_credit<'py>(
        &self,
        py: Python<'py>,
        api: PyRef<'_, PascalClient>,
        owner: String,
        signature: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let chain = self.state.client()?.client;
        let api = api.state.client()?.client;
        self.state.run(py, async move {
            chain
                .reconcile_deposit_credit(
                    &api,
                    &signature,
                    serde_json::from_value(serde_json::json!({"owner":owner}))
                        .map_err(|_| errors::ValidationError::new_err("invalid owner"))?,
                )
                .await
                .map_err(native)
        })
    }
    fn account_info<'py>(&self, py: Python<'py>, address: String) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.client;
        self.state.run(
            py,
            async move { c.account_info(&address).await.map_err(native) },
        )
    }
    fn token_balance<'py>(&self, py: Python<'py>, address: String) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.token_balance(&address).await.map_err(native)
        })
    }
    fn block_height<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.client;
        self.state
            .run(py, async move { c.block_height().await.map_err(native) })
    }
    fn signature_status<'py>(
        &self,
        py: Python<'py>,
        signature: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.signature_status(&signature).await.map_err(native)
        })
    }
    fn prepare_deposit<'py>(
        &self,
        py: Python<'py>,
        owner: String,
        source_token_account: String,
        destination_token_account: String,
        amount: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let amount = v::integer(&convert::from_python(&amount)?).map_err(local)?;
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.prepare_deposit(
                &owner,
                &source_token_account,
                &destination_token_account,
                amount,
            )
            .await
            .map_err(native)
        })
    }
    fn sign_deposit<'py>(
        &self,
        py: Python<'py>,
        prepared: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let prepared: v::PreparedDeposit = convert::parse(&prepared)?;
        let signer = self
            .state
            .client()?
            .signer
            .ok_or_else(|| errors::AuthenticationError::new_err("PascalSigner required"))?
            .scoped(py)?;
        self.state.run(
            py,
            async move { prepared.sign(&signer).await.map_err(local) },
        )
    }
    fn submit<'py>(
        &self,
        py: Python<'py>,
        signed: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let signed: v::SignedDeposit = convert::parse(&signed)?;
        let c = self.state.client()?.client;
        self.state
            .run(py, async move { c.submit(&signed).await.map_err(native) })
    }
    fn reconcile<'py>(&self, py: Python<'py>, signature: String) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.client;
        self.state.run(
            py,
            async move { c.reconcile(&signature).await.map_err(native) },
        )
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "PascalChainClient()"
    }
}
impl Drop for PascalChainClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}

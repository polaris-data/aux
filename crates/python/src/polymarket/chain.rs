use crate::metric::{Signer, transaction_request};
use crate::{
    convert, errors,
    prediction_signers::PolymarketSigner,
    state::{self, ClientState},
};
use aux_rs::{StandardTransport, polymarket as m};
use pyo3::{prelude::*, types::PyAny};
use std::sync::Arc;
fn local(e: m::Error) -> PyErr {
    errors::polymarket(aux_rs::Error::Exchange(e))
}
#[derive(Clone)]
struct ChainClients {
    client: m::PolymarketChainClient<StandardTransport>,
    signer: Option<Signer>,
}
impl state::ClientResources for ChainClients {
    fn transport(&self) -> &StandardTransport {
        self.client.transport()
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct PolymarketChainClient {
    state: Arc<ClientState<ChainClients>>,
}
fn parse<T: std::str::FromStr>(s: &str) -> PyResult<T> {
    s.parse()
        .map_err(|_| errors::ValidationError::new_err("invalid native EVM identifier"))
}
#[pymethods]
impl PolymarketChainClient {
    #[new]
    #[pyo3(signature=(*,chain_id,read_rpc,write_rpc=None,signer=None,timeout_ms=10_000,code_hashes=None))]
    fn new(
        py: Python<'_>,
        chain_id: u64,
        read_rpc: String,
        write_rpc: Option<String>,
        signer: Option<PyRef<'_, PolymarketSigner>>,
        timeout_ms: u64,
        code_hashes: Option<std::collections::BTreeMap<String, String>>,
    ) -> PyResult<Self> {
        let mut config =
            m::ChainConfig::new(chain_id, read_rpc.clone(), write_rpc.unwrap_or(read_rpc))
                .map_err(|e| errors::polymarket_chain(aux_rs::Error::Exchange(e)))?;
        if let Some(hashes) = code_hashes {
            for (address, hash) in hashes {
                config = config.with_code_hash(parse(&address)?, parse(&hash)?);
            }
        }
        let transport_config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            ..Default::default()
        };
        let client = state::build(py, || {
            m::PolymarketChainClient::with_config(config, transport_config)
        })?;
        Ok(Self {
            state: ClientState::new(ChainClients {
                client,
                signer: signer.map(|s| s.inner.clone()),
            }),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    fn validate_chain<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.validate_chain().await.map_err(errors::polymarket_chain)
        })
    }
    fn block_number<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.block_number().await.map_err(errors::polymarket_chain)
        })
    }
    #[pyo3(signature=(number="latest"))]
    fn block<'py>(&self, py: Python<'py>, number: &str) -> PyResult<Bound<'py, PyAny>> {
        let number = number.to_string();
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.block(&number).await.map_err(errors::polymarket_chain)
        })
    }
    #[pyo3(signature=(address,*,block="latest"))]
    fn native_balance<'py>(
        &self,
        py: Python<'py>,
        address: &str,
        block: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let address = parse(address)?;
        let block = block.to_string();
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.native_balance(address, &block)
                .await
                .map_err(errors::polymarket_chain)
        })
    }
    #[pyo3(signature=(address,*,block="latest"))]
    fn code<'py>(
        &self,
        py: Python<'py>,
        address: &str,
        block: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let address = parse(address)?;
        let block = block.to_string();
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.code(address, &block)
                .await
                .map_err(errors::polymarket_chain)
        })
    }
    fn encode_call(
        &self,
        contract: &str,
        function: &str,
        args: Bound<'_, PyAny>,
    ) -> PyResult<String> {
        self.state.client()?;
        Ok(
            m::encode_contract_call(contract, function, &convert::from_python(&args)?)
                .map_err(local)?
                .to_string(),
        )
    }
    #[pyo3(signature=(contract,function,to,args,*,value="0x0"))]
    fn prepare_call(
        &self,
        contract: &str,
        function: &str,
        to: &str,
        args: Bound<'_, PyAny>,
        value: &str,
    ) -> PyResult<Py<PyAny>> {
        self.state.client()?;
        let data = m::encode_contract_call(contract, function, &convert::from_python(&args)?)
            .map_err(local)?;
        convert::output(m::PreparedCall {
            to: parse(to)?,
            data,
            value: parse(value)?,
        })
    }

    fn gas_price<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.gas_price().await.map_err(errors::polymarket_chain)
        })
    }
    fn estimate_gas<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r = convert::from_python(&request)?;
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.estimate_gas(r).await.map_err(errors::polymarket_chain)
        })
    }
    fn resync_nonce<'py>(
        &self,
        py: Python<'py>,
        address: &str,
        expected_hash: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let address = parse(address)?;
        let hash = parse(expected_hash)?;
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.resync_nonce(address, hash)
                .await
                .map_err(errors::polymarket_chain)
        })
    }
    fn decode_input(&self, contract: &str, function: &str, data: &str) -> PyResult<Py<PyAny>> {
        self.state.client()?;
        let b: m::Bytes = parse(data)?;
        convert::output(m::decode_contract_input(contract, function, &b).map_err(local)?)
    }
    fn decode_output(&self, contract: &str, function: &str, data: &str) -> PyResult<Py<PyAny>> {
        self.state.client()?;
        let b: m::Bytes = parse(data)?;
        convert::output(m::decode_contract_output(contract, function, &b).map_err(local)?)
    }
    fn decode_event(
        &self,
        contract: &str,
        event: &str,
        topics: Vec<String>,
        data: &str,
    ) -> PyResult<Py<PyAny>> {
        self.state.client()?;
        let topics: Vec<m::B256> = topics.iter().map(|s| parse(s)).collect::<PyResult<_>>()?;
        let b: m::Bytes = parse(data)?;
        convert::output(m::decode_contract_event(contract, event, &topics, &b).map_err(local)?)
    }
    fn decode_error(&self, contract: &str, data: &str) -> PyResult<Py<PyAny>> {
        self.state.client()?;
        let b: m::Bytes = parse(data)?;
        convert::output(m::decode_contract_error(contract, &b).map_err(local)?)
    }
    #[pyo3(signature=(contract,function,to,args,*,block="latest",from_address=None,value="0x0"))]
    #[allow(clippy::too_many_arguments)]
    fn call<'py>(
        &self,
        py: Python<'py>,
        contract: String,
        function: String,
        to: String,
        args: Bound<'_, PyAny>,
        block: &str,
        from_address: Option<String>,
        value: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let data = m::encode_contract_call(&contract, &function, &convert::from_python(&args)?)
            .map_err(local)?;
        let to = parse(&to)?;
        let from = from_address.as_ref().map(|s| parse(s)).transpose()?;
        let value = parse(value)?;
        let block = block.to_string();
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            let b = c
                .eth_call(to, data, from, value, &block)
                .await
                .map_err(errors::polymarket_chain)?;
            m::decode_contract_output(&contract, &function, &b).map_err(local)
        })
    }
    fn send_transaction<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r = transaction_request(&request)?;
        let c = self.state.client()?;
        let s = c
            .signer
            .ok_or_else(|| errors::AuthenticationError::new_err("PolymarketSigner required"))?
            .scoped(py)?;
        self.state.run(py, async move {
            c.client
                .send_transaction(r, &s)
                .await
                .map_err(errors::polymarket_chain)
        })
    }
    fn submit_raw<'py>(&self, py: Python<'py>, data: &str) -> PyResult<Bound<'py, PyAny>> {
        let data = parse(data)?;
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.submit_raw(data).await.map_err(errors::polymarket_chain)
        })
    }
    fn receipt<'py>(&self, py: Python<'py>, hash: &str) -> PyResult<Bound<'py, PyAny>> {
        let h = parse(hash)?;
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.receipt(h).await.map_err(errors::polymarket_chain)
        })
    }
    fn transaction<'py>(&self, py: Python<'py>, hash: &str) -> PyResult<Bound<'py, PyAny>> {
        let h = parse(hash)?;
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.transaction(h).await.map_err(errors::polymarket_chain)
        })
    }
    fn logs<'py>(&self, py: Python<'py>, filter: Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let r = convert::from_python(&filter)?;
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.logs(r).await.map_err(errors::polymarket_chain)
        })
    }
    fn pending(&self, address: &str) -> PyResult<Py<PyAny>> {
        convert::output(self.state.client()?.client.pending(parse(address)?))
    }
    fn reconcile<'py>(&self, py: Python<'py>, address: &str) -> PyResult<Bound<'py, PyAny>> {
        let address = parse(address)?;
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.reconcile(address).await.map_err(errors::polymarket_chain)
        })
    }
    fn transaction_count<'py>(
        &self,
        py: Python<'py>,
        address: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let address = parse(address)?;
        let c = self.state.client()?.client;
        self.state.run(py, async move {
            c.transaction_count(address)
                .await
                .map_err(errors::polymarket_chain)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
}
impl Drop for PolymarketChainClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}

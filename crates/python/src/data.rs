//! Thin Python ownership and asyncio glue for the Rust data clients.
use crate::{
    errors,
    state::{ClientResources, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{StandardTransport, TransportConfig, databento as db, tardis as td};
use pyo3::prelude::*;
use std::sync::Arc;
fn build<T: Send>(py: Python<'_>, f: impl FnOnce() -> td::Result<T> + Send) -> PyResult<T> {
    py.detach(move || {
        let _entered = pyo3_async_runtimes::tokio::get_runtime().enter();
        f()
    })
    .map_err(errors::data)
}
fn config(timeout_ms: u64) -> td::Result<TransportConfig> {
    if timeout_ms == 0 {
        return Err(td::Error::InvalidRequest("timeout_ms must be positive"));
    }
    Ok(TransportConfig {
        timeout: std::time::Duration::from_millis(timeout_ms),
        ..TransportConfig::default()
    })
}
impl ClientResources for td::TardisClient<StandardTransport> {
    fn transport(&self) -> &StandardTransport {
        self.transport()
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct TardisClient {
    state: Arc<ClientState<td::TardisClient<StandardTransport>>>,
}
impl Drop for TardisClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
#[pymethods]
impl TardisClient {
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    #[new]
    #[pyo3(signature=(*,api_key=None,api_url="https://api.tardis.dev/v1",datasets_url="https://datasets.tardis.dev/v1",timeout_ms=10000))]
    fn new(
        py: Python<'_>,
        api_key: Option<String>,
        api_url: &str,
        datasets_url: &str,
        timeout_ms: u64,
    ) -> PyResult<Self> {
        let client = build(py, || {
            let transport = StandardTransport::new(config(timeout_ms)?)?;
            let client = td::TardisClient::with_transport(api_url, datasets_url, transport)?;
            Ok(if let Some(key) = api_key {
                client.with_api_key(key)
            } else {
                client
            })
        })?;
        Ok(Self {
            state: ClientState::new(client),
        })
    }
    fn exchanges<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?;
        self.state.run(
            py,
            async move { client.exchanges().await.map_err(errors::data) },
        )
    }
    fn api_key_info<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.api_key_info().await.map_err(errors::data)
        })
    }
    fn exchange_details<'py>(
        &self,
        py: Python<'py>,
        exchange: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .exchange_details(&exchange)
                .await
                .map_err(errors::data)
        })
    }
    fn exchange_summary<'py>(
        &self,
        py: Python<'py>,
        exchange: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .exchange_summary(&exchange)
                .await
                .map_err(errors::data)
        })
    }
    fn instrument<'py>(
        &self,
        py: Python<'py>,
        exchange: String,
        symbol: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .instrument(&exchange, &symbol)
                .await
                .map_err(errors::data)
        })
    }
    fn symbols<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.symbols(parameters).await.map_err(errors::data)
        })
    }
    fn instruments<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
        exchange: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .instruments(&exchange, parameters)
                .await
                .map_err(errors::data)
        })
    }
    fn data_feeds<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataFeed(Box::new(
                client.data_feeds(request).await.map_err(errors::data)?,
            )))
        })
    }
    fn replay<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataReplay(Box::new(
                client.replay(request).map_err(errors::data)?,
            )))
        })
    }
    fn dataset<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataDownload(
                client.dataset(request).await.map_err(errors::data)?,
            ))
        })
    }
    fn download_dataset<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
        path: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .download_dataset(request, path)
                .await
                .map_err(errors::data)
        })
    }
}
impl ClientResources for td::TardisMachineClient<StandardTransport> {
    fn transport(&self) -> &StandardTransport {
        self.transport()
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct TardisMachineClient {
    state: Arc<ClientState<td::TardisMachineClient<StandardTransport>>>,
}
impl Drop for TardisMachineClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
#[pymethods]
impl TardisMachineClient {
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    #[new]
    #[pyo3(signature=(url,*,ws_url=None,timeout_ms=10000))]
    fn new(py: Python<'_>, url: &str, ws_url: Option<&str>, timeout_ms: u64) -> PyResult<Self> {
        let client = build(py, || {
            let client = td::TardisMachineClient::with_transport(
                url,
                StandardTransport::new(config(timeout_ms)?)?,
            )?;
            if let Some(ws_url) = ws_url {
                client.with_ws_url(ws_url)
            } else {
                Ok(client)
            }
        })?;
        Ok(Self {
            state: ClientState::new(client),
        })
    }
    fn replay<'py>(
        &self,
        py: Python<'py>,
        options: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let options = crate::convert::from_python(options)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataFeed(Box::new(
                client.replay(options).await.map_err(errors::data)?,
            )))
        })
    }
    fn replay_normalized<'py>(
        &self,
        py: Python<'py>,
        options: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let options = crate::convert::from_python(options)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataFeed(Box::new(
                client
                    .replay_normalized(options)
                    .await
                    .map_err(errors::data)?,
            )))
        })
    }
    fn ws_replay_normalized<'py>(
        &self,
        py: Python<'py>,
        options: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let options = crate::convert::from_python(options)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataMachine(Box::new(
                client
                    .ws_replay_normalized(options)
                    .await
                    .map_err(errors::data)?,
            )))
        })
    }
    fn stream_normalized<'py>(
        &self,
        py: Python<'py>,
        options: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let options = crate::convert::from_python(options)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataMachine(Box::new(
                client
                    .stream_normalized(options)
                    .await
                    .map_err(errors::data)?,
            )))
        })
    }
    #[pyo3(signature=(exchange,from_date,to_date,subscribe_messages,session=None))]
    fn ws_replay<'py>(
        &self,
        py: Python<'py>,
        exchange: String,
        from_date: String,
        to_date: String,
        subscribe_messages: &Bound<'py, PyAny>,
        session: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let messages = crate::convert::parse(subscribe_messages)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataMachine(Box::new(
                client
                    .ws_replay(
                        &exchange,
                        &from_date,
                        &to_date,
                        messages,
                        session.as_deref(),
                    )
                    .await
                    .map_err(errors::data)?,
            )))
        })
    }
}
impl ClientResources for db::HistoricalClient<StandardTransport> {
    fn transport(&self) -> &StandardTransport {
        self.transport()
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct DatabentoHistoricalClient {
    state: Arc<ClientState<db::HistoricalClient<StandardTransport>>>,
}
impl Drop for DatabentoHistoricalClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
#[pymethods]
impl DatabentoHistoricalClient {
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    #[new]
    #[pyo3(signature=(api_key,*,api_url="https://hist.databento.com/v0",timeout_ms=10000))]
    fn new(py: Python<'_>, api_key: String, api_url: &str, timeout_ms: u64) -> PyResult<Self> {
        let client = build(py, || {
            db::HistoricalClient::with_transport(
                api_url,
                api_key,
                StandardTransport::new(config(timeout_ms)?)?,
            )
        })?;
        Ok(Self {
            state: ClientState::new(client),
        })
    }
    fn list_publishers<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .list_publishers(parameters)
                .await
                .map_err(errors::data)
        })
    }
    fn list_datasets<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.list_datasets(parameters).await.map_err(errors::data)
        })
    }
    fn list_schemas<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.list_schemas(parameters).await.map_err(errors::data)
        })
    }
    fn list_fields<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.list_fields(parameters).await.map_err(errors::data)
        })
    }
    fn list_unit_prices<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .list_unit_prices(parameters)
                .await
                .map_err(errors::data)
        })
    }
    fn get_dataset_condition<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .get_dataset_condition(parameters)
                .await
                .map_err(errors::data)
        })
    }
    fn get_dataset_range<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .get_dataset_range(parameters)
                .await
                .map_err(errors::data)
        })
    }
    fn get_record_count<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .get_record_count(parameters)
                .await
                .map_err(errors::data)
        })
    }
    fn get_billable_size<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .get_billable_size(parameters)
                .await
                .map_err(errors::data)
        })
    }
    fn get_cost<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.get_cost(parameters).await.map_err(errors::data)
        })
    }
    fn resolve<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.resolve(parameters).await.map_err(errors::data)
        })
    }
    fn submit_job<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.submit_job(parameters).await.map_err(errors::data)
        })
    }
    fn list_jobs<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.list_jobs(parameters).await.map_err(errors::data)
        })
    }
    fn get_job_details<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client
                .get_job_details(parameters)
                .await
                .map_err(errors::data)
        })
    }
    fn list_files<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.list_files(parameters).await.map_err(errors::data)
        })
    }
    fn timeseries<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataDownload(
                client.timeseries(parameters).await.map_err(errors::data)?,
            ))
        })
    }
    fn get_range<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataDbn(Box::new(
                client.get_range(parameters).await.map_err(errors::data)?,
            )))
        })
    }
    fn download<'py>(&self, py: Python<'py>, url: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataDownload(
                client.download(&url).await.map_err(errors::data)?,
            ))
        })
    }
    fn download_to<'py>(
        &self,
        py: Python<'py>,
        url: String,
        path: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.download_to(&url, path).await.map_err(errors::data)
        })
    }
}
impl ClientResources for db::ReferenceClient<StandardTransport> {
    fn transport(&self) -> &StandardTransport {
        self.transport()
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct DatabentoReferenceClient {
    state: Arc<ClientState<db::ReferenceClient<StandardTransport>>>,
}
impl Drop for DatabentoReferenceClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
#[pymethods]
impl DatabentoReferenceClient {
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    #[new]
    #[pyo3(signature=(api_key,*,api_url="https://hist.databento.com/v0",timeout_ms=10000))]
    fn new(py: Python<'_>, api_key: String, api_url: &str, timeout_ms: u64) -> PyResult<Self> {
        let client = build(py, || {
            db::ReferenceClient::with_transport(
                api_url,
                api_key,
                StandardTransport::new(config(timeout_ms)?)?,
            )
        })?;
        Ok(Self {
            state: ClientState::new(client),
        })
    }
    fn list_events<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.list_events(parameters).await.map_err(errors::data)
        })
    }
    fn list_enums<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.run(py, async move {
            client.list_enums(parameters).await.map_err(errors::data)
        })
    }
    fn security_master_get_range<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataReference(Box::new(
                client
                    .security_master_get_range(parameters)
                    .await
                    .map_err(errors::data)?,
            )))
        })
    }
    fn security_master_get_last<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataReference(Box::new(
                client
                    .security_master_get_last(parameters)
                    .await
                    .map_err(errors::data)?,
            )))
        })
    }
    fn corporate_actions_get_range<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataReference(Box::new(
                client
                    .corporate_actions_get_range(parameters)
                    .await
                    .map_err(errors::data)?,
            )))
        })
    }
    fn adjustment_factors_get_range<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let parameters = crate::convert::parse(request)?;
        let client = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataReference(Box::new(
                client
                    .adjustment_factors_get_range(parameters)
                    .await
                    .map_err(errors::data)?,
            )))
        })
    }
}
#[derive(Clone)]
struct LiveResources {
    client: db::LiveClient,
    transport: StandardTransport,
}
impl ClientResources for LiveResources {
    fn transport(&self) -> &StandardTransport {
        &self.transport
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct DatabentoLiveClient {
    state: Arc<ClientState<LiveResources>>,
}
impl Drop for DatabentoLiveClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
#[pymethods]
impl DatabentoLiveClient {
    #[new]
    #[pyo3(signature=(api_key,*,timeout_ms=10000,idle_timeout_ms=60000))]
    fn new(
        py: Python<'_>,
        api_key: String,
        timeout_ms: u64,
        idle_timeout_ms: u64,
    ) -> PyResult<Self> {
        if idle_timeout_ms == 0 {
            return Err(errors::ValidationError::new_err(
                "idle timeout must be positive",
            ));
        }
        let resources = build(py, || {
            let transport = StandardTransport::new(config(timeout_ms)?)?;
            let connector = db::TcpConnector {
                connect_timeout: std::time::Duration::from_millis(timeout_ms),
                idle_timeout: std::time::Duration::from_millis(idle_timeout_ms),
            };
            Ok(LiveResources {
                client: db::LiveClient::with_connector(api_key, connector)?,
                transport,
            })
        })?;
        Ok(Self {
            state: ClientState::new(resources),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn connect<'py>(
        &self,
        py: Python<'py>,
        options: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let options = crate::convert::parse(options)?;
        let resources = self.state.client()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::DataLive(Box::new(
                resources
                    .client
                    .connect(options)
                    .await
                    .map_err(errors::data)?,
            )))
        })
    }
}

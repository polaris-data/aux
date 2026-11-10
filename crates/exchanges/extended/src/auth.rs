use crate::*;
use aux_rs_core::{HttpRequest, Unauthenticated};
use std::{
    future::Future,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;

pub trait PublicAuthentication: Send + Sync {
    fn prepare(
        &self,
        exchange: &Extended,
        operation: &Operation,
        request: &mut HttpRequest,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
pub trait Authentication: PublicAuthentication {}
impl PublicAuthentication for Unauthenticated {
    async fn prepare(
        &self,
        _: &Extended,
        op: &Operation,
        _: &mut HttpRequest,
    ) -> Result<(), Error> {
        if op.authenticated {
            Err(Error::Authentication("credentials required"))
        } else {
            Ok(())
        }
    }
}
#[derive(Default)]
struct Secrets {
    api_key: Option<Zeroizing<String>>,
    access_token: Option<Zeroizing<String>>,
    expires_at: Option<u64>,
}
/// Clones share explicit credential replacements. No background renewal or persistence.
#[derive(Clone, Default)]
pub struct Credentials(Arc<Mutex<Secrets>>);
impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials").finish_non_exhaustive()
    }
}
impl Credentials {
    pub fn new(
        api_key: Option<Zeroizing<String>>,
        access_token: Option<Zeroizing<String>>,
        expires_at: Option<u64>,
    ) -> Result<Self, Error> {
        for value in [api_key.as_deref(), access_token.as_deref()]
            .into_iter()
            .flatten()
        {
            if value.is_empty() || value.contains(['\r', '\n']) {
                return Err(Error::Authentication("empty or invalid credential"));
            }
        }
        Ok(Self(Arc::new(Mutex::new(Secrets {
            api_key,
            access_token,
            expires_at,
        }))))
    }
    pub fn replace(&self, other: Self) {
        if Arc::ptr_eq(&self.0, &other.0) {
            return;
        }
        let mut source = other.0.lock().unwrap_or_else(|e| e.into_inner());
        let replacement = std::mem::take(&mut *source);
        drop(source);
        *self.0.lock().unwrap_or_else(|e| e.into_inner()) = replacement;
    }
    pub fn clear(&self) {
        *self.0.lock().unwrap_or_else(|e| e.into_inner()) = Secrets::default();
    }
    pub fn headers(&self, required: bool) -> Result<Vec<(String, String)>, Error> {
        let state = self.0.lock().unwrap_or_else(|e| e.into_inner());
        if state.expires_at.is_some_and(|t| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(true, |now| now.as_secs() >= t)
        }) {
            return Err(Error::Authentication(
                "credentials expired; replace explicitly",
            ));
        }
        let mut headers = vec![];
        if let Some(key) = &state.api_key {
            headers.push(("X-Api-Key".into(), key.to_string()));
        }
        if let Some(token) = &state.access_token {
            headers.push(("Authorization".into(), format!("Bearer {}", &**token)));
        }
        if required && headers.is_empty() {
            return Err(Error::Authentication("credentials required"));
        }
        Ok(headers)
    }
    pub(crate) fn identity(&self) -> String {
        format!("{:p}", Arc::as_ptr(&self.0))
    }
}
impl PublicAuthentication for Credentials {
    async fn prepare(
        &self,
        _: &Extended,
        op: &Operation,
        request: &mut HttpRequest,
    ) -> Result<(), Error> {
        request.headers.extend(self.headers(op.authenticated)?);
        Ok(())
    }
}
impl Authentication for Credentials {}

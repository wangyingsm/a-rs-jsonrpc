use std::{collections::HashMap, sync::LazyLock};

use futures::future::BoxFuture;
use linkme::distributed_slice;
pub use proc_macros::{jsonrpc_service_fn_array, jsonrpc_service_fn_obj};
use serde::Deserialize;

use crate::RpcError;

#[async_trait::async_trait]
pub trait JsonRpcServiceFn {
    type Result;

    async fn handle(
        req: &[u8],
    ) -> Result<crate::response::JsonRpcResponse<Self::Result>, crate::error::RpcError>
    where
        Self::Result: serde::Serialize;
}

#[derive(Deserialize)]
struct MethodEnvelope<'a> {
    #[serde(borrow)]
    method: &'a str,
}

#[derive(Debug)]
pub struct RpcServiceEntry {
    pub method: &'static str,
    pub handler: RpcHandlerFn,
}

#[distributed_slice]
pub static RPC_SERVICES: [RpcServiceEntry];

type RpcHandlerFn = fn(req: &[u8]) -> BoxFuture<'static, Result<String, crate::error::RpcError>>;

static ROUTE_TABLE: LazyLock<HashMap<&'static str, RpcHandlerFn>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    for entry in RPC_SERVICES {
        if m.insert(entry.method, entry.handler).is_some() {
            panic!("Duplicate method registered: {}", entry.method);
        }
    }
    m
});

pub async fn dispatch(body: &[u8]) -> Result<String, RpcError> {
    let MethodEnvelope { method } = serde_json::from_slice(body)?;
    if let Some(handler) = ROUTE_TABLE.get(method) {
        return handler(body).await;
    }
    Err(RpcError::MethodNotFound)
}

pub fn init() {
    tracing::info!("RPC Service initialized with {} methods", ROUTE_TABLE.len());
    for method in ROUTE_TABLE.keys() {
        tracing::info!("  - {}", method);
    }
}

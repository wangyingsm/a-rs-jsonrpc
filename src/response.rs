use serde::{Deserialize, Serialize};

use crate::{JsonRpcId, request::JsonRpcVersion};

#[derive(Debug, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: JsonRpcVersion,
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
    pub id: JsonRpcId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

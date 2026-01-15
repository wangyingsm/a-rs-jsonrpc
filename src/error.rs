use thiserror::Error;

use crate::response::JsonRpcError;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("serialize/deserialize error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("invalid json rpc version: {0}")]
    InvalidJsonRpcVersion(String),
    #[error("json rpc method not found")]
    MethodNotFound,
    #[error("custom error: {0}")]
    CustomError(String),
    #[error("invalid parameters: {0}")]
    InvalidParams(String),
}

impl From<RpcError> for JsonRpcError {
    fn from(err: RpcError) -> Self {
        match err {
            RpcError::IoError(e) => JsonRpcError {
                code: -32000,
                message: e.to_string(),
                data: None,
            },
            RpcError::ReqwestError(e) => JsonRpcError {
                code: -32001,
                message: e.to_string(),
                data: None,
            },
            RpcError::SerdeError(e) => JsonRpcError {
                code: -32002,
                message: e.to_string(),
                data: None,
            },
            RpcError::InvalidJsonRpcVersion(v) => JsonRpcError {
                code: -32600,
                message: format!("Invalid JSON-RPC version: {}", v),
                data: None,
            },
            RpcError::MethodNotFound => JsonRpcError {
                code: -32601,
                message: "method not found".to_string(),
                data: None,
            },
            RpcError::CustomError(msg) => JsonRpcError {
                code: -32003,
                message: msg,
                data: None,
            },
            RpcError::InvalidParams(msg) => JsonRpcError {
                code: -32602,
                message: format!("Invalid parameters: {}", msg),
                data: None,
            },
        }
    }
}

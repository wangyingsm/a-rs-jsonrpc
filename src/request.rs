use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{JsonRpcId, RpcError};

#[derive(Debug, PartialEq, Eq)]
pub enum JsonRpcVersion {
    V1_0,
    V2_0,
}

impl Serialize for JsonRpcVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            JsonRpcVersion::V1_0 => serializer.serialize_str("1.0"),
            JsonRpcVersion::V2_0 => serializer.serialize_str("2.0"),
        }
    }
}

impl<'de> Deserialize<'de> for JsonRpcVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "1.0" => Ok(JsonRpcVersion::V1_0),
            "2.0" => Ok(JsonRpcVersion::V2_0),
            _ => Err(serde::de::Error::custom("invalid JSON-RPC version")),
        }
    }
}

impl FromStr for JsonRpcVersion {
    type Err = RpcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.0" => Ok(JsonRpcVersion::V1_0),
            "2.0" => Ok(JsonRpcVersion::V2_0),
            _ => Err(RpcError::InvalidJsonRpcVersion(s.to_string())),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde_with::skip_serializing_none]
pub struct JsonRpcRequest<T> {
    pub jsonrpc: JsonRpcVersion,
    pub method: String,
    pub params: Option<T>,
    pub id: JsonRpcId,
}

impl<T> JsonRpcRequest<T> {
    pub fn new_v1(id: JsonRpcId, method: &str) -> Self {
        JsonRpcRequest {
            jsonrpc: JsonRpcVersion::V1_0,
            method: method.to_string(),
            params: None,
            id,
        }
    }

    pub fn new_v2(id: JsonRpcId, method: &str) -> Self {
        JsonRpcRequest {
            jsonrpc: JsonRpcVersion::V2_0,
            method: method.to_string(),
            params: None,
            id,
        }
    }

    pub fn set_params(&mut self, params: T) {
        self.params = Some(params);
    }
}

impl JsonRpcRequest<Vec<serde_json::Value>> {
    pub fn add_param<P>(&mut self, param: P)
    where
        P: Serialize,
        serde_json::Value: From<P>,
    {
        if self.params.is_none() {
            self.params = Some(vec![serde_json::Value::from(param)]);
            return;
        }
        self.params
            .as_mut()
            .unwrap()
            .push(serde_json::Value::from(param));
    }
}

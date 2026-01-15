use serde::Serialize;

use crate::{JsonRpcId, error::RpcError, request::JsonRpcRequest, response::JsonRpcResponse};

pub use proc_macros::JsonRpcClient;
pub use proc_macros::rpc_method;

#[async_trait::async_trait]
pub trait JsonRpcClient {
    async fn send_v1_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned;

    async fn send_v2_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned;

    async fn send_v1_request_obj<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        self.send_v1_request(url, content_type, method).await
    }

    async fn send_v2_request_obj<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        self.send_v2_request(url, content_type, method).await
    }
}

#[async_trait::async_trait]
pub trait JsonRpcClientCall {
    async fn call_rpc_v1<R>(&self) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned;

    async fn call_rpc_v2<R>(&self) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned;

    async fn call_rpc_v1_obj<R>(&self) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned;

    async fn call_rpc_v2_obj<R>(&self) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned;
}

macro_rules! impl_scalar_jsonrpc_client {
    ($rec:ty) => {
        #[async_trait::async_trait]
        impl JsonRpcClient for $rec where $rec: Serialize {
            async fn send_v1_request<R>(
                &self,
                url: &str,
                content_type: &str,
                method: &str,
            ) -> Result<JsonRpcResponse<R>, RpcError>
            where
                R: serde::de::DeserializeOwned,
            {
                let id = JsonRpcId::next_number();
                let mut body: JsonRpcRequest<Vec<serde_json::Value>> =
                    JsonRpcRequest::new_v1(id, method);
                body.add_param(*self);
                tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
                let resp = reqwest::Client::new()
                    .post(url)
                    .header("Content-Type", content_type)
                    .json(&body)
                    .send()
                    .await?;
                let text = resp.text().await?;
                tracing::debug!("jsonrpc response body: {}", text);
                Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
            }

            async fn send_v2_request<R>(
                &self,
                url: &str,
                content_type: &str,
                method: &str,
            ) -> Result<JsonRpcResponse<R>, RpcError>
            where
                R: serde::de::DeserializeOwned,
            {
                let id = JsonRpcId::next_number();
                let mut body: JsonRpcRequest<Vec<serde_json::Value>> =
                    JsonRpcRequest::new_v2(id, method);
                body.add_param(*self);
                tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
                let resp = reqwest::Client::new()
                    .post(url)
                    .header("Content-Type", content_type)
                    .json(&body)
                    .send()
                    .await
                    .unwrap();
                let text = resp.text().await?;
                tracing::debug!("jsonrpc response body: {}", text);
                Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
            }
        }
    };
    ($($rec:ty),*) => {
        $(impl_scalar_jsonrpc_client!($rec);)*
    };
}

impl_scalar_jsonrpc_client!(
    i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, bool
);

macro_rules! impl_tuple_jsonrpc_client {
    ($($ty:ident),*) => {
        #[async_trait::async_trait]
        #[allow(non_snake_case)]
        impl<$($ty),*> JsonRpcClient for ($($ty,)*)
        where
            $(
                serde_json::Value: From<$ty>,
                $ty: Clone + Serialize + Send + Sync,
            )*
        {
            async fn send_v1_request<R>(
                &self,
                url: &str,
                content_type: &str,
                method: &str,
            ) -> Result<JsonRpcResponse<R>, RpcError>
            where
                R: serde::de::DeserializeOwned,
            {
                let id = JsonRpcId::next_number();
                let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v1(id, method);

                let ($($ty,)*) = self;
                $(
                    body.add_param($ty.clone());
                )*
                tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));

                let resp = reqwest::Client::new()
                    .post(url)
                    .header("Content-Type", content_type)
                    .json(&body)
                    .send()
                    .await?;
                let text = resp.text().await?;
                tracing::debug!("jsonrpc response body: {}", text);
                Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
            }

            async fn send_v2_request<R>(
                &self,
                url: &str,
                content_type: &str,
                method: &str,
            ) -> Result<JsonRpcResponse<R>, RpcError>
            where
                R: serde::de::DeserializeOwned,
            {
                let id = JsonRpcId::next_number();
                let mut body: JsonRpcRequest<Vec<serde_json::Value>> =
                    JsonRpcRequest::new_v2(id, method);

                let ($($ty,)*) = self;
                $(
                    body.add_param($ty.clone());
                )*
                tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));

                let resp = reqwest::Client::new()
                    .post(url)
                    .header("Content-Type", content_type)
                    .json(&body)
                    .send()
                    .await?;
                let text = resp.text().await?;
                tracing::debug!("jsonrpc response body: {}", text);
                Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
            }
        }
    };
}

macro_rules! generate_tuple_impls {
    ($first:ident) => {
        impl_tuple_jsonrpc_client!($first);
    };
    ($first:ident, $($rest:ident),*) => {
        impl_tuple_jsonrpc_client!($first, $($rest),*);
        generate_tuple_impls!($($rest),*);
    };
}

generate_tuple_impls!(
    T15, T14, T13, T12, T11, T10, T9, T8, T7, T6, T5, T4, T3, T2, T1, T0
);

#[async_trait::async_trait]
impl<T> JsonRpcClient for Vec<T>
where
    T: Clone + Serialize + Send + Sync,
    serde_json::Value: From<T>,
{
    async fn send_v1_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v1(id, method);
        for item in self {
            body.add_param(item.clone());
        }
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }

    async fn send_v2_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v2(id, method);
        for item in self {
            body.add_param(item.clone());
        }
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }
}

#[async_trait::async_trait]
impl<T> JsonRpcClient for &[T]
where
    T: Clone + Serialize + Send + Sync,
    serde_json::Value: From<T>,
{
    async fn send_v1_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v1(id, method);
        for item in *self {
            body.add_param(item.clone());
        }
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }

    async fn send_v2_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v2(id, method);
        for item in *self {
            body.add_param(item.clone());
        }
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }
}

#[async_trait::async_trait]
impl JsonRpcClient for () {
    async fn send_v1_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v1(id, method);
        body.set_params(vec![]);
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }

    async fn send_v2_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v2(id, method);
        body.set_params(vec![]);
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }

    async fn send_v1_request_obj<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<serde_json::Value> = JsonRpcRequest::new_v1(id, method);
        body.set_params(serde_json::json!({}));
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }

    async fn send_v2_request_obj<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<serde_json::Value> = JsonRpcRequest::new_v2(id, method);
        body.set_params(serde_json::json!({}));
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }
}

#[async_trait::async_trait]
impl<T> JsonRpcClient for Option<T>
where
    T: JsonRpcClient + Serialize + Send + Sync,
    serde_json::Value: From<T>,
{
    async fn send_v1_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v1(id, method);
        if let Some(inner) = self {
            return inner.send_v1_request(url, content_type, method).await;
        } else {
            body.set_params(vec![]);
        }
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }

    async fn send_v2_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v2(id, method);
        if let Some(inner) = self {
            return inner.send_v2_request(url, content_type, method).await;
        } else {
            body.set_params(vec![]);
        }
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }
}

#[async_trait::async_trait]
impl JsonRpcClient for String {
    async fn send_v1_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v1(id, method);
        body.add_param(self.as_str());
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }

    async fn send_v2_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v2(id, method);
        body.add_param(self.as_str());
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }
}

#[async_trait::async_trait]
impl JsonRpcClient for &str {
    async fn send_v1_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v1(id, method);
        body.add_param(*self);
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }

    async fn send_v2_request<R>(
        &self,
        url: &str,
        content_type: &str,
        method: &str,
    ) -> Result<JsonRpcResponse<R>, RpcError>
    where
        R: serde::de::DeserializeOwned,
    {
        let id = JsonRpcId::next_number();
        let mut body: JsonRpcRequest<Vec<serde_json::Value>> = JsonRpcRequest::new_v2(id, method);
        body.add_param(*self);
        tracing::debug!("jsonrpc request body: {:?}", serde_json::to_string(&body));
        let resp = reqwest::Client::new()
            .post(url)
            .header("Content-Type", content_type)
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        tracing::debug!("jsonrpc response body: {}", text);
        Ok(serde_json::from_str::<JsonRpcResponse<R>>(&text)?)
    }
}

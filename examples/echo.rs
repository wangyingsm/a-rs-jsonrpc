use a_rs_jsonrpc::{RpcError, response::JsonRpcError};
use tracing::Level;

#[a_rs_jsonrpc::jsonrpc_service_fn_array(method = "echoArray", version = "v2")]
async fn echo_array(msg: String) -> Result<String, RpcError> {
    tracing::debug!("got client request message: {}", msg);
    Ok(msg)
}

#[a_rs_jsonrpc::jsonrpc_service_fn_obj(method = "echoObj", version = "v2")]
async fn echo_obj(msg: String) -> Result<serde_json::Value, RpcError> {
    tracing::debug!("got client request message: {}", msg);
    Ok(serde_json::json!({"msg": msg}))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .try_init()
        .ok();

    a_rs_jsonrpc::init_rpc_service();

    let app = axum::Router::new().route(
        "/",
        axum::routing::post(|body: axum::body::Bytes| async move {
            match a_rs_jsonrpc::dispatch_rpc_request(&body).await {
                Ok(resp_body) => {
                    let val: serde_json::Value =
                        serde_json::from_slice(&resp_body).unwrap_or_default();
                    axum::response::Json(val)
                }
                Err(err) => response_error(&body, err),
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("RPC Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

fn response_error(
    req_body: &axum::body::Bytes,
    err: RpcError,
) -> axum::response::Json<serde_json::Value> {
    let Ok((id, version)) = serde_json::from_slice::<serde_json::Value>(req_body).map(|v| {
        (
            v.get("id").cloned().unwrap_or(serde_json::Value::Null),
            v.get("jsonrpc").cloned().unwrap_or(serde_json::Value::Null),
        )
    }) else {
        return axum::response::Json(serde_json::json!({
            "jsonrpc": null,
            "error": { "code": -32603, "message": err.to_string() },
            "id": null
        }));
    };
    axum::response::Json(serde_json::json!({
        "jsonrpc": version,
        "error": JsonRpcError::from(err),
        "id": id
    }))
}

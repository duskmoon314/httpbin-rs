use std::collections::HashMap;

use axum::{headers::UserAgent, http::HeaderMap, routing::get, Json, Router, TypedHeader};
use axum_client_ip::InsecureClientIp;
use serde::Serialize;

#[derive(Serialize)]
struct HeadersRes {
    /// The incoming request's HTTP headers
    headers: HashMap<String, String>,
}

#[derive(Serialize)]
struct IpRes {
    /// The incoming request's IP address
    origin: String,
}

#[derive(Serialize)]
struct UserAgentRes {
    /// The incoming request's User-Agent header
    user_agent: String,
}

pub fn api() -> Router {
    Router::new()
        .route("/headers", get(headers))
        .route("/ip", get(ip))
        .route("/user-agent", get(user_agent))
}

async fn headers(headers: HeaderMap) -> Json<HeadersRes> {
    let headers = headers
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.to_str()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|err| err.to_string()),
            )
        })
        .collect();
    Json(HeadersRes { headers })
}

async fn ip(origin: InsecureClientIp) -> Json<IpRes> {
    Json(IpRes {
        origin: origin.0.to_string(),
    })
}

async fn user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> Json<UserAgentRes> {
    Json(UserAgentRes {
        user_agent: user_agent.to_string(),
    })
}

use std::collections::HashMap;

use axum::{
    headers::UserAgent as UserAgentHeader, http::HeaderMap, routing::get, Json, Router, TypedHeader,
};
use axum_client_ip::InsecureClientIp;
use serde::Serialize;

#[derive(Serialize)]
struct Headers {
    /// The incoming request's HTTP headers
    headers: HashMap<String, String>,
}

#[derive(Serialize)]
struct Ip {
    /// The incoming request's IP address
    origin: String,
}

#[derive(Serialize)]
struct UserAgent {
    /// The incoming request's User-Agent header
    user_agent: String,
}

pub fn api() -> Router {
    Router::new()
        .route("/headers", get(headers))
        .route("/ip", get(ip))
        .route("/user-agent", get(user_agent))
}

async fn headers(headers: HeaderMap) -> Json<Headers> {
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
    Json(Headers { headers })
}

async fn ip(origin: InsecureClientIp) -> Json<Ip> {
    Json(Ip {
        origin: origin.0.to_string(),
    })
}

async fn user_agent(TypedHeader(user_agent): TypedHeader<UserAgentHeader>) -> Json<UserAgent> {
    Json(UserAgent {
        user_agent: user_agent.to_string(),
    })
}

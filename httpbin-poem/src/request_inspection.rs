use std::collections::HashMap;

use anyhow::anyhow;
use poem::{
    get, handler,
    http::HeaderMap,
    web::{Json, RealIp, TypedHeader},
    Result, Route,
};
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

pub fn api(route: Route) -> Route {
    route
        .at("/headers", get(headers))
        .at("/ip", get(ip))
        .at("/user-agent", get(user_agent))
}

#[handler]
fn headers(headers: &HeaderMap) -> Json<Headers> {
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

#[handler]
fn ip(RealIp(origin): RealIp) -> Result<Json<Ip>> {
    origin
        .map(|origin| {
            Json(Ip {
                origin: origin.to_string(),
            })
        })
        .ok_or_else(|| {
            anyhow!("Could not determine the IP address through headers and socket address").into()
        })
}

#[handler]
fn user_agent(
    TypedHeader(user_agent): TypedHeader<poem::web::headers::UserAgent>,
) -> Json<UserAgent> {
    Json(UserAgent {
        user_agent: user_agent.to_string(),
    })
}

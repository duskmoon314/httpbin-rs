use std::collections::HashMap;

use poem::{http::HeaderMap, web::RealIp};
use poem_openapi::{payload::Json, Object, OpenApi};

use super::ApiTags;

#[derive(Debug, Clone, Object)]
struct IpRes {
    origin: Option<String>,
}

#[derive(Debug, Clone, Object)]
struct HeadersRes {
    headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Object)]
struct UserAgentRes {
    user_agent: Option<String>,
}

pub struct Api;

#[OpenApi]
impl Api {
    /// Return the incoming request's HTTP headers.
    #[oai(path = "/headers", method = "get", tag = "ApiTags::RequestInspection")]
    async fn headers(&self, headers: &HeaderMap) -> Json<HeadersRes> {
        let headers: HashMap<String, String> = headers
            .iter()
            .map(|(header_name, header_value)| {
                (
                    header_name.to_string(),
                    header_value.to_str().unwrap().to_string(),
                )
            })
            .collect();
        Json(HeadersRes { headers })
    }

    /// Return the requester's IP Address.
    #[oai(path = "/ip", method = "get", tag = "ApiTags::RequestInspection")]
    async fn ip(&self, ip: RealIp) -> Json<IpRes> {
        Json(IpRes {
            origin: ip.0.map(|ip| ip.to_string()),
        })
    }

    /// Return the incoming requester's User-Agent header.
    #[oai(
        path = "/user-agent",
        method = "get",
        tag = "ApiTags::RequestInspection"
    )]
    async fn user_agent(&self, headers: &HeaderMap) -> Json<UserAgentRes> {
        let user_agent = headers
            .get("user-agent")
            .map(|user_agent| user_agent.to_str().unwrap().to_string());
        Json(UserAgentRes { user_agent })
    }
}

use std::collections::HashMap;

use poem::{
    http::HeaderMap,
    web::{
        headers::{self, HeaderMapExt},
        RealIp,
    },
};
use poem_openapi::{
    payload::{Json, PlainText},
    types::Example,
    ApiResponse, Object, OpenApi, Tags,
};

#[derive(Tags)]
enum ReqInspTag {
    /// Inspect the request data
    #[oai(rename = "Request Inspection")]
    RequestInspection,
}

#[derive(Debug, Clone, Object)]
#[oai(example)]
struct Headers {
    /// The incoming request's HTTP headers
    headers: HashMap<String, String>,
}

impl Example for Headers {
    fn example() -> Self {
        let mut headers = HashMap::new();
        headers.insert("accept".to_string(), "*/*".to_string());
        headers.insert("host".to_string(), "httpbin.rs".to_string());
        headers.insert("user-agent".to_string(), "curl/7.86.0".to_string());
        Self { headers }
    }
}

#[derive(Debug, Clone, Object)]
#[oai(example)]
struct Ip {
    /// The incoming request's IP address
    origin: String,
}

impl Example for Ip {
    fn example() -> Self {
        Self {
            origin: "1.2.3.4".to_string(),
        }
    }
}

#[derive(ApiResponse)]
enum IpRes {
    /// The incoming request's IP address
    #[oai(status = 200)]
    Ok(Json<Ip>),

    /// Could not determine the IP address through headers and socket address
    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[derive(Debug, Clone, Object)]
#[oai(example)]
struct UserAgent {
    /// The incoming request's User-Agent header
    user_agent: String,
}

impl Example for UserAgent {
    fn example() -> Self {
        Self {
            user_agent: "curl/7.86.0".to_string(),
        }
    }
}

#[derive(ApiResponse)]
enum UserAgentRes {
    /// The incoming request's User-Agent header
    #[oai(status = 200)]
    Ok(Json<UserAgent>),

    /// The incoming request does not have a User-Agent header
    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub struct Api;

#[OpenApi(tag = "ReqInspTag::RequestInspection")]
impl Api {
    /// Return the incoming request's HTTP headers.
    #[oai(path = "/headers", method = "get")]
    async fn headers(&self, headers: &HeaderMap) -> Json<Headers> {
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

    /// Return the incoming request's IP address.
    #[oai(path = "/ip", method = "get")]
    async fn ip(&self, origin: RealIp) -> IpRes {
        match origin.0 {
            Some(origin) => IpRes::Ok(Json(Ip {
                origin: origin.to_string(),
            })),
            None => IpRes::NotFound(PlainText(
                "Could not determine the IP address through headers and socket address".to_string(),
            )),
        }
    }

    /// Return the incoming request's User-Agent header.
    #[oai(path = "/user-agent", method = "get")]
    async fn user_agent(&self, headers: &HeaderMap) -> UserAgentRes {
        match headers.typed_get::<headers::UserAgent>() {
            Some(ua) => UserAgentRes::Ok(Json(UserAgent {
                user_agent: ua.to_string(),
            })),
            None => UserAgentRes::NotFound(PlainText(
                "The incoming request does not have a User-Agent header".to_string(),
            )),
        }
    }
}

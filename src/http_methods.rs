//! HTTP Methods

use std::collections::HashMap;

use poem::{web::RealIp, FromRequest, Request, RequestBody, Result};
use poem_openapi::{payload::Json, Object, OpenApi};

use super::ApiTags;

#[derive(Debug, Clone, Object)]
struct HttpRes {
    method: String,
    headers: HashMap<String, String>,
    uri: String,
    origin: Option<String>,
    query: HashMap<String, String>,
    body: Option<String>,
    json: Option<serde_json::Value>,
    form: Option<HashMap<String, String>>,
}

#[poem::async_trait]
impl<'a> FromRequest<'a> for HttpRes {
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self> {
        let method = req.method().to_string();

        let headers = req
            .headers()
            .iter()
            .map(|(header_name, header_value)| {
                (
                    header_name.to_string(),
                    header_value.to_str().unwrap().to_string(),
                )
            })
            .collect();

        let uri = req.uri().to_string();

        let origin = RealIp::from_request_without_body(req).await;
        let origin = origin
            .ok()
            .and_then(|real_ip| real_ip.0.map(|ip| ip.to_string()));

        let mut query = HashMap::new();
        let query_str = req.uri().query().unwrap_or("");
        query_str.split('&').for_each(|key_value| {
            let pair = key_value.split('=').collect::<Vec<_>>();
            if pair.len() > 1 {
                query.insert(pair[0].to_string(), pair[1].to_string());
            }
        });

        let body = body.take().unwrap().into_vec().await.ok();
        let body = body.map(|body| match String::from_utf8(body.clone()) {
            Ok(body) => body,
            Err(_) => "data:application/octet-stream;base64,".to_string() + &base64::encode(body),
        });

        let json = if req.content_type() == Some("application/json") {
            body.as_ref()
                .and_then(|body| serde_json::from_str(body).ok())
        } else {
            None
        };

        let form = if req.content_type() == Some("application/x-www-form-urlencoded") {
            body.as_ref()
                .and_then(|body| serde_urlencoded::from_str(body).ok())
        } else {
            None
        };

        Ok(Self {
            method,
            headers,
            uri,
            origin,
            query,
            body,
            json,
            form,
        })
    }
}

pub struct Api;

#[OpenApi]
impl Api {
    /// The request's GET parameters
    #[oai(path = "/get", method = "get", tag = "ApiTags::HttpMethods")]
    async fn get(&self, req: HttpRes) -> Json<HttpRes> {
        Json(req)
    }

    /// The request's POST parameters
    #[oai(path = "/post", method = "post", tag = "ApiTags::HttpMethods")]
    async fn post(&self, req: HttpRes) -> Json<HttpRes> {
        Json(req)
    }

    /// The request's PUT parameters
    #[oai(path = "/put", method = "put", tag = "ApiTags::HttpMethods")]
    async fn put(&self, req: HttpRes) -> Json<HttpRes> {
        Json(req)
    }

    /// The request's DELETE parameters
    #[oai(path = "/delete", method = "delete", tag = "ApiTags::HttpMethods")]
    async fn delete(&self, req: HttpRes) -> Json<HttpRes> {
        Json(req)
    }

    /// The request's PATCH parameters
    #[oai(path = "/patch", method = "patch", tag = "ApiTags::HttpMethods")]
    async fn patch(&self, req: HttpRes) -> Json<HttpRes> {
        Json(req)
    }

    /// Returns anything passed in request data.
    #[oai(
        path = "/anything/:anything",
        method = "get",
        method = "post",
        method = "put",
        method = "delete",
        method = "patch",
        tag = "ApiTags::Anything"
    )]
    async fn anything(&self, req: HttpRes) -> Json<HttpRes> {
        Json(req)
    }
}

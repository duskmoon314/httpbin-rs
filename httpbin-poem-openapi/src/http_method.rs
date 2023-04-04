use std::collections::HashMap;

use poem::{
    http::{HeaderMap, Method, Uri},
    web::{
        headers::{ContentType, HeaderMapExt},
        RealIp,
    },
};
use poem_openapi::{payload::Json, Object, OpenApi, Tags};

#[derive(Tags)]
enum HttpMethodTag {
    /// Testing different HTTP verbs
    #[oai(rename = "HTTP Methods")]
    HttpMethod,

    /// Returns anything that is passed to request
    Anything,
}

#[derive(Object)]
struct Http {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    origin: Option<String>,
    query: Option<HashMap<String, String>>,
    body_string: String,
    json: Option<serde_json::Value>,
}

pub struct Api;

#[OpenApi]
impl Api {
    /// The request's GET parameters
    #[oai(path = "/get", method = "get", tag = "HttpMethodTag::HttpMethod")]
    async fn get(
        &self,
        method: Method,
        uri: &Uri,
        header_map: &HeaderMap,
        origin: RealIp,
        body: Vec<u8>,
    ) -> Json<Http> {
        self.anything(method, uri, header_map, origin, body).await
    }

    /// The request's POST parameters
    #[oai(path = "/post", method = "post", tag = "HttpMethodTag::HttpMethod")]
    async fn post(
        &self,
        method: Method,
        uri: &Uri,
        header_map: &HeaderMap,
        origin: RealIp,
        body: Vec<u8>,
    ) -> Json<Http> {
        self.anything(method, uri, header_map, origin, body).await
    }

    /// The request's PUT parameters
    #[oai(path = "/put", method = "put", tag = "HttpMethodTag::HttpMethod")]
    async fn put(
        &self,
        method: Method,
        uri: &Uri,
        header_map: &HeaderMap,
        origin: RealIp,
        body: Vec<u8>,
    ) -> Json<Http> {
        self.anything(method, uri, header_map, origin, body).await
    }

    /// The request's DELETE parameters
    #[oai(path = "/delete", method = "delete", tag = "HttpMethodTag::HttpMethod")]
    async fn delete(
        &self,
        method: Method,
        uri: &Uri,
        header_map: &HeaderMap,
        origin: RealIp,
        body: Vec<u8>,
    ) -> Json<Http> {
        self.anything(method, uri, header_map, origin, body).await
    }

    /// The request's PATCH parameters
    #[oai(path = "/patch", method = "patch", tag = "HttpMethodTag::HttpMethod")]
    async fn patch(
        &self,
        method: Method,
        uri: &Uri,
        header_map: &HeaderMap,
        origin: RealIp,
        body: Vec<u8>,
    ) -> Json<Http> {
        self.anything(method, uri, header_map, origin, body).await
    }

    /// Returns anything passed in request data.
    #[oai(
        path = "/anything",
        method = "get",
        method = "post",
        method = "put",
        method = "delete",
        method = "patch",
        tag = "HttpMethodTag::Anything"
    )]
    async fn anything_no_path(
        &self,
        method: Method,
        uri: &Uri,
        header_map: &HeaderMap,
        origin: RealIp,
        body: Vec<u8>,
    ) -> Json<Http> {
        self.anything(method, uri, header_map, origin, body).await
    }

    /// Returns anything passed in request data.
    #[oai(
        path = "/anything/*anything",
        method = "get",
        method = "post",
        method = "put",
        method = "delete",
        method = "patch",
        tag = "HttpMethodTag::Anything"
    )]
    async fn anything(
        &self,
        method: Method,
        uri: &Uri,
        header_map: &HeaderMap,
        origin: RealIp,
        body: Vec<u8>,
    ) -> Json<Http> {
        let headers = header_map
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

        let query = uri.query().map(|query_str| {
            serde_qs::from_str(query_str)
                .unwrap_or_else(|err| [("error".to_string(), err.to_string())].into())
        });

        let body_string = match String::from_utf8(body.clone()) {
            Ok(body) => body,
            Err(_) => {
                match httpbin::data::base64::encode(
                    &body,
                    httpbin::data::base64::Base64Engine::Standard,
                    None,
                ) {
                    Ok(body) => body,
                    Err(err) => err.to_string(),
                }
            }
        };

        let json = header_map
            .typed_get::<ContentType>()
            .and_then(|content_type| {
                if content_type == ContentType::json() {
                    Some(serde_json::from_slice(&body).unwrap_or_else(|err| {
                        serde_json::json!({
                            "error": err.to_string(),
                        })
                    }))
                } else {
                    None
                }
            });

        Json(Http {
            method: method.to_string(),
            uri: uri.to_string(),
            headers,
            origin: origin.0.map(|origin| origin.to_string()),
            query,
            body_string,
            json,
        })
    }
}

use std::collections::HashMap;

use axum::{
    body::Bytes,
    extract::RawQuery,
    headers::ContentType,
    http::{HeaderMap, Method, Uri},
    routing::{delete, get, patch, post, put},
    Json, Router, TypedHeader,
};
use axum_client_ip::InsecureClientIp;
use serde::Serialize;

#[derive(Serialize)]
struct Http {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    origin: String,
    query: Option<HashMap<String, String>>,
    body_string: String,
    json: Option<serde_json::Value>,
}

pub fn api() -> Router {
    Router::new()
        .route("/get", get(anything))
        .route("/post", post(anything))
        .route("/put", put(anything))
        .route("/delete", delete(anything))
        .route("/patch", patch(anything))
        .route(
            "/anything",
            get(anything)
                .post(anything)
                .put(anything)
                .delete(anything)
                .patch(anything),
        )
        .route(
            "/anything/*anything",
            get(anything)
                .post(anything)
                .put(anything)
                .delete(anything)
                .patch(anything),
        )
}

async fn anything(
    method: Method,
    uri: Uri,
    query: RawQuery,
    header_map: HeaderMap,
    content_type: Option<TypedHeader<ContentType>>,
    origin: InsecureClientIp,
    body: Bytes,
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

    let query = query.0.map(|query_str| {
        serde_qs::from_str(&query_str)
            .unwrap_or_else(|err| [("error".to_string(), err.to_string())].into())
    });

    let body_string = match String::from_utf8(body.to_vec()) {
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

    let json = content_type.and_then(|TypedHeader(content_type)| {
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
        origin: origin.0.to_string(),
        query,
        body_string,
        json,
    })
}

use std::collections::HashMap;

use poem::{
    delete, get, handler,
    http::{HeaderMap, Method, Uri},
    patch, post, put,
    web::{
        headers::{ContentType, HeaderMapExt},
        Json, RealIp,
    },
    Route,
};
use serde::Serialize;

#[derive(Serialize)]
struct Http {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    origin: Option<String>,
    query: Option<HashMap<String, String>>,
    body_string: String,
    json: Option<serde_json::Value>,
}

pub fn api(route: Route) -> Route {
    route
        .at("/get", get(anything))
        .at("/post", post(anything))
        .at("/put", put(anything))
        .at("/delete", delete(anything))
        .at("/patch", patch(anything))
        .at(
            "/anything",
            get(anything)
                .post(anything)
                .put(anything)
                .delete(anything)
                .patch(anything),
        )
        .at(
            "/anything/*anything",
            get(anything)
                .post(anything)
                .put(anything)
                .delete(anything)
                .patch(anything),
        )
}

#[handler]
fn anything(
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

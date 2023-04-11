use std::collections::HashMap;

use salvo::{http::mime::APPLICATION_JSON, prelude::*};
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

pub fn api() -> Router {
    Router::new()
        .push(Router::with_path("/get").get(anything))
        .push(Router::with_path("/post").post(anything))
        .push(Router::with_path("/put").put(anything))
        .push(Router::with_path("/delete").delete(anything))
        .push(Router::with_path("/patch").patch(anything))
        .push(
            Router::with_path("/anything")
                .get(anything)
                .post(anything)
                .put(anything)
                .delete(anything)
                .patch(anything),
        )
        .push(
            Router::with_path("/anything/<*anything>")
                .get(anything)
                .post(anything)
                .put(anything)
                .delete(anything)
                .patch(anything),
        )
}

#[handler]
async fn anything(req: &mut Request) -> Json<Http> {
    let headers = req
        .headers()
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

    let query = if req.queries().is_empty() {
        None
    } else {
        Some(
            req.queries()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        )
    };

    let body = req.payload().await;

    let (body_string, json) = match body {
        Ok(body) => {
            let body = body.clone();
            let body_string = match String::from_utf8(body.clone()) {
                Ok(body) => body,
                Err(_) => match httpbin::data::base64::encode(
                    &body,
                    httpbin::data::base64::Base64Engine::Standard,
                    None,
                ) {
                    Ok(body) => body,
                    Err(err) => err.to_string(),
                },
            };
            let json = if req.content_type() == Some(APPLICATION_JSON) {
                Some(serde_json::from_slice(&body).unwrap_or_else(|err| {
                    serde_json::json!({
                        "error": err.to_string(),
                    })
                }))
            } else {
                None
            };

            (body_string, json)
        }
        Err(err) => (err.to_string(), None),
    };

    Json(Http {
        method: req.method().to_string(),
        uri: req.uri().to_string(),
        headers,
        origin: req.remote_addr().map(|addr| addr.to_string()),
        query,
        body_string,
        json,
    })
}

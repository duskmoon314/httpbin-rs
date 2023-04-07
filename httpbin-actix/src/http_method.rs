use std::collections::HashMap;

use actix_web::{
    http::header::CONTENT_TYPE,
    web::{delete, get, patch, post, put, resource, scope, Bytes, Json, ServiceConfig},
    HttpRequest,
};
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

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(resource("/get").route(get().to(anything)))
        .service(resource("/post").route(post().to(anything)))
        .service(resource("/put").route(put().to(anything)))
        .service(resource("/delete").route(delete().to(anything)))
        .service(resource("/patch").route(patch().to(anything)))
        .service(
            scope("/anything")
                .service(
                    resource("")
                        .route(get().to(anything))
                        .route(post().to(anything))
                        .route(put().to(anything))
                        .route(delete().to(anything))
                        .route(patch().to(anything)),
                )
                .service(
                    resource("/{anything}*")
                        .route(get().to(anything))
                        .route(post().to(anything))
                        .route(put().to(anything))
                        .route(delete().to(anything))
                        .route(patch().to(anything)),
                ),
        );
}

async fn anything(req: HttpRequest, data: Bytes) -> Json<Http> {
    let method = req.method().to_string();

    let uri = req.uri().to_string();

    let headers = req.headers();
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

    let origin = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("")
        .to_string();

    let query = req.query_string();
    let query = if query.is_empty() {
        None
    } else {
        Some(
            serde_qs::from_str(query)
                .unwrap_or_else(|err| [("error".to_string(), err.to_string())].into()),
        )
    };

    let body_string = match String::from_utf8(data.to_vec()) {
        Ok(body) => body,
        Err(_) => {
            match httpbin::data::base64::encode(
                &data,
                httpbin::data::base64::Base64Engine::Standard,
                None,
            ) {
                Ok(body) => body,
                Err(err) => err.to_string(),
            }
        }
    };

    let json = req.headers().get(CONTENT_TYPE).and_then(|content_type| {
        if content_type == "application/json" {
            Some(serde_json::from_slice(&data).unwrap_or_else(|err| {
                serde_json::json!({
                    "error": err.to_string(),
                })
            }))
        } else {
            None
        }
    });

    Json(Http {
        method,
        uri,
        headers,
        origin,
        query,
        body_string,
        json,
    })
}

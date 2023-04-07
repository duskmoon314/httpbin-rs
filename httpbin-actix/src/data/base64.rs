use std::fmt::Display;

use actix_web::{
    web::{post, resource, scope, Bytes, Path, Query, ServiceConfig},
    HttpResponse, ResponseError, Result,
};
use httpbin::data::base64::Base64Engine;
use serde::Deserialize;

#[derive(Deserialize)]
struct Base64Config {
    pub alphabet: Option<String>,
    pub pad: Option<bool>,
}

#[derive(Debug)]
struct Base64Error(pub httpbin::data::base64::Base64Error);

impl Display for Base64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ResponseError for Base64Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().body(self.0.to_string())
    }
}

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/base64")
            .service(resource("/encode/{engine}").route(post().to(base64_encode)))
            .service(resource("/decode/{engine}").route(post().to(base64_decode))),
    );
}

async fn base64_encode(
    data: Bytes,
    engine: Path<Base64Engine>,
    Query(config): Query<Base64Config>,
) -> Result<HttpResponse> {
    let engine = engine.into_inner();

    let config = match engine {
        Base64Engine::Custom => Some(httpbin::data::base64::Base64Config {
            alphabet: config.alphabet.unwrap_or_default(),
            pad: config.pad.unwrap_or_default(),
        }),
        _ => None,
    };

    let encoded = httpbin::data::base64::encode(&data, engine, config).map_err(Base64Error)?;

    Ok(HttpResponse::Ok().body(encoded))
}

async fn base64_decode(
    data: String,
    engine: Path<Base64Engine>,
    Query(config): Query<Base64Config>,
) -> Result<HttpResponse> {
    let engine = engine.into_inner();

    let config = match engine {
        Base64Engine::Custom => Some(httpbin::data::base64::Base64Config {
            alphabet: config.alphabet.unwrap_or_default(),
            pad: config.pad.unwrap_or_default(),
        }),
        _ => None,
    };

    let decoded = httpbin::data::base64::decode(&data, engine, config).map_err(Base64Error)?;

    match String::from_utf8(decoded.clone()) {
        Ok(text) => Ok(HttpResponse::Ok().body(text)),
        Err(_) => {
            let kind = infer::get(&decoded);

            Ok(HttpResponse::Ok()
                .content_type(
                    kind.map(|k| k.mime_type())
                        .unwrap_or("application/octet-stream"),
                )
                .body(decoded))
        }
    }
}

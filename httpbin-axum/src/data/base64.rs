use axum::{
    body,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use httpbin::data::base64::Base64Engine;
use serde::Deserialize;

#[derive(Deserialize)]
struct Base64Config {
    pub alphabet: Option<String>,
    pub pad: Option<bool>,
}

enum Base64Res {
    OkText(String),
    OkBinary { data: Vec<u8>, content_type: String },
}

impl IntoResponse for Base64Res {
    fn into_response(self) -> Response {
        match self {
            Base64Res::OkText(text) => (StatusCode::OK, text).into_response(),
            Base64Res::OkBinary { data, content_type } => {
                (StatusCode::OK, [("Content-Type", content_type)], data).into_response()
            }
        }
    }
}

pub fn api() -> Router {
    Router::new()
        .route("/encode/:engine", post(base64_encode))
        .route("/decode/:engine", post(base64_decode))
}

async fn base64_encode(
    Path(engine): Path<Base64Engine>,
    Query(config): Query<Base64Config>,
    data: body::Bytes,
) -> Result<Base64Res, String> {
    let config = match engine {
        Base64Engine::Custom => Some(httpbin::data::base64::Base64Config {
            alphabet: config.alphabet.unwrap_or_default(),
            pad: config.pad.unwrap_or_default(),
        }),
        _ => None,
    };

    let encoded =
        httpbin::data::base64::encode(&data, engine, config).map_err(|e| e.to_string())?;

    Ok(Base64Res::OkText(encoded))
}

async fn base64_decode(
    Path(engine): Path<Base64Engine>,
    Query(config): Query<Base64Config>,
    data: String,
) -> Result<Base64Res, String> {
    let config = match engine {
        Base64Engine::Custom => Some(httpbin::data::base64::Base64Config {
            alphabet: config.alphabet.unwrap_or_default(),
            pad: config.pad.unwrap_or_default(),
        }),
        _ => None,
    };

    let decoded =
        httpbin::data::base64::decode(&data, engine, config).map_err(|e| e.to_string())?;

    match String::from_utf8(decoded.clone()) {
        Ok(decoded) => Ok(Base64Res::OkText(decoded)),
        Err(_) => {
            let kind = infer::get(&decoded);

            Ok(Base64Res::OkBinary {
                data: decoded,
                content_type: kind
                    .map(|k| k.mime_type())
                    .unwrap_or("application/octet-stream")
                    .to_string(),
            })
        }
    }
}

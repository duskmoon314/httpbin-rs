use anyhow::anyhow;
use httpbin::data::base64::Base64Engine;
use poem::{
    handler, post,
    web::{Path, Query},
    IntoResponse, Result, Route,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Base64Config {
    alphabet: Option<String>,
    pad: Option<bool>,
}

enum Base64Res {
    OkText(String),
    OkBinary(Vec<u8>, String),
}

impl IntoResponse for Base64Res {
    fn into_response(self) -> poem::Response {
        match self {
            Base64Res::OkText(s) => s.into_response(),
            Base64Res::OkBinary(b, content_type) => poem::Response::builder()
                .header("Content-Type", content_type)
                .body(b),
        }
    }
}

pub fn api(route: Route) -> Route {
    route.nest(
        "/base64",
        Route::new()
            .at("/encode/:engine", post(base64_encode))
            .at("/decode/:engine", post(base64_decode)),
    )
}

#[handler]
fn base64_encode(
    data: Vec<u8>,
    Path(engine): Path<Base64Engine>,
    Query(Base64Config { alphabet, pad }): Query<Base64Config>,
) -> Result<Base64Res> {
    let config = match engine {
        Base64Engine::Custom => Some(httpbin::data::base64::Base64Config {
            alphabet: alphabet.unwrap_or_default(),
            pad: pad.unwrap_or_default(),
        }),
        _ => None,
    };

    let encoded = httpbin::data::base64::encode(&data, engine, config).map_err(|e| anyhow!(e))?;

    Ok(Base64Res::OkText(encoded))
}

#[handler]
fn base64_decode(
    data: String,
    Path(engine): Path<Base64Engine>,
    Query(Base64Config { alphabet, pad }): Query<Base64Config>,
) -> Result<Base64Res> {
    let config = match engine {
        Base64Engine::Custom => Some(httpbin::data::base64::Base64Config {
            alphabet: alphabet.unwrap_or_default(),
            pad: pad.unwrap_or_default(),
        }),
        _ => None,
    };

    let decoded = httpbin::data::base64::decode(&data, engine, config).map_err(|e| anyhow!(e))?;

    match String::from_utf8(decoded.clone()) {
        Ok(s) => Ok(Base64Res::OkText(s)),
        Err(_) => {
            let kind = infer::get(&decoded);

            Ok(Base64Res::OkBinary(
                decoded,
                kind.map(|k| k.mime_type())
                    .unwrap_or("application/octet-stream")
                    .to_string(),
            ))
        }
    }
}

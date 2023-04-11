use anyhow::Result;
use httpbin::data::base64::Base64Engine;
use salvo::{
    hyper::Body,
    hyper::{body::to_bytes, header::CONTENT_TYPE},
    prelude::*,
};
use serde::Deserialize;

#[derive(Deserialize, Extractible, Debug)]
struct Base64Req {
    #[extract(source(from = "param"))]
    pub engine: Base64Engine,

    #[extract(source(from = "query"))]
    pub alphabet: Option<String>,

    #[extract(source(from = "query"))]
    pub pad: Option<bool>,
}

enum Base64Res {
    OkText(String),
    OkBinary { data: Vec<u8>, content_type: String },
}

impl Piece for Base64Res {
    fn render(self, res: &mut Response) {
        match self {
            Base64Res::OkText(text) => {
                res.set_status_code(StatusCode::OK);
                res.render(text);
            }
            Base64Res::OkBinary { data, content_type } => {
                res.set_status_code(StatusCode::OK);
                // TODO: how to handle the error?
                let _ = res.add_header(CONTENT_TYPE, content_type, true);
                res.set_body(Body::from(data).into());
            }
        }
    }
}

pub fn api() -> Router {
    Router::new()
        .push(Router::with_path("/encode/<engine>").post(base64_encode))
        .push(Router::with_path("/decode/<engine>").post(base64_decode))
}

#[handler]
async fn base64_encode(req: &mut Request) -> Result<Base64Res> {
    let data = req.payload().await?.clone();
    let req = req.extract::<Base64Req>().await?;

    let config = match req.engine {
        Base64Engine::Custom => Some(httpbin::data::base64::Base64Config {
            alphabet: req.alphabet.unwrap_or_default(),
            pad: req.pad.unwrap_or_default(),
        }),
        _ => None,
    };

    let encoded = httpbin::data::base64::encode(&data, req.engine, config)?;

    Ok(Base64Res::OkText(encoded))
}

#[handler]
async fn base64_decode(req: &mut Request) -> Result<Base64Res> {
    let data = match req.take_body() {
        Some(body) => {
            let bytes = to_bytes(body).await?;
            String::from_utf8(bytes.to_vec())?
        }
        None => "".to_string(),
    };
    let req = req.extract::<Base64Req>().await?;

    let config = match req.engine {
        Base64Engine::Custom => Some(httpbin::data::base64::Base64Config {
            alphabet: req.alphabet.unwrap_or_default(),
            pad: req.pad.unwrap_or_default(),
        }),
        _ => None,
    };

    let decoded = httpbin::data::base64::decode(&data, req.engine, config)?;

    match String::from_utf8(decoded.clone()) {
        Ok(text) => Ok(Base64Res::OkText(text)),
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

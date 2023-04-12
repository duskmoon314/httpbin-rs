use std::str::FromStr;

use rocket::{
    http::{ContentType, Header},
    post,
    request::FromParam,
    routes, Build, FromForm, Responder, Rocket,
};

struct Base64Engine(httpbin::data::base64::Base64Engine);

impl<'r> FromParam<'r> for Base64Engine {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "standard" => Ok(Base64Engine(httpbin::data::base64::Base64Engine::Standard)),
            "standard_no_pad" => Ok(Base64Engine(
                httpbin::data::base64::Base64Engine::StandardNoPad,
            )),
            "url_safe" => Ok(Base64Engine(httpbin::data::base64::Base64Engine::UrlSafe)),
            "url_safe_no_pad" => Ok(Base64Engine(
                httpbin::data::base64::Base64Engine::UrlSafeNoPad,
            )),
            "bcrypt" => Ok(Base64Engine(httpbin::data::base64::Base64Engine::Bcrypt)),
            "bin_hex" => Ok(Base64Engine(httpbin::data::base64::Base64Engine::BinHex)),
            "crypt" => Ok(Base64Engine(httpbin::data::base64::Base64Engine::Crypt)),
            "imap_mutf7" => Ok(Base64Engine(httpbin::data::base64::Base64Engine::ImapMutf7)),
            "custom" => Ok(Base64Engine(httpbin::data::base64::Base64Engine::Custom)),
            _ => Err(param),
        }
    }
}

#[derive(FromForm)]
struct Base64Config {
    pub alphabet: Option<String>,
    pub pad: Option<bool>,
}

struct ContentTypeHeader(Box<ContentType>);

impl<'r> From<ContentTypeHeader> for Header<'r> {
    fn from(content_type: ContentTypeHeader) -> Self {
        Header::new("Content-Type", content_type.0.to_string())
    }
}

#[derive(Responder)]
enum Base64Res {
    OkText(String),
    OkBinary(Vec<u8>, ContentTypeHeader),
}

pub async fn api(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/base64", routes![base64_encode, base64_decode])
}

#[post("/encode/<engine>?<config>", data = "<data>")]
fn base64_encode(
    engine: Base64Engine,
    config: Base64Config,
    data: Vec<u8>,
) -> Result<Base64Res, String> {
    let config = match engine {
        Base64Engine(httpbin::data::base64::Base64Engine::Custom) => {
            Some(httpbin::data::base64::Base64Config {
                alphabet: config.alphabet.unwrap_or_default(),
                pad: config.pad.unwrap_or_default(),
            })
        }
        _ => None,
    };

    let encoded =
        httpbin::data::base64::encode(&data, engine.0, config).map_err(|e| e.to_string())?;

    Ok(Base64Res::OkText(encoded))
}

#[post("/decode/<engine>?<config>", data = "<data>")]
fn base64_decode(
    engine: Base64Engine,
    config: Base64Config,
    data: String,
) -> Result<Base64Res, String> {
    let config = match engine {
        Base64Engine(httpbin::data::base64::Base64Engine::Custom) => {
            Some(httpbin::data::base64::Base64Config {
                alphabet: config.alphabet.unwrap_or_default(),
                pad: config.pad.unwrap_or_default(),
            })
        }
        _ => None,
    };

    let decoded =
        httpbin::data::base64::decode(&data, engine.0, config).map_err(|e| e.to_string())?;

    match String::from_utf8(decoded.clone()) {
        Ok(decoded) => Ok(Base64Res::OkText(decoded)),
        Err(_) => {
            let kind = infer::get(&decoded);

            Ok(Base64Res::OkBinary(
                decoded,
                ContentTypeHeader(Box::new(
                    ContentType::from_str(
                        kind.map(|k| k.mime_type())
                            .unwrap_or("application/octet-stream"),
                    )
                    .unwrap_or(ContentType::Binary),
                )),
            ))
        }
    }
}

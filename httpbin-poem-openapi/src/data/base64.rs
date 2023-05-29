use poem::Result;
use poem_openapi::{
    param::{Path, Query},
    payload::Binary,
    payload::PlainText,
    ApiRequest, ApiResponse, Enum, OpenApi,
};

use super::DataTag;

/// Base64 engines for encoding and decoding
///
/// `bcrypt` `binhex` `crypt` `imap-mutf7` are not using padding
///
/// The `custom` engine allows you to specify your own alphabet and padding
#[derive(Debug, Enum)]
#[oai(rename_all = "snake_case")]
enum Base64Engine {
    Standard,
    StandardNoPad,
    UrlSafe,
    UrlSafeNoPad,
    Bcrypt,
    BinHex,
    Crypt,
    ImapMutf7,
    Custom,
}

impl From<Base64Engine> for httpbin::data::base64::Base64Engine {
    fn from(engine: Base64Engine) -> Self {
        match engine {
            Base64Engine::Standard => httpbin::data::base64::Base64Engine::Standard,
            Base64Engine::StandardNoPad => httpbin::data::base64::Base64Engine::StandardNoPad,
            Base64Engine::UrlSafe => httpbin::data::base64::Base64Engine::UrlSafe,
            Base64Engine::UrlSafeNoPad => httpbin::data::base64::Base64Engine::UrlSafeNoPad,
            Base64Engine::Bcrypt => httpbin::data::base64::Base64Engine::Bcrypt,
            Base64Engine::BinHex => httpbin::data::base64::Base64Engine::BinHex,
            Base64Engine::Crypt => httpbin::data::base64::Base64Engine::Crypt,
            Base64Engine::ImapMutf7 => httpbin::data::base64::Base64Engine::ImapMutf7,
            Base64Engine::Custom => httpbin::data::base64::Base64Engine::Custom,
        }
    }
}

#[derive(ApiRequest, Debug)]
enum Base64Req {
    Binary(Binary<Vec<u8>>),
    Text(PlainText<String>),
}

#[derive(ApiResponse)]
enum Base64Res {
    /// The encoded or decoded data
    #[oai(status = 200)]
    Ok(
        Binary<Vec<u8>>,
        /// Content-Type is `application/octet-stream` by default and is set to
        /// the actual type inferred by [infer](https://crates.io/crates/infer)
        #[oai(header = "Content-Type")]
        String,
    ),

    /// Bad request
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

pub struct Api;

#[OpenApi(tag = "DataTag::Data")]
impl Api {
    /// Encode data to a base64 string
    #[oai(path = "/base64/encode/:engine", method = "post")]
    async fn base64_encode(
        &self,
        /// The data to encode
        /// 
        /// The data can be binary or text
        data: Base64Req,

        /// Base64 engines for encoding and decoding
        ///
        /// `bcrypt` `binhex` `crypt` `imap-mutf7` are not using padding
        ///
        /// The `custom` engine allows you to specify your own alphabet and padding
        engine: Path<Base64Engine>,

        /// The alphabet to use for encoding
        alphabet: Query<Option<String>>,

        /// Whether to use padding
        pad: Query<Option<bool>>,
    ) -> Result<Base64Res> {
        let config = match engine.0 {
            Base64Engine::Custom => Some(httpbin::data::base64::Base64Config {
                alphabet: alphabet.0.unwrap_or_default(),
                pad: pad.0.unwrap_or_default(),
            }),
            _ => None,
        };

        match data {
            Base64Req::Binary(data) => {
                let encoded = httpbin::data::base64::encode(&data.0, engine.0.into(), config)
                    .map_err(|e| Base64Res::BadRequest(PlainText(e.to_string())))?;

                Ok(Base64Res::Ok(
                    Binary(encoded.into_bytes()),
                    "text/plain; charset=utf-8".to_string(),
                ))
            }
            Base64Req::Text(data) => {
                let encoded =
                    httpbin::data::base64::encode(data.0.as_bytes(), engine.0.into(), config)
                        .map_err(|e| Base64Res::BadRequest(PlainText(e.to_string())))?;

                Ok(Base64Res::Ok(
                    Binary(encoded.into_bytes()),
                    "text/plain; charset=utf-8".to_string(),
                ))
            }
        }
    }

    /// Decode data from a base64 string
    #[oai(path = "/base64/decode/:engine", method = "post")]
    async fn base64_decode(
        &self,
        /// The string to decode
        data: PlainText<String>,

        /// Base64 engines for encoding and decoding
        ///
        /// `bcrypt` `binhex` `crypt` `imap-mutf7` are not using padding
        ///
        /// The `custom` engine allows you to specify your own alphabet and padding
        engine: Path<Base64Engine>,

        /// The alphabet to use for decoding
        alphabet: Query<Option<String>>,

        /// Whether to use padding
        pad: Query<Option<bool>>,
    ) -> Result<Base64Res> {
        let config = match engine.0 {
            Base64Engine::Custom => Some(httpbin::data::base64::Base64Config {
                alphabet: alphabet.0.unwrap_or_default(),
                pad: pad.0.unwrap_or_default(),
            }),
            _ => None,
        };

        let decoded = httpbin::data::base64::decode(&data.0, engine.0.into(), config)
            .map_err(|e| Base64Res::BadRequest(PlainText(e.to_string())))?;

        match String::from_utf8(decoded.clone()) {
            Ok(_) => Ok(Base64Res::Ok(
                Binary(decoded),
                "text/plain; charset=utf-8".to_string(),
            )),
            Err(_) => {
                let kind = infer::get(&decoded);

                Ok(Base64Res::Ok(
                    Binary(decoded),
                    kind.map(|k| k.mime_type())
                        .unwrap_or("application/octet-stream")
                        .to_string(),
                ))
            }
        }
    }
}

use std::str::FromStr;

use digest::DynDigest;
use poem_openapi::{
    param::Path,
    payload::{Binary, Json, PlainText, Response},
    types::Example,
    ApiRequest, ApiResponse, Enum, Object, OpenApi,
};
use uuid::Uuid;

use super::ApiTags;

#[derive(ApiRequest)]
enum Base64Req {
    Binary(Binary<Vec<u8>>),
    Text(PlainText<String>),
}

impl AsRef<[u8]> for Base64Req {
    fn as_ref(&self) -> &[u8] {
        match self {
            Base64Req::Binary(binary) => binary.as_ref(),
            Base64Req::Text(text) => text.as_bytes(),
        }
    }
}

#[derive(Debug, Clone, Enum)]
#[oai(rename_all = "snake_case")]
#[non_exhaustive]
enum Base64Config {
    Bcrypt,
    BinHex,
    Crypt,
    ImapMutf7,
    Standard,
    StandardNoPad,
    UrlSafe,
    UrlSafeNoPad,
}

#[allow(clippy::from_over_into)]
impl Into<base64::Config> for Base64Config {
    fn into(self) -> base64::Config {
        match self {
            Self::Bcrypt => base64::BCRYPT,
            Self::BinHex => base64::BINHEX,
            Self::Crypt => base64::CRYPT,
            Self::ImapMutf7 => base64::IMAP_MUTF7,
            Self::Standard => base64::STANDARD,
            Self::StandardNoPad => base64::STANDARD_NO_PAD,
            Self::UrlSafe => base64::URL_SAFE,
            Self::UrlSafeNoPad => base64::URL_SAFE_NO_PAD,
        }
    }
}

#[derive(ApiResponse)]
enum Base64Res {
    #[oai(status = 200)]
    OkText(PlainText<String>),

    #[oai(status = 200)]
    OkBinary(Binary<Vec<u8>>),

    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

#[derive(Debug, Clone, Object)]
struct UuidReq {
    namespace: Uuid,
    name: String,
}

#[derive(Debug, Default, Clone, Object)]
#[oai(default, example)]
struct CronReq {
    cron: String,
    /// number of upcoming schedules
    ///
    /// default: null (will be treated as 10)
    num: Option<usize>,
    /// timezone to use for the upcoming schedules
    ///
    /// default: null (will be treated as "UTC")
    tz: Option<String>,
}

impl Example for CronReq {
    fn example() -> Self {
        CronReq {
            cron: "1 2 3 4 5 6".to_string(),
            num: Some(8),
            tz: Some("Hongkong".to_string()),
        }
    }
}

#[derive(ApiResponse)]
enum CronRes {
    #[oai(status = 200)]
    Ok(Json<Vec<String>>),

    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

#[derive(Debug, Clone, Enum)]
#[oai(rename_all = "snake_case")]
#[non_exhaustive]
enum Hasher {
    Md5,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
}

#[derive(ApiRequest)]
enum HashReq {
    Binary(Binary<Vec<u8>>),
    Text(PlainText<String>),
}

impl AsRef<[u8]> for HashReq {
    fn as_ref(&self) -> &[u8] {
        match self {
            HashReq::Binary(binary) => binary.as_ref(),
            HashReq::Text(text) => text.as_bytes(),
        }
    }
}

#[derive(Debug, Clone, Object)]
struct HashRes {
    slice: Vec<u8>,
    slice_hex: String,
    string_hex: String,
    string_base64: String,
}

pub struct Api;

#[OpenApi]
impl Api {
    /// Decode base64 encoded string.
    #[oai(path = "/base64/decode", method = "post", tag = "ApiTags::Data")]
    async fn base64_decode(&self, base64: PlainText<String>) -> Response<Base64Res> {
        self.base64_decode_config(base64, Path(Base64Config::Standard))
            .await
    }

    /// Decode base64 encoded string with config.
    #[oai(
        path = "/base64/decode/:config",
        method = "post",
        tag = "ApiTags::Data"
    )]
    async fn base64_decode_config(
        &self,
        base64: PlainText<String>,
        config: Path<Base64Config>,
    ) -> Response<Base64Res> {
        match base64::decode_config::<&[u8]>(base64.as_ref(), config.0.into()) {
            Ok(decoded) => match String::from_utf8(decoded.clone()) {
                Ok(decoded) => Response::new(Base64Res::OkText(PlainText(decoded))),
                Err(_err) => {
                    let kind = infer::get(&decoded);
                    Response::new(Base64Res::OkBinary(Binary(decoded))).header(
                        "content-type",
                        kind.map(|k| k.mime_type())
                            .unwrap_or("application/octet-stream"),
                    )
                }
            },
            Err(err) => Response::new(Base64Res::BadRequest(PlainText(format!("{}", err)))),
        }
    }

    /// Encode data as base64.
    #[oai(path = "/base64/encode/", method = "post", tag = "ApiTags::Data")]
    async fn base64_encode(&self, data: Base64Req) -> Base64Res {
        self.base64_encode_config(data, Path(Base64Config::Standard))
            .await
    }

    /// Encode data as base64 with config.
    #[oai(
        path = "/base64/encode/:config",
        method = "post",
        tag = "ApiTags::Data"
    )]
    async fn base64_encode_config(&self, data: Base64Req, config: Path<Base64Config>) -> Base64Res {
        Base64Res::OkText(PlainText(base64::encode_config(data, config.0.into())))
    }

    /// Generate UUID v3.
    #[oai(path = "/uuid/v3", method = "post", tag = "ApiTags::Data")]
    async fn uuid_v3(&self, req: Json<UuidReq>) -> PlainText<String> {
        PlainText(Uuid::new_v3(&req.namespace, req.name.as_bytes()).to_string())
    }

    /// Generate UUID v4.
    #[oai(
        path = "/uuid/v4",
        method = "get",
        method = "post",
        tag = "ApiTags::Data"
    )]
    async fn uuid_v4(&self) -> PlainText<String> {
        PlainText(Uuid::new_v4().to_string())
    }

    /// Generate UUID v5.
    #[oai(path = "/uuid/v5", method = "post", tag = "ApiTags::Data")]
    async fn uuid_v5(&self, req: Json<UuidReq>) -> PlainText<String> {
        PlainText(Uuid::new_v5(&req.namespace, req.name.as_bytes()).to_string())
    }

    /// Get upcoming cron schedule.
    #[oai(path = "/cron", method = "post", tag = "ApiTags::Data")]
    async fn cron(&self, req: Json<CronReq>) -> CronRes {
        let schedule = match cron::Schedule::from_str(&req.cron) {
            Ok(schedule) => schedule,
            Err(err) => return CronRes::BadRequest(PlainText(format!("{}", err))),
        };
        let num = req.num.unwrap_or(10);
        let tz: chrono_tz::Tz = match &req
            .tz
            .clone()
            .unwrap_or_else(|| "UTC".to_string())
            .parse::<chrono_tz::Tz>()
        {
            Ok(tz) => *tz,
            Err(err) => return CronRes::BadRequest(PlainText(err.to_string())),
        };
        CronRes::Ok(Json(
            schedule
                .upcoming(tz)
                .take(num)
                .map(|t| format!("{}", t))
                .collect(),
        ))
    }

    /// Get hashed string.
    #[allow(clippy::box_default)]
    #[oai(path = "/hash/:hasher", method = "post", tag = "ApiTags::Data")]
    async fn hash(&self, hasher: Path<Hasher>, data: HashReq) -> Json<HashRes> {
        let mut hasher: Box<dyn DynDigest> = match hasher.0 {
            Hasher::Md5 => Box::new(md5::Md5::default()),
            Hasher::Sha224 => Box::new(sha2::Sha224::default()),
            Hasher::Sha256 => Box::new(sha2::Sha256::default()),
            Hasher::Sha384 => Box::new(sha2::Sha384::default()),
            Hasher::Sha512 => Box::new(sha2::Sha512::default()),
            Hasher::Sha512_224 => Box::new(sha2::Sha512_224::default()),
            Hasher::Sha512_256 => Box::new(sha2::Sha512_256::default()),
        };
        hasher.update(data.as_ref());
        let hash = hasher.finalize_reset();
        let hash = hash.as_ref();
        Json(HashRes {
            slice: hash.to_vec(),
            slice_hex: format!("{:X?}", hash),
            string_hex: hash
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<String>(),
            string_base64: base64::encode(hash),
        })
    }
}

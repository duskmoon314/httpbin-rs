use std::str::FromStr;

use digest::DynDigest;
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    types::Example,
    ApiResponse, Enum, Object, OpenApi,
};
use uuid::Uuid;

use super::ApiTags;

#[derive(ApiResponse)]
enum Base64Res {
    #[oai(status = 200)]
    Ok(PlainText<String>),

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
    async fn base64_decode(&self, base64: String) -> Base64Res {
        match base64::decode(&base64) {
            Ok(decoded) => match String::from_utf8(decoded) {
                Ok(decoded) => Base64Res::Ok(PlainText(decoded)),
                Err(err) => Base64Res::BadRequest(PlainText(format!("{}", err))),
            },
            Err(err) => Base64Res::BadRequest(PlainText(format!("{}", err))),
        }
    }

    /// Encode string as base64.
    #[oai(path = "/base64/encode", method = "post", tag = "ApiTags::Data")]
    async fn base64_encode(&self, string: String) -> Base64Res {
        Base64Res::Ok(PlainText(base64::encode(&string)))
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
    #[oai(path = "/hash/:hasher", method = "post", tag = "ApiTags::Data")]
    async fn hash(&self, hasher: Path<Hasher>, string: String) -> Json<HashRes> {
        let mut hasher: Box<dyn DynDigest> = match hasher.0 {
            Hasher::Md5 => Box::new(md5::Md5::default()),
            Hasher::Sha224 => Box::new(sha2::Sha224::default()),
            Hasher::Sha256 => Box::new(sha2::Sha256::default()),
            Hasher::Sha384 => Box::new(sha2::Sha384::default()),
            Hasher::Sha512 => Box::new(sha2::Sha512::default()),
            Hasher::Sha512_224 => Box::new(sha2::Sha512_224::default()),
            Hasher::Sha512_256 => Box::new(sha2::Sha512_256::default()),
        };
        hasher.update(string.as_bytes());
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

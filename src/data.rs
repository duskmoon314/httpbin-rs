use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi,
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
        PlainText(Uuid::new_v3(&req.0.namespace, req.0.name.as_bytes()).to_string())
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
        PlainText(Uuid::new_v5(&req.0.namespace, req.0.name.as_bytes()).to_string())
    }
}

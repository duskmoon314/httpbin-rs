use poem::Result;
use poem_openapi::{param::Query, payload::PlainText, ApiResponse, Enum, NewType, OpenApi};

use super::DataTag;

#[derive(NewType)]
struct UuidNamespace(String);

impl From<UuidNamespace> for httpbin::data::uuid::UuidNamespace {
    fn from(namespace: UuidNamespace) -> Self {
        match namespace.0.as_str() {
            "dns" => httpbin::data::uuid::UuidNamespace::Dns,
            "url" => httpbin::data::uuid::UuidNamespace::Url,
            "oid" => httpbin::data::uuid::UuidNamespace::Oid,
            "x500" => httpbin::data::uuid::UuidNamespace::X500,
            _ => httpbin::data::uuid::UuidNamespace::Custom(namespace.0),
        }
    }
}

#[derive(Enum, Default)]
#[oai(rename_all = "snake_case")]
enum UuidFormat {
    /// 8-4-4-4-12
    #[default]
    Hyphenated,

    /// 32 hex digits
    Simple,

    /// urn:uuid:8-4-4-4-12
    Urn,

    /// {8-4-4-4-12}
    Braced,
}

impl From<UuidFormat> for httpbin::data::uuid::UuidFormat {
    fn from(format: UuidFormat) -> Self {
        match format {
            UuidFormat::Hyphenated => httpbin::data::uuid::UuidFormat::Hyphenated,
            UuidFormat::Simple => httpbin::data::uuid::UuidFormat::Simple,
            UuidFormat::Urn => httpbin::data::uuid::UuidFormat::Urn,
            UuidFormat::Braced => httpbin::data::uuid::UuidFormat::Braced,
        }
    }
}

#[derive(ApiResponse)]
enum UuidRes {
    /// The generated UUID
    #[oai(status = 200)]
    Ok(PlainText<String>),

    /// Bad request
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

pub struct Api;

#[OpenApi(prefix_path = "/uuid", tag = "DataTag::Data")]
impl Api {
    /// Generate a v1 UUID
    #[oai(path = "/v1", method = "get")]
    async fn uuid_v1(
        &self,
        timestamp: Query<Option<u64>>,
        counter: Query<Option<u16>>,
        #[oai(explode = false)] node_id: Query<[u8; 6]>,
        format: Query<Option<UuidFormat>>,
    ) -> Result<UuidRes> {
        let timestamp = timestamp.and_then(|timestamp| {
            counter.map_or(Some((timestamp, 0)), |counter| Some((timestamp, counter)))
        });
        let format = format.0.unwrap_or_default();

        let uuid = httpbin::data::uuid::new_v1(timestamp, &node_id, format.into());

        Ok(UuidRes::Ok(PlainText(uuid)))
    }

    /// Generate a v3 UUID
    #[oai(path = "/v3", method = "get")]
    async fn uuid_v3(
        &self,
        namespace: Query<UuidNamespace>,
        name: Query<String>,
        format: Query<Option<UuidFormat>>,
    ) -> Result<UuidRes> {
        let format = format.0.unwrap_or_default();

        let uuid = httpbin::data::uuid::new_v3(namespace.0.into(), &name, format.into())
            .map_err(|e| UuidRes::BadRequest(PlainText(e.to_string())))?;

        Ok(UuidRes::Ok(PlainText(uuid)))
    }

    /// Generate a v4 UUID
    #[oai(path = "/v4", method = "get")]
    async fn uuid_v4(&self, format: Query<Option<UuidFormat>>) -> Result<UuidRes> {
        let format = format.0.unwrap_or_default();

        let uuid = httpbin::data::uuid::new_v4(format.into());

        Ok(UuidRes::Ok(PlainText(uuid)))
    }

    /// Generate a v5 UUID
    #[oai(path = "/v5", method = "get")]
    async fn uuid_v5(
        &self,
        namespace: Query<UuidNamespace>,
        name: Query<String>,
        format: Query<Option<UuidFormat>>,
    ) -> Result<UuidRes> {
        let format = format.0.unwrap_or_default();

        let uuid = httpbin::data::uuid::new_v5(namespace.0.into(), &name, format.into())
            .map_err(|e| UuidRes::BadRequest(PlainText(e.to_string())))?;

        Ok(UuidRes::Ok(PlainText(uuid)))
    }

    /// Generate a v6 UUID
    #[oai(path = "/v6", method = "get")]
    async fn uuid_v6(
        &self,
        timestamp: Query<Option<u64>>,
        counter: Query<Option<u16>>,
        #[oai(explode = false)] node_id: Query<[u8; 6]>,
        format: Query<Option<UuidFormat>>,
    ) -> Result<UuidRes> {
        let timestamp = timestamp.and_then(|timestamp| {
            counter.map_or(Some((timestamp, 0)), |counter| Some((timestamp, counter)))
        });
        let format = format.0.unwrap_or_default();

        let uuid = httpbin::data::uuid::new_v6(timestamp, &node_id, format.into());

        Ok(UuidRes::Ok(PlainText(uuid)))
    }

    /// Generate a v7 UUID
    #[oai(path = "/v7", method = "get")]
    async fn uuid_v7(
        &self,
        timestamp: Query<Option<u64>>,
        counter: Query<Option<u16>>,
        format: Query<Option<UuidFormat>>,
    ) -> Result<UuidRes> {
        let timestamp = timestamp.and_then(|timestamp| {
            counter.map_or(Some((timestamp, 0)), |counter| Some((timestamp, counter)))
        });
        let format = format.0.unwrap_or_default();

        let uuid = httpbin::data::uuid::new_v7(timestamp, format.into());

        Ok(UuidRes::Ok(PlainText(uuid)))
    }

    /// Generate a v8 UUID
    #[oai(path = "/v8", method = "get")]
    async fn uuid_v8(
        &self,
        buf: Query<[u8; 16]>,
        format: Query<Option<UuidFormat>>,
    ) -> Result<UuidRes> {
        let format = format.0.unwrap_or_default();

        let uuid = httpbin::data::uuid::new_v8(&buf, format.into());

        Ok(UuidRes::Ok(PlainText(uuid)))
    }
}

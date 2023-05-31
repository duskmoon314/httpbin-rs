use std::fmt::Display;

use actix_web::{
    web::{get, resource, scope, Query, ServiceConfig},
    HttpResponse, ResponseError, Result,
};
use httpbin::data::uuid::{UuidBuffer, UuidFormat, UuidNamespace, UuidNodeId};
use serde::Deserialize;

#[derive(Deserialize)]
struct Format {
    format: Option<UuidFormat>,
}

#[derive(Deserialize)]
struct TimestampCounter {
    timestamp: Option<u64>,
    counter: Option<u16>,
}

#[derive(Deserialize)]
struct NodeId {
    node_id: UuidNodeId,
}

#[derive(Deserialize)]
struct NamespaceName {
    namespace: UuidNamespace,
    name: String,
}

#[derive(Deserialize)]
struct Buf {
    buf: UuidBuffer,
}

#[derive(Debug)]
struct UuidError(pub httpbin::data::uuid::UuidError);

impl Display for UuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ResponseError for UuidError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::BadRequest().body(self.0.to_string())
    }
}

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/uuid")
            .service(resource("/v1").route(get().to(uuid_v1)))
            .service(resource("/v3").route(get().to(uuid_v3)))
            .service(resource("/v4").route(get().to(uuid_v4)))
            .service(resource("/v5").route(get().to(uuid_v5)))
            .service(resource("/v6").route(get().to(uuid_v6)))
            .service(resource("/v7").route(get().to(uuid_v7)))
            .service(resource("/v8").route(get().to(uuid_v8))),
    );
}

async fn uuid_v1(
    Query(TimestampCounter { timestamp, counter }): Query<TimestampCounter>,
    Query(NodeId { node_id }): Query<NodeId>,
    Query(Format { format }): Query<Format>,
) -> Result<String, UuidError> {
    let timestamp = timestamp.and_then(|timestamp| {
        counter.map_or(Some((timestamp, 0)), |counter| Some((timestamp, counter)))
    });
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v1(timestamp, node_id, format).map_err(UuidError)?;

    Ok(uuid)
}

async fn uuid_v3(
    Query(NamespaceName { namespace, name }): Query<NamespaceName>,
    Query(Format { format }): Query<Format>,
) -> Result<String, UuidError> {
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v3(namespace, &name, format).map_err(UuidError)?;

    Ok(uuid)
}

async fn uuid_v4(Query(Format { format }): Query<Format>) -> Result<String> {
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v4(format);

    Ok(uuid)
}

async fn uuid_v5(
    Query(NamespaceName { namespace, name }): Query<NamespaceName>,
    Query(Format { format }): Query<Format>,
) -> Result<String, UuidError> {
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v5(namespace, &name, format).map_err(UuidError)?;

    Ok(uuid)
}

async fn uuid_v6(
    Query(TimestampCounter { timestamp, counter }): Query<TimestampCounter>,
    Query(NodeId { node_id }): Query<NodeId>,
    Query(Format { format }): Query<Format>,
) -> Result<String, UuidError> {
    let timestamp = timestamp.and_then(|timestamp| {
        counter.map_or(Some((timestamp, 0)), |counter| Some((timestamp, counter)))
    });
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v6(timestamp, node_id, format).map_err(UuidError)?;

    Ok(uuid)
}

async fn uuid_v7(
    Query(TimestampCounter { timestamp, counter }): Query<TimestampCounter>,
    Query(Format { format }): Query<Format>,
) -> Result<String> {
    let timestamp = timestamp.and_then(|timestamp| {
        counter.map_or(Some((timestamp, 0)), |counter| Some((timestamp, counter)))
    });
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v7(timestamp, format);

    Ok(uuid)
}

async fn uuid_v8(
    Query(Buf { buf }): Query<Buf>,
    Query(Format { format }): Query<Format>,
) -> Result<String, UuidError> {
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v8(buf, format).map_err(UuidError)?;

    Ok(uuid)
}

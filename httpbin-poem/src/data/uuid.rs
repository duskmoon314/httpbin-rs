use anyhow::anyhow;
use httpbin::data::uuid::{UuidBuffer, UuidFormat, UuidNamespace, UuidNodeId};
use poem::{get, handler, web::Query, Result, Route};
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

pub fn api(route: Route) -> Route {
    route.nest(
        "/uuid",
        Route::new()
            .at("/v1", get(uuid_v1))
            .at("/v3", get(uuid_v3))
            .at("/v4", get(uuid_v4))
            .at("/v5", get(uuid_v5))
            .at("/v6", get(uuid_v6))
            .at("/v7", get(uuid_v7))
            .at("/v8", get(uuid_v8)),
    )
}

#[handler]
fn uuid_v1(
    Query(TimestampCounter { timestamp, counter }): Query<TimestampCounter>,
    Query(NodeId { node_id }): Query<NodeId>,
    Query(Format { format }): Query<Format>,
) -> Result<String> {
    let timestamp = timestamp.and_then(|timestamp| {
        counter.map_or(Some((timestamp, 0)), |counter| Some((timestamp, counter)))
    });
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v1(timestamp, node_id, format).map_err(|e| anyhow!(e))?;

    Ok(uuid)
}

#[handler]
fn uuid_v3(
    Query(NamespaceName { namespace, name }): Query<NamespaceName>,
    Query(Format { format }): Query<Format>,
) -> Result<String> {
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v3(namespace, &name, format).map_err(|e| anyhow!(e))?;

    Ok(uuid)
}

#[handler]
fn uuid_v4(Query(Format { format }): Query<Format>) -> Result<String> {
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v4(format);

    Ok(uuid)
}

#[handler]
fn uuid_v5(
    Query(NamespaceName { namespace, name }): Query<NamespaceName>,
    Query(Format { format }): Query<Format>,
) -> Result<String> {
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v5(namespace, &name, format).map_err(|e| anyhow!(e))?;

    Ok(uuid)
}

#[handler]
fn uuid_v6(
    Query(TimestampCounter { timestamp, counter }): Query<TimestampCounter>,
    Query(NodeId { node_id }): Query<NodeId>,
    Query(Format { format }): Query<Format>,
) -> Result<String> {
    let timestamp = timestamp.and_then(|timestamp| {
        counter.map_or(Some((timestamp, 0)), |counter| Some((timestamp, counter)))
    });
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v6(timestamp, node_id, format).map_err(|e| anyhow!(e))?;

    Ok(uuid)
}

#[handler]
fn uuid_v7(
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

#[handler]
fn uuid_v8(
    Query(Buf { buf }): Query<Buf>,
    Query(Format { format }): Query<Format>,
) -> Result<String> {
    let format = format.unwrap_or_default();

    let uuid = httpbin::data::uuid::new_v8(buf, format).map_err(|e| anyhow!(e))?;

    Ok(uuid)
}

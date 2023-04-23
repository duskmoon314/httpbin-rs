use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum UuidNamespace {
    Dns,
    Oid,
    Url,
    X500,
    Custom(String),
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum UuidFormat {
    #[default]
    Hyphenated,
    Simple,
    Urn,
    Braced,
}

#[derive(Error, Debug)]
pub enum UuidError {
    #[error("the uuid string is invalid: {0}")]
    InvalidUuid(#[from] uuid::Error),
}

/// Generate a v1 UUID with the given timestamp and node_id
pub fn new_v1(timestamp: Option<(u64, u16)>, node_id: &[u8; 6], format: UuidFormat) -> String {
    let uuid = match timestamp {
        Some((ticks, counter)) => {
            uuid::Uuid::new_v1(uuid::Timestamp::from_rfc4122(ticks, counter), node_id)
        }
        None => uuid::Uuid::now_v1(node_id),
    };

    match format {
        UuidFormat::Hyphenated => format!("{}", uuid.as_hyphenated()),
        UuidFormat::Simple => format!("{}", uuid.as_simple()),
        UuidFormat::Urn => format!("{}", uuid.as_urn()),
        UuidFormat::Braced => format!("{}", uuid.as_braced()),
    }
}

/// Generate a v3 UUID with the given namespace and name
pub fn new_v3(
    namespace: UuidNamespace,
    name: &str,
    format: UuidFormat,
) -> Result<String, UuidError> {
    let ns = match namespace {
        UuidNamespace::Dns => uuid::Uuid::NAMESPACE_DNS,
        UuidNamespace::Oid => uuid::Uuid::NAMESPACE_OID,
        UuidNamespace::Url => uuid::Uuid::NAMESPACE_URL,
        UuidNamespace::X500 => uuid::Uuid::NAMESPACE_X500,
        UuidNamespace::Custom(ns) => uuid::Uuid::parse_str(&ns)?,
    };
    let uuid = uuid::Uuid::new_v3(&ns, name.as_bytes());
    Ok(match format {
        UuidFormat::Hyphenated => format!("{}", uuid.as_hyphenated()),
        UuidFormat::Simple => format!("{}", uuid.as_simple()),
        UuidFormat::Urn => format!("{}", uuid.as_urn()),
        UuidFormat::Braced => format!("{}", uuid.as_braced()),
    })
}

/// Generate a v4 UUID
pub fn new_v4(format: UuidFormat) -> String {
    let uuid = uuid::Uuid::new_v4();
    match format {
        UuidFormat::Hyphenated => format!("{}", uuid.as_hyphenated()),
        UuidFormat::Simple => format!("{}", uuid.as_simple()),
        UuidFormat::Urn => format!("{}", uuid.as_urn()),
        UuidFormat::Braced => format!("{}", uuid.as_braced()),
    }
}

/// Generate a v5 UUID with the given namespace and name
pub fn new_v5(
    namespace: UuidNamespace,
    name: &str,
    format: UuidFormat,
) -> Result<String, UuidError> {
    let ns = match namespace {
        UuidNamespace::Dns => uuid::Uuid::NAMESPACE_DNS,
        UuidNamespace::Oid => uuid::Uuid::NAMESPACE_OID,
        UuidNamespace::Url => uuid::Uuid::NAMESPACE_URL,
        UuidNamespace::X500 => uuid::Uuid::NAMESPACE_X500,
        UuidNamespace::Custom(ns) => uuid::Uuid::parse_str(&ns)?,
    };
    let uuid = uuid::Uuid::new_v5(&ns, name.as_bytes());
    Ok(match format {
        UuidFormat::Hyphenated => format!("{}", uuid.as_hyphenated()),
        UuidFormat::Simple => format!("{}", uuid.as_simple()),
        UuidFormat::Urn => format!("{}", uuid.as_urn()),
        UuidFormat::Braced => format!("{}", uuid.as_braced()),
    })
}

/// Generate a v6 UUID with the given timestamp and node_id
pub fn new_v6(timestamp: Option<(u64, u16)>, node_id: &[u8; 6], format: UuidFormat) -> String {
    let uuid = match timestamp {
        Some((ticks, counter)) => {
            uuid::Uuid::new_v6(uuid::Timestamp::from_rfc4122(ticks, counter), node_id)
        }
        None => uuid::Uuid::now_v6(node_id),
    };

    match format {
        UuidFormat::Hyphenated => format!("{}", uuid.as_hyphenated()),
        UuidFormat::Simple => format!("{}", uuid.as_simple()),
        UuidFormat::Urn => format!("{}", uuid.as_urn()),
        UuidFormat::Braced => format!("{}", uuid.as_braced()),
    }
}

/// Generate a v7 UUID with the given timestamp
pub fn new_v7(timestamp: Option<(u64, u16)>, format: UuidFormat) -> String {
    let uuid = match timestamp {
        Some((ticks, counter)) => uuid::Uuid::new_v7(uuid::Timestamp::from_rfc4122(ticks, counter)),
        None => uuid::Uuid::now_v7(),
    };

    match format {
        UuidFormat::Hyphenated => format!("{}", uuid.as_hyphenated()),
        UuidFormat::Simple => format!("{}", uuid.as_simple()),
        UuidFormat::Urn => format!("{}", uuid.as_urn()),
        UuidFormat::Braced => format!("{}", uuid.as_braced()),
    }
}

pub fn new_v8(buf: &[u8; 16], format: UuidFormat) -> String {
    let uuid = uuid::Uuid::new_v8(*buf);
    match format {
        UuidFormat::Hyphenated => format!("{}", uuid.as_hyphenated()),
        UuidFormat::Simple => format!("{}", uuid.as_simple()),
        UuidFormat::Urn => format!("{}", uuid.as_urn()),
        UuidFormat::Braced => format!("{}", uuid.as_braced()),
    }
}

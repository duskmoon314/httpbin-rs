use std::{convert::Infallible, str::FromStr};

use serde::Deserialize;
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};
use thiserror::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum UuidConstNamespace {
    Dns,
    Oid,
    Url,
    X500,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum UuidNamespace {
    Const(UuidConstNamespace),
    Custom(String),
}

impl FromStr for UuidNamespace {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "dns" => Self::Const(UuidConstNamespace::Dns),
            "oid" => Self::Const(UuidConstNamespace::Oid),
            "url" => Self::Const(UuidConstNamespace::Url),
            "x500" => Self::Const(UuidConstNamespace::X500),
            _ => Self::Custom(s.to_owned()),
        })
    }
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

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct UuidNodeId(#[serde_as(as = "StringWithSeparator::<CommaSeparator, u8>")] pub Vec<u8>);

impl From<Vec<u8>> for UuidNodeId {
    fn from(node_id: Vec<u8>) -> Self {
        Self(node_id)
    }
}

impl TryInto<[u8; 6]> for UuidNodeId {
    type Error = ();

    fn try_into(self) -> Result<[u8; 6], Self::Error> {
        let node_id = self.0.try_into().map_err(|_| ())?;

        Ok(node_id)
    }
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct UuidBuffer(#[serde_as(as = "StringWithSeparator::<CommaSeparator, u8>")] pub Vec<u8>);

impl From<Vec<u8>> for UuidBuffer {
    fn from(buffer: Vec<u8>) -> Self {
        Self(buffer)
    }
}

impl TryInto<[u8; 16]> for UuidBuffer {
    type Error = ();

    fn try_into(self) -> Result<[u8; 16], Self::Error> {
        let buffer = self.0.try_into().map_err(|_| ())?;

        Ok(buffer)
    }
}

#[derive(Error, Debug)]
pub enum UuidError {
    #[error("the uuid string is invalid: {0}")]
    InvalidUuid(#[from] uuid::Error),
    #[error("the length of node_id must be 6")]
    InvalidNodeIdLength,
    #[error("the length of buffer must be 16")]
    InvalidBufferLength,
}

/// Generate a v1 UUID with the given timestamp and node_id
pub fn new_v1(
    timestamp: Option<(u64, u16)>,
    node_id: UuidNodeId,
    format: UuidFormat,
) -> Result<String, UuidError> {
    let node_id: [u8; 6] = node_id
        .try_into()
        .map_err(|_| UuidError::InvalidNodeIdLength)?;

    let uuid = match timestamp {
        Some((ticks, counter)) => {
            uuid::Uuid::new_v1(uuid::Timestamp::from_rfc4122(ticks, counter), &node_id)
        }
        None => uuid::Uuid::now_v1(&node_id),
    };

    Ok(match format {
        UuidFormat::Hyphenated => format!("{}", uuid.as_hyphenated()),
        UuidFormat::Simple => format!("{}", uuid.as_simple()),
        UuidFormat::Urn => format!("{}", uuid.as_urn()),
        UuidFormat::Braced => format!("{}", uuid.as_braced()),
    })
}

/// Generate a v3 UUID with the given namespace and name
pub fn new_v3(
    namespace: UuidNamespace,
    name: &str,
    format: UuidFormat,
) -> Result<String, UuidError> {
    let ns = match namespace {
        UuidNamespace::Const(UuidConstNamespace::Dns) => uuid::Uuid::NAMESPACE_DNS,
        UuidNamespace::Const(UuidConstNamespace::Oid) => uuid::Uuid::NAMESPACE_OID,
        UuidNamespace::Const(UuidConstNamespace::Url) => uuid::Uuid::NAMESPACE_URL,
        UuidNamespace::Const(UuidConstNamespace::X500) => uuid::Uuid::NAMESPACE_X500,
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
        UuidNamespace::Const(UuidConstNamespace::Dns) => uuid::Uuid::NAMESPACE_DNS,
        UuidNamespace::Const(UuidConstNamespace::Oid) => uuid::Uuid::NAMESPACE_OID,
        UuidNamespace::Const(UuidConstNamespace::Url) => uuid::Uuid::NAMESPACE_URL,
        UuidNamespace::Const(UuidConstNamespace::X500) => uuid::Uuid::NAMESPACE_X500,
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
pub fn new_v6(
    timestamp: Option<(u64, u16)>,
    node_id: UuidNodeId,
    format: UuidFormat,
) -> Result<String, UuidError> {
    let node_id: [u8; 6] = node_id
        .try_into()
        .map_err(|_| UuidError::InvalidNodeIdLength)?;

    let uuid = match timestamp {
        Some((ticks, counter)) => {
            uuid::Uuid::new_v6(uuid::Timestamp::from_rfc4122(ticks, counter), &node_id)
        }
        None => uuid::Uuid::now_v6(&node_id),
    };

    Ok(match format {
        UuidFormat::Hyphenated => format!("{}", uuid.as_hyphenated()),
        UuidFormat::Simple => format!("{}", uuid.as_simple()),
        UuidFormat::Urn => format!("{}", uuid.as_urn()),
        UuidFormat::Braced => format!("{}", uuid.as_braced()),
    })
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

pub fn new_v8(buf: UuidBuffer, format: UuidFormat) -> Result<String, UuidError> {
    let buf = buf.try_into().map_err(|_| UuidError::InvalidBufferLength)?;

    let uuid = uuid::Uuid::new_v8(buf);
    Ok(match format {
        UuidFormat::Hyphenated => format!("{}", uuid.as_hyphenated()),
        UuidFormat::Simple => format!("{}", uuid.as_simple()),
        UuidFormat::Urn => format!("{}", uuid.as_urn()),
        UuidFormat::Braced => format!("{}", uuid.as_braced()),
    })
}

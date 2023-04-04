use std::{collections::HashMap, net::IpAddr};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    // /// The IP to listen on. (default: 127.0.0.1)
    // #[clap(long, value_parser, default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    // pub ip: IpAddr,

    // /// The port to listen on. (default: 8000)
    // #[clap(long, value_parser, default_value_t = 8000)]
    // pub port: u16,
    /// The config file to use.
    #[clap(long, value_parser, default_value_t = String::from("httpbin.toml"))]
    pub config: String,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub fn load_config(&self) -> Config {
        let config = std::fs::read_to_string(&self.config).unwrap();
        toml::from_str(&config).unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub ip: IpAddr,
    pub port: u16,
    pub openapi: OpenApiConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenApiConfig {
    pub contact: OpenApiContact,
    pub external_document: OpenApiExternalDocument,
    pub servers: HashMap<String, OpenApiServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenApiContact {
    pub name: String,
    pub url: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenApiExternalDocument {
    pub description: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenApiServer {
    pub url: String,
    pub description: String,
}

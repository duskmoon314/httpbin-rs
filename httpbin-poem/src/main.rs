use anyhow::Result;
use httpbin::cli::Cli;
use poem::{listener::TcpListener, middleware, EndpointExt, Route, Server};

mod data;
mod http_method;
mod request_inspection;
mod utils;

use utils::*;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Cli::parse().load_config();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=info");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .attach(data::api)
        .attach(request_inspection::api)
        .attach(http_method::api)
        .with(middleware::Cors::new().allow_origins_fn(|_| true))
        .with(middleware::NormalizePath::new(
            middleware::TrailingSlash::Trim,
        ))
        .with(middleware::Tracing);

    Ok(Server::new(TcpListener::bind((cfg.ip, cfg.port)))
        .run(app)
        .await?)
}

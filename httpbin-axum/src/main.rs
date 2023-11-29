use anyhow::Result;
use axum::{extract::Request, Router, ServiceExt};
use httpbin::cli::Cli;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    normalize_path::NormalizePathLayer,
    trace::TraceLayer,
};
use tower_layer::Layer;

mod data;
mod http_method;
mod request_inspection;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Cli::parse().load_config();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tower_http=debug");
    }
    tracing_subscriber::fmt::init();

    let app = ServiceExt::<Request>::into_make_service(
        NormalizePathLayer::trim_trailing_slash().layer(
            Router::new()
                .merge(data::api())
                .merge(request_inspection::api())
                .merge(http_method::api())
                .layer(CorsLayer::new().allow_origin(AllowOrigin::mirror_request()))
                .layer(TraceLayer::new_for_http()),
        ),
    );

    let listener = tokio::net::TcpListener::bind((cfg.ip, cfg.port)).await?;

    Ok(axum::serve(listener, app).await?)
}

use httpbin::cli::Cli;
use salvo::cors::Cors;
use salvo::prelude::*;

mod data;
mod http_method;
mod request_inspection;

#[tokio::main]
async fn main() {
    let cfg = Cli::parse().load_config();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let cors = Cors::builder().allow_any_origin().build();

    let router = Router::new()
        .hoop(cors)
        .hoop(TrailingSlash::new_remove())
        .hoop(Logger)
        .push(data::api())
        .push(http_method::api())
        .push(request_inspection::api());

    Server::new(TcpListener::bind((cfg.ip, cfg.port)))
        .serve(router)
        .await
}

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use anyhow::Result;
use httpbin::cli::Cli;

mod data;
mod http_method;
mod request_inspection;

#[actix_web::main]
async fn main() -> Result<()> {
    let cfg = Cli::parse().load_config();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting httpbin-actix on {}:{}", cfg.ip, cfg.port);

    Ok(HttpServer::new(|| {
        let cors = Cors::default().allowed_origin_fn(|_, _| true);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(data::api)
            .configure(http_method::api)
            .configure(request_inspection::api)
    })
    .bind((cfg.ip, cfg.port))?
    .run()
    .await?)
}

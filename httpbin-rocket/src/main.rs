use anyhow::Result;
use httpbin::cli::Cli;
use rocket::{fairing::AdHoc, Config};

mod data;
mod http_method;
mod request_inspection;

#[rocket::main]
async fn main() -> Result<()> {
    let cfg = Cli::parse().load_config();

    let rocket_config = Config {
        address: cfg.ip,
        port: cfg.port,
        ..Config::default()
    };

    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::all())
        .to_cors()?;

    let _ = rocket::custom(rocket_config)
        .attach(cors)
        .attach(AdHoc::on_ignite("mount_data", data::api))
        .attach(AdHoc::on_ignite("mount_http_method", http_method::api))
        .attach(AdHoc::on_ignite(
            "mount_request_inspection",
            request_inspection::api,
        ))
        .launch()
        .await?;

    Ok(())
}

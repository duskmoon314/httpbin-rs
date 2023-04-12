mod data;
mod http_method;
mod request_inspection;

use httpbin::cli::Cli;
use rocket::{fairing::AdHoc, Config};

#[rocket::launch]
fn rocket() -> _ {
    let cfg = Cli::parse().load_config();

    let rocket_config = Config {
        address: cfg.ip,
        port: cfg.port,
        ..Config::default()
    };

    rocket::custom(rocket_config)
        .attach(AdHoc::on_ignite("mount_data", data::api))
        .attach(AdHoc::on_ignite("mount_http_method", http_method::api))
        .attach(AdHoc::on_ignite(
            "mount_request_inspection",
            request_inspection::api,
        ))
}

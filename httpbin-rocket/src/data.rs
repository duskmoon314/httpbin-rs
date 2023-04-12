use rocket::{fairing::AdHoc, Build, Rocket};

mod base64;

pub async fn api(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.attach(AdHoc::on_ignite("mount_data_base64", base64::api))
}

use actix_web::web::ServiceConfig;

mod base64;
mod uuid;

pub fn api(cfg: &mut ServiceConfig) {
    cfg.configure(base64::api).configure(uuid::api);
}

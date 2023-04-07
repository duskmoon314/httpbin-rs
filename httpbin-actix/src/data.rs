use actix_web::web::ServiceConfig;

pub mod base64;

pub fn api(cfg: &mut ServiceConfig) {
    cfg.configure(base64::api);
}

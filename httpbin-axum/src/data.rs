use axum::Router;

pub mod base64;
pub mod uuid;

pub fn api() -> Router {
    Router::new()
        .nest("/base64", base64::api())
        .nest("/uuid", uuid::api())
}

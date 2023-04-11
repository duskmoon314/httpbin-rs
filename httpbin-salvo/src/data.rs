use salvo::Router;

mod base64;

pub fn api() -> Router {
    Router::new().push(Router::with_path("/base64").push(base64::api()))
}

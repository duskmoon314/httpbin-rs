use poem_openapi::{OpenApi, Tags};

pub mod base64;
pub mod uuid;

#[derive(Tags)]
enum DataTag {
    /// Generates useful data
    Data,
}

pub fn api() -> impl OpenApi {
    (base64::Api, uuid::Api)
}

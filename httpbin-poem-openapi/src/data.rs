use poem_openapi::Tags;

pub mod base64;

#[derive(Tags)]
enum DataTag {
    /// Generates useful data
    Data,
}

pub struct Api;

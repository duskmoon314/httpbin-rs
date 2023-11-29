use std::collections::HashMap;

use rocket::{
    data::ToByteUnit,
    http::{
        ContentType,
        Method::{Delete, Get, Patch, Post, Put},
        Status,
    },
    route::{Handler, Outcome},
    serde::json::Json,
    Build, Data, Request, Rocket, Route,
};
use serde::Serialize;

#[derive(Serialize)]
struct Http {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    origin: Option<String>,
    query: Option<HashMap<String, String>>,
    body_string: String,
    json: Option<serde_json::Value>,
}

pub async fn api(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", Anything)
}

#[derive(Clone)]
struct Anything;

impl From<Anything> for Vec<Route> {
    fn from(value: Anything) -> Vec<Route> {
        vec![
            Route::new(Get, "/get", value.clone()),
            Route::new(Post, "/post", value.clone()),
            Route::new(Put, "/put", value.clone()),
            Route::new(Delete, "/delete", value.clone()),
            Route::new(Patch, "/patch", value.clone()),
            Route::new(Get, "/anything/<anything..>", value.clone()),
            Route::new(Post, "/anything/<anything..>", value.clone()),
            Route::new(Put, "/anything/<anything..>", value.clone()),
            Route::new(Delete, "/anything/<anything..>", value.clone()),
            Route::new(Patch, "/anything/<anything..>", value),
        ]
    }
}

#[rocket::async_trait]
impl Handler for Anything {
    async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
        let method = req.method().to_string();

        let uri = req.uri().to_string();

        let headers = req
            .headers()
            .iter()
            .map(|header| (header.name().to_string(), header.value().to_string()))
            .collect();

        let origin = req.client_ip().map(|origin| origin.to_string());

        let query = req.uri().query().map(|query_str| {
            serde_qs::from_str(query_str.as_str())
                .unwrap_or_else(|err| [("error".to_string(), err.to_string())].into())
        });

        let body = match data.open(512.kibibytes()).into_bytes().await {
            // TODO: Handle incomplete body
            Ok(body) => body.into_inner(),
            Err(_) => return Outcome::error(Status::InternalServerError),
        };

        let body_string = match String::from_utf8(body.clone()) {
            Ok(body) => body,
            Err(_) => {
                match httpbin::data::base64::encode(
                    &body,
                    httpbin::data::base64::Base64Engine::Standard,
                    None,
                ) {
                    Ok(body) => body,
                    Err(err) => err.to_string(),
                }
            }
        };

        let json = if req.content_type() == Some(&ContentType::JSON) {
            Some(serde_json::from_slice(&body).unwrap_or_else(|err| {
                serde_json::json!({
                    "error": err.to_string(),
                })
            }))
        } else {
            None
        };

        Outcome::from(
            req,
            Json(Http {
                method,
                uri,
                headers,
                origin,
                query,
                body_string,
                json,
            }),
        )
    }
}

use std::collections::HashMap;

use actix_web::{
    dev::ConnectionInfo,
    get,
    http::header::USER_AGENT,
    web::{Json, ServiceConfig},
    Either, HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;

#[derive(Serialize)]
struct Headers {
    /// The incoming request's HTTP headers
    headers: HashMap<String, String>,
}

#[derive(Serialize)]
struct Ip {
    /// The incoming request's IP address
    origin: String,
}

#[derive(Serialize)]
struct UserAgent {
    /// The incoming request's User-Agent header
    user_agent: String,
}

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(headers).service(ip).service(user_agent);
}

#[get("/headers")]
async fn headers(req: HttpRequest) -> Json<Headers> {
    let headers = req.headers();

    let headers = headers
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.to_str()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|err| err.to_string()),
            )
        })
        .collect();
    Json(Headers { headers })
}

#[get("/ip")]
async fn ip(conn: ConnectionInfo) -> Either<Json<Ip>, impl Responder> {
    match conn.realip_remote_addr() {
        Some(origin) => Either::Left(Json(Ip {
            origin: origin.to_string(),
        })),
        None => Either::Right(
            HttpResponse::BadRequest()
                .body("Could not determine the IP address through headers and socket address"),
        ),
    }
}

#[get("/user-agent")]
async fn user_agent(req: HttpRequest) -> Either<Json<UserAgent>, impl Responder> {
    match req.headers().get(USER_AGENT) {
        Some(user_agent) => match user_agent.to_str() {
            Ok(user_agent) => Either::Left(Json(UserAgent {
                user_agent: user_agent.to_string(),
            })),
            Err(err) => Either::Right(
                HttpResponse::BadRequest()
                    .body(format!("Could not parse the User-Agent header: {}", err)),
            ),
        },
        None => Either::Right(
            HttpResponse::BadRequest()
                .body("The incoming request does not have a User-Agent header"),
        ),
    }
}

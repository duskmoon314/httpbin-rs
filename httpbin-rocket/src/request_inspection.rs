use std::{collections::HashMap, net::IpAddr, ops::Deref};

use rocket::{get, request::FromRequest, routes, serde::json::Json, Build, Request, Rocket};
use serde::Serialize;

struct HeaderMap<'r>(&'r rocket::http::HeaderMap<'r>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HeaderMap<'r> {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        rocket::request::Outcome::Success(HeaderMap(request.headers()))
    }
}

impl<'r> Deref for HeaderMap<'r> {
    type Target = rocket::http::HeaderMap<'r>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

struct UserAgentHeader<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgentHeader<'r> {
    type Error = anyhow::Error;

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        match request.headers().get_one("User-Agent") {
            Some(user_agent) => rocket::request::Outcome::Success(UserAgentHeader(user_agent)),
            None => rocket::request::Outcome::Failure((
                rocket::http::Status::BadRequest,
                anyhow::anyhow!("The incoming request does not have a User-Agent header"),
            )),
        }
    }
}

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

pub async fn api(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![headers, ip, user_agent])
}

#[get("/headers")]
fn headers(headers: HeaderMap) -> Json<Headers> {
    let headers = headers
        .iter()
        .map(|header| (header.name().to_string(), header.value().to_string()))
        .collect();
    Json(Headers { headers })
}

#[get("/ip")]
fn ip(origin: IpAddr) -> Json<Ip> {
    Json(Ip {
        origin: origin.to_string(),
    })
}

#[get("/user-agent")]
fn user_agent(user_agent: UserAgentHeader) -> Json<UserAgent> {
    Json(UserAgent {
        user_agent: user_agent.0.to_string(),
    })
}

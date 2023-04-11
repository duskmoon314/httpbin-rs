use std::collections::HashMap;

use salvo::{hyper::header::USER_AGENT, prelude::*};
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

pub fn api() -> Router {
    Router::new()
        .push(Router::with_path("/headers").get(headers))
        .push(Router::with_path("/ip").get(ip))
        .push(Router::with_path("/user-agent").get(user_agent))
}

#[handler]
async fn headers(req: &Request) -> Json<Headers> {
    let headers_map = req
        .headers()
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
    Json(Headers {
        headers: headers_map,
    })
}

#[handler]
async fn ip(req: &Request, res: &mut Response) {
    match req.remote_addr() {
        Some(origin) => res.render(Json(Ip {
            origin: origin.to_string(),
        })),
        None => {
            res.set_status_error(StatusError::bad_request().with_summary(
                "Could not determine the IP address through headers and socket address",
            ))
        }
    }
}

#[handler]
async fn user_agent(req: &Request, res: &mut Response) {
    match req.headers().get(USER_AGENT) {
        Some(user_agent_value) => match user_agent_value.to_str() {
            Ok(user_agent_value) => res.render(Json(UserAgent {
                user_agent: user_agent_value.to_string(),
            })),
            Err(err) => res.set_status_error(
                StatusError::bad_request()
                    .with_summary("Could not parse the User-Agent header")
                    .with_detail(err.to_string()),
            ),
        },
        None => res.set_status_error(
            StatusError::bad_request()
                .with_summary("Could not determine the User-Agent through headers"),
        ),
    }
}

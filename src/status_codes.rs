use poem_openapi::{param::Path, payload::PlainText, ApiResponse, OpenApi};
use rand::prelude::*;

use super::ApiTags;

#[derive(ApiResponse)]
enum StatusRes {
    /// Continue
    #[oai(status = 100)]
    Continue,
    /// Switching Protocols
    #[oai(status = 101)]
    SwitchingProtocols,
    /// Processing
    #[oai(status = 102)]
    Processing,
    /// Ok
    #[oai(status = 200)]
    Ok,
    /// Created
    #[oai(status = 201)]
    Created,
    /// Accepted
    #[oai(status = 202)]
    Accepted,
    /// Non-Authoritative Information
    #[oai(status = 203)]
    NonAuthoritativeInformation,
    /// No Content
    #[oai(status = 204)]
    NoContent,
    /// Reset Content
    #[oai(status = 205)]
    ResetContent,
    /// Partial Content
    #[oai(status = 206)]
    PartialContent,
    /// Multi-Status
    #[oai(status = 207)]
    MultiStatus,
    /// Already Reported
    #[oai(status = 208)]
    AlreadyReported,
    /// IM Used
    #[oai(status = 226)]
    IMUsed,
    /// Multiple Choices
    #[oai(status = 300)]
    MultipleChoices,
    /// Moved Permanently
    #[oai(status = 301)]
    MovedPermanently,
    /// Found
    #[oai(status = 302)]
    Found,
    /// See Other
    #[oai(status = 303)]
    SeeOther,
    /// Not Modified
    #[oai(status = 304)]
    NotModified,
    /// Use Proxy
    #[oai(status = 305)]
    UseProxy,
    /// Temporary Redirect
    #[oai(status = 307)]
    TemporaryRedirect,
    /// Permanent Redirect
    #[oai(status = 308)]
    PermanentRedirect,
    /// Bad Request
    #[oai(status = 400)]
    BadRequest,
    /// Unauthorized
    #[oai(status = 401)]
    Unauthorized,
    /// Payment Required
    #[oai(status = 402)]
    PaymentRequired,
    /// Forbidden
    #[oai(status = 403)]
    Forbidden,
    /// Not Found
    #[oai(status = 404)]
    NotFound,
    /// Method Not Allowed
    #[oai(status = 405)]
    MethodNotAllowed,
    /// Not Acceptable
    #[oai(status = 406)]
    NotAcceptable,
    /// Proxy Authentication Required
    #[oai(status = 407)]
    ProxyAuthenticationRequired,
    /// Request Timeout
    #[oai(status = 408)]
    RequestTimeout,
    /// Conflict
    #[oai(status = 409)]
    Conflict,
    /// Gone
    #[oai(status = 410)]
    Gone,
    /// Length Required
    #[oai(status = 411)]
    LengthRequired,
    /// Precondition Failed
    #[oai(status = 412)]
    PreconditionFailed,
    /// Payload Too Large
    #[oai(status = 413)]
    PayloadTooLarge,
    /// URI Too Long
    #[oai(status = 414)]
    UriTooLong,
    /// Unsupported Media Type
    #[oai(status = 415)]
    UnsupportedMediaType,
    /// Range Not Satisfiable
    #[oai(status = 416)]
    RangeNotSatisfiable,
    /// Expectation Failed
    #[oai(status = 417)]
    ExpectationFailed,
    /// I'm a Teapot
    #[oai(status = 418, content_type = "message/teapot")]
    ImATeapot(PlainText<String>),
    /// Misdirected Request
    #[oai(status = 421)]
    MisdirectedRequest,
    /// Unprocessable Entity
    #[oai(status = 422)]
    UnprocessableEntity,
    /// Locked
    #[oai(status = 423)]
    Locked,
    /// Failed Dependency
    #[oai(status = 424)]
    FailedDependency,
    /// Upgrade Required
    #[oai(status = 426)]
    UpgradeRequired,
    /// Precondition Required
    #[oai(status = 428)]
    PreconditionRequired,
    /// Too Many Requests
    #[oai(status = 429)]
    TooManyRequests,
    /// Request Header Fields Too Large
    #[oai(status = 431)]
    RequestHeaderFieldsTooLarge,
    /// Unavailable For Legal Reasons
    #[oai(status = 451)]
    UnavailableForLegalReasons,
    /// Internal Server Error
    #[oai(status = 500)]
    InternalServerError,
    /// Not Implemented
    #[oai(status = 501)]
    NotImplemented,
    /// Bad Gateway
    #[oai(status = 502)]
    BadGateway,
    /// Service Unavailable
    #[oai(status = 503)]
    ServiceUnavailable,
    /// Gateway Timeout
    #[oai(status = 504)]
    GatewayTimeout,
    /// HTTP Version Not Supported
    #[oai(status = 505)]
    HttpVersionNotSupported,
    /// Variant Also Negotiates
    #[oai(status = 506)]
    VariantAlsoNegotiates,
    /// Insufficient Storage
    #[oai(status = 507)]
    InsufficientStorage,
    /// Loop Detected
    #[oai(status = 508)]
    LoopDetected,
    /// Not Extended
    #[oai(status = 510)]
    NotExtended,
    /// Network Authentication Required
    #[oai(status = 511)]
    NetworkAuthenticationRequired,
}

impl From<u16> for StatusRes {
    fn from(code: u16) -> Self {
        match code {
            100 => StatusRes::Continue,
            101 => StatusRes::SwitchingProtocols,
            102 => StatusRes::Processing,
            200 => StatusRes::Ok,
            201 => StatusRes::Created,
            202 => StatusRes::Accepted,
            203 => StatusRes::NonAuthoritativeInformation,
            204 => StatusRes::NoContent,
            205 => StatusRes::ResetContent,
            206 => StatusRes::PartialContent,
            207 => StatusRes::MultiStatus,
            208 => StatusRes::AlreadyReported,
            226 => StatusRes::IMUsed,
            300 => StatusRes::MultipleChoices,
            301 => StatusRes::MovedPermanently,
            302 => StatusRes::Found,
            303 => StatusRes::SeeOther,
            304 => StatusRes::NotModified,
            305 => StatusRes::UseProxy,
            307 => StatusRes::TemporaryRedirect,
            308 => StatusRes::PermanentRedirect,
            400 => StatusRes::BadRequest,
            401 => StatusRes::Unauthorized,
            402 => StatusRes::PaymentRequired,
            403 => StatusRes::Forbidden,
            404 => StatusRes::NotFound,
            405 => StatusRes::MethodNotAllowed,
            406 => StatusRes::NotAcceptable,
            407 => StatusRes::ProxyAuthenticationRequired,
            408 => StatusRes::RequestTimeout,
            409 => StatusRes::Conflict,
            410 => StatusRes::Gone,
            411 => StatusRes::LengthRequired,
            412 => StatusRes::PreconditionFailed,
            413 => StatusRes::PayloadTooLarge,
            414 => StatusRes::UriTooLong,
            415 => StatusRes::UnsupportedMediaType,
            416 => StatusRes::RangeNotSatisfiable,
            417 => StatusRes::ExpectationFailed,
            418 => StatusRes::ImATeapot(PlainText(format!("I'm a teapot\n{}", TEAPOT_ASCII_ART))),
            421 => StatusRes::MisdirectedRequest,
            422 => StatusRes::UnprocessableEntity,
            423 => StatusRes::Locked,
            424 => StatusRes::FailedDependency,
            426 => StatusRes::UpgradeRequired,
            428 => StatusRes::PreconditionRequired,
            429 => StatusRes::TooManyRequests,
            431 => StatusRes::RequestHeaderFieldsTooLarge,
            451 => StatusRes::UnavailableForLegalReasons,
            500 => StatusRes::InternalServerError,
            501 => StatusRes::NotImplemented,
            502 => StatusRes::BadGateway,
            503 => StatusRes::ServiceUnavailable,
            504 => StatusRes::GatewayTimeout,
            505 => StatusRes::HttpVersionNotSupported,
            506 => StatusRes::VariantAlsoNegotiates,
            507 => StatusRes::InsufficientStorage,
            508 => StatusRes::LoopDetected,
            510 => StatusRes::NotExtended,
            511 => StatusRes::NetworkAuthenticationRequired,
            _ => StatusRes::BadRequest,
        }
    }
}

pub struct Api;

#[OpenApi]
impl Api {
    /// Return status code or random status code if more than one is given
    #[oai(
        path = "/status/:codes",
        method = "get",
        method = "post",
        method = "put",
        method = "delete",
        method = "patch",
        tag = "ApiTags::StatusCodes"
    )]
    async fn status(&self, codes: Path<Vec<u16>>) -> StatusRes {
        if codes.len() > 1 {
            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0..codes.len());
            StatusRes::from(codes[idx])
        } else {
            StatusRes::from(codes[0])
        }
    }
}

const TEAPOT_ASCII_ART: &str = r#"
             ;,'
     _o_    ;:;'
 ,-.'---`.__ ;
((j`=====',-'
 `-\     /
    `-=-'     hjw
"#;

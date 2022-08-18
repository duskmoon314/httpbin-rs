use std::net::{IpAddr, Ipv4Addr};

use clap::Parser;
use poem::{listener::TcpListener, middleware, EndpointExt, Route, Server};
use poem_openapi::{OpenApiService, Tags};

pub(crate) mod data;
pub(crate) mod http_methods;
pub(crate) mod request_inspection;
pub(crate) mod status_codes;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The address to listen on.
    #[clap(long, value_parser)]
    address: Option<String>,

    /// The ip to listen on.
    #[clap(long, value_parser, default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    ip: IpAddr,

    /// The port to listen on.
    #[clap(long, value_parser, default_value = "8000")]
    port: u16,
}

#[derive(Tags)]
enum ApiTags {
    /// Testing different HTTP verbs
    #[oai(rename = "HTTP Methods")]
    HttpMethods,

    /// Generates responses with given status code
    #[oai(rename = "Status codes")]
    StatusCodes,

    /// Inspect the request data
    #[oai(rename = "Request Inspection")]
    RequestInspection,

    /// Returns anything that is passed to request
    Anything,

    /// Generates useful data
    Data,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api_service = OpenApiService::new(
        (
            http_methods::Api,
            status_codes::Api,
            request_inspection::Api,
            data::Api,
        ),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
    .description(env!("CARGO_PKG_DESCRIPTION"))
    .license(env!("CARGO_PKG_LICENSE"))
    .server(if args.address.is_some() {
        args.address.unwrap()
    } else {
        format!("http://{}:{}", args.ip, args.port)
    });

    let swagger = api_service.swagger_ui();
    let rapidoc = api_service.rapidoc();
    let redoc = api_service.redoc();
    let spec_json = api_service.spec_endpoint();
    let spec_yaml = api_service.spec_endpoint_yaml();

    Server::new(TcpListener::bind(format!("{}:{}", args.ip, args.port)))
        .run(
            Route::new()
                .nest("/", api_service)
                .nest("/swagger", swagger)
                .nest("/rapidoc", rapidoc)
                .nest("/redoc", redoc)
                .nest("/spec/json", spec_json)
                .nest("/spec/yaml", spec_yaml)
                .with(middleware::Tracing),
        )
        .await
}

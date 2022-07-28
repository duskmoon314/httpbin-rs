use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{OpenApiService, Tags};

pub(crate) mod http_methods;
pub(crate) mod request_inspection;

#[derive(Tags)]
enum ApiTags {
    /// Testing different HTTP verbs
    #[oai(rename = "HTTP Methods")]
    HttpMethods,

    /// Inspect the request data
    #[oai(rename = "Request Inspection")]
    RequestInspection,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api_service = OpenApiService::new(
        (http_methods::Api, request_inspection::Api),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
    .description(env!("CARGO_PKG_DESCRIPTION"))
    .server("http://127.0.0.1:3000/");
    // let ui = api_service.rapidoc();
    let swagger = api_service.swagger_ui();
    let rapidoc = api_service.rapidoc();
    let redoc = api_service.redoc();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(
            Route::new()
                .nest("/", api_service)
                .nest("/swagger", swagger)
                .nest("/rapidoc", rapidoc)
                .nest("/redoc", redoc),
        )
        .await
}

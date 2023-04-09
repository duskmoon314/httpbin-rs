use anyhow::Result;
use httpbin::cli::Cli;
use poem::{listener::TcpListener, middleware, EndpointExt, Route, Server};
use poem_openapi::{ContactObject, ExternalDocumentObject, OpenApiService, ServerObject};

mod data;
mod http_method;
mod request_inspection;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Cli::parse().load_config();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let mut api_service = OpenApiService::new(
        (http_method::Api, request_inspection::Api, data::Api),
        "httpbin-rs",
        env!("CARGO_PKG_VERSION"),
    )
    .description(env!("CARGO_PKG_DESCRIPTION"))
    .license(env!("CARGO_PKG_LICENSE"))
    .contact(
        ContactObject::new()
            .name(cfg.openapi.contact.name)
            .url(cfg.openapi.contact.url)
            .email(cfg.openapi.contact.email),
    )
    .external_document(
        ExternalDocumentObject::new(cfg.openapi.external_document.url)
            .description(cfg.openapi.external_document.description),
    );

    for server in cfg.openapi.servers.values() {
        let server = ServerObject::new(server.url.clone()).description(server.description.clone());
        api_service = api_service.server(server);
    }

    let swagger = api_service.swagger_ui();
    let rapidoc = api_service.rapidoc();
    let redoc = api_service.redoc();
    let openapi_explorer = api_service.openapi_explorer();
    let spec_json = api_service.spec_endpoint();
    let spec_yaml = api_service.spec_endpoint_yaml();

    Ok(Server::new(TcpListener::bind((cfg.ip, cfg.port)))
        .run(
            Route::new()
                .nest("/", api_service)
                .nest("/swagger", swagger)
                .nest("/rapidoc", rapidoc)
                .nest("/redoc", redoc)
                .nest("/openapi-explorer", openapi_explorer)
                .nest("/spec/json", spec_json)
                .nest("/spec/yaml", spec_yaml)
                .with(middleware::Cors::new().allow_origins_fn(|_| true))
                .with(middleware::NormalizePath::new(
                    middleware::TrailingSlash::Trim,
                ))
                .with(middleware::Tracing),
        )
        .await?)
}

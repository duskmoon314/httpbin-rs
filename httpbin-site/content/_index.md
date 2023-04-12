+++
title = "httpbin-rs"
sort_by = "weight"
template = "index.html"
+++

Httpbin-rs provides a series of HTTP request & response services powered by Rust and its HTTP frameworks.

# API docs

Httpbin-rs provides many online API docs via features of [`poem-openapi`](https://crates.io/crates/poem-openapi):

- [swagger](https://httpbin.rs/swagger)
- [rapidoc](https://httpbin.rs/rapidoc)
- [redoc](https://httpbin.rs/redoc)
- [openapi-explorer](https://httpbin.rs/openapi-explorer)

# Implementations

Httpbin-rs is currently implemented by the following HTTP frameworks:

| framework                                               | url                                                |
| :------------------------------------------------------ | :------------------------------------------------- |
| [`poem-openapi`](https://crates.io/crates/poem-openapi) | [httpbin.rs](https://httpbin.rs/get)               |
| [`axum`](https://crates.io/crates/axum)                 | [axum.httpbin.rs](https://axum.httpbin.rs/get)     |
| [`actix-web`](https://crates.io/crates/actix-web)       | [actix.httpbin.rs](https://actix.httpbin.rs/get)   |
| [`salvo`](https://crates.io/crates/salvo)               | [salvo.httpbin.rs](https://salvo.httpbin.rs/get)   |
| [`rocket`](https://crates.io/crates/rocket)             | [rocket.httpbin.rs](https://rocket.httpbin.rs/get) |

# Contribution

Httpbin-rs is an open-source project and welcomes contributions. If you are familiar with any HTTP framework, please feel free to create a PR to improve httpbin-rs.

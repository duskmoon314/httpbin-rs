FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG HTTPBIN_IMPL=httpbin-poem-openapi
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --profile release-min --recipe-path recipe.json
COPY . .
RUN cargo build --profile release-min --bin $HTTPBIN_IMPL

FROM alpine:latest AS runtime
ARG HTTPBIN_IMPL=httpbin-poem-openapi
COPY --from=builder /app/target/release-min/$HTTPBIN_IMPL /usr/local/bin/httpbin
COPY httpbin.toml /etc/httpbin.toml
EXPOSE 8080
ENTRYPOINT httpbin --config /etc/httpbin.toml


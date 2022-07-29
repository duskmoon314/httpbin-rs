FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin httpbin-rs

FROM alpine:latest
ENV HTTPBIN_RS_IP=0.0.0.0
ENV HTTPBIN_RS_PORT=8000
ENV HTTPBIN_RS_ADDRESS=http://$HTTPBIN_RS_IP:$HTTPBIN_RS_PORT
COPY --from=builder /app/target/release/httpbin-rs /usr/local/bin/httpbin-rs
ENTRYPOINT httpbin-rs --address $HTTPBIN_RS_ADDRESS --ip $HTTPBIN_RS_IP --port $HTTPBIN_RS_PORT
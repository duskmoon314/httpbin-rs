FROM ghcr.io/getzola/zola:v0.19.2 as builder
COPY . /app
WORKDIR /app
RUN ["/bin/zola", "build"]

FROM nginx:stable-alpine as site
COPY --from=builder /app/public /usr/share/nginx/html
EXPOSE 8080
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef sqlx-cli
WORKDIR /app


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
ARG DATABASE_URL
ARG APP_ENV
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo sqlx migrate run \
    && cargo build --release --target x86_64-unknown-linux-musl --bin zero2prod


FROM alpine:latest AS runtime
ENV AWS_ACCESS_KEY_ID
ENV AWS_SECRET_ACCESS_KEY
ENV APP_ENVIRONMENT production
WORKDIR /app
RUN addgroup -S jesse \
    && adduser -S jesse -G jesse \
    && apk add --update-cache \
       openssl \
       ca-certificates \
    && rm -rf /var/cache/apk/*
COPY --chown=jesse:jesse --from=builder /app/target/x86_64-unknown-linux-musl/release/zero2prod zero2prod
COPY --chown=jesse:jesse config config
EXPOSE 8080
USER jesse
ENTRYPOINT ["./zero2prod"]

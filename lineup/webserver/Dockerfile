FROM ghcr.io/loopercorp/rust:latest AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM ghcr.io/loopercorp/rust:latest AS builder
COPY --from=planner /home/rust/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin lineup


FROM alpine:latest AS runtime
RUN addgroup -S looper; \
    adduser -S looper -G looper; \
    apk add --update-cache \
    #openssl \
    #ca-certificates; \
    rm -rf /var/cache/apk/*
WORKDIR /home/looper
COPY --chown=looper:looper \
     --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/lineup lineup
COPY --chown=looper:looper \
     --from=builder /home/rust/config.toml config.toml
EXPOSE 8080
USER looper
ENTRYPOINT ["./lineup"]

FROM clux/muslrust:stable AS tool-builder
WORKDIR /tools
ENV TARGET=x86_64-unknown-linux-musl
RUN cargo install sqlx-cli cargo-chef --target $TARGET --root .


FROM rust:alpine AS builder
COPY --from=tool-builder /tools/bin/* $CARGO_HOME/bin
RUN addgroup -S rust; \
    adduser -S rust -G rust; \
    apk add --update-cache \
    musl-dev \
    openssl-dev; \
    rm -rf /var/cache/apk/*
USER rust
WORKDIR /home/rust
CMD ["/bin/ash"]

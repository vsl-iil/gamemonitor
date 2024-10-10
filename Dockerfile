FROM clux/muslrust:latest as builder

WORKDIR /usr/src/app

RUN rustup target add aarch64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

COPY src src
RUN cargo build --target aarch64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-musl/release/gamemonitor /usr/bin/gamemonitor
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

ENTRYPOINT ["gamemonitor", "-a", "2183900", "-t", "1", "-d", "3h", "telegram"]

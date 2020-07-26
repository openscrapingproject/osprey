# syntax=docker/dockerfile:experimental
# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools pkg-config libssl-dev -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY osplib/Cargo.toml osplib/Cargo.toml
COPY osprey-remote-plugins/Cargo.toml osprey-remote-plugins/Cargo.toml
COPY osprey/Cargo.toml osprey/Cargo.toml
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

# TODO: maybe make this a shell script
RUN mkdir -p osplib/src osprey/src osprey-remote-plugins/src
RUN echo "fn main(){}" > osplib/src/lib.rs
RUN echo "fn main(){}" > osprey-remote-plugins/src/lib.rs
RUN echo "fn main(){}" > osprey/src/main.rs
RUN ls *


# cache dependency compilation
# target-feature=-crt-static 
RUN --mount=type=cache,target=/root/.cargo/registry/ RUSTFLAGS="-C linker=musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN ls target/x86_64-unknown-linux-musl/release/
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/*osp*

COPY . .

RUN RUSTFLAGS="-C linker=musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 osprey

RUN adduser -D -s /bin/sh -u 1000 -G osprey osprey

WORKDIR /home/osprey/bin/

COPY --from=cargo-build /app/target/x86_64-unknown-linux-musl/release/osprey .
COPY ./deploy/executor/config.json default-config.json

RUN chown osprey:osprey osprey

USER osprey

CMD ["./osprey"]
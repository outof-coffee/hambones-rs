FROM rustlang/rust:nightly AS builder
RUN USER=root apt-get update && apt-get install -y musl musl-tools
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new hambones-rs
WORKDIR /usr/src/hambones-rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/hambones-rs .
USER 1000
CMD ["./hambones-rs"]

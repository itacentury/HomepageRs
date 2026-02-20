FROM rust:1.86-slim-bookworm AS builder

RUN cargo new --bin site
WORKDIR /site

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo fetch

COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates

RUN cargo build --release

FROM debian:12-slim
WORKDIR /site

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

COPY --from=builder /site/target/release/site ./site
COPY --from=builder /site/static ./static
COPY --from=builder /site/templates ./templates

CMD ["./site"]

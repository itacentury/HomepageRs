FROM alpine:latest AS nav-pkg
RUN apk add --no-cache unzip curl
RUN mkdir /pkg \
    && curl -L https://github.com/itacentury/HomepageRs/releases/download/pkg/pkg.zip \
         -o /tmp/pkg.zip \
    && unzip /tmp/pkg.zip -d /pkg \
    && rm /tmp/pkg.zip

FROM rust:1.86-slim-bookworm AS builder

RUN USER=root cargo new --bin site
WORKDIR /site

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo fetch

COPY ./src ./src
COPY ./static /site/static
COPY ./templates /site/templates

RUN cargo build --release

FROM debian:12-slim
WORKDIR /site

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

COPY --from=builder /site/target/release/site   ./site
COPY --from=builder /site/static                ./static
COPY --from=builder /site/templates             ./templates
COPY  --from=nav-pkg /pkg/*                     ./static/pkg/

CMD ["./site"]
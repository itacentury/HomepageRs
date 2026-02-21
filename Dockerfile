FROM rust:1.93-slim-bookworm AS builder

ARG TARGETPLATFORM

RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc-aarch64-linux-gnu libc6-dev-arm64-cross \
    gcc-x86-64-linux-gnu libc6-dev-amd64-cross \
    && rm -rf /var/lib/apt/lists/*

RUN case "${TARGETPLATFORM}" in \
      "linux/amd64") echo "x86_64-unknown-linux-gnu" > /tmp/target ;; \
      "linux/arm64") echo "aarch64-unknown-linux-gnu" > /tmp/target ;; \
      *) echo "Unsupported platform: ${TARGETPLATFORM}" && exit 1 ;; \
    esac && \
    rustup target add "$(cat /tmp/target)"

RUN cargo new --bin site
WORKDIR /site

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo fetch

COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates

RUN TARGET=$(cat /tmp/target) && \
    case "${TARGET}" in \
      "aarch64-unknown-linux-gnu") \
        export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc ;; \
    esac && \
    cargo build --release --target "${TARGET}" && \
    cp "target/${TARGET}/release/site" /site/site-binary

FROM debian:12-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /site

COPY --from=builder /site/site-binary ./site
COPY --from=builder /site/static ./static
COPY --from=builder /site/templates ./templates

CMD ["./site"]

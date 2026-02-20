# HomepageRs

[![Rust lint, build & test](https://github.com/itacentury/HomepageRs/actions/workflows/rust-lint-build-test.yml/badge.svg)](https://github.com/itacentury/HomepageRs/actions/workflows/rust-lint-build-test.yml)
[![Docker build & push](https://github.com/itacentury/HomepageRs/actions/workflows/docker-build-push.yml/badge.svg)](https://github.com/itacentury/HomepageRs/actions/workflows/docker-build-push.yml)

HomepageRs is a lightweight web project built with [Rocket](https://rocket.rs) in Rust.

## Prerequisites

Before building and running this project, make sure the following tools are installed:

1. **Rust toolchain**

   [Install Rust](https://rustup.rs) via rustup.

2. **Build tools**

   Some platforms (like Linux) require development tools and dependencies.

3. **cargo-watch** (optional)

    For automatically rebuilding and restarting the server on file changes:

    ```bash
    cargo install cargo-watch
    ```

## Build & Run

- To start the server:

    ```bash
    cargo run --release
    ```

    This starts the Rocket server and serves the frontend at <http://localhost:8000>.

- For automatic rebuilds during development:

    ```bash
    cargo watch -s 'cargo run --release'
    ```

## Notes

- Repository preview images have been generated with [Github Social Image Generator](https://www.bannerbear.com/demos/github-social-preview-generator-tool/)

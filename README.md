# HomepageRs

[![Rust CI](https://github.com/itacentury/HomepageRs/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/itacentury/HomepageRs/actions/workflows/rust-ci.yml)
[![Docker Image](https://github.com/itacentury/HomepageRs/actions/workflows/docker-image.yml/badge.svg)](https://github.com/itacentury/HomepageRs/actions/workflows/docker-image.yml)

HomepageRs is a lightweight web project built with [Rocket](https://rocket.rs) in Rust.

## Prerequisites

Before building and running this project, make sure the following tools are installed:

1. **Rust toolchain**

   [Install Rust](https://rustup.rs) via rustup.

2. **Build tools**

   Some platforms (like Linux) require development tools and dependencies.

3. **wasm-pack**

   Used to compile the WebAssembly module:

   ```bash
   cargo install wasm-pack
    ```

4. **cargo-watch**

    For automatically rebuilding and restarting the server on file changes:

    ```bash
    cargo install cargo-watch
    ```

## Build & Run

1. Build the WebAssembly

    Navigate to the nav directory and compile it for the web:

    ```bash
    wasm-pack build --target web --out-dir ../site/static/pkg
    ```

    This outputs the compiled WebAssembly and bindings to the site/static/pkg folder, where the Rocket site can serve it.

2. Run the Rocket site

    - To manually start the backend server, navigate to the `site` directory and run:

        ```bash
        cargo run --release
        ```

        This starts the Rocket server and serves the frontend at <http://localhost:8000>.

    - For automatic rebuilds during development, run the following command from the root of the project:

        ```bash
        cargo watch -w site -s 'cd nav && wasm-pack build --target web --out-dir ../site/static/pkg && cd ../site && cargo run --release'
        ```

        This watches changes in the site directory, rebuilds WebAssembly, and restarts the Rocket server automatically.

## Notes

- Be sure to re-run wasm-pack build whenever you update the nav code.

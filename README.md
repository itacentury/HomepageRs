# HomepageRs

[![Lint](https://github.com/itacentury/HomepageRs/actions/workflows/lint.yml/badge.svg)](https://github.com/itacentury/HomepageRs/actions/workflows/lint.yml)
[![Docker](https://github.com/itacentury/HomepageRs/actions/workflows/docker.yml/badge.svg)](https://github.com/itacentury/HomepageRs/actions/workflows/docker.yml)

HomepageRs is a lightweight personal portfolio website built with [Actix-web](https://actix.rs) in Rust. The projects section dynamically displays pinned GitHub repositories via the GitHub GraphQL API.

## Prerequisites

Before building and running this project, make sure the following tools are installed:

1. **Rust toolchain**

   [Install Rust](https://rustup.rs) via rustup.

2. **bacon** (optional)

   For automatically rebuilding and restarting the server on file changes:

   ```bash
   cargo install bacon
   ```

## Configuration

The projects section fetches pinned repositories from GitHub. Create a `.env` file in the project root:

```bash
GITHUB_USERNAME=yourGitHubUsername
GITHUB_TOKEN=ghp_yourTokenHere
```

| Variable          | Required | Description                                                                                        |
| ----------------- | -------- | -------------------------------------------------------------------------------------------------- |
| `GITHUB_USERNAME` | **Yes**  | GitHub username whose pinned repos are displayed                                                   |
| `GITHUB_TOKEN`    | No       | Personal access token ([create one](https://github.com/settings/tokens), no special scopes needed) |
| `CACHE_TTL_SECS`  | No       | Cache lifetime in seconds (default: `300`)                                                         |

If no token is set, the site falls back to a "Visit my GitHub" link.

## Build & Run

- To start the server:

  ```bash
  cargo run --release
  ```

  This starts the Actix-web server and serves the frontend at <http://localhost:8000>.

- For automatic rebuilds during development:

  ```bash
  bacon run-long
  ```

## Docker Compose

A sample [`docker-compose.yml`](docker-compose.yml) is included in the project root. To run:

1. Make sure your `.env` file with the `GITHUB_TOKEN` exists in the project root (see [Configuration](#configuration)).

2. Start the container:

   ```bash
   docker compose up -d
   ```

   The site will be available at <http://localhost:8000>.

3. To stop the container:

   ```bash
   docker compose down
   ```

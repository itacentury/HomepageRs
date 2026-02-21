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

All portfolio content is configured through a single YAML file. Copy the example config and customize it:

```bash
cp config.example.yaml config.yaml
```

Edit `config.yaml` with your personal information, education, experience, and links. See [`config.example.yaml`](config.example.yaml) for the full schema.

### GitHub integration

The projects section fetches pinned repositories from GitHub. Set your GitHub token either in `config.yaml` or via environment variable:

```yaml
github:
  username: "yourGitHubUsername"
  token: "ghp_yourTokenHere" # or set GITHUB_TOKEN env var
  # cache_ttl_secs: 300         # optional, default 300
```

If no token is set, the site falls back to a "Visit my GitHub" link.

### Environment variable overrides

Environment variables take precedence over YAML values. You can also use a `.env` file:

| Variable          | Description                                  |
| ----------------- | -------------------------------------------- |
| `CONFIG_PATH`     | Path to config file (default: `config.yaml`) |
| `GITHUB_TOKEN`    | Overrides `github.token` from YAML           |
| `GITHUB_USERNAME` | Overrides `github.username` from YAML        |
| `CACHE_TTL_SECS`  | Overrides `github.cache_ttl_secs` from YAML  |

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

1. Make sure your `config.yaml` exists in the project root (see [Configuration](#configuration)).

2. Start the container:

   ```bash
   docker compose up -d
   ```

   The site will be available at <http://localhost:8000>.

3. To stop the container:

   ```bash
   docker compose down
   ```

The Docker image ships with `config.example.yaml` as a default config. Mount your own `config.yaml` via the volume in `docker-compose.yml` and set `GITHUB_TOKEN` via environment variable.

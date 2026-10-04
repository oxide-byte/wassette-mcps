# postgres-wasm-mcp

A [Wassette](https://github.com/microsoft/wassette) WebAssembly component that exposes a Postgres
database as an MCP tool. Wassette only permits outbound HTTP, so the component talks to Postgres
through [PostgREST](https://postgrest.org) (REST over HTTP) instead of the Postgres wire protocol.

```
Wassette ── wasi:http ──▶ PostgREST (:3000) ──▶ Postgres (:5432)
```

## Tools

| Tool | Description |
|---|---|
| `fetch-user-from-db(user-id: s32) -> result<string, string>` | Returns the `users` row with the given id as JSON, e.g. `{"id":1,"name":"Alice","email":"alice@example.com"}`. Unknown ids return the error `User 99 not found`. |

The interface is defined in [`wit/world.wit`](wit/world.wit) (world `db-tools`).

## Configuration

| Environment variable | Default | Meaning |
|---|---|---|
| `POSTGREST_URL` | `http://127.0.0.1:3000` | PostgREST base URL |

## Project layout

| File | Purpose |
|---|---|
| `src/lib.rs` | Component implementation |
| `wit/world.wit` | Tool interface |
| `policy.yaml` | Wassette permission policy |
| `docker-compose.yml`, `dockerfile`, `init.sql` | Local Postgres + PostgREST backend |

## Quick start

Prerequisites: Rust with the `wasm32-wasip2` target, Docker, and Wassette.

### 1. Start the backend

`docker-compose.yml` starts Postgres 16 (seeded from `init.sql` with a `users` table: Alice, Bob,
Charlie) and PostgREST, which serves the `public` schema as the read-only `web_anon` role.

```bash
docker compose up -d --build
curl "127.0.0.1:3000/users?id=eq.1" -H "Accept: application/vnd.pgrst.object+json"
```

`init.sql` only runs on a fresh container. After changing it, run `docker compose down` before `up`.
The `postgres/postgres` credentials are throwaway defaults for local development only.

### 2. Build the component

```bash
rustup target add wasm32-wasip2
cargo build --release   # -> ../target/wasm32-wasip2/release/postgres_wasm_mcp.wasm
```

### 3. Use it with Wassette

```bash
wassette component load file://$PWD/../target/wasm32-wasip2/release/postgres_wasm_mcp.wasm
wassette permission grant network postgres_wasm_mcp 127.0.0.1:3000
wassette tool invoke fetch-user-from-db --args '{"user-id":1}'
```

`policy.yaml` allows the PostgREST `host:port` and the `POSTGREST_URL` environment variable. If
PostgREST runs elsewhere, update the host and set the URL:

```bash
wassette secret set postgres_wasm_mcp POSTGREST_URL=http://<host>:<port>
```

## Notes

- PostgREST is exposed without authentication, and `web_anon` can read `users`. For anything beyond
  local use, add JWT auth (`PGRST_JWT_SECRET`) and put TLS in front of it.
- A raw Postgres client (`tokio-postgres` over `wasi:sockets`) builds but is denied by Wassette 0.8.0
  (`Permission denied`), which is why this component uses HTTP.

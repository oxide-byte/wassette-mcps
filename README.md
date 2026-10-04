# wassette-mcps

A Cargo workspace of [Wassette](https://github.com/microsoft/wassette) WebAssembly components that
expose external systems as sandboxed MCP tools. Each component ships a `policy.yaml` that grants only
the network hosts and environment variables it needs.

| Component | Tool | Backend |
|---|---|---|
| [`jira-wasm-mcp`](jira-wasm-mcp/README.md) | `get-issue` | Jira REST API |
| [`postgres-wasm-mcp`](postgres-wasm-mcp/README.md) | `fetch-user-from-db` | Postgres via PostgREST |

## Build

Prerequisites: Rust with the `wasm32-wasip2` target.

```bash
rustup target add wasm32-wasip2
cargo build --release   # -> target/wasm32-wasip2/release/<crate_name>.wasm
```

## Use with Wassette

```bash
wassette component load file://$PWD/target/wasm32-wasip2/release/<crate_name>.wasm
```

See each component's README for its configuration, permissions and setup.

## License

See [LICENSE](LICENSE).

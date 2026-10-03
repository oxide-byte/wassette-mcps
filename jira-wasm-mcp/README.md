# jira-wasm-mcp

A [Wassette](https://github.com/microsoft/wassette) WebAssembly component that exposes Jira as an MCP tool.
Port of the native `jira-mcp` server.

## Tools

- `get-issue(issue-key)` – returns the raw JSON of `GET /rest/api/3/issue/{key}`.

## Configuration (environment variables)

| Variable | Meaning |
|---|---|
| `JIRA_URL` | Jira base URL (default `http://localhost:8080`) |
| `JIRA_USERNAME` | Username / email |
| `JIRA_PASSWORD` | Password → Basic auth |
| `JIRA_API_KEY` | API token → Bearer auth (takes precedence over password) |

## Build

```bash
rustup target add wasm32-wasip2
cargo build --release   # -> ../target/wasm32-wasip2/release/jira_wasm_mcp.wasm
```

## Use with Wassette

Edit `policy.yaml` so the network host matches your Jira, then:

```bash
wassette component load file://$PWD/../target/wasm32-wasip2/release/jira_wasm_mcp.wasm
wassette policy get jira_wasm_mcp   # inspect
# wassette permission grant network <component-id> your-domain.atlassian.net
wassette permission grant network jira_wasm_mcp https://bindstone.atlassian.net
```

Secrets are supplied through Wassette's environment-variable configuration for the component
(`wassette secret set <component-id> JIRA_API_KEY=...`).

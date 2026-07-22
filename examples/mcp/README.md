# mcp example

A2A server backed by an LLM agent that reaches tools on external **MCP (Model
Context Protocol)** servers over the Streamable HTTP transport. Instead of
registering every discovered MCP tool into the LLM context, the agent exposes
just two *selector* tools regardless of how many tools the servers publish:

- `mcp_list_tools` - list the discovered catalog (server, name, description,
  input schema), optionally filtered by a `search` substring.
- `mcp_call_tool` - invoke a tool by `name` (optionally disambiguated by
  `server`) with an `arguments` object.

## What's in the box

```
mcp/
├── server/main.rs                 Agent wired to McpClient behind MCP_ENABLE
├── server/.well-known/agent.json  Agent metadata loaded at startup
├── client/main.rs                 Two-prompt demo via message/send + poll
└── README.md
```

## What this shows

- **`McpClient::from_config(&McpConfig)`** - returns `None` when `MCP_ENABLE`
  is false or `MCP_SERVERS` is empty, so the selector tools are registered
  only when MCP is on.
- **`McpClient::start()`** - spawns background discovery + refresh. Initial
  connects honour `MCP_MAX_RETRIES` with exponential backoff; once connected,
  refresh failures back off and retry forever while keeping the last good
  catalog, so one failing server never drops the healthy ones.
- **`AgentBuilder::with_mcp_client(client)`** - appends the two selector-tool
  definitions to the toolbox and wires their handlers.

## Configuration (`MCP_*`)

| Var | Default | Purpose |
| --- | --- | --- |
| `MCP_ENABLE` | `false` | Enable the MCP client |
| `MCP_SERVERS` | - | Comma-separated MCP server base URLs |
| `MCP_ENDPOINT` | `/mcp` | Path appended to each server URL |
| `MCP_REFRESH_INTERVAL` | `5m` | Tool-catalog refresh interval |
| `MCP_DIAL_TIMEOUT` | `30s` | Init / list-tools timeout |
| `MCP_CALL_TIMEOUT` | `30s` | Single tool-invocation timeout |
| `MCP_MAX_RETRIES` | `0` | Max initial connection attempts (0 = retry forever) |
| `MCP_RETRY_INTERVAL` | `2s` | Initial backoff (doubles) |
| `MCP_RETRY_MAX_INTERVAL` | `30s` | Max backoff |

## Running locally

```bash
# Start an Inference Gateway and one or more MCP servers separately, then run
# the server from inside its subdir so .well-known/agent.json resolves:
cd examples/mcp/server
export MCP_ENABLE=true
export MCP_SERVERS=http://localhost:3000   # your MCP server base URL(s)
cargo run -p mcp-server
# or: task examples:mcp-server

cargo run -p mcp-client
# or: task examples:mcp-client
```

The server listens on `0.0.0.0:8086`. The client honours `SERVER_URL`
(default `http://localhost:8086`). With `MCP_ENABLE` unset the server still
runs - it just has no MCP tools to offer.

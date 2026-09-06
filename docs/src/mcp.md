# Connect with MCP

`pred mcp` exposes catalog queries, instance creation, reductions, and solving over a local stdio MCP server.

## Install with MCP support

```bash
cargo install problemreductions-cli --features mcp
pred mcp --help
```

For the current checkout, use `cargo install --path problemreductions-cli --features mcp`. MCP is an optional feature and is not included in the default CLI install.

## Configure your client

Register a local stdio server with executable `pred` and argument `mcp`. Clients using an `mcpServers` configuration accept this shape:

```json
{
  "mcpServers": {
    "problemreductions": {
      "command": "pred",
      "args": ["mcp"]
    }
  }
}
```

Ensure the client can resolve `pred` on its `PATH`, or supply the executable's absolute path. Reload the client and request the server's tool list to verify the connection.

Next: [tool reference](mcp-tools.md) or [example session](mcp-walkthrough.md).

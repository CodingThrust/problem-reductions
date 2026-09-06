# Install the CLI

The `pred` executable explores models and reductions, creates instances, and runs solvers.

## Install a release

```bash
cargo install problemreductions-cli
pred --version
```

Rust and a native build toolchain are required. The default solver backend is HiGHS.

## Build the current repository

```bash
git clone https://github.com/CodingThrust/problem-reductions
cd problem-reductions
cargo install --path problemreductions-cli
pred --version
```

The website catalog is built from the repository. A published crate may have fewer models or rules; use the source build when reproducing current catalog entries.

## Optional features

For the MCP server, install with `--features mcp`. The CLI also exposes `cplex` and `lp-solvers` for separately installed solver backends; consult the [CLI manifest](https://github.com/CodingThrust/problem-reductions/blob/main/problemreductions-cli/Cargo.toml) before configuring one.

Next: [first solve](cli.md) or [connect with MCP](mcp.md).

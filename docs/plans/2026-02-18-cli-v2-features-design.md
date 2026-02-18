# CLI v2 Features Design

Tracking issue: #81

## Overview

Three features for the `pred` CLI tool, building on the v1 foundation:

1. **k-Neighbor Exploration** — discover nearby problems in the reduction graph
2. **Colored Output** — terminal colors and aligned formatting
3. **Shell Completions** — bash/zsh/fish auto-completion

## Feature 1: k-Neighbor Exploration

### UX

Extend the existing `pred show` command with `--hops` and `--direction` flags:

```
pred show MIS --hops 2                     # 2-hop outgoing tree (default)
pred show MIS --hops 2 --direction in      # 2-hop incoming tree
pred show MIS --hops 2 --direction both    # both directions
pred show MIS --hops 3 -o neighbors.json   # JSON output
```

When `--hops` is not specified, `pred show` behaves exactly as v1 (no tree, just 1-hop lists).

### Output Format — Tree View

```
MaximumIndependentSet — 2-hop neighbors (outgoing)

MaximumIndependentSet
├── QUBO
│   ├── SpinGlass
│   └── ILP
├── MinimumVertexCover
│   ├── ILP
│   └── MinimumSetCovering
└── MaxCut
    └── SpinGlass

6 reachable problems in 2 hops
```

Tree characters: `├──`, `└──`, `│   `, `    ` (4-char indentation per level).

Deduplicated: if a problem appears at multiple hop distances, show it at the shortest distance only. Mention the total unique count at the bottom.

### JSON Output

When `-o` is specified:

```json
{
  "source": "MaximumIndependentSet",
  "hops": 2,
  "direction": "out",
  "neighbors": [
    {"name": "QUBO", "variant": {}, "hops": 1},
    {"name": "SpinGlass", "variant": {}, "hops": 2},
    {"name": "ILP", "variant": {}, "hops": 2},
    {"name": "MinimumVertexCover", "variant": {"graph": "SimpleGraph", "weight": "One"}, "hops": 1},
    {"name": "MinimumSetCovering", "variant": {}, "hops": 2},
    {"name": "MaxCut", "variant": {}, "hops": 1}
  ]
}
```

### Library Change

Add to `ReductionGraph` in `src/rules/graph.rs`:

```rust
pub struct NeighborInfo {
    pub name: &'static str,
    pub variant: BTreeMap<String, String>,
    pub hops: usize,
}

pub enum TraversalDirection {
    Outgoing,
    Incoming,
    Both,
}

pub fn k_neighbors(
    &self,
    name: &str,
    variant: &BTreeMap<String, String>,
    max_hops: usize,
    direction: TraversalDirection,
) -> Vec<NeighborInfo>
```

Implementation: BFS on the petgraph `DiGraph`, following edges in the specified direction. Track visited nodes to avoid cycles. Return nodes sorted by (hops, name).

### CLI Changes

In `cli.rs`, add to `Commands::Show`:

```rust
/// Explore k-hop neighbors in the reduction graph
#[arg(long)]
hops: Option<usize>,

/// Direction for neighbor exploration: out, in, both [default: out]
#[arg(long, default_value = "out")]
direction: String,
```

In `commands/graph.rs`, when `hops` is `Some(k)`, call the new library method and render as a tree instead of the existing show output.

## Feature 2: Colored Output

### Dependency

Add `owo-colors` to `problemreductions-cli/Cargo.toml`:

```toml
owo-colors = { version = "4", features = ["supports-colors"] }
```

### Color Scheme

| Element | Style | Example |
|---------|-------|---------|
| Problem names | Bold | **MaximumIndependentSet** |
| Section headers | Cyan | Variants (4): |
| Outgoing arrows `→` | Green | → QUBO |
| Incoming arrows `←` | Red | ← Satisfiability |
| Hop distance | Yellow | (2 hops) |
| Tree branches | Dim | `├──` `└──` `│` |
| Aliases | Dim | (MIS, mis) |
| Error messages | Red bold | Error: unknown problem |

### Color Respect

- Detect terminal capability via `owo-colors`'s `supports-color` feature
- Respect `NO_COLOR` environment variable (https://no-color.org/)
- JSON output (`-o`) is never colored
- Piped output (non-TTY stdout) disables colors automatically

### Aligned Columns for `pred list`

Hand-format with `format!("{:<width$}")`:

```
Registered problems: 20 types, 35 reductions, 42 variant nodes

  Problem                    Aliases    Variants  Reduces to
  ─────────────────────────  ─────────  ────────  ──────────
  BicliqueCover                              1          2
  MaximumIndependentSet      MIS             4          5
  QUBO                                       1          0
```

Column widths computed dynamically from the longest problem name.

## Feature 3: Shell Completions

### Dependency

Add `clap_complete` to `problemreductions-cli/Cargo.toml`:

```toml
clap_complete = "4"
```

### UX

New subcommand:

```
pred completions bash    # output bash completions to stdout
pred completions zsh     # output zsh completions to stdout
pred completions fish    # output fish completions to stdout
```

Users pipe to their shell config:

```bash
pred completions bash > ~/.local/share/bash-completion/completions/pred
pred completions zsh > ~/.zfunc/_pred
pred completions fish > ~/.config/fish/completions/pred.fish
```

### CLI Changes

In `cli.rs`:

```rust
/// Generate shell completions
Completions {
    /// Shell type: bash, zsh, fish
    shell: clap_complete::Shell,
},
```

In `main.rs`, the handler calls `clap_complete::generate()` with the Cli command factory, writing to stdout.

## Dependencies Summary

| Crate | Version | Purpose | Transitive deps |
|-------|---------|---------|----------------|
| `owo-colors` | 4.x | Terminal colors | 1-2 (tiny) |
| `clap_complete` | 4.x | Shell completions | 0 (clap already present) |

## Implementation Order

1. **Shell completions** — smallest, self-contained, quick win
2. **Colored output** — add `owo-colors`, apply colors across all existing commands
3. **k-neighbor exploration** — largest: library method + CLI flag + tree renderer

Each feature is independently shippable and testable.

## Testing

- **Shell completions:** integration test that runs `pred completions bash` and checks output contains expected completion markers
- **Colored output:** unit tests for the formatting functions (test the text content, not ANSI codes); manual visual verification
- **k-neighbors:** unit test for `ReductionGraph::k_neighbors` with known graph topology; integration test for `pred show MIS --hops 2` output structure

## Out of Scope

- Interactive REPL mode (deferred to v3+)
- Table library dependency (hand-formatted alignment is sufficient)
- Dynamic problem-name completion (would require runtime invocation; deferred)

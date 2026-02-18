# CLI v2 Features Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add shell completions, colored terminal output, and k-neighbor graph exploration to the `pred` CLI tool.

**Architecture:** Three independent features built on the existing `problemreductions-cli` crate. Shell completions and colored output are CLI-only changes. k-neighbor exploration requires a new BFS method in the core library's `ReductionGraph` plus a tree renderer in the CLI.

**Tech Stack:** `clap_complete 4` (shell completions), `owo-colors 4` with `supports-colors` feature (terminal colors), `petgraph` BFS (already a transitive dep via `ReductionGraph`)

---

### Task 1: Add shell completions — dependencies and CLI enum

**Files:**
- Modify: `problemreductions-cli/Cargo.toml`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Add `clap_complete` dependency**

In `problemreductions-cli/Cargo.toml`, add to `[dependencies]`:

```toml
clap_complete = "4"
```

**Step 2: Add `Completions` variant to `Commands` enum**

In `problemreductions-cli/src/cli.rs`, add a new variant to the `Commands` enum (after `Solve`):

```rust
/// Generate shell completions for bash, zsh, fish, etc.
#[command(after_help = "\
Examples:
  pred completions bash > ~/.local/share/bash-completion/completions/pred
  pred completions zsh > ~/.zfunc/_pred
  pred completions fish > ~/.config/fish/completions/pred.fish")]
Completions {
    /// Shell type
    shell: clap_complete::Shell,
},
```

**Step 3: Wire up the handler in `main.rs`**

In `problemreductions-cli/src/main.rs`, add the match arm:

```rust
Commands::Completions { shell } => {
    let mut cmd = Cli::command();
    clap_complete::generate(shell, &mut cmd, "pred", &mut std::io::stdout());
    Ok(())
}
```

Also add `use clap::CommandFactory;` at the top of main.rs (needed for `.command()`).

**Step 4: Build and verify**

Run: `cargo build -p problemreductions-cli`
Expected: Compiles successfully.

Run: `cargo run -p problemreductions-cli -- completions bash | head -5`
Expected: Outputs bash completion script starting with `_pred()` or similar.

**Step 5: Commit**

```
feat(cli): add shell completions command (bash/zsh/fish)
```

---

### Task 2: Add shell completions — integration test

**Files:**
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write the test**

Add to `problemreductions-cli/tests/cli_tests.rs`:

```rust
#[test]
fn test_completions_bash() {
    let output = pred().args(["completions", "bash"]).output().unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Bash completions should reference the binary name
    assert!(stdout.contains("pred"), "completions should reference 'pred'");
}

#[test]
fn test_completions_zsh() {
    let output = pred().args(["completions", "zsh"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("pred"));
}

#[test]
fn test_completions_fish() {
    let output = pred().args(["completions", "fish"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("pred"));
}
```

**Step 2: Run tests**

Run: `cargo test -p problemreductions-cli test_completions`
Expected: All 3 tests pass.

**Step 3: Commit**

```
test(cli): add integration tests for shell completions
```

---

### Task 3: Add colored output — dependency and color helper module

**Files:**
- Modify: `problemreductions-cli/Cargo.toml`
- Modify: `problemreductions-cli/src/output.rs`

**Step 1: Add `owo-colors` dependency**

In `problemreductions-cli/Cargo.toml`, add to `[dependencies]`:

```toml
owo-colors = { version = "4", features = ["supports-colors"] }
```

**Step 2: Add color helper functions to `output.rs`**

In `problemreductions-cli/src/output.rs`, add color formatting helpers after the existing `OutputConfig` impl:

```rust
use owo_colors::OwoColorize;
use std::io::IsTerminal;

/// Whether colored output should be used (TTY + not NO_COLOR).
pub fn use_color() -> bool {
    std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none()
}

/// Format a problem name (bold when color is enabled).
pub fn fmt_problem_name(name: &str) -> String {
    if use_color() {
        format!("{}", name.bold())
    } else {
        name.to_string()
    }
}

/// Format a section header (cyan when color is enabled).
pub fn fmt_section(text: &str) -> String {
    if use_color() {
        format!("{}", text.cyan())
    } else {
        text.to_string()
    }
}

/// Format an outgoing arrow (green when color is enabled).
pub fn fmt_arrow_out() -> &'static str {
    // We return static str, so we use ANSI directly for the arrow
    "→"
}

pub fn fmt_outgoing(text: &str) -> String {
    if use_color() {
        format!("{}", text.green())
    } else {
        text.to_string()
    }
}

pub fn fmt_incoming(text: &str) -> String {
    if use_color() {
        format!("{}", text.red())
    } else {
        text.to_string()
    }
}

/// Format dim text (for aliases, tree branches).
pub fn fmt_dim(text: &str) -> String {
    if use_color() {
        format!("{}", text.dimmed())
    } else {
        text.to_string()
    }
}
```

**Step 3: Build**

Run: `cargo build -p problemreductions-cli`
Expected: Compiles.

**Step 4: Commit**

```
feat(cli): add owo-colors dependency and color helper functions
```

---

### Task 4: Apply colors to `pred list` with aligned columns

**Files:**
- Modify: `problemreductions-cli/src/commands/graph.rs`

**Step 1: Rewrite `list()` to use aligned columns and colors**

Replace the body of `pub fn list(out: &OutputConfig)` in `problemreductions-cli/src/commands/graph.rs`. The new version:
- Computes column widths dynamically
- Shows a header row with separator line
- Shows variant count and outgoing reduction count per problem
- Uses `fmt_problem_name`, `fmt_section`, `fmt_dim` from `output.rs`

```rust
pub fn list(out: &OutputConfig) -> Result<()> {
    let graph = ReductionGraph::new();

    let mut types = graph.problem_types();
    types.sort();

    // Collect data for each problem
    struct Row {
        name: String,
        aliases: Vec<&'static str>,
        num_variants: usize,
        num_reduces_to: usize,
    }
    let rows: Vec<Row> = types
        .iter()
        .map(|name| {
            let aliases = aliases_for(name);
            let num_variants = graph.variants_for(name).len();
            let num_reduces_to = graph.outgoing_reductions(name).len();
            Row {
                name: name.to_string(),
                aliases,
                num_variants,
                num_reduces_to,
            }
        })
        .collect();

    // Compute column widths
    let name_width = rows.iter().map(|r| r.name.len()).max().unwrap_or(7).max(7);
    let alias_width = rows
        .iter()
        .map(|r| {
            if r.aliases.is_empty() {
                0
            } else {
                r.aliases.join(", ").len()
            }
        })
        .max()
        .unwrap_or(7)
        .max(7);

    let summary = format!(
        "Registered problems: {} types, {} reductions, {} variant nodes\n",
        graph.num_types(),
        graph.num_reductions(),
        graph.num_variant_nodes(),
    );

    let mut text = String::new();
    text.push_str(&crate::output::fmt_section(&summary));
    text.push_str(&format!(
        "\n  {:<name_w$}  {:<alias_w$}  {:>8}  {:>10}\n",
        "Problem",
        "Aliases",
        "Variants",
        "Reduces to",
        name_w = name_width,
        alias_w = alias_width,
    ));
    text.push_str(&format!(
        "  {:<name_w$}  {:<alias_w$}  {:>8}  {:>10}\n",
        "─".repeat(name_width),
        "─".repeat(alias_width),
        "────────",
        "──────────",
        name_w = name_width,
        alias_w = alias_width,
    ));

    for row in &rows {
        let alias_str = if row.aliases.is_empty() {
            String::new()
        } else {
            row.aliases.join(", ")
        };
        text.push_str(&format!(
            "  {:<name_w$}  {:<alias_w$}  {:>8}  {:>10}\n",
            crate::output::fmt_problem_name(&row.name),
            crate::output::fmt_dim(&alias_str),
            row.num_variants,
            row.num_reduces_to,
            name_w = name_width,
            alias_w = alias_width,
        ));
    }

    text.push_str(&format!(
        "\nUse `pred show <problem>` to see variants, reductions, and fields.\n"
    ));

    let json = serde_json::json!({
        "num_types": graph.num_types(),
        "num_reductions": graph.num_reductions(),
        "num_variant_nodes": graph.num_variant_nodes(),
        "problems": rows.iter().map(|r| {
            serde_json::json!({
                "name": r.name,
                "aliases": r.aliases,
                "num_variants": r.num_variants,
                "num_reduces_to": r.num_reduces_to,
            })
        }).collect::<Vec<_>>(),
    });

    out.emit_with_default_name("pred_graph_list.json", &text, &json)
}
```

Note: When colors are enabled, ANSI escape codes in `fmt_problem_name` will make the string longer than the visible width. To handle this correctly, compute the padding based on the raw name length, not the colored string length. The approach above passes `name_w` based on the raw `name_width`, and `format!("{:<name_w$}")` will pad based on byte length. Since ANSI codes add invisible bytes, the colored name will already be wider — so we need to adjust. A simpler approach: format the padding with the raw name, then replace the raw name with the colored version. Or: only apply color after padding.

**Refined approach for color+padding:** Format the plain-text column first, then apply color to the name portion only:

```rust
let padded_name = format!("{:<name_w$}", row.name, name_w = name_width);
let colored_name = crate::output::fmt_problem_name(&padded_name);
```

This works because `fmt_problem_name` wraps the already-padded string in bold ANSI codes.

**Step 2: Build and run**

Run: `cargo run -p problemreductions-cli -- list`
Expected: Aligned columns with bold problem names (if terminal supports color).

**Step 3: Commit**

```
feat(cli): add aligned columns and color to pred list
```

---

### Task 5: Apply colors to `pred show`

**Files:**
- Modify: `problemreductions-cli/src/commands/graph.rs`

**Step 1: Add colors to `show()` function**

In the `show()` function in `problemreductions-cli/src/commands/graph.rs`, apply color helpers:
- Problem name line: `fmt_problem_name`
- Section headers ("Variants", "Fields", "Size fields", "Reduces to", "Reduces from"): `fmt_section`
- Outgoing reduction lines: `fmt_outgoing` on the arrow/target
- Incoming reduction lines: `fmt_incoming` on the arrow/source
- Alias list: `fmt_dim`

The changes are straightforward string formatting replacements. For example:

```rust
// Before:
let mut text = format!("{}\n", spec.name);
// After:
let mut text = format!("{}\n", crate::output::fmt_problem_name(&spec.name));

// Before:
text.push_str(&format!("\nVariants ({}):\n", variants.len()));
// After:
text.push_str(&format!("\n{}\n", crate::output::fmt_section(&format!("Variants ({}):", variants.len()))));

// Before (outgoing):
text.push_str(&format!(
    "  {} {} -> {} {}\n",
    e.source_name, ..., e.target_name, ...
));
// After:
text.push_str(&format!(
    "  {} {} {} {} {}\n",
    e.source_name,
    format_variant(&e.source_variant),
    crate::output::fmt_outgoing("→"),
    crate::output::fmt_problem_name(e.target_name),
    format_variant(&e.target_variant),
));
```

Similar pattern for incoming reductions using `fmt_incoming`.

**Step 2: Build and verify**

Run: `cargo run -p problemreductions-cli -- show MIS`
Expected: Colored output with bold name, cyan headers, green outgoing arrows, red incoming arrows.

**Step 3: Run existing tests**

Run: `cargo test -p problemreductions-cli`
Expected: All existing tests still pass. (Tests check for content like "MaximumIndependentSet" and "Reduces to" — these strings are still present, possibly wrapped in ANSI codes. Since tests run in a non-TTY pipe, `use_color()` returns false and no ANSI codes are emitted.)

**Step 4: Commit**

```
feat(cli): add colors to pred show output
```

---

### Task 6: Apply colors to `pred path`

**Files:**
- Modify: `problemreductions-cli/src/commands/graph.rs`

**Step 1: Color the `format_path_text` function**

In `format_path_text()`:
- Step labels ("Step 1:", "Step 2:"): `fmt_section`
- Problem names in steps: `fmt_problem_name`
- Arrow `→`: `fmt_outgoing`

```rust
fn format_path_text(
    graph: &ReductionGraph,
    reduction_path: &problemreductions::rules::ReductionPath,
) -> String {
    let mut text = format!(
        "Path ({} steps): {}\n",
        reduction_path.len(),
        reduction_path
    );

    let overheads = graph.path_overheads(reduction_path);
    let steps = &reduction_path.steps;
    for i in 0..steps.len().saturating_sub(1) {
        let from = &steps[i];
        let to = &steps[i + 1];
        text.push_str(&format!(
            "\n  {}: {} {} {}\n",
            crate::output::fmt_section(&format!("Step {}", i + 1)),
            crate::output::fmt_problem_name(&from.to_string()),
            crate::output::fmt_outgoing("→"),
            crate::output::fmt_problem_name(&to.to_string()),
        ));
        let oh = &overheads[i];
        for (field, poly) in &oh.output_size {
            text.push_str(&format!("    {field} = {poly}\n"));
        }
    }

    text
}
```

**Step 2: Run existing path tests**

Run: `cargo test -p problemreductions-cli test_path`
Expected: All path tests pass (non-TTY = no ANSI codes).

**Step 3: Commit**

```
feat(cli): add colors to pred path output
```

---

### Task 7: k-neighbor BFS — library method with tests

**Files:**
- Modify: `src/rules/graph.rs`
- Modify: `src/unit_tests/reduction_graph.rs`

**Step 1: Add types and method to `ReductionGraph`**

In `src/rules/graph.rs`, add the public types before the `ReductionGraph` struct:

```rust
/// Information about a neighbor in the reduction graph.
#[derive(Debug, Clone)]
pub struct NeighborInfo {
    /// Problem name.
    pub name: &'static str,
    /// Variant attributes.
    pub variant: BTreeMap<String, String>,
    /// Hop distance from the source.
    pub hops: usize,
}

/// Direction for graph traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalDirection {
    /// Follow outgoing edges (what can this reduce to?).
    Outgoing,
    /// Follow incoming edges (what can reduce to this?).
    Incoming,
    /// Follow edges in both directions.
    Both,
}
```

Add the BFS method to the `impl ReductionGraph` block (after `incoming_reductions`):

```rust
/// Find all problems reachable within `max_hops` edges from a starting node.
///
/// Returns neighbors sorted by (hops, name). The starting node itself is excluded.
/// If a node is reachable at multiple distances, it appears at the shortest distance only.
pub fn k_neighbors(
    &self,
    name: &str,
    variant: &BTreeMap<String, String>,
    max_hops: usize,
    direction: TraversalDirection,
) -> Vec<NeighborInfo> {
    use petgraph::Direction;
    use std::collections::VecDeque;

    // Find the starting node index
    let start = self.name_to_nodes.get(name).and_then(|indices| {
        indices.iter().find(|&&idx| {
            let node = &self.nodes[self.graph[idx]];
            node.variant == *variant
        }).copied()
    });

    let Some(start_idx) = start else {
        return vec![];
    };

    let mut visited: HashSet<NodeIndex> = HashSet::new();
    visited.insert(start_idx);
    let mut queue: VecDeque<(NodeIndex, usize)> = VecDeque::new();
    queue.push_back((start_idx, 0));
    let mut results: Vec<NeighborInfo> = Vec::new();

    while let Some((node_idx, hops)) = queue.pop_front() {
        if hops >= max_hops {
            continue;
        }

        let directions: Vec<Direction> = match direction {
            TraversalDirection::Outgoing => vec![Direction::Outgoing],
            TraversalDirection::Incoming => vec![Direction::Incoming],
            TraversalDirection::Both => vec![Direction::Outgoing, Direction::Incoming],
        };

        for dir in directions {
            for neighbor_idx in self.graph.neighbors_directed(node_idx, dir) {
                if visited.insert(neighbor_idx) {
                    let neighbor_node = &self.nodes[self.graph[neighbor_idx]];
                    results.push(NeighborInfo {
                        name: neighbor_node.name,
                        variant: neighbor_node.variant.clone(),
                        hops: hops + 1,
                    });
                    queue.push_back((neighbor_idx, hops + 1));
                }
            }
        }
    }

    results.sort_by(|a, b| a.hops.cmp(&b.hops).then_with(|| a.name.cmp(&b.name)));
    results
}
```

**Step 2: Export the new types from `src/rules/mod.rs`**

In `src/rules/mod.rs`, update the `pub use graph::` line:

```rust
pub use graph::{
    NeighborInfo, ReductionChain, ReductionEdgeInfo, ReductionGraph, ReductionPath,
    ReductionStep, TraversalDirection,
};
```

**Step 3: Write unit tests**

In `src/unit_tests/reduction_graph.rs`, add:

```rust
#[test]
fn test_k_neighbors_outgoing() {
    let graph = ReductionGraph::new();
    let variants = graph.variants_for("MaximumIndependentSet");
    assert!(!variants.is_empty());
    let default_variant = &variants[0];

    // 1-hop outgoing: should include direct reduction targets
    let neighbors = graph.k_neighbors(
        "MaximumIndependentSet",
        default_variant,
        1,
        TraversalDirection::Outgoing,
    );
    assert!(!neighbors.is_empty());
    assert!(neighbors.iter().all(|n| n.hops == 1));

    // 2-hop outgoing: should include more problems
    let neighbors_2 = graph.k_neighbors(
        "MaximumIndependentSet",
        default_variant,
        2,
        TraversalDirection::Outgoing,
    );
    assert!(neighbors_2.len() >= neighbors.len());
}

#[test]
fn test_k_neighbors_incoming() {
    let graph = ReductionGraph::new();
    let variants = graph.variants_for("QUBO");
    assert!(!variants.is_empty());

    let neighbors = graph.k_neighbors(
        "QUBO",
        &variants[0],
        1,
        TraversalDirection::Incoming,
    );
    // QUBO is a common target — should have incoming reductions
    assert!(!neighbors.is_empty());
}

#[test]
fn test_k_neighbors_both() {
    let graph = ReductionGraph::new();
    let variants = graph.variants_for("MaximumIndependentSet");
    let default_variant = &variants[0];

    let out_only = graph.k_neighbors(
        "MaximumIndependentSet", default_variant, 1, TraversalDirection::Outgoing,
    );
    let in_only = graph.k_neighbors(
        "MaximumIndependentSet", default_variant, 1, TraversalDirection::Incoming,
    );
    let both = graph.k_neighbors(
        "MaximumIndependentSet", default_variant, 1, TraversalDirection::Both,
    );
    // Both should be >= max of either direction
    assert!(both.len() >= out_only.len());
    assert!(both.len() >= in_only.len());
}

#[test]
fn test_k_neighbors_unknown_problem() {
    let graph = ReductionGraph::new();
    let empty = BTreeMap::new();
    let neighbors = graph.k_neighbors("NonExistent", &empty, 2, TraversalDirection::Outgoing);
    assert!(neighbors.is_empty());
}

#[test]
fn test_k_neighbors_zero_hops() {
    let graph = ReductionGraph::new();
    let variants = graph.variants_for("MaximumIndependentSet");
    let default_variant = &variants[0];
    let neighbors = graph.k_neighbors(
        "MaximumIndependentSet", default_variant, 0, TraversalDirection::Outgoing,
    );
    assert!(neighbors.is_empty());
}
```

**Step 4: Run tests**

Run: `cargo test -p problemreductions test_k_neighbors`
Expected: All 5 tests pass.

**Step 5: Commit**

```
feat(lib): add k_neighbors BFS method to ReductionGraph
```

---

### Task 8: k-neighbor CLI — tree renderer and show integration

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/graph.rs`

**Step 1: Add `--hops` and `--direction` flags to `Commands::Show`**

In `problemreductions-cli/src/cli.rs`, change `Show` from a simple struct variant to a named-fields variant:

```rust
/// Show details for a problem type (variants, fields, reductions)
#[command(after_help = "\
Examples:
  pred show MIS                   # using alias
  pred show MaximumIndependentSet # full name
  pred show MIS/UnitDiskGraph     # specific graph variant
  pred show MIS --hops 2          # 2-hop outgoing neighbor tree
  pred show MIS --hops 2 --direction in  # incoming neighbors

Use `pred list` to see all available problem types and aliases.")]
Show {
    /// Problem name or alias (e.g., MIS, QUBO, MIS/UnitDiskGraph)
    problem: String,
    /// Explore k-hop neighbors in the reduction graph
    #[arg(long)]
    hops: Option<usize>,
    /// Direction for neighbor exploration: out, in, both [default: out]
    #[arg(long, default_value = "out")]
    direction: String,
},
```

**Step 2: Update `main.rs` dispatch**

In `problemreductions-cli/src/main.rs`, update the `Show` match arm:

```rust
Commands::Show { problem, hops, direction } => {
    commands::graph::show(&problem, hops, &direction, &out)
}
```

**Step 3: Update `show()` signature and add branching**

In `problemreductions-cli/src/commands/graph.rs`, update the `show` function:

```rust
pub fn show(problem: &str, hops: Option<usize>, direction: &str, out: &OutputConfig) -> Result<()> {
    let spec = parse_problem_spec(problem)?;
    let graph = ReductionGraph::new();

    let variants = graph.variants_for(&spec.name);
    if variants.is_empty() {
        anyhow::bail!("Unknown problem: {}", spec.name);
    }

    if let Some(max_hops) = hops {
        return show_neighbors(&graph, &spec, &variants, max_hops, direction, out);
    }

    // ... existing show logic unchanged ...
}
```

**Step 4: Implement `show_neighbors` with tree rendering**

Add a new function in `commands/graph.rs`:

```rust
use problemreductions::rules::{NeighborInfo, TraversalDirection};

fn parse_direction(s: &str) -> Result<TraversalDirection> {
    match s {
        "out" => Ok(TraversalDirection::Outgoing),
        "in" => Ok(TraversalDirection::Incoming),
        "both" => Ok(TraversalDirection::Both),
        _ => anyhow::bail!(
            "Unknown direction: {}. Use 'out', 'in', or 'both'.",
            s
        ),
    }
}

fn show_neighbors(
    graph: &ReductionGraph,
    spec: &crate::problem_name::ProblemSpec,
    variants: &[BTreeMap<String, String>],
    max_hops: usize,
    direction_str: &str,
    out: &OutputConfig,
) -> Result<()> {
    let direction = parse_direction(direction_str)?;

    let variant = if spec.variant_values.is_empty() {
        variants[0].clone()
    } else {
        resolve_variant(spec, variants)?
    };

    let neighbors = graph.k_neighbors(&spec.name, &variant, max_hops, direction);

    let dir_label = match direction {
        TraversalDirection::Outgoing => "outgoing",
        TraversalDirection::Incoming => "incoming",
        TraversalDirection::Both => "both directions",
    };

    // Build tree structure: group by parent chain
    // For a tree view, we do a fresh BFS that tracks parent relationships
    let tree = build_neighbor_tree(graph, &spec.name, &variant, max_hops, direction);

    let mut text = format!(
        "{} — {}-hop neighbors ({})\n\n",
        crate::output::fmt_problem_name(&spec.name),
        max_hops,
        dir_label,
    );

    text.push_str(&crate::output::fmt_problem_name(&spec.name));
    text.push('\n');
    render_tree(&tree, &mut text, "", true);

    // Count unique problem names
    let unique_names: HashSet<&str> = neighbors.iter().map(|n| n.name).collect();
    text.push_str(&format!(
        "\n{} reachable problems in {} hops\n",
        unique_names.len(),
        max_hops,
    ));

    let json = serde_json::json!({
        "source": spec.name,
        "hops": max_hops,
        "direction": direction_str,
        "neighbors": neighbors.iter().map(|n| {
            serde_json::json!({
                "name": n.name,
                "variant": n.variant,
                "hops": n.hops,
            })
        }).collect::<Vec<_>>(),
    });

    let default_name = format!("pred_show_{}_hops{}.json", spec.name, max_hops);
    out.emit_with_default_name(&default_name, &text, &json)
}

/// Tree node for neighbor rendering.
struct TreeNode {
    name: String,
    children: Vec<TreeNode>,
}

/// Build a tree of neighbors via BFS, tracking parent relationships.
fn build_neighbor_tree(
    graph: &ReductionGraph,
    name: &str,
    variant: &BTreeMap<String, String>,
    max_hops: usize,
    direction: TraversalDirection,
) -> Vec<TreeNode> {
    use petgraph::Direction;
    use std::collections::VecDeque;

    let start = graph.find_node_index(name, variant);
    let Some(start_idx) = start else {
        return vec![];
    };

    // BFS with parent tracking to build a tree
    let mut visited: HashSet<petgraph::graph::NodeIndex> = HashSet::new();
    visited.insert(start_idx);

    // (node_idx, depth) -> children to fill
    struct BfsItem {
        node_idx: petgraph::graph::NodeIndex,
        depth: usize,
    }

    let mut queue: VecDeque<BfsItem> = VecDeque::new();
    queue.push_back(BfsItem { node_idx: start_idx, depth: 0 });

    // Map from node_idx -> TreeNode
    let mut node_children: HashMap<petgraph::graph::NodeIndex, Vec<petgraph::graph::NodeIndex>> =
        HashMap::new();

    while let Some(item) = queue.pop_front() {
        if item.depth >= max_hops {
            continue;
        }

        let directions: Vec<Direction> = match direction {
            TraversalDirection::Outgoing => vec![Direction::Outgoing],
            TraversalDirection::Incoming => vec![Direction::Incoming],
            TraversalDirection::Both => vec![Direction::Outgoing, Direction::Incoming],
        };

        let mut children = Vec::new();
        for dir in directions {
            for neighbor_idx in graph.neighbor_indices(item.node_idx, dir) {
                if visited.insert(neighbor_idx) {
                    children.push(neighbor_idx);
                    queue.push_back(BfsItem { node_idx: neighbor_idx, depth: item.depth + 1 });
                }
            }
        }
        children.sort_by(|a, b| {
            let na = graph.node_name(*a);
            let nb = graph.node_name(*b);
            na.cmp(&nb)
        });
        node_children.insert(item.node_idx, children);
    }

    // Recursively build TreeNode from start's children
    fn build_tree(
        idx: petgraph::graph::NodeIndex,
        node_children: &HashMap<petgraph::graph::NodeIndex, Vec<petgraph::graph::NodeIndex>>,
        graph: &ReductionGraph,
    ) -> TreeNode {
        let children = node_children
            .get(&idx)
            .map(|cs| cs.iter().map(|&c| build_tree(c, node_children, graph)).collect())
            .unwrap_or_default();
        TreeNode {
            name: graph.node_name(idx).to_string(),
            children,
        }
    }

    node_children
        .get(&start_idx)
        .map(|cs| cs.iter().map(|&c| build_tree(c, &node_children, graph)).collect())
        .unwrap_or_default()
}

/// Render a tree with box-drawing characters.
fn render_tree(nodes: &[TreeNode], text: &mut String, prefix: &str, is_root: bool) {
    for (i, node) in nodes.iter().enumerate() {
        let is_last = i == nodes.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        text.push_str(&format!(
            "{}{}{}\n",
            crate::output::fmt_dim(prefix),
            crate::output::fmt_dim(connector),
            crate::output::fmt_problem_name(&node.name),
        ));

        if !node.children.is_empty() {
            let new_prefix = format!("{}{}", prefix, child_prefix);
            render_tree(&node.children, text, &new_prefix, false);
        }
    }
}
```

**Step 5: Add helper methods to `ReductionGraph`**

The tree builder needs `find_node_index`, `neighbor_indices`, and `node_name` on `ReductionGraph`. Add these to `src/rules/graph.rs`:

```rust
/// Find the NodeIndex for a specific (name, variant) pair.
pub fn find_node_index(&self, name: &str, variant: &BTreeMap<String, String>) -> Option<NodeIndex> {
    self.name_to_nodes.get(name).and_then(|indices| {
        indices.iter().find(|&&idx| {
            let node = &self.nodes[self.graph[idx]];
            node.variant == *variant
        }).copied()
    })
}

/// Get neighbors of a node in a specific direction.
pub fn neighbor_indices(&self, idx: NodeIndex, dir: petgraph::Direction) -> Vec<NodeIndex> {
    self.graph.neighbors_directed(idx, dir).collect()
}

/// Get the problem name for a node index.
pub fn node_name(&self, idx: NodeIndex) -> &str {
    self.nodes[self.graph[idx]].name
}
```

Also export them from `src/rules/mod.rs` (they're inherent methods, so just exporting `ReductionGraph` is enough).

**Step 6: Build and test**

Run: `cargo run -p problemreductions-cli -- show MIS --hops 2`
Expected: Tree output showing 2-hop outgoing neighbors of MIS.

Run: `cargo run -p problemreductions-cli -- show MIS --hops 2 --direction in`
Expected: Tree output showing 2-hop incoming neighbors.

**Step 7: Commit**

```
feat(cli): add --hops and --direction flags to pred show for neighbor exploration
```

---

### Task 9: k-neighbor CLI — integration tests

**Files:**
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write integration tests**

Add to `problemreductions-cli/tests/cli_tests.rs`:

```rust
#[test]
fn test_show_hops_outgoing() {
    let output = pred()
        .args(["show", "MIS", "--hops", "2"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("reachable problems"));
    // Should contain tree characters
    assert!(stdout.contains("├── ") || stdout.contains("└── "));
}

#[test]
fn test_show_hops_incoming() {
    let output = pred()
        .args(["show", "QUBO", "--hops", "1", "--direction", "in"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("QUBO"));
    assert!(stdout.contains("incoming"));
}

#[test]
fn test_show_hops_both() {
    let output = pred()
        .args(["show", "MIS", "--hops", "1", "--direction", "both"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("both directions"));
}

#[test]
fn test_show_hops_json() {
    let tmp = std::env::temp_dir().join("pred_test_show_hops.json");
    let output = pred()
        .args(["-o", tmp.to_str().unwrap(), "show", "MIS", "--hops", "2"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(tmp.exists());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["source"], "MaximumIndependentSet");
    assert_eq!(json["hops"], 2);
    assert!(json["neighbors"].is_array());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_show_hops_bad_direction() {
    let output = pred()
        .args(["show", "MIS", "--hops", "1", "--direction", "bad"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown direction"));
}
```

**Step 2: Run all CLI tests**

Run: `cargo test -p problemreductions-cli`
Expected: All tests pass, including both new and existing ones.

**Step 3: Commit**

```
test(cli): add integration tests for k-neighbor exploration
```

---

### Task 10: Final verification and cleanup

**Files:**
- None (verification only)

**Step 1: Run full test suite**

Run: `make test`
Expected: All tests pass.

**Step 2: Run clippy**

Run: `make clippy`
Expected: No warnings.

**Step 3: Run fmt check**

Run: `make fmt-check`
Expected: No formatting issues.

**Step 4: Update issue #81**

Check off the completed items in issue #81:
- [x] Shell completions
- [x] Colored/table output
- [x] k-neighbor exploration

Note that ILP solver integration and All paths were already implemented in v1.

**Step 5: Commit any final cleanup**

If any cleanup was needed, commit with:
```
chore(cli): cleanup for v2 features
```

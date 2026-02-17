# `pred` CLI Tool Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build the `pred` CLI tool as a workspace crate so researchers can explore the reduction graph and solve problem instances from the command line.

**Architecture:** Separate `problemreductions-cli` crate using `clap` derive for subcommands. Graph exploration commands work via `ReductionGraph` string API. Computation commands use a dispatch table to map problem names to concrete Rust types.

**Tech Stack:** clap (derive), anyhow, serde_json, problemreductions (all features)

---

### Task 1: Scaffold the workspace crate

**Files:**
- Create: `problemreductions-cli/Cargo.toml`
- Create: `problemreductions-cli/src/main.rs`
- Modify: `Cargo.toml:1-2` (add workspace member)

**Step 1: Add workspace member**

In root `Cargo.toml`, add `"problemreductions-cli"` to the workspace members:

```toml
[workspace]
members = [".", "problemreductions-macros", "problemreductions-cli"]
```

**Step 2: Create Cargo.toml**

```toml
[package]
name = "problemreductions-cli"
version = "0.1.0"
edition = "2021"
description = "CLI tool for exploring NP-hard problem reductions"
license = "MIT"

[[bin]]
name = "pred"
path = "src/main.rs"

[dependencies]
problemreductions = { path = "..", features = ["ilp"] }
clap = { version = "4", features = ["derive"] }
anyhow = "1"
serde_json = "1"
```

**Step 3: Create minimal main.rs**

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "pred", about = "Explore NP-hard problem reductions")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Placeholder
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Version => {
            println!("pred {}", env!("CARGO_PKG_VERSION"));
        }
    }
    Ok(())
}
```

**Step 4: Verify it builds**

Run: `cargo build -p problemreductions-cli`
Expected: Compiles successfully.

**Step 5: Verify it runs**

Run: `cargo run -p problemreductions-cli -- version`
Expected: `pred 0.1.0`

**Step 6: Commit**

```bash
git add Cargo.toml problemreductions-cli/
git commit -m "feat: scaffold problemreductions-cli crate with pred binary"
```

---

### Task 2: Problem name resolver with aliases and variant parsing

**Files:**
- Create: `problemreductions-cli/src/problem_name.rs`

**Step 1: Write tests**

Add tests at the bottom of `problem_name.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_resolution() {
        assert_eq!(resolve_alias("MIS"), "MaximumIndependentSet");
        assert_eq!(resolve_alias("mis"), "MaximumIndependentSet");
        assert_eq!(resolve_alias("MVC"), "MinimumVertexCover");
        assert_eq!(resolve_alias("SAT"), "Satisfiability");
        assert_eq!(resolve_alias("3SAT"), "KSatisfiability");
        assert_eq!(resolve_alias("QUBO"), "QUBO");
        assert_eq!(resolve_alias("MaxCut"), "MaxCut");
        // Pass-through for full names
        assert_eq!(resolve_alias("MaximumIndependentSet"), "MaximumIndependentSet");
    }

    #[test]
    fn test_parse_problem_spec_bare() {
        let spec = parse_problem_spec("MIS").unwrap();
        assert_eq!(spec.name, "MaximumIndependentSet");
        assert!(spec.variant_values.is_empty());
    }

    #[test]
    fn test_parse_problem_spec_with_variants() {
        let spec = parse_problem_spec("MIS/UnitDiskGraph").unwrap();
        assert_eq!(spec.name, "MaximumIndependentSet");
        assert_eq!(spec.variant_values, vec!["UnitDiskGraph"]);
    }

    #[test]
    fn test_parse_problem_spec_two_variants() {
        let spec = parse_problem_spec("MIS/SimpleGraph/f64").unwrap();
        assert_eq!(spec.name, "MaximumIndependentSet");
        assert_eq!(spec.variant_values, vec!["SimpleGraph", "f64"]);
    }

    #[test]
    fn test_parse_problem_spec_3sat_alias() {
        let spec = parse_problem_spec("3SAT").unwrap();
        assert_eq!(spec.name, "KSatisfiability");
        assert_eq!(spec.variant_values, vec!["K3"]);
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p problemreductions-cli`
Expected: Compilation fails (functions not defined).

**Step 3: Implement**

```rust
use std::collections::BTreeMap;

/// A parsed problem specification: name + optional variant values.
#[derive(Debug, Clone)]
pub struct ProblemSpec {
    /// Resolved canonical problem name.
    pub name: String,
    /// Positional variant values (e.g., ["UnitDiskGraph", "i32"]).
    pub variant_values: Vec<String>,
}

/// Resolve a short alias to the canonical problem name.
pub fn resolve_alias(input: &str) -> String {
    match input.to_lowercase().as_str() {
        "mis" => "MaximumIndependentSet".to_string(),
        "mvc" | "minimumvertexcover" => "MinimumVertexCover".to_string(),
        "sat" | "satisfiability" => "Satisfiability".to_string(),
        "3sat" => "KSatisfiability".to_string(),
        "ksat" | "ksatisfiability" => "KSatisfiability".to_string(),
        "qubo" => "QUBO".to_string(),
        "maxcut" => "MaxCut".to_string(),
        "spinglass" => "SpinGlass".to_string(),
        "ilp" => "ILP".to_string(),
        "circuitsat" => "CircuitSAT".to_string(),
        "factoring" => "Factoring".to_string(),
        "maximumindependentset" => "MaximumIndependentSet".to_string(),
        "maximumclique" => "MaximumClique".to_string(),
        "maximummatching" => "MaximumMatching".to_string(),
        "minimumdominatingset" => "MinimumDominatingSet".to_string(),
        "minimumsetcovering" => "MinimumSetCovering".to_string(),
        "maximumsetpacking" => "MaximumSetPacking".to_string(),
        "kcoloring" => "KColoring".to_string(),
        "maximalis" | "maximalis" => "MaximalIS".to_string(),
        "travelingsalesman" | "tsp" => "TravelingSalesman".to_string(),
        "paintshop" => "PaintShop".to_string(),
        "bmf" => "BMF".to_string(),
        "bicliquecover" => "BicliqueCover".to_string(),
        _ => input.to_string(), // pass-through for exact names
    }
}

/// Parse a problem spec string like "MIS/UnitDiskGraph/i32" into name + variant values.
pub fn parse_problem_spec(input: &str) -> anyhow::Result<ProblemSpec> {
    let parts: Vec<&str> = input.split('/').collect();
    let raw_name = parts[0];
    let mut variant_values: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

    let name = resolve_alias(raw_name);

    // Special case: "3SAT" implies K3 variant
    if raw_name.to_lowercase() == "3sat" && variant_values.is_empty() {
        variant_values.push("K3".to_string());
    }

    Ok(ProblemSpec {
        name,
        variant_values,
    })
}

/// Build a variant BTreeMap by matching positional values against a problem's
/// known variant keys from the reduction graph.
pub fn resolve_variant(
    spec: &ProblemSpec,
    known_variants: &[BTreeMap<String, String>],
) -> anyhow::Result<BTreeMap<String, String>> {
    if spec.variant_values.is_empty() {
        // Return the first (default) variant, or empty
        return Ok(known_variants.first().cloned().unwrap_or_default());
    }

    // Get the variant keys from the first known variant
    let keys: Vec<String> = known_variants
        .first()
        .map(|v| v.keys().cloned().collect())
        .unwrap_or_default();

    if spec.variant_values.len() > keys.len() {
        anyhow::bail!(
            "Too many variant values for {}: expected at most {} but got {}",
            spec.name,
            keys.len(),
            spec.variant_values.len()
        );
    }

    // Build the variant map: fill specified positions, use defaults for the rest
    let mut result = known_variants.first().cloned().unwrap_or_default();
    for (i, value) in spec.variant_values.iter().enumerate() {
        if let Some(key) = keys.get(i) {
            result.insert(key.clone(), value.clone());
        }
    }

    // Verify this variant exists
    if !known_variants.contains(&result) {
        anyhow::bail!(
            "Unknown variant for {}: {:?}. Known variants: {:?}",
            spec.name,
            result,
            known_variants
        );
    }

    Ok(result)
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test -p problemreductions-cli`
Expected: All tests pass.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/problem_name.rs
git commit -m "feat(cli): add problem name resolver with aliases and variant parsing"
```

---

### Task 3: Output module

**Files:**
- Create: `problemreductions-cli/src/output.rs`

**Step 1: Implement output module**

```rust
use anyhow::Context;
use std::path::PathBuf;

/// Output configuration derived from CLI flags.
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// If true, output as JSON to a file.
    pub json: bool,
    /// Custom output file path. If None, a default name is used.
    pub output: Option<PathBuf>,
}

impl OutputConfig {
    /// Print text to stdout (human-readable mode) or save JSON to file.
    pub fn emit(&self, human_text: &str, json_value: &serde_json::Value) -> anyhow::Result<()> {
        if self.json {
            let path = self
                .output
                .clone()
                .unwrap_or_else(|| PathBuf::from("pred_output.json"));
            let content =
                serde_json::to_string_pretty(json_value).context("Failed to serialize JSON")?;
            std::fs::write(&path, &content)
                .with_context(|| format!("Failed to write {}", path.display()))?;
            eprintln!("Wrote {}", path.display());
        } else {
            println!("{human_text}");
        }
        Ok(())
    }

    /// Emit with a custom default filename.
    pub fn emit_with_default_name(
        &self,
        default_name: &str,
        human_text: &str,
        json_value: &serde_json::Value,
    ) -> anyhow::Result<()> {
        if self.json {
            let path = self
                .output
                .clone()
                .unwrap_or_else(|| PathBuf::from(default_name));
            let content =
                serde_json::to_string_pretty(json_value).context("Failed to serialize JSON")?;
            std::fs::write(&path, &content)
                .with_context(|| format!("Failed to write {}", path.display()))?;
            eprintln!("Wrote {}", path.display());
        } else {
            println!("{human_text}");
        }
        Ok(())
    }
}
```

**Step 2: Verify it compiles**

Run: `cargo build -p problemreductions-cli`
Expected: Compiles (add `mod output;` to main.rs).

**Step 3: Commit**

```bash
git add problemreductions-cli/src/output.rs
git commit -m "feat(cli): add output module for human/JSON output modes"
```

---

### Task 4: CLI structure with clap derive

**Files:**
- Modify: `problemreductions-cli/src/main.rs`
- Create: `problemreductions-cli/src/cli.rs`

**Step 1: Create cli.rs with full command hierarchy**

```rust
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pred", about = "Explore NP-hard problem reductions", version)]
pub struct Cli {
    /// Output as JSON (saved to file)
    #[arg(long, global = true)]
    pub json: bool,

    /// Output file path (used with --json)
    #[arg(long, short, global = true)]
    pub output: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Explore the reduction graph
    Graph {
        #[command(subcommand)]
        command: GraphCommands,
    },
    /// Solve a problem instance via reduction
    Solve(SolveArgs),
    /// Reduce a problem to a target type
    Reduce(ReduceArgs),
    /// Evaluate a configuration against a problem
    Evaluate(EvaluateArgs),
    /// Show the JSON schema for a problem type
    Schema(SchemaArgs),
}

#[derive(Subcommand)]
pub enum GraphCommands {
    /// List all registered problem types
    List,
    /// Show details for a problem type
    Show {
        /// Problem name (e.g., MIS, QUBO, MIS/UnitDiskGraph)
        problem: String,
        /// List all variants
        #[arg(long)]
        variants: bool,
    },
    /// Find the cheapest reduction path
    Path {
        /// Source problem (e.g., MIS, MIS/UnitDiskGraph)
        source: String,
        /// Target problem (e.g., QUBO)
        target: String,
        /// Cost function: "minimize-steps" (default) or "minimize:<field>"
        #[arg(long, default_value = "minimize-steps")]
        cost: String,
    },
    /// Export the reduction graph to JSON
    Export {
        /// Output file path
        #[arg(long, default_value = "reduction_graph.json")]
        output: PathBuf,
    },
}

#[derive(clap::Args)]
pub struct SolveArgs {
    /// Path to a JSON problem file
    pub input: Option<PathBuf>,
    /// Problem type for inline construction (e.g., MIS)
    #[arg(long)]
    pub problem: Option<String>,
    /// Edges for inline graph problems (e.g., 0-1,1-2,2-0)
    #[arg(long)]
    pub edges: Option<String>,
    /// Weights for inline problems (e.g., 1,1,1)
    #[arg(long)]
    pub weights: Option<String>,
    /// Target problem to reduce to before solving
    #[arg(long)]
    pub via: Option<String>,
    /// Solver to use
    #[arg(long, default_value = "brute-force")]
    pub solver: String,
}

#[derive(clap::Args)]
pub struct ReduceArgs {
    /// Path to a JSON problem file
    pub input: PathBuf,
    /// Target problem type
    #[arg(long)]
    pub to: String,
}

#[derive(clap::Args)]
pub struct EvaluateArgs {
    /// Path to a JSON problem file
    pub input: PathBuf,
    /// Configuration to evaluate (comma-separated, e.g., 1,0,1)
    #[arg(long)]
    pub config: String,
}

#[derive(clap::Args)]
pub struct SchemaArgs {
    /// Problem name (e.g., MIS, QUBO)
    pub problem: String,
}
```

**Step 2: Update main.rs to use cli module**

```rust
mod cli;
mod output;
mod problem_name;

use cli::{Cli, Commands, GraphCommands};
use clap::Parser;
use output::OutputConfig;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let out = OutputConfig {
        json: cli.json,
        output: cli.output,
    };

    match cli.command {
        Commands::Graph { command } => match command {
            GraphCommands::List => todo!("graph list"),
            GraphCommands::Show { problem, variants } => todo!("graph show"),
            GraphCommands::Path { source, target, cost } => todo!("graph path"),
            GraphCommands::Export { output } => todo!("graph export"),
        },
        Commands::Solve(args) => todo!("solve"),
        Commands::Reduce(args) => todo!("reduce"),
        Commands::Evaluate(args) => todo!("evaluate"),
        Commands::Schema(args) => todo!("schema"),
    }
}
```

**Step 3: Verify it builds and --help works**

Run: `cargo run -p problemreductions-cli -- --help`
Expected: Shows help with `graph`, `solve`, `reduce`, `evaluate`, `schema` subcommands.

Run: `cargo run -p problemreductions-cli -- graph --help`
Expected: Shows `list`, `show`, `path`, `export` subcommands.

**Step 4: Commit**

```bash
git add problemreductions-cli/src/
git commit -m "feat(cli): add clap command hierarchy for all subcommands"
```

---

### Task 5: Implement `pred graph list`

**Files:**
- Create: `problemreductions-cli/src/commands/graph.rs`
- Create: `problemreductions-cli/src/commands/mod.rs`
- Modify: `problemreductions-cli/src/main.rs`

**Step 1: Create commands/mod.rs**

```rust
pub mod graph;
```

**Step 2: Implement graph list**

`commands/graph.rs`:

```rust
use crate::output::OutputConfig;
use anyhow::Result;
use problemreductions::registry::collect_schemas;
use problemreductions::rules::ReductionGraph;
use std::collections::BTreeMap;
use std::path::PathBuf;

pub fn list(out: &OutputConfig) -> Result<()> {
    let graph = ReductionGraph::new();
    let schemas = collect_schemas();

    let mut problems: Vec<(&str, usize)> = Vec::new();
    let mut types = graph.problem_types();
    types.sort();

    // Human-readable
    let mut text = format!(
        "Registered problems: {} types, {} reductions, {} variant nodes\n\n",
        graph.num_types(),
        graph.num_reductions(),
        graph.num_variant_nodes(),
    );

    for name in &types {
        text.push_str(&format!("  {name}\n"));
    }

    // JSON
    let json = serde_json::json!({
        "num_types": graph.num_types(),
        "num_reductions": graph.num_reductions(),
        "num_variant_nodes": graph.num_variant_nodes(),
        "problems": types,
    });

    out.emit_with_default_name("pred_graph_list.json", &text, &json)
}
```

**Step 3: Wire it up in main.rs**

Replace the `GraphCommands::List` arm:
```rust
GraphCommands::List => commands::graph::list(&out),
```

Add `mod commands;` at the top.

**Step 4: Verify**

Run: `cargo run -p problemreductions-cli -- graph list`
Expected: Lists all registered problem types.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/commands/
git commit -m "feat(cli): implement 'pred graph list' command"
```

---

### Task 6: Implement `pred graph show`

**Files:**
- Modify: `problemreductions-cli/src/commands/graph.rs`

**Step 1: Implement show command**

Add to `graph.rs`:

```rust
use crate::problem_name::parse_problem_spec;

pub fn show(problem: &str, show_variants: bool, out: &OutputConfig) -> Result<()> {
    let spec = parse_problem_spec(problem)?;
    let graph = ReductionGraph::new();
    let graph_json = serde_json::from_str::<serde_json::Value>(
        &graph.to_json_string()?
    )?;
    let nodes = graph_json["nodes"].as_array().unwrap();
    let edges = graph_json["edges"].as_array().unwrap();

    // Find all nodes matching this problem name
    let matching_nodes: Vec<(usize, &serde_json::Value)> = nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n["name"].as_str() == Some(&spec.name))
        .collect();

    if matching_nodes.is_empty() {
        anyhow::bail!("Unknown problem: {}", spec.name);
    }

    let mut text = format!("{}\n", spec.name);

    if show_variants {
        text.push_str(&format!("\nVariants ({}):\n", matching_nodes.len()));
        for (_, node) in &matching_nodes {
            let variant = &node["variant"];
            if variant.as_object().map_or(true, |v| v.is_empty()) {
                text.push_str("  (no variants)\n");
            } else {
                let pairs: Vec<String> = variant
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                    .collect();
                text.push_str(&format!("  {{{}}}\n", pairs.join(", ")));
            }
        }
    }

    // Show reductions from/to this problem
    let node_indices: Vec<usize> = matching_nodes.iter().map(|(i, _)| *i).collect();

    let outgoing: Vec<&serde_json::Value> = edges
        .iter()
        .filter(|e| node_indices.contains(&(e["source"].as_u64().unwrap() as usize)))
        .collect();
    let incoming: Vec<&serde_json::Value> = edges
        .iter()
        .filter(|e| node_indices.contains(&(e["target"].as_u64().unwrap() as usize)))
        .collect();

    text.push_str(&format!("\nReduces to ({}):\n", outgoing.len()));
    for edge in &outgoing {
        let target = &nodes[edge["target"].as_u64().unwrap() as usize];
        text.push_str(&format!("  → {}", target["name"].as_str().unwrap()));
        let variant = &target["variant"];
        if let Some(obj) = variant.as_object() {
            if !obj.is_empty() {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                    .collect();
                text.push_str(&format!(" {{{}}}", pairs.join(", ")));
            }
        }
        text.push('\n');
    }

    text.push_str(&format!("\nReduces from ({}):\n", incoming.len()));
    for edge in &incoming {
        let source = &nodes[edge["source"].as_u64().unwrap() as usize];
        text.push_str(&format!("  ← {}", source["name"].as_str().unwrap()));
        let variant = &source["variant"];
        if let Some(obj) = variant.as_object() {
            if !obj.is_empty() {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                    .collect();
                text.push_str(&format!(" {{{}}}", pairs.join(", ")));
            }
        }
        text.push('\n');
    }

    let json = serde_json::json!({
        "name": spec.name,
        "variants": matching_nodes.iter().map(|(_, n)| &n["variant"]).collect::<Vec<_>>(),
        "reduces_to": outgoing.iter().map(|e| {
            let t = &nodes[e["target"].as_u64().unwrap() as usize];
            serde_json::json!({"name": t["name"], "variant": t["variant"]})
        }).collect::<Vec<_>>(),
        "reduces_from": incoming.iter().map(|e| {
            let s = &nodes[e["source"].as_u64().unwrap() as usize];
            serde_json::json!({"name": s["name"], "variant": s["variant"]})
        }).collect::<Vec<_>>(),
    });

    let default_name = format!("pred_show_{}.json", spec.name);
    out.emit_with_default_name(&default_name, &text, &json)
}
```

**Step 2: Wire it up in main.rs**

```rust
GraphCommands::Show { problem, variants } => commands::graph::show(&problem, variants, &out),
```

**Step 3: Verify**

Run: `cargo run -p problemreductions-cli -- graph show MIS`
Expected: Shows MIS with its outgoing and incoming reductions.

Run: `cargo run -p problemreductions-cli -- graph show MIS --variants`
Expected: Also lists all 4 MIS variant nodes.

**Step 4: Commit**

```bash
git add problemreductions-cli/src/
git commit -m "feat(cli): implement 'pred graph show' command"
```

---

### Task 7: Implement `pred graph path`

**Files:**
- Modify: `problemreductions-cli/src/commands/graph.rs`

**Step 1: Implement path command**

Add to `graph.rs`:

```rust
use problemreductions::rules::{MinimizeSteps, Minimize, ReductionPath};
use problemreductions::types::ProblemSize;
use std::collections::BTreeMap;

/// Collect all variants for a given problem name from the graph JSON.
fn collect_variants(nodes: &[serde_json::Value], name: &str) -> Vec<BTreeMap<String, String>> {
    nodes
        .iter()
        .filter(|n| n["name"].as_str() == Some(name))
        .map(|n| {
            n["variant"]
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect()
                })
                .unwrap_or_default()
        })
        .collect()
}

pub fn path(source: &str, target: &str, cost: &str, out: &OutputConfig) -> Result<()> {
    let src_spec = parse_problem_spec(source)?;
    let dst_spec = parse_problem_spec(target)?;
    let graph = ReductionGraph::new();
    let graph_json: serde_json::Value =
        serde_json::from_str(&graph.to_json_string()?)?;
    let nodes = graph_json["nodes"].as_array().unwrap();

    let src_variants = collect_variants(nodes, &src_spec.name);
    let dst_variants = collect_variants(nodes, &dst_spec.name);

    if src_variants.is_empty() {
        anyhow::bail!("Unknown problem: {}", src_spec.name);
    }
    if dst_variants.is_empty() {
        anyhow::bail!("Unknown problem: {}", dst_spec.name);
    }

    use crate::problem_name::resolve_variant;
    let input_size = ProblemSize::new(vec![]);

    // Try all matching source/target variant combinations, find cheapest
    let mut best_path: Option<ReductionPath> = None;

    let src_resolved = if src_spec.variant_values.is_empty() {
        src_variants.clone()
    } else {
        vec![resolve_variant(&src_spec, &src_variants)?]
    };
    let dst_resolved = if dst_spec.variant_values.is_empty() {
        dst_variants.clone()
    } else {
        vec![resolve_variant(&dst_spec, &dst_variants)?]
    };

    for sv in &src_resolved {
        for dv in &dst_resolved {
            let found = if cost == "minimize-steps" {
                graph.find_cheapest_path(
                    &src_spec.name, sv, &dst_spec.name, dv, &input_size, &MinimizeSteps,
                )
            } else if let Some(field) = cost.strip_prefix("minimize:") {
                // For minimize:<field>, we need a concrete ProblemSize.
                // Without input size, fall back to minimize-steps.
                graph.find_cheapest_path(
                    &src_spec.name, sv, &dst_spec.name, dv, &input_size, &MinimizeSteps,
                )
            } else {
                anyhow::bail!("Unknown cost function: {}. Use 'minimize-steps' or 'minimize:<field>'", cost);
            };

            if let Some(p) = found {
                let is_better = best_path.as_ref().map_or(true, |bp| p.len() < bp.len());
                if is_better {
                    best_path = Some(p);
                }
            }
        }
    }

    match best_path {
        Some(path) => {
            let text = format!("Path ({} steps): {}", path.len(), path);

            let steps_json: Vec<serde_json::Value> = path
                .steps
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "name": s.name,
                        "variant": s.variant,
                    })
                })
                .collect();

            let json = serde_json::json!({
                "steps": path.len(),
                "path": steps_json,
            });

            let default_name = format!("pred_path_{}_to_{}.json", src_spec.name, dst_spec.name);
            out.emit_with_default_name(&default_name, &text, &json)
        }
        None => {
            eprintln!("No path found from {} to {}", src_spec.name, dst_spec.name);
            std::process::exit(1);
        }
    }
}
```

**Step 2: Wire it up in main.rs**

```rust
GraphCommands::Path { source, target, cost } => commands::graph::path(&source, &target, &cost, &out),
```

**Step 3: Verify**

Run: `cargo run -p problemreductions-cli -- graph path MIS QUBO`
Expected: Shows a path like `MaximumIndependentSet → QUBO` with step count.

Run: `cargo run -p problemreductions-cli -- graph path Factoring SpinGlass`
Expected: Shows a multi-step path.

**Step 4: Commit**

```bash
git add problemreductions-cli/src/
git commit -m "feat(cli): implement 'pred graph path' command"
```

---

### Task 8: Implement `pred graph export`

**Files:**
- Modify: `problemreductions-cli/src/commands/graph.rs`

**Step 1: Implement export**

Add to `graph.rs`:

```rust
pub fn export(output: &PathBuf) -> Result<()> {
    let graph = ReductionGraph::new();

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    graph
        .to_json_file(output)
        .map_err(|e| anyhow::anyhow!("Failed to export: {}", e))?;

    eprintln!(
        "Exported reduction graph ({} types, {} reductions, {} variant nodes) to {}",
        graph.num_types(),
        graph.num_reductions(),
        graph.num_variant_nodes(),
        output.display()
    );

    Ok(())
}
```

**Step 2: Wire it up in main.rs**

```rust
GraphCommands::Export { output } => commands::graph::export(&output),
```

**Step 3: Verify**

Run: `cargo run -p problemreductions-cli -- graph export --output /tmp/test_graph.json`
Expected: Creates the JSON file.

**Step 4: Commit**

```bash
git add problemreductions-cli/src/
git commit -m "feat(cli): implement 'pred graph export' command"
```

---

### Task 9: Implement `pred schema`

**Files:**
- Create: `problemreductions-cli/src/commands/schema.rs`
- Modify: `problemreductions-cli/src/commands/mod.rs`

**Step 1: Implement schema command**

```rust
use crate::output::OutputConfig;
use crate::problem_name::parse_problem_spec;
use anyhow::Result;
use problemreductions::registry::collect_schemas;

pub fn schema(problem: &str, out: &OutputConfig) -> Result<()> {
    let spec = parse_problem_spec(problem)?;
    let schemas = collect_schemas();

    let schema = schemas
        .iter()
        .find(|s| s.name == spec.name)
        .ok_or_else(|| anyhow::anyhow!("No schema found for: {}", spec.name))?;

    let mut text = format!("{}\n", schema.name);
    if !schema.description.is_empty() {
        text.push_str(&format!("  {}\n", schema.description));
    }
    text.push_str("\nFields:\n");
    for field in &schema.fields {
        text.push_str(&format!("  {} ({})", field.name, field.type_name));
        if !field.description.is_empty() {
            text.push_str(&format!(" — {}", field.description));
        }
        text.push('\n');
    }

    let json = serde_json::to_value(schema)?;
    let default_name = format!("pred_schema_{}.json", spec.name);
    out.emit_with_default_name(&default_name, &text, &json)
}
```

**Step 2: Add to mod.rs and wire up in main.rs**

```rust
// commands/mod.rs
pub mod graph;
pub mod schema;
```

```rust
// main.rs
Commands::Schema(args) => commands::schema::schema(&args.problem, &out),
```

**Step 3: Verify**

Run: `cargo run -p problemreductions-cli -- schema MIS`
Expected: Shows the MIS schema with its fields.

**Step 4: Commit**

```bash
git add problemreductions-cli/src/
git commit -m "feat(cli): implement 'pred schema' command"
```

---

### Task 10: Integration tests

**Files:**
- Create: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write integration tests using command-line execution**

```rust
use std::process::Command;

fn pred() -> Command {
    Command::new(env!("CARGO_BIN_EXE_pred"))
}

#[test]
fn test_help() {
    let output = pred().arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Explore NP-hard problem reductions"));
}

#[test]
fn test_graph_list() {
    let output = pred().args(["graph", "list"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("QUBO"));
}

#[test]
fn test_graph_show() {
    let output = pred().args(["graph", "show", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("Reduces to"));
}

#[test]
fn test_graph_show_variants() {
    let output = pred()
        .args(["graph", "show", "MIS", "--variants"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Variants"));
}

#[test]
fn test_graph_path() {
    let output = pred()
        .args(["graph", "path", "MIS", "QUBO"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Path"));
    assert!(stdout.contains("step"));
}

#[test]
fn test_graph_export() {
    let tmp = std::env::temp_dir().join("pred_test_export.json");
    let output = pred()
        .args(["graph", "export", "--output", tmp.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(tmp.exists());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json["nodes"].is_array());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_schema() {
    let output = pred().args(["schema", "MIS"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("MaximumIndependentSet"));
    assert!(stdout.contains("Fields"));
}

#[test]
fn test_graph_list_json() {
    let tmp = std::env::temp_dir().join("pred_test_list.json");
    let output = pred()
        .args([
            "--json",
            "--output",
            tmp.to_str().unwrap(),
            "graph",
            "list",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(tmp.exists());
    let content = std::fs::read_to_string(&tmp).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json["problems"].is_array());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_unknown_problem() {
    let output = pred().args(["graph", "show", "NonExistent"]).output().unwrap();
    assert!(!output.status.success());
}
```

**Step 2: Run tests**

Run: `cargo test -p problemreductions-cli`
Expected: All tests pass.

**Step 3: Commit**

```bash
git add problemreductions-cli/tests/
git commit -m "test(cli): add integration tests for graph and schema commands"
```

---

### Task 11: Add Makefile target and update CLAUDE.md

**Files:**
- Modify: `Makefile` (add `cli` target)

**Step 1: Add Makefile target**

Add after the existing targets:

```makefile
cli:  ## Build the pred CLI tool
	cargo build -p problemreductions-cli --release
```

**Step 2: Verify**

Run: `make cli`
Expected: Builds the CLI in release mode.

**Step 3: Commit**

```bash
git add Makefile
git commit -m "build: add Makefile target for pred CLI tool"
```

---

### Task 12: Final verification

**Step 1: Run full test suite**

Run: `make test clippy`
Expected: All tests pass, no clippy warnings.

**Step 2: Run CLI integration tests**

Run: `cargo test -p problemreductions-cli`
Expected: All tests pass.

**Step 3: Smoke test all commands**

```bash
cargo run -p problemreductions-cli -- graph list
cargo run -p problemreductions-cli -- graph show MIS --variants
cargo run -p problemreductions-cli -- graph path Factoring SpinGlass
cargo run -p problemreductions-cli -- graph export --output /tmp/graph.json
cargo run -p problemreductions-cli -- schema QUBO
cargo run -p problemreductions-cli -- --help
```

Expected: All commands produce reasonable output.

**Step 4: Commit if any fixups needed, then done**

//! Detect problems that have no reduction path connecting them to the main graph.
//!
//! Finds:
//! 1. Completely isolated problem types (no reductions in or out)
//! 2. Disconnected components (groups not reachable from the largest component)
//!
//! Run with: `cargo run --example detect_isolated_problems`

use problemreductions::rules::ReductionGraph;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn main() {
    let graph = ReductionGraph::new();

    let mut types = graph.problem_types();
    types.sort();

    // Build undirected adjacency at the problem-type level
    let mut adj: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for &name in &types {
        adj.entry(name).or_default();
        for edge in graph.outgoing_reductions(name) {
            adj.entry(name).or_default().insert(edge.target_name);
            adj.entry(edge.target_name).or_default().insert(name);
        }
    }

    // Find connected components via BFS
    let mut visited: BTreeSet<&str> = BTreeSet::new();
    let mut components: Vec<Vec<&str>> = Vec::new();

    for &name in &types {
        if visited.contains(name) {
            continue;
        }
        let mut component = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(name);
        visited.insert(name);

        while let Some(current) = queue.pop_front() {
            component.push(current);
            if let Some(neighbors) = adj.get(current) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        component.sort();
        components.push(component);
    }

    // Sort components by size (largest first)
    components.sort_by_key(|b| std::cmp::Reverse(b.len()));

    // Identify isolated types (no edges at all)
    let isolated: Vec<&str> = types
        .iter()
        .copied()
        .filter(|name| adj.get(name).is_some_and(|n| n.is_empty()))
        .collect();

    // Report
    println!("Reduction Graph Connectivity Report");
    println!("====================================");
    println!("Total problem types: {}", types.len());
    println!("Total reductions:    {}", graph.num_reductions());
    println!("Connected components: {}", components.len());
    println!();

    if !isolated.is_empty() {
        println!(
            "Isolated problems ({}) — no reductions in or out:",
            isolated.len()
        );
        for name in &isolated {
            let num_variants = graph.variants_for(name).len();
            println!("  {name} ({num_variants} variant(s))");
        }
        println!();
    }

    if components.len() > 1 {
        println!("Disconnected components:");
        for (i, comp) in components.iter().enumerate() {
            let marker = if i == 0 { " (main)" } else { "" };
            println!("\n  Component {}{marker} — {} types:", i + 1, comp.len());
            for name in comp {
                let num_variants = graph.variants_for(name).len();
                let out_count = graph.outgoing_reductions(name).len();
                let in_count = graph.incoming_reductions(name).len();
                println!("    {name} ({num_variants} variant(s), {out_count} out, {in_count} in)");
            }
        }
    } else {
        println!("All problem types with reductions are in a single connected component.");
    }

    // Also report at the variant level
    println!();
    println!("Variant-level detail for isolated problems:");
    for name in &isolated {
        let variants = graph.variants_for(name);
        for v in &variants {
            let label = if v.is_empty() {
                name.to_string()
            } else {
                let parts: Vec<String> = v.iter().map(|(k, val)| format!("{k}: {val}")).collect();
                format!("{name} {{{}}}", parts.join(", "))
            };
            if let Some(c) = graph.variant_complexity(name, v) {
                println!("  {label}  complexity: {c}");
            } else {
                println!("  {label}");
            }
        }
    }

    // Exit with non-zero if there are isolated types or multiple components
    if !isolated.is_empty() || components.len() > 1 {
        std::process::exit(1);
    }
}

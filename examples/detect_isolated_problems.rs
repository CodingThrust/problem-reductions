//! Detect problems that have no reduction path connecting them to the main graph.
//!
//! Run with: `cargo run --example detect_isolated_problems`

use problemreductions::rules::analysis::check_connectivity;
use problemreductions::rules::ReductionGraph;

fn main() {
    let graph = ReductionGraph::new();
    let report = check_connectivity(&graph);

    println!("Reduction Graph Connectivity Report");
    println!("====================================");
    println!("Total problem types: {}", report.total_types);
    println!("Total reductions:    {}", report.total_reductions);
    println!("Connected components: {}", report.components.len());
    println!();

    if !report.isolated.is_empty() {
        println!(
            "Isolated problems ({}) — no reductions in or out:",
            report.isolated.len()
        );
        for p in &report.isolated {
            println!("  {} ({} variant(s))", p.name, p.num_variants);
        }
        println!();
    }

    if report.components.len() > 1 {
        println!("Disconnected components:");
        for (i, comp) in report.components.iter().enumerate() {
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

    println!();
    println!("Variant-level detail for isolated problems:");
    for p in &report.isolated {
        for (variant, complexity) in &p.variant_complexities {
            let label = if variant.is_empty() {
                p.name.to_string()
            } else {
                let parts: Vec<String> = variant
                    .iter()
                    .map(|(k, val)| format!("{k}: {val}"))
                    .collect();
                format!("{} {{{}}}", p.name, parts.join(", "))
            };
            if let Some(c) = complexity {
                println!("  {label}  complexity: {c}");
            } else {
                println!("  {label}");
            }
        }
    }

    // Exit with non-zero if there are isolated types or multiple components
    if !report.isolated.is_empty() || report.components.len() > 1 {
        std::process::exit(1);
    }
}

//! Detect problems not reachable from 3-SAT via directed reduction paths.
//!
//! A directed path from 3-SAT to a problem constitutes an NP-hardness proof
//! chain. Problems without such a path are either:
//! - In P (correctly unreachable)
//! - Missing a reduction from a known NP-complete source
//!
//! Run with: `cargo run --example detect_unreachable_from_3sat`

use problemreductions::rules::analysis::{
    check_reachability_from_3sat, UnreachableReason,
};
use problemreductions::rules::ReductionGraph;

fn main() {
    let graph = ReductionGraph::new();
    let report = check_reachability_from_3sat(&graph);

    println!("NP-Hardness Proof Chain Report (from 3-SAT)");
    println!("=============================================");
    println!("Total problem types: {}", report.total_types);
    println!("Reachable from 3-SAT: {}", report.reachable.len());
    println!(
        "Not reachable: {}",
        report.unreachable.len()
    );
    println!();

    // Show reachable problems sorted by hop distance
    println!("Reachable from 3-SAT ({}):", report.reachable.len());
    let mut by_hops: Vec<(&&str, &usize)> = report.reachable.iter().collect();
    by_hops.sort_by_key(|(name, hops)| (**hops, **name));
    for (name, hops) in &by_hops {
        println!("  [{hops} hops] {name}");
    }
    println!();

    if report.unreachable.is_empty() {
        println!("All problems are reachable from 3-SAT.");
        return;
    }

    let missing: Vec<_> = report
        .unreachable
        .iter()
        .filter(|p| p.reason == UnreachableReason::MissingProofChain)
        .collect();
    let in_p: Vec<_> = report
        .unreachable
        .iter()
        .filter(|p| p.reason == UnreachableReason::InP)
        .collect();
    let intermediate: Vec<_> = report
        .unreachable
        .iter()
        .filter(|p| p.reason == UnreachableReason::Intermediate)
        .collect();
    let orphans: Vec<_> = report
        .unreachable
        .iter()
        .filter(|p| p.reason == UnreachableReason::Orphan)
        .collect();

    if !missing.is_empty() {
        println!(
            "NP-hard but NO proof chain from 3-SAT ({}) — missing reductions:",
            missing.len()
        );
        for p in &missing {
            println!(
                "  {} ({} out, {} in)",
                p.name, p.outgoing_count, p.incoming_count
            );
        }
        println!();
    }

    if !in_p.is_empty() {
        println!("In P — correctly unreachable ({}):", in_p.len());
        for p in &in_p {
            println!("  {}", p.name);
        }
        println!();
    }

    if !intermediate.is_empty() {
        println!(
            "Intermediate complexity — correctly unreachable ({}):",
            intermediate.len()
        );
        for p in &intermediate {
            println!("  {}", p.name);
        }
        println!();
    }

    if !orphans.is_empty() {
        println!("Orphans — no reductions at all ({}):", orphans.len());
        for p in &orphans {
            println!("  {}", p.name);
        }
        println!();
    }

    // Exit with non-zero if there are NP-hard problems missing proof chains
    if !missing.is_empty() {
        std::process::exit(1);
    }
}

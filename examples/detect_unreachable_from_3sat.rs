//! Detect problems not reachable from 3-SAT via directed reduction paths.
//!
//! A directed path from 3-SAT to a problem constitutes an NP-hardness proof
//! chain. Problems without such a path are either:
//! - In P (correctly unreachable)
//! - Missing a reduction from a known NP-complete source
//!
//! Run with: `cargo run --example detect_unreachable_from_3sat`

use problemreductions::rules::ReductionGraph;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

const SOURCE: &str = "KSatisfiability";

fn main() {
    let graph = ReductionGraph::new();

    let mut types = graph.problem_types();
    types.sort();

    // Build directed adjacency at the type level
    let mut adj: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for &name in &types {
        adj.entry(name).or_default();
        for edge in graph.outgoing_reductions(name) {
            adj.entry(name).or_default().insert(edge.target_name);
        }
    }

    // BFS from 3-SAT (KSatisfiability) following directed edges
    let mut reachable: BTreeMap<&str, usize> = BTreeMap::new(); // name -> min hops
    let mut queue: VecDeque<(&str, usize)> = VecDeque::new();
    reachable.insert(SOURCE, 0);
    queue.push_back((SOURCE, 0));

    while let Some((current, hops)) = queue.pop_front() {
        if let Some(neighbors) = adj.get(current) {
            for &neighbor in neighbors {
                if !reachable.contains_key(neighbor) {
                    reachable.insert(neighbor, hops + 1);
                    queue.push_back((neighbor, hops + 1));
                }
            }
        }
    }

    // Classify unreachable problems
    let unreachable_types: Vec<&str> = types
        .iter()
        .copied()
        .filter(|name| !reachable.contains_key(name))
        .collect();

    // Report
    println!("NP-Hardness Proof Chain Report (from 3-SAT)");
    println!("=============================================");
    println!("Total problem types: {}", types.len());
    println!("Reachable from 3-SAT: {}", reachable.len());
    println!("Not reachable: {}", unreachable_types.len());
    println!();

    // Show reachable problems sorted by hop distance
    println!("Reachable from 3-SAT ({}):", reachable.len());
    let mut by_hops: Vec<(&&str, &usize)> = reachable.iter().collect();
    by_hops.sort_by_key(|(name, hops)| (**hops, **name));
    for (name, hops) in &by_hops {
        println!("  [{hops} hops] {name}");
    }
    println!();

    if unreachable_types.is_empty() {
        println!("All problems are reachable from 3-SAT.");
        return;
    }

    // Categorize unreachable problems
    let mut np_hard_missing: Vec<&str> = Vec::new();
    let mut p_time: Vec<&str> = Vec::new();
    let mut intermediate: Vec<&str> = Vec::new();
    let mut orphans: Vec<&str> = Vec::new();

    // Known P-time problems and variants
    let p_time_checks: &[(&str, Option<(&str, &str)>)] = &[
        ("MaximumMatching", None),
        ("KSatisfiability", Some(("k", "K2"))),
        ("KColoring", Some(("graph", "SimpleGraph"))),
    ];

    // Known intermediate-complexity problems
    let intermediate_names: &[&str] = &["Factoring"];

    for &name in &unreachable_types {
        // Check if it's an orphan (no edges at all)
        let out = graph.outgoing_reductions(name);
        let inc = graph.incoming_reductions(name);
        if out.is_empty() && inc.is_empty() {
            orphans.push(name);
            continue;
        }

        // Check if it's a known P-time problem
        let is_p = p_time_checks.iter().any(|(pname, variant_check)| {
            if *pname != name {
                return false;
            }
            match variant_check {
                None => true,
                Some((key, val)) => {
                    // Check if ALL variants of this problem are P-time
                    // (conservative: if any variant could be hard, don't classify as P)
                    let variants = graph.variants_for(name);
                    variants.len() == 1 && variants[0].get(*key).map(|s| s.as_str()) == Some(*val)
                }
            }
        });
        if is_p {
            p_time.push(name);
            continue;
        }

        // Check if it's known intermediate complexity
        if intermediate_names.contains(&name) {
            intermediate.push(name);
            continue;
        }

        // Otherwise it's NP-hard but missing a proof chain
        np_hard_missing.push(name);
    }

    if !np_hard_missing.is_empty() {
        println!(
            "NP-hard but NO proof chain from 3-SAT ({}) — missing reductions:",
            np_hard_missing.len()
        );
        for name in &np_hard_missing {
            let out_count = graph.outgoing_reductions(name).len();
            let in_count = graph.incoming_reductions(name).len();
            println!("  {name} ({out_count} out, {in_count} in)");
        }
        println!();
    }

    if !p_time.is_empty() {
        println!("In P — correctly unreachable ({}):", p_time.len());
        for name in &p_time {
            println!("  {name}");
        }
        println!();
    }

    if !intermediate.is_empty() {
        println!(
            "Intermediate complexity — correctly unreachable ({}):",
            intermediate.len()
        );
        for name in &intermediate {
            println!("  {name}");
        }
        println!();
    }

    if !orphans.is_empty() {
        println!("Orphans — no reductions at all ({}):", orphans.len());
        for name in &orphans {
            println!("  {name}");
        }
        println!();
    }

    // Exit with non-zero if there are NP-hard problems missing proof chains
    if !np_hard_missing.is_empty() {
        std::process::exit(1);
    }
}

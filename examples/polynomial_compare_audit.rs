//! Verdict distribution of the V1 polynomial comparator over every simple path of
//! 1-4 non-Turing hops in the registered reduction graph, grouped by variant endpoints.
//! Simple means no repeated (problem name, complete variant) node; variant casts count
//! as hops. Uncomposable paths are counted as skipped and excluded from comparisons.
//! Each endpoint group is one generic batch, including every nonzero field certificate.
//!
//! cargo run --release --example polynomial_compare_audit
use problemreductions::poly_compare::{compare_all, Direction, Profile, Reason, Verdict};
use problemreductions::rules::{ReductionGraph, ReductionPath, ReductionStep};
use std::collections::BTreeMap;
use std::time::Instant;

const MAX_HOPS: usize = 4;

type Node = (String, BTreeMap<String, String>);

fn extend(graph: &ReductionGraph, path: &mut Vec<Node>, found: &mut Vec<ReductionPath>) {
    if path.len() > MAX_HOPS {
        return;
    }
    let (name, variant) = path.last().expect("non-empty path").clone();
    for edge in graph.outgoing_reductions(&name) {
        if edge.source_variant != variant || edge.capabilities.turing {
            continue;
        }
        let next = (edge.target_name.to_string(), edge.target_variant);
        if path.contains(&next) {
            continue;
        }
        path.push(next);
        found.push(ReductionPath {
            steps: path
                .iter()
                .map(|(name, variant)| ReductionStep {
                    name: name.clone(),
                    variant: variant.clone(),
                })
                .collect(),
        });
        extend(graph, path, found);
        path.pop();
    }
}

fn record_reason(field: &str, reason: &Reason, totals: &mut BTreeMap<String, usize>) {
    let kind = match reason {
        Reason::NotPolynomial(_) => "not polynomial",
        Reason::Unavailable(_) => "unavailable",
        Reason::MissingField => "missing field",
        Reason::Validity => "validity",
        Reason::ResourceLimit(_) => "resource limit",
    };
    *totals.entry(format!("{kind} ({field})")).or_default() += 1;
}

fn main() {
    let graph = ReductionGraph::new();
    let started = Instant::now();
    let mut paths = Vec::new();
    for name in graph.problem_types() {
        for variant in graph.variants_for(name) {
            extend(&graph, &mut vec![(name.to_string(), variant)], &mut paths);
        }
    }
    let mut groups: BTreeMap<(String, String), Vec<Profile>> = BTreeMap::new();
    let mut skipped = 0;
    for path in &paths {
        let (Some(first), Some(last)) = (path.steps.first(), path.steps.last()) else {
            continue;
        };
        match graph.path_polynomial_profile(path) {
            Ok(profile) => groups
                .entry((first.to_string(), last.to_string()))
                .or_default()
                .push(profile),
            Err(_) => skipped += 1,
        }
    }
    let enumerated = started.elapsed();
    let mut counts = [0; 4];
    let mut front = 0;
    let mut reasons = BTreeMap::new();
    for profiles in groups.values() {
        let result = compare_all(profiles);
        for (_, _, pair) in &result.pairs {
            for (field, f) in &pair.fields {
                if pair.verdict == Verdict::Unknown {
                    for direction in [&f.forward, &f.reverse] {
                        if let Direction::Unknown(reason) = direction {
                            record_reason(field, reason, &mut reasons);
                        }
                    }
                }
            }
        }
        for (total, count) in counts.iter_mut().zip(result.counts()) {
            *total += count;
        }
        front += result.front.len();
    }
    println!(
        "paths: {} ({skipped} without a composable transform)",
        paths.len()
    );
    println!("endpoint pairs: {}", groups.len());
    println!("generic comparison (batch-relative; regular valid families):");
    for (name, count) in Verdict::ALL.iter().zip(counts) {
        println!("{name}: {count}");
    }
    println!("unresolved directions in Unknown pairs, by reason (field):");
    for (reason, count) in &reasons {
        println!("  {reason}: {count}");
    }
    println!("candidate front size (summed over endpoint pairs): {front}");
    println!(
        "time: {:.2?} enumeration and composition, {:.2?} total",
        enumerated,
        started.elapsed()
    );
}

use super::{check_connectivity, check_reachability_from_3sat, UnreachableReason};
use crate::rules::ReductionGraph;

// ---- Connectivity checks ----

#[test]
fn test_check_connectivity_returns_valid_report() {
    let graph = ReductionGraph::new();
    let report = check_connectivity(&graph);

    assert!(report.total_types > 0);
    assert!(report.total_reductions > 0);
    assert!(!report.components.is_empty());

    // Components should be sorted largest-first
    for w in report.components.windows(2) {
        assert!(w[0].len() >= w[1].len());
    }

    // Each component should be internally sorted
    for comp in &report.components {
        let mut sorted = comp.clone();
        sorted.sort();
        assert_eq!(comp, &sorted);
    }

    // All types should appear in exactly one component
    let total_in_components: usize = report.components.iter().map(|c| c.len()).sum();
    assert_eq!(total_in_components, report.total_types);
}

#[test]
fn test_isolated_problems_have_no_reductions() {
    let graph = ReductionGraph::new();
    let report = check_connectivity(&graph);

    for p in &report.isolated {
        assert!(
            graph.outgoing_reductions(p.name).is_empty(),
            "{} has outgoing reductions but is marked isolated",
            p.name
        );
        assert!(
            graph.incoming_reductions(p.name).is_empty(),
            "{} has incoming reductions but is marked isolated",
            p.name
        );
        assert!(p.num_variants > 0);
    }
}

// ---- Reachability checks ----

#[test]
fn test_reachability_from_3sat_returns_valid_report() {
    let graph = ReductionGraph::new();
    let report = check_reachability_from_3sat(&graph);

    assert!(report.total_types > 0);
    // 3-SAT (KSatisfiability) should be reachable at distance 0
    assert_eq!(report.reachable.get("KSatisfiability"), Some(&0));
    // Satisfiability should be reachable (KSat -> Sat exists)
    assert!(report.reachable.contains_key("Satisfiability"));
    // Total should add up
    assert_eq!(
        report.reachable.len() + report.unreachable.len(),
        report.total_types
    );
}

#[test]
fn test_reachability_classifies_known_problems() {
    let graph = ReductionGraph::new();
    let report = check_reachability_from_3sat(&graph);

    // MaximumMatching is in P
    if let Some(p) = report
        .unreachable
        .iter()
        .find(|p| p.name == "MaximumMatching")
    {
        assert_eq!(p.reason, UnreachableReason::InP);
    }

    // Factoring is intermediate
    if let Some(p) = report.unreachable.iter().find(|p| p.name == "Factoring") {
        assert_eq!(p.reason, UnreachableReason::Intermediate);
    }
}

#[test]
fn test_reachability_hop_distances_are_monotonic() {
    let graph = ReductionGraph::new();
    let report = check_reachability_from_3sat(&graph);

    // For every reachable problem at distance d > 0, there must be a
    // predecessor at distance d-1 that has an outgoing reduction to it
    for (&name, &hops) in &report.reachable {
        if hops == 0 {
            continue;
        }
        let has_predecessor = graph.incoming_reductions(name).iter().any(|edge| {
            report
                .reachable
                .get(edge.source_name)
                .is_some_and(|&h| h < hops)
        });
        assert!(
            has_predecessor,
            "{name} at distance {hops} has no predecessor with smaller distance"
        );
    }
}

#[test]
fn test_reachability_missing_proof_chains_filter() {
    let graph = ReductionGraph::new();
    let report = check_reachability_from_3sat(&graph);
    let missing = report.missing_proof_chains();
    // All items returned should have MissingProofChain reason
    for p in &missing {
        assert_eq!(p.reason, UnreachableReason::MissingProofChain);
    }
    // Count should match manual filter
    let manual_count = report
        .unreachable
        .iter()
        .filter(|p| p.reason == UnreachableReason::MissingProofChain)
        .count();
    assert_eq!(missing.len(), manual_count);
}

#[test]
fn test_connectivity_reports_components() {
    let graph = ReductionGraph::new();
    let report = check_connectivity(&graph);
    // There should be at least one component
    assert!(!report.components.is_empty());
    // The largest component should be sorted
    if let Some(largest) = report.components.first() {
        let mut sorted = largest.clone();
        sorted.sort();
        assert_eq!(*largest, sorted, "components should be sorted");
    }
    // Components should be sorted by size (descending)
    for window in report.components.windows(2) {
        assert!(
            window[0].len() >= window[1].len(),
            "components should be sorted largest-first"
        );
    }
}

#[test]
fn test_connectivity_isolated_problems_have_variant_info() {
    let graph = ReductionGraph::new();
    let report = check_connectivity(&graph);
    for iso in &report.isolated {
        assert!(
            iso.num_variants > 0,
            "isolated problem {} should have at least one variant",
            iso.name
        );
        assert_eq!(
            iso.variant_complexities.len(),
            iso.num_variants,
            "variant_complexities count should match num_variants for {}",
            iso.name
        );
    }
}

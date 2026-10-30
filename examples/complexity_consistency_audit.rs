//! Audit registered variant complexities against the reduction graph.
//!
//! For every simple path A -> ... -> B of 1-3 witness hops, substitute the
//! path's composed parameter transform into B's registered complexity. The
//! resulting expression is a candidate bound on solving A. Substitution of
//! upper-bound parameters requires the target complexity to be monotone in those
//! parameters; a known growth form alone does not prove that property.
//! The audit compares represented growth bounds and reports the shortest route
//! for each candidate improvement. Widening can make a registered bound loose,
//! and the reduction's own running time is not modeled. Every finding therefore
//! requires manual verification of monotonicity, tightness, and reduction cost.
//!
//! cargo run --release --example complexity_consistency_audit
use problemreductions::Growth;
use problemreductions::expr::Expr;
use problemreductions::rules::{ReductionGraph, ReductionPath, ReductionStep};
use std::collections::{BTreeMap, HashMap};

const MAX_HOPS: usize = 3;

type Node = (String, BTreeMap<String, String>);

struct Finding {
    path: ReductionPath,
    registered: String,
    derived: String,
    target_complexity: String,
}

#[derive(Default)]
struct Totals {
    paths: usize,
    no_complexity: usize,
    uncomposable: usize,
    unknown_growth: usize,
    incomparable: usize,
    consistent: usize,
    violations: usize,
    unknown_reasons: BTreeMap<String, usize>,
}

fn extend(graph: &ReductionGraph, path: &mut Vec<Node>, found: &mut Vec<ReductionPath>) {
    if path.len() > MAX_HOPS {
        return;
    }
    let (name, variant) = path.last().expect("non-empty path").clone();
    for edge in graph.outgoing_reductions(&name) {
        // Aggregate-only edges bound a value, not a witness search.
        if edge.source_variant != variant || edge.capabilities.turing || !edge.capabilities.witness
        {
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

/// The target complexity rewritten in source parameters, if every step composes.
///
/// Substituting upper-bound fields is sound only for a complexity that is
/// monotone in the substituted parameters. The caller skips unsupported target
/// growth forms, but support is not a monotonicity proof. Findings must be
/// checked against the target formula and each transform's accuracy.
fn derived_complexity(graph: &ReductionGraph, path: &ReductionPath, target: &str) -> Option<Expr> {
    let composed = graph.compose_path_parameter_transform(path).ok()??;
    let replacements: HashMap<&str, &Expr> = composed.expressions().collect();
    Expr::parse(target).substitute_complete(&replacements).ok()
}

fn label(step: &ReductionStep) -> String {
    let variant: Vec<&str> = step.variant.values().map(String::as_str).collect();
    if variant.is_empty() {
        step.name.clone()
    } else {
        format!("{}/{}", step.name, variant.join("/"))
    }
}

fn main() {
    let graph = ReductionGraph::new();
    let mut paths = Vec::new();
    for name in graph.problem_types() {
        for variant in graph.variants_for(name) {
            extend(&graph, &mut vec![(name.to_string(), variant)], &mut paths);
        }
    }

    let mut totals = Totals::default();
    let mut findings: BTreeMap<String, Vec<Finding>> = BTreeMap::new();
    for path in paths {
        totals.paths += 1;
        let (first, last) = (
            &path.steps[0],
            path.steps.last().expect("path has a target"),
        );
        let (Some(registered), Some(target)) = (
            graph.variant_complexity(&first.name, &first.variant),
            graph.variant_complexity(&last.name, &last.variant),
        ) else {
            totals.no_complexity += 1;
            continue;
        };
        if Growth::from_expr(&Expr::parse(target)).failures().is_some() {
            totals.unknown_growth += 1;
            *totals
                .unknown_reasons
                .entry("target complexity unsupported".into())
                .or_default() += 1;
            continue;
        }
        let Some(derived) = derived_complexity(&graph, &path, target) else {
            totals.uncomposable += 1;
            continue;
        };
        let derived_growth = Growth::from_expr(&derived);
        let registered_growth = Growth::from_expr(&Expr::parse(registered));
        let failure = derived_growth
            .failures()
            .map(|failures| ("derived", failures))
            .or_else(|| {
                registered_growth
                    .failures()
                    .map(|failures| ("registered", failures))
            });
        if let Some((side, failures)) = failure {
            totals.unknown_growth += 1;
            for failure in failures {
                let kind = failure.to_string();
                let kind = kind.split(':').next().unwrap_or_default();
                *totals
                    .unknown_reasons
                    .entry(format!("{side}: {kind}"))
                    .or_default() += 1;
            }
            continue;
        }
        let derived_dominates = derived_growth.dominates(&registered_growth);
        let registered_dominates = registered_growth.dominates(&derived_growth);
        match (registered_dominates, derived_dominates) {
            (true, false) => {
                totals.violations += 1;
                let derived_big_o = derived_growth.to_big_o();
                findings.entry(label(first)).or_default().push(Finding {
                    path,
                    registered: registered.to_string(),
                    derived: derived_big_o,
                    target_complexity: target.to_string(),
                });
            }
            (false, false) => totals.incomparable += 1,
            _ => totals.consistent += 1,
        }
    }

    println!("Complexity consistency audit (paths of 1-{MAX_HOPS} witness hops)");
    println!("paths examined:        {}", totals.paths);
    println!("  consistent:          {}", totals.consistent);
    println!("  candidate improvements:   {}", totals.violations);
    println!("  incomparable Big-O:  {}", totals.incomparable);
    println!("  unknown growth:      {}", totals.unknown_growth);
    println!("  uncomposable:        {}", totals.uncomposable);
    println!("  no complexity:       {}", totals.no_complexity);
    for (reason, count) in &totals.unknown_reasons {
        println!("    unknown growth, {reason}: {count}");
    }

    println!(
        "\n## Candidate improvements requiring manual review ({} variants)",
        findings.len()
    );
    for (source, list) in &findings {
        let best = list
            .iter()
            .min_by_key(|f| f.path.steps.len())
            .expect("non-empty finding list");
        let route: Vec<String> = best.path.steps.iter().map(label).collect();
        println!("- {source}");
        println!("    registered: {}", best.registered);
        println!(
            "    derived:    {}  ({} witness paths)",
            best.derived,
            list.len()
        );
        println!("    via:        {}", route.join(" -> "));
        println!("    target:     {}", best.target_complexity);
    }
}

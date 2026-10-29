//! Presentation only: decisions and the front come from `poly_compare::compare_all`.
use super::{ReductionGraph, ReductionPath, fmt_path_chain};
use problemreductions::poly_compare::{
    Direction, PathComparison, Profile, ProfileComparison, Side, Verdict, compare_all,
};
use serde_json::{Value, json};

/// 1-based `(winner, loser)` of a dominated pair.
fn winner_loser(i: usize, j: usize, verdict: Verdict) -> Option<(usize, usize)> {
    verdict.winner_loser(i, j).map(|(w, l)| (w + 1, l + 1))
}

fn strict_fields(c: &ProfileComparison, worse: Side) -> String {
    c.fields
        .iter()
        .filter(|(_, f)| f.verdict() == Verdict::Dominated { worse })
        .map(|(n, _)| n.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn description(i: usize, j: usize, c: &ProfileComparison) -> String {
    let (a, b) = (i + 1, j + 1);
    let verdict = c.verdict;
    match verdict {
        Verdict::Dominated { worse } => {
            let (winner, loser) = winner_loser(i, j, verdict).expect("dominated");
            format!(
                "Path {winner} dominates Path {loser} (strict on: {})",
                strict_fields(c, worse)
            )
        }
        Verdict::Equivalent => format!("Path {a} ≡ Path {b}"),
        Verdict::Unknown => "UNRESOLVED (details below)".into(),
        Verdict::Incomparable => {
            let mut details = Vec::new();
            for (path, worse) in [(a, Side::Right), (b, Side::Left)] {
                let fields = strict_fields(c, worse);
                if !fields.is_empty() {
                    details.push(format!("Path {path} better on {fields}"));
                }
            }
            for (name, f) in &c.fields {
                match f.verdict() {
                    Verdict::Incomparable => {
                        details.push(format!("{name}: mutually unbounded ratios"))
                    }
                    Verdict::Unknown => {
                        for (no, left, right) in [
                            (matches!(f.forward, Direction::No(_)), a, b),
                            (matches!(f.reverse, Direction::No(_)), b, a),
                        ] {
                            if no {
                                details.push(format!("{name}: Path {left} is not O(Path {right})"));
                            }
                        }
                    }
                    _ => {}
                }
            }
            format!("proven incomparable — {}", details.join("; "))
        }
    }
}

pub(super) struct ComparisonView {
    profiles: Vec<Profile>,
    result: PathComparison,
}
impl ComparisonView {
    pub(super) fn new(profiles: Vec<Profile>) -> Self {
        let result = compare_all(&profiles);
        Self { profiles, result }
    }

    pub(super) fn to_json(&self) -> Value {
        let counts = self.result.counts();
        json!({
            "schema_version": 5,
            "mode": "generic",
            "domain": "growth families x_i = c_i*t^e_i; c_i >= 2, e_i >= 0, e != 0",
            "counts": Verdict::ALL.iter().zip(counts).collect::<std::collections::BTreeMap<_, _>>(),
            "scope": "all nonzero polynomial certificates in profiles; regular and positive top faces",
            "front": self.result.front.iter().map(|i| i + 1).collect::<Vec<_>>(),
            "pairs": self.result.pairs.iter().map(|(i, j, c)| {
                let mut pair = json!({
                    "paths": [i + 1, j + 1],
                    "verdict": c.verdict,
                    "description": description(*i, *j, c),
                    "fields": c.fields,
                });
                if let Some((winner, loser)) = winner_loser(*i, *j, c.verdict) {
                    pair["dominates"] = json!({"winner": winner, "loser": loser});
                }
                pair
            }).collect::<Vec<_>>(),
            "profiles": self.profiles,
        })
    }

    pub(super) fn render(
        &self,
        graph: &ReductionGraph,
        paths: &[ReductionPath],
        proofs: bool,
    ) -> String {
        let mut text = String::from(
            "Polynomial comparison: generic order of chosen certificates, batch-relative regular valid growth families; smaller certificates are better. Ignores exact-cancellation families such as complete graphs; keeps bounded strata and exponent cells.\n",
        );
        text.push_str(&format!(
            "\nPair verdicts ({} pairs):\n",
            self.result.pairs.len()
        ));
        let [dominated, equivalent, incomparable, unknown] = self.result.counts();
        text.push_str(&format!("  {dominated} dominated, {equivalent} equivalent, {incomparable} proven incomparable, {unknown} unresolved\n"));
        for (i, j, c) in &self.result.pairs {
            text.push_str(&format!(
                "  Paths {}, {}: {}\n",
                i + 1,
                j + 1,
                description(*i, *j, c)
            ));
        }
        text.push_str("\nUnresolved pairs:\n");
        if self.result.counts()[3] == 0 {
            text.push_str("  None.\n");
        }
        for (i, j, c) in self
            .result
            .pairs
            .iter()
            .filter(|(_, _, c)| c.verdict == Verdict::Unknown)
        {
            text.push_str(&format!("  Paths {}, {}:\n", i + 1, j + 1));
            for (name, f) in &c.fields {
                for (d, left, right) in [(&f.forward, i + 1, j + 1), (&f.reverse, j + 1, i + 1)] {
                    if let Direction::Unknown(reason) = d {
                        text.push_str(&format!(
                            "    {name}, Path {left} ≤O Path {right}: missing obligation: {reason}\n"
                        ));
                    }
                }
            }
        }
        text.push_str("\nCandidate front among listed paths:\n");
        for &index in &self.result.front {
            let chain = fmt_path_chain(graph, &paths[index]);
            text.push_str(&format!("  Path {}: {chain}\n", index + 1));
            for (name, f) in &self.profiles[index].fields {
                text.push_str(&format!("    {name} {f}\n"));
                let certificate = f.certificate.as_ref().ok().map(|(poly, _)| poly);
                if let Some(original) = f
                    .original_signed
                    .as_ref()
                    .filter(|o| Some(*o) != certificate)
                {
                    text.push_str(&format!("      original signed: {}\n", original.to_expr()));
                }
            }
        }
        if proofs {
            text.push_str(
                "\nField proofs (forward: first path ≤O second; reverse: second ≤O first):\n",
            );
            for (i, j, c) in &self.result.pairs {
                text.push_str(&format!(
                    "  Paths {}, {}: {}\n",
                    i + 1,
                    j + 1,
                    description(*i, *j, c)
                ));
                for (name, f) in &c.fields {
                    for (label, d, left, right) in [
                        ("forward", &f.forward, *i, *j),
                        ("reverse", &f.reverse, *j, *i),
                    ] {
                        let formula = |index: usize| {
                            self.profiles[index]
                                .fields
                                .get(name)
                                .map_or_else(|| "missing".into(), ToString::to_string)
                        };
                        text.push_str(&format!(
                            "    {name} {label} (Path {} ≤O Path {}): f {}; g {}\n",
                            left + 1,
                            right + 1,
                            formula(left),
                            formula(right)
                        ));
                        for line in d.to_string().lines() {
                            text.push_str(&format!("      {line}\n"));
                        }
                    }
                }
            }
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use problemreductions::{Expr, parameters::ParameterRelation, poly_compare::FieldCertificate};
    use std::collections::BTreeMap;

    fn certificate(s: &str) -> FieldCertificate {
        FieldCertificate::new(&Expr::parse(s), ParameterRelation::Exact)
    }
    fn profile(vars: &str, terms: &str) -> Profile {
        Profile::new(BTreeMap::from([
            ("vars".into(), certificate(vars)),
            ("terms".into(), certificate(terms)),
        ]))
    }
    fn first(view: &ComparisonView) -> String {
        let (i, j, c) = &view.result.pairs[0];
        description(*i, *j, c)
    }

    #[test]
    fn polynomial_pair_descriptions_distinguish_tradeoffs_from_refutations_and_missing_proofs() {
        let view = ComparisonView::new(vec![profile("n", "m"), profile("n^2", "1")]);
        assert_eq!(
            first(&view),
            "proven incomparable — Path 1 better on vars; Path 2 better on terms"
        );
        let view = ComparisonView::new(vec![profile("n", "n^2-2*n"), profile("m", "n-2")]);
        let description = first(&view);
        assert!(
            description.contains("proven incomparable")
                && description.contains("vars: mutually unbounded ratios")
        );
        assert!(description.contains("Path 2 better on terms"));
        let view = ComparisonView::new(vec![
            profile("m", "1"),
            profile("n", "1"),
            profile("n-m", "1"),
        ]);
        assert_eq!(view.result.pairs[0].2.verdict, Verdict::Unknown);
        assert!(first(&view).contains("UNRESOLVED"));
    }

    #[test]
    fn polynomial_front_render_keeps_signed_provenance_and_unavailable_reasons() {
        let graph = ReductionGraph::new();
        let batch = super::super::find_path_batch(
            &graph,
            "MaximumIndependentSet",
            &graph.default_variant_for("MaximumIndependentSet").unwrap(),
            "QUBO",
            &graph.default_variant_for("QUBO").unwrap(),
            2,
        )
        .unwrap();
        let mut left = profile("n", "n^2-m");
        let vars = left.fields.get_mut("vars").unwrap();
        vars.certificate = Ok((
            problemreductions::polynomial::Poly::from_expr(&Expr::parse("n")).unwrap(),
            ParameterRelation::UpperBound,
        ));
        vars.original_signed = problemreductions::polynomial::Poly::from_expr(&Expr::parse("n-m"));
        left.fields.insert("missing".into(), certificate("log(n)"));
        let mut right = profile("n", "n");
        right.fields.get_mut("vars").unwrap().certificate = Ok((
            problemreductions::polynomial::Poly::from_expr(&Expr::parse("n")).unwrap(),
            ParameterRelation::UpperBound,
        ));
        let view = ComparisonView::new(vec![left, right]);
        let text = view.render(&graph, &batch.paths, true);
        assert!(text.contains("vars <= n\n"));
        assert!(text.contains("original signed:") && text.contains("unavailable"));
        assert!(text.contains("UNRESOLVED") && text.contains("missing obligation:"));
        assert!(text.contains("not polynomial:") && text.contains("Field proofs"));
        assert!(
            view.to_json()["profiles"][0]["fields"]["missing"]["accuracy"]["reason"].is_object()
        );
    }
}

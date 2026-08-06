//! Reduction from EulerianPath to ILP (Integer Linear Programming).
//!
//! Encodes the directed Eulerian-trail witness structure as an ILP feasibility
//! instance with integer variables. Given a directed multigraph
//! `D = (V, A)` with arc occurrences `A = {a_1, ..., a_m}`:
//!
//! - For every compatible ordered pair `(a, b) in P` (where
//!   `P = { (a, b) : a != b and head(a) = tail(b) }`), introduce an integer
//!   successor variable `y_{a,b}` (intended `0/1`: `1` iff `b` immediately
//!   follows `a` in the trail).
//! - For every arc `a in A`, introduce integer variables `s_a`, `e_a`
//!   (`0/1`: `s_a = 1` iff `a` is first, `e_a = 1` iff `a` is last) and an
//!   integer position variable `u_a` (intended value `0..m-1`).
//! - The predecessor and successor equalities, together with the unique start
//!   and unique end constraints and Miller--Tucker--Zemlin-style ordering
//!   constraints, force any feasible solution to encode a directed trail that
//!   uses every arc occurrence exactly once.
//!
//! The empty-arc instance (`m = 0`) maps to the empty ILP with no variables
//! and no constraints, which is vacuously feasible.
//!
//! References: Ebert, "Computing Eulerian trails," IPL 28(2):93--97 (1988);
//! Bang-Jensen and Gutin, *Digraphs: Theory, Algorithms and Applications*,
//! 2nd ed., Springer (2009).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::EulerianPath;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing EulerianPath to `ILP<i32>`.
///
/// Variable layout (all in the non-negative integer domain, with explicit
/// upper bounds enforcing the intended `0/1` and `0..m-1` ranges):
/// - `y_{a,b}` at index `k` for the `k`-th compatible pair in `pairs` order
///   (a single sweep over `(a, b)` with `a, b in 0..m`, `a != b`, in
///   row-major order),
/// - `s_a` at index `p + a` for `a in 0..m`,
/// - `e_a` at index `p + m + a` for `a in 0..m`,
/// - `u_a` at index `p + 2 * m + a` for `a in 0..m`,
///
/// where `p = pairs.len()` is the number of compatible ordered pairs.
#[derive(Debug, Clone)]
pub struct ReductionEulerianPathToILP {
    target: ILP<i32>,
    /// Compatible ordered pairs `(a, b)` in the order their `y_{a,b}` variables
    /// appear in the ILP, for `m > 0`. Empty when `m = 0`.
    pairs: Vec<(usize, usize)>,
    /// Number of arc occurrences in the source instance.
    num_arcs: usize,
}

impl ReductionEulerianPathToILP {
    fn s_idx(&self, a: usize) -> usize {
        self.pairs.len() + a
    }
}

impl ReductionResult for ReductionEulerianPathToILP {
    type Source = EulerianPath;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Decode an ILP assignment into a source arc ordering.
    ///
    /// Reads the unique active start arc (`s_a = 1`) and walks the active
    /// successor relation (`y_{a,b} = 1`) one step at a time, producing an arc
    /// permutation of length `m`. Malformed assignments return an extraction
    /// error instead of fabricating an ordering.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let m = self.num_arcs;
            if m == 0 {
                return Ok(Vec::new());
            }

            // Find the unique active start arc.
            let mut current = match (0..m).find(|&a| target_solution[self.s_idx(a)] == 1) {
                Some(a) => a,
                None => {
                    return Err(crate::rules::ExtractionError::invalid(
                        "ILP witness has no active Eulerian-path start arc",
                    ));
                }
            };

            // Walk the active successor relation, recording each visited arc.
            let mut order = Vec::with_capacity(m);
            let mut visited = vec![false; m];
            order.push(current);
            visited[current] = true;

            for _ in 1..m {
                let next = self
                    .pairs
                    .iter()
                    .enumerate()
                    .find(|&(k, &(a, _))| a == current && target_solution[k] == 1)
                    .map(|(_, &(_, b))| b);

                match next {
                    Some(b) if !visited[b] => {
                        order.push(b);
                        visited[b] = true;
                        current = b;
                    }
                    _ => {
                        return Err(crate::rules::ExtractionError::invalid(format!(
                            "ILP witness has no unvisited successor for arc {current}",
                        )));
                    }
                }
            }
            order
        })
    }
}

/// Enumerate compatible ordered pairs `(a, b)` with `a != b` and
/// `head(a) = tail(b)`. The order is `a`-major then `b`-major, matching the
/// nested-loop construction below.
fn compatible_pairs(arcs: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for (a, &(_, head_a)) in arcs.iter().enumerate() {
        for (b, &(tail_b, _)) in arcs.iter().enumerate() {
            if a != b && tail_b == head_a {
                pairs.push((a, b));
            }
        }
    }
    pairs
}

#[reduction(
    overhead = {
        num_vars = "3 * num_arcs + num_arcs * num_arcs",
        num_constraints = "5 * num_arcs + 2 * num_arcs * num_arcs + 2",
    }
)]
impl ReduceTo<ILP<i32>> for EulerianPath {
    type Result = ReductionEulerianPathToILP;

    fn reduce_to(&self) -> Self::Result {
        let arcs = self.graph().arcs();
        let m = arcs.len();

        // Empty-arc instance: vacuously feasible empty ILP.
        if m == 0 {
            let target = ILP::new(0, Vec::new(), Vec::new(), ObjectiveSense::Minimize);
            return ReductionEulerianPathToILP {
                target,
                pairs: Vec::new(),
                num_arcs: 0,
            };
        }

        let pairs = compatible_pairs(&arcs);
        let p = pairs.len();
        let num_vars = p + 3 * m;

        // Index helpers (mirroring the struct's accessors, but we need them
        // before the struct exists).
        let y_idx = |k: usize| -> usize { k };
        let s_idx = |a: usize| -> usize { p + a };
        let e_idx = |a: usize| -> usize { p + m + a };
        let u_idx = |a: usize| -> usize { p + 2 * m + a };

        // Inverse index: for each arc `a`, list `k` indices of pairs ending in
        // `a` (`pairs[k].1 == a`) and pairs starting at `a` (`pairs[k].0 == a`).
        let mut incoming: Vec<Vec<usize>> = vec![Vec::new(); m];
        let mut outgoing: Vec<Vec<usize>> = vec![Vec::new(); m];
        for (k, &(a, b)) in pairs.iter().enumerate() {
            outgoing[a].push(k);
            incoming[b].push(k);
        }

        let mut constraints: Vec<LinearConstraint> = Vec::new();

        // (1) Predecessor equality: s_a + sum_{(b,a) in P} y_{b,a} = 1.
        // (2) Successor equality:   e_a + sum_{(a,b) in P} y_{a,b} = 1.
        for a in 0..m {
            let mut pred_terms: Vec<(usize, f64)> = vec![(s_idx(a), 1.0)];
            for &k in &incoming[a] {
                pred_terms.push((y_idx(k), 1.0));
            }
            constraints.push(LinearConstraint::eq(pred_terms, 1.0));

            let mut succ_terms: Vec<(usize, f64)> = vec![(e_idx(a), 1.0)];
            for &k in &outgoing[a] {
                succ_terms.push((y_idx(k), 1.0));
            }
            constraints.push(LinearConstraint::eq(succ_terms, 1.0));
        }

        // (3) Binary upper bounds on start / end variables, and position
        //     upper bound on `u_a`.
        for a in 0..m {
            constraints.push(LinearConstraint::le(vec![(s_idx(a), 1.0)], 1.0));
            constraints.push(LinearConstraint::le(vec![(e_idx(a), 1.0)], 1.0));
            constraints.push(LinearConstraint::le(
                vec![(u_idx(a), 1.0)],
                (m as f64) - 1.0,
            ));
        }

        // (4) Binary upper bounds on successor variables.
        // (5) Order consistency (MTZ): u_b >= u_a + 1 - m * (1 - y_{a,b})
        //     i.e.  u_a - u_b + m * y_{a,b} <= m - 1.
        for (k, &(a, b)) in pairs.iter().enumerate() {
            constraints.push(LinearConstraint::le(vec![(y_idx(k), 1.0)], 1.0));
            constraints.push(LinearConstraint::le(
                vec![(u_idx(a), 1.0), (u_idx(b), -1.0), (y_idx(k), m as f64)],
                (m as f64) - 1.0,
            ));
        }

        // (6) Unique start: sum_a s_a = 1.
        // (7) Unique end:   sum_a e_a = 1.
        let start_sum: Vec<(usize, f64)> = (0..m).map(|a| (s_idx(a), 1.0)).collect();
        let end_sum: Vec<(usize, f64)> = (0..m).map(|a| (e_idx(a), 1.0)).collect();
        constraints.push(LinearConstraint::eq(start_sum, 1.0));
        constraints.push(LinearConstraint::eq(end_sum, 1.0));

        let target = ILP::new(num_vars, constraints, Vec::new(), ObjectiveSense::Minimize);

        ReductionEulerianPathToILP {
            target,
            pairs,
            num_arcs: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "eulerianpath_to_ilp",
        build: || {
            // Canonical issue #1025 instance: V = {0,1,2},
            // A = [(0,1), (0,1), (1,2), (2,0)] (parallel arcs a_0, a_1).
            // Witness ordering (a_0, a_2, a_3, a_1) traces 0->1->2->0->1.
            let source =
                EulerianPath::new(DirectedGraph::new(3, vec![(0, 1), (0, 1), (1, 2), (2, 0)]));
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/eulerianpath_ilp.rs"]
mod tests;

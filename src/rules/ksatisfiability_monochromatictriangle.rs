//! Reduction from 3-SAT to edge colouring without monochromatic triangles.
//!
//! The construction uses the signal-sender framework of Burr, Erdős and Lovász
//! (On graphs of Ramsey type, 1976). Here an equality sender is two K5 copies
//! sharing a private triangle: their two complementary edges must have the same
//! colour. NAE clause triangles are linked to disjoint literal signal edges by
//! these senders, so their colour constraints actually encode the formula.
//!
//! See the full sender, composition and extraction proof in reductions.typ.

use crate::models::formula::{KSatisfiability, NAESatisfiability, Satisfiability};
use crate::models::graph::MonochromaticTriangle;
use crate::reduction;
use crate::rules::sat_helpers::SatVariableAllocator;
use crate::rules::satisfiability_naesatisfiability::ReductionSATToNAESAT;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::variant::K3;

/// Append the fifteen nonterminal edges of an equality sender.
///
/// The terminal edges already exist and have four distinct endpoints. The
/// three vertices starting at `private` belong only to this sender. Both K5
/// copies induce the same private triangle; their complementary edges have
/// its majority colour in every valid colouring. Either equal colour extends.
fn add_equality_sender(
    edges: &mut Vec<(usize, usize)>,
    first: (usize, usize),
    second: (usize, usize),
    private: usize,
) {
    let [u, v, w] = [private, private + 1, private + 2];
    edges.extend([(u, v), (u, w), (v, w)]);
    for endpoint in [first.0, first.1, second.0, second.1] {
        edges.extend([(endpoint, u), (endpoint, v), (endpoint, w)]);
    }
}

/// Result of reducing KSatisfiability<K3> to MonochromaticTriangle.
#[derive(Debug, Clone)]
pub struct Reduction3SATToMonochromaticTriangle {
    target: MonochromaticTriangle<SimpleGraph>,
    nae_reduction: ReductionSATToNAESAT,
}

impl ReductionResult for Reduction3SATToMonochromaticTriangle {
    type Source = KSatisfiability<K3>;
    type Target = MonochromaticTriangle<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        let nae_solution = (0..self.nae_reduction.target_problem().num_vars())
            .map(|index| target_solution[2 * index])
            .collect();
        // Reuse the formal SAT -> NAE extraction (including sentinel
        // normalization); no assignment search or speculative complement.
        self.nae_reduction.extract_solution(&nae_solution)
    }
}

#[reduction(
    transform = upper_bound {
        num_vertices = "16 * num_vars + 40 * num_clauses + 16",
        num_edges = "50 * num_vars + 146 * num_clauses + 50",
        num_triangles = "58 * num_vars + 174 * num_clauses + 58",
    }
)]
impl ReduceTo<MonochromaticTriangle<SimpleGraph>> for KSatisfiability<K3> {
    type Result = Reduction3SATToMonochromaticTriangle;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let sat = ReduceTo::<Satisfiability>::reduce_to(self)?;
        let nae_reduction = ReduceTo::<NAESatisfiability>::reduce_to(sat.target_problem())?;
        let nae = nae_reduction.target_problem();
        let auxiliary_count = nae.clauses().iter().filter(|c| c.len() == 4).count();
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<Self, MonochromaticTriangle<SimpleGraph>>(
                "computing signal-sender graph size",
            )
        };
        let variable_count = nae
            .num_vars()
            .checked_add(auxiliary_count)
            .ok_or_else(overflow)?;
        let triple_count = variable_count
            .checked_add(nae.num_clauses())
            .and_then(|n| n.checked_add(auxiliary_count))
            .ok_or_else(overflow)?;
        let num_vertices = variable_count
            .checked_mul(4)
            .and_then(|n| triple_count.checked_mul(12).and_then(|t| n.checked_add(t)))
            .ok_or_else(overflow)?;
        let num_edges = variable_count
            .checked_mul(2)
            .and_then(|n| triple_count.checked_mul(48).and_then(|t| n.checked_add(t)))
            .ok_or_else(overflow)?;

        let mut variables =
            SatVariableAllocator::new("KSatisfiability -> MonochromaticTriangle", nae.num_vars())
                .map_err(
                crate::rules::ReductionError::construction::<
                    Self,
                    MonochromaticTriangle<SimpleGraph>,
                >,
            )?;
        let mut triples = Vec::with_capacity(triple_count);
        for index in 0..variable_count {
            let literal = i64::try_from(index + 1).map_err(|_| overflow())?;
            triples.push([literal, literal, -literal]);
        }
        for clause in nae.clauses() {
            let literals = &clause.literals;
            if literals.len() == 4 {
                let z = variables.allocate().map_err(
                    crate::rules::ReductionError::construction::<
                        Self,
                        MonochromaticTriangle<SimpleGraph>,
                    >,
                )?;
                triples.push([literals[0], literals[1], z]);
                triples.push([-z, literals[2], literals[3]]);
            } else {
                // NAE(a,b) = NAE(a,b,b). Formal SAT -> NAE produces only
                // lengths 2, 3 and 4, including NAE(s,s) for an empty clause.
                triples.push([literals[0], literals[1], literals[literals.len() - 1]]);
            }
        }

        let mut edges = Vec::with_capacity(num_edges);
        for index in 0..variable_count {
            edges.extend([(4 * index, 4 * index + 1), (4 * index + 2, 4 * index + 3)]);
        }
        // All following offsets are bounded by the checked total above.
        let mut next_vertex = 4 * variable_count;
        for triple in triples {
            let [a, b, c] = [next_vertex, next_vertex + 1, next_vertex + 2];
            next_vertex += 3;
            let sides = [(a, b), (a, c), (b, c)];
            edges.extend(sides);
            for (literal, side) in triple.into_iter().zip(sides) {
                let index = usize::try_from(literal.unsigned_abs() - 1)
                    .expect("validated SAT variable index fits usize");
                let endpoint = 4 * index + if literal < 0 { 2 } else { 0 };
                add_equality_sender(&mut edges, (endpoint, endpoint + 1), side, next_vertex);
                next_vertex += 3;
            }
        }
        debug_assert_eq!(next_vertex, num_vertices);
        debug_assert_eq!(edges.len(), num_edges);
        let target = MonochromaticTriangle::new(SimpleGraph::new(num_vertices, edges));
        Ok(Reduction3SATToMonochromaticTriangle {
            target,
            nae_reduction,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;
    use crate::solvers::ILPSolver;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_monochromatictriangle",
        build: || {
            let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
            let reduction = ReduceTo::<MonochromaticTriangle<SimpleGraph>>::reduce_to(&source)
                .expect("reduction should succeed");
            let target_config = ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("canonical target must be colourable");
            let source_config = reduction.extract_solution(&target_config).unwrap();
            crate::example_db::specs::assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config: serde_json::to_value(source_config)
                        .expect("solution serialization must succeed"),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_monochromatictriangle.rs"]
mod tests;

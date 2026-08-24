//! Reduction from KSatisfiability (3-SAT) to AcyclicPartition.
//!
//! The repository's `AcyclicPartition` model is the weighted quotient-DAG
//! partition problem, not the "partition vertices into two induced acyclic
//! subgraphs" variant described in issue #247. The implemented rule therefore
//! uses a witness-preserving composition:
//! 3-SAT -> SubsetSum -> Partition -> AcyclicPartition.

use crate::models::formula::KSatisfiability;
use crate::models::graph::AcyclicPartition;
use crate::models::misc::{Partition, SubsetSum};
use crate::reduction;
use crate::rules::ksatisfiability_subsetsum::Reduction3SATToSubsetSum;
use crate::rules::subsetsum_partition::ReductionSubsetSumToPartition;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::DirectedGraph;
use crate::variant::K3;

#[derive(Debug, Clone)]
struct ReductionPartitionToAcyclicPartition {
    target: AcyclicPartition<i64>,
    source_num_elements: usize,
    source_vertex: usize,
    sink_vertex: usize,
}

impl ReductionPartitionToAcyclicPartition {
    fn new(source: &Partition) -> Result<Self, crate::rules::ReductionError> {
        let num_elements = source.num_elements();
        let source_vertex = num_elements;
        let sink_vertex = num_elements + 1;
        let total_sum = source.total_sum();

        let arcs: Vec<(usize, usize)> = (0..num_elements)
            .flat_map(|item| [(source_vertex, item), (item, sink_vertex)])
            .collect();

        let map_overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<
                KSatisfiability<K3>,
                AcyclicPartition<i64>,
            >(operation)
        };
        let mut vertex_weights: Vec<i64> = source
            .sizes()
            .iter()
            .copied()
            .map(|size| {
                let doubled = size
                    .checked_mul(2)
                    .ok_or_else(|| map_overflow("doubling an item weight"))?;
                Ok(doubled)
            })
            .collect::<Result<_, _>>()?;

        let endpoint_weight = total_sum
            .checked_add(1)
            .ok_or_else(|| map_overflow("computing the endpoint weight"))?;
        let even_prefix = total_sum - (total_sum % 2);
        let weight_bound = endpoint_weight
            .checked_add(even_prefix)
            .ok_or_else(|| map_overflow("computing the weight bound"))?;

        vertex_weights.push(endpoint_weight);
        vertex_weights.push(endpoint_weight);

        let arc_costs = vec![1; arcs.len()];
        let target = AcyclicPartition::new(
            DirectedGraph::new(num_elements + 2, arcs),
            vertex_weights,
            arc_costs,
            weight_bound,
            i64::try_from(num_elements)
                .map_err(|_| map_overflow("converting the cost bound to i64"))?,
        );

        Ok(Self {
            target,
            source_num_elements: num_elements,
            source_vertex,
            sink_vertex,
        })
    }
}

impl ReductionResult for ReductionPartitionToAcyclicPartition {
    type Source = Partition;
    type Target = AcyclicPartition<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let source_label = target_solution[self.source_vertex];
            let sink_label = target_solution[self.sink_vertex];
            if source_label == sink_label {
                return Err(crate::rules::ExtractionError::invalid(
                    "target partition places the source and sink in the same block",
                ));
            }

            (0..self.source_num_elements)
                .map(|item| usize::from(target_solution[item] == sink_label))
                .collect()
        })
    }
}

/// Result of reducing KSatisfiability<K3> to AcyclicPartition<i64>.
#[derive(Debug, Clone)]
pub struct Reduction3SATToAcyclicPartition {
    sat_to_subset: Reduction3SATToSubsetSum,
    subset_to_partition: ReductionSubsetSumToPartition,
    partition_to_acyclic: ReductionPartitionToAcyclicPartition,
}

impl ReductionResult for Reduction3SATToAcyclicPartition {
    type Source = KSatisfiability<K3>;
    type Target = AcyclicPartition<i64>;

    fn target_problem(&self) -> &Self::Target {
        self.partition_to_acyclic.target_problem()
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let partition_solution = self
                .partition_to_acyclic
                .extract_solution(target_solution)?;
            let subset_solution = self
                .subset_to_partition
                .extract_solution(&partition_solution)?;
            self.sat_to_subset.extract_solution(&subset_solution)?
        })
    }
}

#[reduction(
    size = exact {
        num_vertices = "2 * num_vars + 2 * num_clauses + 3",
        num_arcs = "4 * num_vars + 4 * num_clauses + 2",
    }
)]
impl ReduceTo<AcyclicPartition<i64>> for KSatisfiability<K3> {
    type Result = Reduction3SATToAcyclicPartition;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let sat_to_subset = <KSatisfiability<K3> as ReduceTo<SubsetSum>>::reduce_to(self)?;
        let subset_to_partition =
            <SubsetSum as ReduceTo<Partition>>::reduce_to(sat_to_subset.target_problem())?;
        let partition_to_acyclic =
            ReductionPartitionToAcyclicPartition::new(subset_to_partition.target_problem())?;

        Ok(Reduction3SATToAcyclicPartition {
            sat_to_subset,
            subset_to_partition,
            partition_to_acyclic,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_acyclicpartition",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, AcyclicPartition<i64>>(
                KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]),
                SolutionPair {
                    source_config: vec![1],
                    target_config: vec![1, 0, 1, 1, 0, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_acyclicpartition.rs"]
mod tests;

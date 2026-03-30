//! Equilibrium Point problem implementation.
//!
//! Given n players, polynomial payoff functions F_i, and finite strategy sets M_i,
//! determine whether there exists a pure-strategy Nash equilibrium: an assignment
//! y = (y_1, ..., y_n) with y_i ∈ M_i such that for every player i,
//! F_i(y) ≥ F_i(y with y_i replaced by any y' ∈ M_i).

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "EquilibriumPoint",
        display_name: "Equilibrium Point",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Decide whether a pure-strategy Nash equilibrium exists for a multi-player game with polynomial payoff functions",
        fields: &[
            FieldInfo {
                name: "polynomials",
                type_name: "Vec<Vec<Vec<i64>>>",
                description: "polynomials[i] is a list of affine factors for F_i; each factor [a0,a1,...,an] represents a0 + a1*x1 + ... + an*xn",
            },
            FieldInfo {
                name: "range_sets",
                type_name: "Vec<Vec<i64>>",
                description: "range_sets[i] is the finite strategy set M_i for player i",
            },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "EquilibriumPoint",
        fields: &["num_players"],
    }
}

/// Equilibrium Point problem.
///
/// Given n players, each with a finite strategy set M_i and a polynomial payoff
/// function F_i, decide whether there exists a pure-strategy Nash equilibrium:
/// an assignment y = (y_1, ..., y_n) with y_i ∈ M_i such that no player can
/// improve their payoff by unilaterally deviating.
///
/// F_i is expressed as a product of affine factors. Each factor is represented
/// as a coefficient vector `[a0, a1, ..., an]` evaluating to
/// `a0 + a1*y_1 + ... + an*y_n`.
///
/// # Configuration
///
/// `config[i]` is an index into `range_sets[i]`; the assignment is
/// `y_i = range_sets[i][config[i]]`.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::EquilibriumPoint;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 3 players, M_i = {0, 1} for all i.
/// // F1 = x1*x2*x3, F2 = (1-x1)*x2, F3 = x1*(1-x3)
/// let polynomials = vec![
///     vec![vec![0,1,0,0], vec![0,0,1,0], vec![0,0,0,1]],
///     vec![vec![1,-1,0,0], vec![0,0,1,0]],
///     vec![vec![0,1,0,0], vec![1,0,0,-1]],
/// ];
/// let range_sets = vec![vec![0,1], vec![0,1], vec![0,1]];
/// let problem = EquilibriumPoint::new(polynomials, range_sets).unwrap();
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem);
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct EquilibriumPoint {
    /// polynomials[i] is a list of affine factors for F_i.
    /// F_i(y) = product over all factors of (a0 + a1*y1 + ... + an*yn).
    polynomials: Vec<Vec<Vec<i64>>>,
    /// range_sets[i] is the finite strategy set M_i for player i.
    range_sets: Vec<Vec<i64>>,
}

impl EquilibriumPoint {
    fn validate_inputs(
        polynomials: &[Vec<Vec<i64>>],
        range_sets: &[Vec<i64>],
    ) -> Result<(), String> {
        let n = polynomials.len();
        if range_sets.len() != n {
            return Err(format!(
                "polynomials has {n} entries but range_sets has {} entries; lengths must match",
                range_sets.len()
            ));
        }
        for (i, m) in range_sets.iter().enumerate() {
            if m.is_empty() {
                return Err(format!("range_sets[{i}] must be non-empty"));
            }
        }
        // Each factor must have length n+1 (constant + one coefficient per player).
        let expected_factor_len = n + 1;
        for (i, factors) in polynomials.iter().enumerate() {
            for (j, factor) in factors.iter().enumerate() {
                if factor.len() != expected_factor_len {
                    return Err(format!(
                        "polynomials[{i}][{j}] has {} coefficients but expected {expected_factor_len} (1 + num_players)",
                        factor.len()
                    ));
                }
            }
        }
        Ok(())
    }

    /// Create a new `EquilibriumPoint` instance, returning an error on invalid input.
    pub fn new(polynomials: Vec<Vec<Vec<i64>>>, range_sets: Vec<Vec<i64>>) -> Result<Self, String> {
        Self::validate_inputs(&polynomials, &range_sets)?;
        Ok(Self {
            polynomials,
            range_sets,
        })
    }

    /// Get the number of players.
    pub fn num_players(&self) -> usize {
        self.polynomials.len()
    }

    /// Get the polynomial factor lists.
    pub fn polynomials(&self) -> &[Vec<Vec<i64>>] {
        &self.polynomials
    }

    /// Get the strategy sets.
    pub fn range_sets(&self) -> &[Vec<i64>] {
        &self.range_sets
    }

    /// Evaluate F_i at a given assignment y (as i64 slice).
    ///
    /// Returns the product of all affine factors for player i.
    fn eval_payoff(&self, player: usize, assignment: &[i64]) -> i64 {
        let factors = &self.polynomials[player];
        if factors.is_empty() {
            return 0;
        }
        factors.iter().fold(1i64, |prod, coeffs| {
            // coeffs[0] + coeffs[1]*y_1 + ... + coeffs[n]*y_n
            let val: i64 = coeffs[0]
                + coeffs[1..]
                    .iter()
                    .zip(assignment.iter())
                    .map(|(&c, &y)| c * y)
                    .sum::<i64>();
            prod * val
        })
    }
}

#[derive(Deserialize)]
struct EquilibriumPointData {
    polynomials: Vec<Vec<Vec<i64>>>,
    range_sets: Vec<Vec<i64>>,
}

impl<'de> Deserialize<'de> for EquilibriumPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = EquilibriumPointData::deserialize(deserializer)?;
        Self::new(data.polynomials, data.range_sets).map_err(D::Error::custom)
    }
}

impl Problem for EquilibriumPoint {
    const NAME: &'static str = "EquilibriumPoint";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.range_sets.iter().map(|m| m.len()).collect()
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        let n = self.num_players();
        if config.len() != n {
            return Or(false);
        }
        // Validate config indices are in-bounds.
        for (i, &idx) in config.iter().enumerate() {
            if idx >= self.range_sets[i].len() {
                return Or(false);
            }
        }

        // Extract assignment y_i = range_sets[i][config[i]].
        let assignment: Vec<i64> = config
            .iter()
            .enumerate()
            .map(|(i, &idx)| self.range_sets[i][idx])
            .collect();

        // Check best-response condition for each player.
        for i in 0..n {
            let current_payoff = self.eval_payoff(i, &assignment);
            // Try every y' in M_i for player i.
            let mut best_response_satisfied = true;
            for &alt in &self.range_sets[i] {
                if alt == assignment[i] {
                    continue;
                }
                // Build alternative assignment with player i using alt.
                let mut alt_assignment = assignment.clone();
                alt_assignment[i] = alt;
                let alt_payoff = self.eval_payoff(i, &alt_assignment);
                if alt_payoff > current_payoff {
                    best_response_satisfied = false;
                    break;
                }
            }
            if !best_response_satisfied {
                return Or(false);
            }
        }
        Or(true)
    }
}

crate::declare_variants! {
    default EquilibriumPoint => "2^num_players",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 3 players, M_i = {0, 1} for all i.
    // F1 = x1*x2*x3:        factors [[0,1,0,0],[0,0,1,0],[0,0,0,1]]
    // F2 = (1-x1)*x2:       factors [[1,-1,0,0],[0,0,1,0]]
    // F3 = x1*(1-x3):       factors [[0,1,0,0],[1,0,0,-1]]
    //
    // config [0,1,0] → assignment (0,1,0).
    // F1(0,1,0) = 0*1*0 = 0. Deviations for player 1: y'=1 → F1(1,1,0)=0. No improvement.
    // F2(0,1,0) = (1-0)*1 = 1. Deviations for player 2: y'=0 → F2(0,0,0)=0. No improvement.
    // F3(0,1,0) = 0*(1-0) = 0. Deviations for player 3: y'=1 → F3(0,1,1)=0. No improvement.
    // → (0,1,0) is a Nash equilibrium.
    let polynomials = vec![
        vec![vec![0, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]],
        vec![vec![1, -1, 0, 0], vec![0, 0, 1, 0]],
        vec![vec![0, 1, 0, 0], vec![1, 0, 0, -1]],
    ];
    let range_sets = vec![vec![0, 1], vec![0, 1], vec![0, 1]];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "equilibrium_point",
        instance: Box::new(EquilibriumPoint::new(polynomials, range_sets).unwrap()),
        optimal_config: vec![0, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/equilibrium_point.rs"]
mod tests;

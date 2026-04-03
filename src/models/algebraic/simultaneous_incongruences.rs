//! Simultaneous Incongruences.
//!
//! Given forbidden residue classes and an upper bound `N`, determine whether
//! there exists an integer `x` with `1 <= x <= N` that avoids every forbidden
//! residue class.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SimultaneousIncongruences",
        display_name: "Simultaneous Incongruences",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find an integer in [1, N] avoiding a list of forbidden residue classes",
        fields: &[
            FieldInfo { name: "moduli", type_name: "Vec<u64>", description: "The moduli m_i defining each incongruence x != r_i (mod m_i)" },
            FieldInfo { name: "residues", type_name: "Vec<u64>", description: "The forbidden residues r_i, stored in canonical form 0 <= r_i < m_i" },
            FieldInfo { name: "bound", type_name: "u64", description: "Upper bound N on the searched integer x, with 1 <= x <= N" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "SimultaneousIncongruences",
        fields: &["bound", "num_incongruences"],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimultaneousIncongruences {
    moduli: Vec<u64>,
    residues: Vec<u64>,
    bound: u64,
}

impl SimultaneousIncongruences {
    pub fn new(moduli: Vec<u64>, residues: Vec<u64>, bound: u64) -> Self {
        assert_eq!(
            moduli.len(),
            residues.len(),
            "moduli and residues must have the same length"
        );
        assert!(
            usize::try_from(bound).is_ok(),
            "bound must fit in usize for brute-force enumeration"
        );
        for (index, (&modulus, &residue)) in moduli.iter().zip(&residues).enumerate() {
            assert!(modulus > 0, "modulus at index {index} must be positive");
            assert!(
                residue < modulus,
                "residue at index {index} must satisfy residue < modulus"
            );
        }

        Self {
            moduli,
            residues,
            bound,
        }
    }

    pub fn moduli(&self) -> &[u64] {
        &self.moduli
    }

    pub fn residues(&self) -> &[u64] {
        &self.residues
    }

    pub fn bound(&self) -> u64 {
        self.bound
    }

    pub fn num_incongruences(&self) -> usize {
        self.moduli.len()
    }
}

impl Problem for SimultaneousIncongruences {
    const NAME: &'static str = "SimultaneousIncongruences";
    type Value = Or;

    fn dims(&self) -> Vec<usize> {
        vec![usize::try_from(self.bound).expect("bound must fit in usize")]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        let Some(&offset) = config.first() else {
            return Or(false);
        };

        let bound = usize::try_from(self.bound).expect("bound must fit in usize");
        if config.len() != 1 || offset >= bound {
            return Or(false);
        }

        let x = offset as u64 + 1;
        Or(self
            .moduli
            .iter()
            .zip(&self.residues)
            .all(|(&modulus, &residue)| x % modulus != residue))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default SimultaneousIncongruences => "bound",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "simultaneous_incongruences",
        instance: Box::new(SimultaneousIncongruences::new(
            vec![2, 3, 5, 7],
            vec![0, 1, 2, 3],
            210,
        )),
        optimal_config: vec![4],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/simultaneous_incongruences.rs"]
mod tests;

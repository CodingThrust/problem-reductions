//! Variant cast reductions for KSatisfiability.

use crate::impl_variant_reduction;
use crate::models::formula::KSatisfiability;
use crate::variant::{K2, K3, KN};

impl_variant_reduction!(
    KSatisfiability,
    <K2> => <KN>,
    id: "ksatisfiability_to_ksatisfiability_k2_kn",
    fields: [num_vars, num_clauses],
    |src| KSatisfiability::new_allow_less(src.num_vars(), src.clauses().to_vec())
);

impl_variant_reduction!(
    KSatisfiability,
    <K3> => <KN>,
    id: "ksatisfiability_to_ksatisfiability_k3_kn",
    fields: [num_vars, num_clauses],
    |src| KSatisfiability::new_allow_less(src.num_vars(), src.clauses().to_vec())
);

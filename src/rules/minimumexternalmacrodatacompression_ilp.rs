//! Reduction from MinimumExternalMacroDataCompression to ILP (Integer Linear Programming).
//!
//! The EMDC problem is formulated as a binary ILP using a flow-on-DAG partition
//! model for the compressed string, combined with dictionary assignment variables.
//!
//! **Variables (all binary):**
//! - `d[j][c]`: D-slot j contains symbol c (j=0..n-1, c=0..k-1)
//! - `d_used[j]`: D-slot j is used (j=0..n-1)
//! - `lit[i]`: position i is covered by a literal in C (i=0..n-1)
//! - `ptr[i][l][d_start]`: segment [i, i+l) is a pointer referencing D[d_start..d_start+l]
//! - Flow conservation ensures positions 0..n are partitioned into segments.
//!
//! **Objective:** minimize |D| + |C_literals| + h * |C_pointers|
//! = sum d_used[j] + sum lit[i] + h * sum ptr[i][l][d_start]

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::MinimumExternalMacroDataCompression;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Index layout for ILP variables.
#[derive(Debug, Clone)]
struct VarLayout {
    n: usize,
    k: usize,
    /// Offset of d[j][c] block: index = d_offset + j * k + c
    d_offset: usize,
    /// Offset of d_used[j] block: index = d_used_offset + j
    d_used_offset: usize,
    /// Offset of lit[i] block: index = lit_offset + i
    lit_offset: usize,
    /// Offset of ptr variables, stored as a flat list.
    /// Each entry in `ptr_triples` is (i, l, d_start) and its var index = ptr_offset + idx.
    ptr_offset: usize,
    /// The (i, l, d_start) triples in order.
    ptr_triples: Vec<(usize, usize, usize)>,
    /// Total number of variables.
    total_vars: usize,
}

impl VarLayout {
    fn new(n: usize, k: usize) -> Self {
        let d_offset = 0;
        let d_used_offset = d_offset + n * k;
        let lit_offset = d_used_offset + n;
        let ptr_offset = lit_offset + n;

        // Enumerate all valid (i, l, d_start) triples
        let mut ptr_triples = Vec::new();
        for i in 0..n {
            for l in 1..=(n - i) {
                for d_start in 0..=(n - l) {
                    ptr_triples.push((i, l, d_start));
                }
            }
        }

        let total_vars = ptr_offset + ptr_triples.len();
        Self {
            n,
            k,
            d_offset,
            d_used_offset,
            lit_offset,
            ptr_offset,
            ptr_triples,
            total_vars,
        }
    }

    fn d_var(&self, j: usize, c: usize) -> usize {
        self.d_offset + j * self.k + c
    }

    fn d_used_var(&self, j: usize) -> usize {
        self.d_used_offset + j
    }

    fn lit_var(&self, i: usize) -> usize {
        self.lit_offset + i
    }

    fn ptr_var(&self, i: usize, l: usize, d_start: usize) -> usize {
        // Find the index of (i, l, d_start) in ptr_triples
        let idx = self
            .ptr_triples
            .iter()
            .position(|&(pi, pl, pd)| pi == i && pl == l && pd == d_start)
            .expect("invalid ptr triple");
        self.ptr_offset + idx
    }

    /// Get all ptr variable indices for segments starting at position i with length l.
    fn ptr_vars_for_segment(&self, i: usize, l: usize) -> Vec<usize> {
        self.ptr_triples
            .iter()
            .enumerate()
            .filter(|(_, &(pi, pl, _))| pi == i && pl == l)
            .map(|(idx, _)| self.ptr_offset + idx)
            .collect()
    }
}

/// Result of reducing MinimumExternalMacroDataCompression to ILP.
#[derive(Debug, Clone)]
pub struct ReductionEMDCToILP {
    target: ILP<bool>,
    /// Variable layout for solution extraction.
    layout: VarLayout,
    /// The source string (needed for extract_solution).
    source_string: Vec<usize>,
    /// Alphabet size.
    alphabet_size: usize,
}

impl ReductionResult for ReductionEMDCToILP {
    type Source = MinimumExternalMacroDataCompression;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.layout.n;
        let k = self.alphabet_size;
        let empty = k; // empty marker

        // Build D-slots
        let mut d_slots = vec![empty; n];
        for j in 0..n {
            if target_solution[self.layout.d_used_var(j)] == 1 {
                for c in 0..k {
                    if target_solution[self.layout.d_var(j, c)] == 1 {
                        d_slots[j] = c;
                        break;
                    }
                }
            }
        }

        // Walk through active segments to build C-slots
        let mut c_slots = vec![empty; n];
        let mut c_pos = 0;
        let mut pos = 0;
        while pos < n {
            // Check if lit[pos] = 1
            if target_solution[self.layout.lit_var(pos)] == 1 {
                // Literal at position pos
                c_slots[c_pos] = self.source_string[pos];
                c_pos += 1;
                pos += 1;
                continue;
            }
            // Check for an active pointer starting at pos
            let mut found = false;
            for l in 1..=(n - pos) {
                for d_start in 0..=(n - l) {
                    let var_idx = self.layout.ptr_var(pos, l, d_start);
                    if target_solution[var_idx] == 1 {
                        // Encode pointer (d_start, l) as EMDC pointer index
                        let ptr_idx = encode_pointer(n, d_start, l);
                        c_slots[c_pos] = k + 1 + ptr_idx;
                        c_pos += 1;
                        pos += l;
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                // Should not happen with a valid ILP solution
                pos += 1;
            }
        }

        // Combine D-slots and C-slots
        let mut config = d_slots;
        config.extend(c_slots);
        config
    }
}

/// Encode a pointer (start, len) into the EMDC pointer index.
/// Pointers enumerate: (0,1),(0,2),...,(0,n), (1,1),(1,2),...,(1,n-1), ...
fn encode_pointer(n: usize, start: usize, len: usize) -> usize {
    let mut idx = 0;
    for s in 0..start {
        idx += n - s;
    }
    idx + len - 1
}

#[reduction(
    overhead = {
        num_vars = "string_length * alphabet_size + 2 * string_length + string_length ^ 3",
        num_constraints = "string_length + string_length * alphabet_size + string_length + string_length + 1 + string_length ^ 3 * string_length",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumExternalMacroDataCompression {
    type Result = ReductionEMDCToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.string_length();
        let k = self.alphabet_size();
        let h = self.pointer_cost();
        let s = self.string();

        // Handle empty string
        if n == 0 {
            let layout = VarLayout::new(0, k);
            let target = ILP::new(0, vec![], vec![], ObjectiveSense::Minimize);
            return ReductionEMDCToILP {
                target,
                layout,
                source_string: vec![],
                alphabet_size: k,
            };
        }

        let layout = VarLayout::new(n, k);
        let num_vars = layout.total_vars;
        let mut constraints = Vec::new();

        // 1. Dictionary one-hot: for each j, sum_c d[j][c] <= 1
        for j in 0..n {
            let terms: Vec<(usize, f64)> = (0..k).map(|c| (layout.d_var(j, c), 1.0)).collect();
            constraints.push(LinearConstraint::le(terms, 1.0));
        }

        // 2. Dictionary linking: d[j][c] <= d_used[j] for all j, c
        for j in 0..n {
            for c in 0..k {
                constraints.push(LinearConstraint::le(
                    vec![(layout.d_var(j, c), 1.0), (layout.d_used_var(j), -1.0)],
                    0.0,
                ));
            }
        }

        // 3. Dictionary contiguous: d_used[j+1] <= d_used[j] for j=0..n-2
        for j in 0..n.saturating_sub(1) {
            constraints.push(LinearConstraint::le(
                vec![
                    (layout.d_used_var(j + 1), 1.0),
                    (layout.d_used_var(j), -1.0),
                ],
                0.0,
            ));
        }

        // 4. Flow conservation on DAG: positions 0..n are nodes.
        // A segment (i, l) contributes to outgoing flow at node i and incoming flow at node i+l.
        // For segment (i, l):
        //   - if l == 1: flow = lit[i] + sum_{d_start} ptr[i][1][d_start]
        //   - if l >= 2: flow = sum_{d_start} ptr[i][l][d_start]
        //
        // Flow constraints:
        // At node 0: sum of outgoing segments = 1
        // At node i (1..n-1): sum of incoming = sum of outgoing
        // At node n: sum of incoming = 1

        // Helper: get all terms for "segment flow" at (i, l)
        // Returns the variable indices with coefficient 1.0
        let segment_terms = |i: usize, l: usize| -> Vec<(usize, f64)> {
            let mut terms = Vec::new();
            if l == 1 {
                terms.push((layout.lit_var(i), 1.0));
            }
            for &var in &layout.ptr_vars_for_segment(i, l) {
                terms.push((var, 1.0));
            }
            terms
        };

        // For each node, compute outgoing and incoming segment terms
        for node in 0..=n {
            let mut all_terms: Vec<(usize, f64)> = Vec::new();

            if node == 0 {
                // sum of outgoing(0, l) = 1
                for l in 1..=n {
                    all_terms.extend(segment_terms(0, l));
                }
                constraints.push(LinearConstraint::eq(all_terms, 1.0));
            } else if node == n {
                // sum of incoming(n) = 1
                // incoming at node n: segments (j, l) where j + l = n
                for j in 0..n {
                    let l = n - j;
                    all_terms.extend(segment_terms(j, l));
                }
                constraints.push(LinearConstraint::eq(all_terms, 1.0));
            } else {
                // node 1..n-1: incoming = outgoing
                // incoming: segments (j, l) where j + l = node
                let mut incoming = Vec::new();
                for j in 0..node {
                    let l = node - j;
                    incoming.extend(segment_terms(j, l));
                }
                // outgoing: segments (node, l) for valid l
                let mut outgoing = Vec::new();
                for l in 1..=(n - node) {
                    outgoing.extend(segment_terms(node, l));
                }
                // incoming - outgoing = 0
                for (var, coef) in incoming {
                    all_terms.push((var, coef));
                }
                for (var, coef) in outgoing {
                    all_terms.push((var, -coef));
                }
                constraints.push(LinearConstraint::eq(all_terms, 0.0));
            }
        }

        // 5. Pointer matching: ptr[i][l][d_start] <= d[d_start+offset][s[i+offset]]
        // for all offset=0..l-1
        for (idx, &(i, l, d_start)) in layout.ptr_triples.iter().enumerate() {
            let ptr_idx = layout.ptr_offset + idx;
            for offset in 0..l {
                let symbol = s[i + offset];
                // ptr[i][l][d_start] <= d[d_start + offset][symbol]
                constraints.push(LinearConstraint::le(
                    vec![
                        (ptr_idx, 1.0),
                        (layout.d_var(d_start + offset, symbol), -1.0),
                    ],
                    0.0,
                ));
            }
        }

        // 6. Literal matching: lit[i] can only be active if position i exists
        // (this is always true for i < n, so no constraint needed).
        // But we do need: if lit[i] = 1, the literal is s[i], which is automatic
        // in the extract_solution. No additional constraint needed because the
        // objective already penalizes literals.

        // Objective: minimize sum d_used[j] + sum lit[i] + h * sum ptr[i][l][d_start]
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for j in 0..n {
            objective.push((layout.d_used_var(j), 1.0));
        }
        for i in 0..n {
            objective.push((layout.lit_var(i), 1.0));
        }
        for (idx, _) in layout.ptr_triples.iter().enumerate() {
            objective.push((layout.ptr_offset + idx, h as f64));
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionEMDCToILP {
            target,
            layout,
            source_string: s.to_vec(),
            alphabet_size: k,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    // s = "ab" (len 2), alphabet {a,b} (size 2), h=2
    // Optimal: uncompressed, D="" C="ab", cost = 0+2+0 = 2
    // Config: D-slots=[2,2], C-slots=[0,1]
    // ILP target_config: all d and d_used = 0, lit[0]=1, lit[1]=1, all ptr = 0
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumexternalmacrodatacompression_to_ilp",
        build: || {
            let source = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
            let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let layout = &reduction.layout;
            let n = 2;
            let k = 2;

            // Build target config: all zeros, then set lit[0]=1, lit[1]=1
            let mut target_config = vec![0usize; layout.total_vars];
            target_config[layout.lit_var(0)] = 1;
            target_config[layout.lit_var(1)] = 1;

            // Verify this is correct
            let source_config = reduction.extract_solution(&target_config);
            debug_assert_eq!(source_config[..n], [k, k]); // D empty
            debug_assert_eq!(source_config[n..], [0, 1]); // C = "ab"

            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumexternalmacrodatacompression_ilp.rs"]
mod tests;

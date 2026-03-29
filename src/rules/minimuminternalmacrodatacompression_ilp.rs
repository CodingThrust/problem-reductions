//! Reduction from MinimumInternalMacroDataCompression to ILP (Integer Linear Programming).
//!
//! The IMDC problem is formulated as a binary ILP using a flow-on-DAG partition
//! model for the compressed string C, where pointers reference earlier segments
//! within C itself.
//!
//! **Variables (all binary):**
//! - `lit[i]`: source position i is covered by a literal in C (i=0..n-1)
//! - `ptr[i][l][r]`: segment [i, i+l) of the source is covered by a pointer in C
//!   that copies from source position r (the first l characters of the decoded
//!   output starting at source position r must equal s[i..i+l])
//! - Flow conservation ensures positions 0..n are partitioned into segments.
//!
//! **Objective:** minimize |C| + (h−1) × pointer_count
//! = (sum lit[i]) + (sum ptr[i][l][r]) + (h−1) × (sum ptr[i][l][r])
//! = (sum lit[i]) + h × (sum ptr[i][l][r])

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::MinimumInternalMacroDataCompression;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Index layout for ILP variables.
#[derive(Debug, Clone)]
struct VarLayout {
    n: usize,
    /// Offset of lit[i] block: index = lit_offset + i
    lit_offset: usize,
    /// Offset of ptr variables, stored as a flat list.
    /// Each entry in `ptr_triples` is (i, l, r) and its var index = ptr_offset + idx.
    ptr_offset: usize,
    /// The (i, l, r) triples in order.
    ptr_triples: Vec<(usize, usize, usize)>,
    /// Total number of variables.
    total_vars: usize,
}

impl VarLayout {
    fn new(n: usize, source_string: &[usize]) -> Self {
        let lit_offset = 0;
        let ptr_offset = lit_offset + n;

        // Enumerate all valid (i, l, r) triples where:
        // - i is the start position in the source (0..n)
        // - l is the segment length (1..n-i)
        // - r is the reference position in the source (0..i), meaning
        //   the pointer copies from source[r..r+l] which must equal source[i..i+l]
        //   AND r < i (pointer references earlier decoded content)
        let mut ptr_triples = Vec::new();
        for i in 0..n {
            for l in 1..=(n - i) {
                for r in 0..i {
                    // The pointer copies from decoded[r..r+l]. With non-overlapping
                    // semantics, decoded has exactly i characters before this pointer,
                    // so we need r + l <= i.
                    if r + l <= i
                        && r + l <= n
                        && source_string[r..r + l] == source_string[i..i + l]
                    {
                        ptr_triples.push((i, l, r));
                    }
                }
            }
        }

        let total_vars = ptr_offset + ptr_triples.len();
        Self {
            n,
            lit_offset,
            ptr_offset,
            ptr_triples,
            total_vars,
        }
    }

    fn lit_var(&self, i: usize) -> usize {
        self.lit_offset + i
    }
}

/// Result of reducing MinimumInternalMacroDataCompression to ILP.
#[derive(Debug, Clone)]
pub struct ReductionIMDCToILP {
    target: ILP<bool>,
    layout: VarLayout,
    source_string: Vec<usize>,
    alphabet_size: usize,
}

impl ReductionResult for ReductionIMDCToILP {
    type Source = MinimumInternalMacroDataCompression;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.layout.n;
        let k = self.alphabet_size;
        let eos = k; // end-of-string marker

        // First pass: collect segments and build source-to-compressed-position map.
        // source_to_c_pos[i] = compressed position that covers source position i.
        let mut source_to_c_pos = vec![0usize; n];
        let mut segments: Vec<(usize, usize, Option<usize>)> = Vec::new(); // (source_start, len, ref_source_pos)
        let mut c_pos = 0;
        let mut pos = 0;

        while pos < n {
            if target_solution[self.layout.lit_var(pos)] == 1 {
                source_to_c_pos[pos] = c_pos;
                segments.push((pos, 1, None));
                c_pos += 1;
                pos += 1;
                continue;
            }
            let mut found = false;
            for (idx, &(i, l, r)) in self.layout.ptr_triples.iter().enumerate() {
                if i == pos && target_solution[self.layout.ptr_offset + idx] == 1 {
                    for offset in 0..l {
                        source_to_c_pos[pos + offset] = c_pos;
                    }
                    segments.push((pos, l, Some(r)));
                    c_pos += 1;
                    pos += l;
                    found = true;
                    break;
                }
            }
            if !found {
                pos += 1;
            }
        }

        // Second pass: build config using source_to_c_pos for pointer references
        let mut config = vec![eos; n];
        for (idx, &(src_start, _len, ref_pos)) in segments.iter().enumerate() {
            match ref_pos {
                None => {
                    config[idx] = self.source_string[src_start];
                }
                Some(r) => {
                    // Pointer references source position r, which is at
                    // compressed position source_to_c_pos[r]
                    config[idx] = k + 1 + source_to_c_pos[r];
                }
            }
        }

        config
    }
}

#[reduction(
    overhead = {
        num_vars = "string_len + string_len ^ 3",
        num_constraints = "string_len + 1 + string_len",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumInternalMacroDataCompression {
    type Result = ReductionIMDCToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.string_len();
        let k = self.alphabet_size();
        let h = self.pointer_cost();
        let s = self.string();

        // Handle empty string
        if n == 0 {
            let layout = VarLayout::new(0, s);
            let target = ILP::new(0, vec![], vec![], ObjectiveSense::Minimize);
            return ReductionIMDCToILP {
                target,
                layout,
                source_string: vec![],
                alphabet_size: k,
            };
        }

        let layout = VarLayout::new(n, s);
        let num_vars = layout.total_vars;
        let mut constraints = Vec::new();

        // Flow conservation on DAG: positions 0..n are nodes.
        // A segment covers source positions [i, i+l).
        // Segments: lit[i] covers [i, i+1), ptr[i][l][r] covers [i, i+l).
        //
        // Flow constraints:
        // At node 0: sum of outgoing segments = 1
        // At node j (1..n-1): sum of incoming = sum of outgoing
        // At node n: sum of incoming = 1

        let segment_terms = |i: usize, l: usize| -> Vec<(usize, f64)> {
            let mut terms = Vec::new();
            if l == 1 {
                terms.push((layout.lit_var(i), 1.0));
            }
            // All ptr variables for segment (i, l, *)
            for (idx, &(pi, pl, _)) in layout.ptr_triples.iter().enumerate() {
                if pi == i && pl == l {
                    terms.push((layout.ptr_offset + idx, 1.0));
                }
            }
            terms
        };

        for node in 0..=n {
            let mut all_terms: Vec<(usize, f64)> = Vec::new();

            if node == 0 {
                for l in 1..=n {
                    all_terms.extend(segment_terms(0, l));
                }
                constraints.push(LinearConstraint::eq(all_terms, 1.0));
            } else if node == n {
                for j in 0..n {
                    let l = n - j;
                    all_terms.extend(segment_terms(j, l));
                }
                constraints.push(LinearConstraint::eq(all_terms, 1.0));
            } else {
                let mut incoming = Vec::new();
                for j in 0..node {
                    let l = node - j;
                    incoming.extend(segment_terms(j, l));
                }
                let mut outgoing = Vec::new();
                for l in 1..=(n - node) {
                    outgoing.extend(segment_terms(node, l));
                }
                for (var, coef) in incoming {
                    all_terms.push((var, coef));
                }
                for (var, coef) in outgoing {
                    all_terms.push((var, -coef));
                }
                constraints.push(LinearConstraint::eq(all_terms, 0.0));
            }
        }

        // Pointer precedence: for ptr[i][l][r], we need r < i (already enforced
        // by the triple enumeration). Additionally, the content at source[r..r+l]
        // must equal source[i..i+l] (also enforced by triple enumeration).
        // No additional constraints needed since we pre-filtered valid triples.

        // Objective: minimize literals + h * pointers
        // = sum lit[i] + h * sum ptr[i][l][r]
        // Since each literal contributes 1 to |C| and each pointer contributes
        // 1 to |C| plus (h-1) to the pointer penalty:
        // cost = |C| + (h-1)*pointers = (lits + ptrs) + (h-1)*ptrs = lits + h*ptrs
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for i in 0..n {
            objective.push((layout.lit_var(i), 1.0));
        }
        for (idx, _) in layout.ptr_triples.iter().enumerate() {
            objective.push((layout.ptr_offset + idx, h as f64));
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionIMDCToILP {
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
    // Optimal: uncompressed C="ab", cost = 2
    // ILP: lit[0]=1, lit[1]=1, no pointers
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimuminternalmacrodatacompression_to_ilp",
        build: || {
            let source = MinimumInternalMacroDataCompression::new(2, vec![0, 1], 2);
            let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let layout = &reduction.layout;

            let mut target_config = vec![0usize; layout.total_vars];
            target_config[layout.lit_var(0)] = 1;
            target_config[layout.lit_var(1)] = 1;

            let source_config = reduction.extract_solution(&target_config);

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
#[path = "../unit_tests/rules/minimuminternalmacrodatacompression_ilp.rs"]
mod tests;

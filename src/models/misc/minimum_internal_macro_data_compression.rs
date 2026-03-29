//! Minimum Internal Macro Data Compression problem implementation.
//!
//! Given an alphabet Σ, a string s ∈ Σ*, and a pointer cost h,
//! find a single compressed string C ∈ (Σ ∪ {pointers})* minimizing the cost
//! |C| + (h−1) × (number of pointer occurrences in C),
//! such that s can be obtained from C by resolving all pointer references
//! within C itself (left-to-right, greedy longest match).
//!
//! Unlike external macro compression, there is no separate dictionary — the
//! compressed string C serves as both dictionary and output.
//!
//! This problem is NP-hard (Storer, 1977; Storer & Szymanski, 1978).
//! Reference: Garey & Johnson A4 SR23.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumInternalMacroDataCompression",
        display_name: "Minimum Internal Macro Data Compression",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find minimum-cost self-referencing compression of a string with embedded pointers",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the alphabet (symbols indexed 0..alphabet_size)" },
            FieldInfo { name: "string", type_name: "Vec<usize>", description: "Source string as symbol indices" },
            FieldInfo { name: "pointer_cost", type_name: "usize", description: "Pointer cost h (each pointer adds h−1 extra to the cost)" },
        ],
    }
}

/// Minimum Internal Macro Data Compression problem.
///
/// Given an alphabet of size `k`, a string `s` over `{0, ..., k-1}`, and
/// a pointer cost `h`, find a compressed string C that minimizes
/// cost = |C| + (h−1) × (pointer count in C), where C uses itself as both
/// dictionary and compressed output.
///
/// # Representation
///
/// The configuration is a vector of `string_len` entries. Each entry is:
/// - A symbol index in `{0, ..., alphabet_size-1}` (literal)
/// - `alphabet_size` (end-of-string marker; positions after this are padding)
/// - A value in `{alphabet_size+1, ..., alphabet_size + string_len}`,
///   encoding a pointer to C\[v − alphabet_size − 1\] with greedy longest match.
///
/// During decoding, pointers are resolved left-to-right. A pointer at position
/// i referencing position j (where j < i in the decoded output) copies symbols
/// from the already-decoded output starting at j using greedy longest match.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumInternalMacroDataCompression;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Alphabet {a, b}, string "abab", pointer cost h=2
/// let problem = MinimumInternalMacroDataCompression::new(2, vec![0, 1, 0, 1], 2);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumInternalMacroDataCompression {
    alphabet_size: usize,
    string: Vec<usize>,
    pointer_cost: usize,
}

impl MinimumInternalMacroDataCompression {
    /// Create a new MinimumInternalMacroDataCompression instance.
    ///
    /// # Panics
    ///
    /// Panics if `alphabet_size` is 0 and the string is non-empty, or if
    /// any symbol in the string is >= `alphabet_size`, or if `pointer_cost` is 0.
    pub fn new(alphabet_size: usize, string: Vec<usize>, pointer_cost: usize) -> Self {
        assert!(
            alphabet_size > 0 || string.is_empty(),
            "alphabet_size must be > 0 when the string is non-empty"
        );
        assert!(
            string
                .iter()
                .all(|&s| s < alphabet_size || alphabet_size == 0),
            "all symbols must be less than alphabet_size"
        );
        assert!(pointer_cost > 0, "pointer_cost must be positive");
        Self {
            alphabet_size,
            string,
            pointer_cost,
        }
    }

    /// Returns the length of the source string.
    pub fn string_len(&self) -> usize {
        self.string.len()
    }

    /// Returns the alphabet size.
    pub fn alphabet_size(&self) -> usize {
        self.alphabet_size
    }

    /// Returns the pointer cost h.
    pub fn pointer_cost(&self) -> usize {
        self.pointer_cost
    }

    /// Returns the source string.
    pub fn string(&self) -> &[usize] {
        &self.string
    }

    /// Decode the compressed string C and return the decoded string,
    /// the active length of C, and the pointer count.
    /// Returns None if decoding fails (invalid pointer, circular reference, etc.).
    fn decode(&self, config: &[usize]) -> Option<(Vec<usize>, usize, usize)> {
        let n = self.string.len();
        let k = self.alphabet_size;
        let eos = k; // end-of-string marker

        // Find active length: prefix before first end-of-string marker
        let active_len = config.iter().position(|&v| v == eos).unwrap_or(n);

        // Verify contiguous: all after first EOS must be EOS or padding
        for &v in &config[active_len..] {
            if v != eos {
                return None;
            }
        }

        // Decode left-to-right. A pointer at compressed position c_idx
        // referencing C[j] copies from the decoded output that existed
        // before this pointer (no overlapping/runaway copy).
        let mut decoded = Vec::new();
        let mut pointer_count: usize = 0;

        for &v in &config[..active_len] {
            if v < k {
                // Literal symbol
                decoded.push(v);
            } else if v > k {
                // Pointer: references C[ref_pos] in the compressed string
                let ref_pos = v - k - 1;
                if ref_pos >= decoded.len() {
                    return None; // pointer references undecoded position
                }
                // Greedy longest match from decoded[ref_pos..copy_start]
                // (only pre-existing decoded content, no overlapping copy)
                let copy_start = decoded.len();
                let mut matched = 0;
                while copy_start + matched < n {
                    let src_idx = ref_pos + matched;
                    if src_idx >= copy_start {
                        break; // cannot read beyond pre-existing content
                    }
                    if decoded[src_idx] != self.string[copy_start + matched] {
                        break;
                    }
                    decoded.push(decoded[src_idx]);
                    matched += 1;
                }
                if matched == 0 {
                    return None; // pointer must copy at least one symbol
                }
                pointer_count += 1;
            } else {
                // v == eos, but we filtered those out above
                return None;
            }
        }

        Some((decoded, active_len, pointer_count))
    }
}

impl Problem for MinimumInternalMacroDataCompression {
    const NAME: &'static str = "MinimumInternalMacroDataCompression";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.string.len();
        let domain = self.alphabet_size + n + 1; // literals + EOS + pointers
        vec![domain; n]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        let n = self.string.len();
        if config.len() != n {
            return Min(None);
        }

        // Handle empty string
        if n == 0 {
            return Min(Some(0));
        }

        match self.decode(config) {
            Some((decoded, active_len, pointer_count)) => {
                if decoded != self.string {
                    Min(None)
                } else {
                    let cost = active_len + (self.pointer_cost - 1) * pointer_count;
                    Min(Some(cost))
                }
            }
            None => Min(None),
        }
    }
}

crate::declare_variants! {
    default MinimumInternalMacroDataCompression => "(alphabet_size + string_len + 1) ^ string_len",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Issue #442 example: alphabet {a,b,c} (3), s="abcabcabc" (9), h=2
    // Optimal: C = [a, b, c, ptr(0), ptr(0), EOS, EOS, EOS, EOS]
    //   active_len = 5, pointers = 2
    //   cost = 5 + (2-1)*2 = 7
    //
    // Config encoding:
    //   alphabet_size = 3, string_len = 9, domain = 3+9+1 = 13
    //   Literals: 0=a, 1=b, 2=c
    //   EOS: 3
    //   Pointers: 4=ptr(C[0]), 5=ptr(C[1]), ...
    let s: Vec<usize> = vec![0, 1, 2, 0, 1, 2, 0, 1, 2];
    let optimal_config = vec![
        0, 1, 2, // literals a, b, c
        4, // ptr(C[0]) -> greedy "abc"
        4, // ptr(C[0]) -> greedy "abc"
        3, 3, 3, 3, // EOS padding
    ];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_internal_macro_data_compression",
        instance: Box::new(MinimumInternalMacroDataCompression::new(3, s, 2)),
        optimal_config,
        optimal_value: serde_json::json!(7),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_internal_macro_data_compression.rs"]
mod tests;

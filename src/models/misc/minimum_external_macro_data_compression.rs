//! Minimum External Macro Data Compression problem implementation.
//!
//! Given an alphabet Sigma, a string s in Sigma*, and a pointer cost h,
//! find a dictionary string D and compressed string C minimizing the total
//! cost |D| + |C| + (h-1) * (number of pointer occurrences in D and C),
//! such that s can be reconstructed from C by replacing pointers with their
//! referenced substrings of D.
//!
//! The configuration uses 2*|s| slots: |s| slots for D (dictionary) and |s|
//! slots for C (compressed string). D-slots use alphabet symbols or empty.
//! C-slots use alphabet symbols, pointers into D (start, len), or empty.
//! D is restricted to be pointer-free (pure alphabet string).
//!
//! This problem is NP-hard (Storer, 1977; Storer & Szymanski, 1978).
//! Reference: Garey & Johnson A4 SR22.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumExternalMacroDataCompression",
        display_name: "Minimum External Macro Data Compression",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find minimum-cost compression using an external dictionary and compressed string with pointers",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the alphabet (symbols indexed 0..alphabet_size)" },
            FieldInfo { name: "string", type_name: "Vec<usize>", description: "Source string as symbol indices" },
            FieldInfo { name: "pointer_cost", type_name: "usize", description: "Pointer cost h (each pointer contributes h to the cost)" },
        ],
    }
}

/// Minimum External Macro Data Compression problem.
///
/// Given an alphabet of size `k`, a string `s` over `{0, ..., k-1}`, and
/// a pointer cost `h`, find dictionary string D and compressed string C
/// that minimize cost = |D| + |C| + (h-1) * (pointer count in C).
///
/// # Representation
///
/// The configuration is a vector of `2 * string_length` entries:
/// - First `string_length` entries are D-slots: each is a symbol index
///   in `{0, ..., alphabet_size-1}` or `alphabet_size` (empty/unused).
/// - Next `string_length` entries are C-slots: each is:
///   - A symbol index in `{0, ..., alphabet_size-1}` (literal)
///   - `alphabet_size` (empty/unused)
///   - A value in `{alphabet_size+1, ..., alphabet_size + |s|*(|s|+1)/2}`
///     encoding a pointer (start, len) into D.
///
/// D is the prefix of non-empty D-slots. C is the prefix of non-empty C-slots.
/// The cost is |D| + |C| + (h-1) * (number of pointer symbols in C).
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumExternalMacroDataCompression;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Alphabet {a, b}, string "abab", pointer cost h=2
/// let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1, 0, 1], 2);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumExternalMacroDataCompression {
    alphabet_size: usize,
    string: Vec<usize>,
    pointer_cost: usize,
}

impl MinimumExternalMacroDataCompression {
    /// Create a new MinimumExternalMacroDataCompression instance.
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
    pub fn string_length(&self) -> usize {
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

    /// Returns the number of valid pointers into D (|s|*(|s|+1)/2).
    fn num_pointers(&self) -> usize {
        let n = self.string.len();
        n * (n + 1) / 2
    }

    /// Returns the C-slot domain size: alphabet_size + 1 (empty) + num_pointers.
    fn c_domain_size(&self) -> usize {
        self.alphabet_size + 1 + self.num_pointers()
    }

    /// Decode a pointer index (offset from alphabet_size+1) into (start, len)
    /// in the dictionary. Pointers are enumerated as:
    /// index 0 -> (0, 1), 1 -> (0, 2), ..., n-1 -> (0, n),
    /// n -> (1, 1), n+1 -> (1, 2), ..., etc.
    fn decode_pointer(&self, ptr_idx: usize) -> Option<(usize, usize)> {
        let n = self.string.len();
        // Enumerate (start, len) pairs where 0 <= start < n, 1 <= len <= n - start
        let mut idx = 0;
        for start in 0..n {
            let max_len = n - start;
            if ptr_idx < idx + max_len {
                let len = ptr_idx - idx + 1;
                return Some((start, len));
            }
            idx += max_len;
        }
        None
    }
}

impl Problem for MinimumExternalMacroDataCompression {
    const NAME: &'static str = "MinimumExternalMacroDataCompression";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.string.len();
        let d_domain = self.alphabet_size + 1; // symbols + empty
        let c_domain = self.c_domain_size(); // symbols + empty + pointers
        let mut dims = vec![d_domain; n]; // D-slots
        dims.extend(vec![c_domain; n]); // C-slots
        dims
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        let n = self.string.len();
        if config.len() != 2 * n {
            return Min(None);
        }

        // Handle empty string case
        if n == 0 {
            return Min(Some(0));
        }

        let empty_d = self.alphabet_size; // empty marker for D-slots
        let empty_c = self.alphabet_size; // empty marker for C-slots

        // Decode D: prefix of non-empty D-slots
        let d_slots = &config[..n];
        let d_len = d_slots.iter().position(|&v| v == empty_d).unwrap_or(n);

        // Verify contiguous: all after first empty must be empty
        for &v in &d_slots[d_len..] {
            if v != empty_d {
                return Min(None);
            }
        }

        // Verify D symbols are valid alphabet symbols
        let d_str: Vec<usize> = d_slots[..d_len].to_vec();
        if d_str.iter().any(|&v| v >= self.alphabet_size) {
            return Min(None);
        }

        // Decode C: prefix of non-empty C-slots
        let c_slots = &config[n..];
        let c_len = c_slots.iter().position(|&v| v == empty_c).unwrap_or(n);

        // Verify contiguous: all after first empty must be empty
        for &v in &c_slots[c_len..] {
            if v != empty_c {
                return Min(None);
            }
        }

        // Decode C into a sequence of symbols, counting pointers
        let mut decoded = Vec::new();
        let mut pointer_count: usize = 0;

        for &v in &c_slots[..c_len] {
            if v < self.alphabet_size {
                // Literal symbol
                decoded.push(v);
            } else if v > self.alphabet_size {
                // Pointer into D
                let ptr_idx = v - (self.alphabet_size + 1);
                if let Some((start, len)) = self.decode_pointer(ptr_idx) {
                    // Pointer must reference valid portion of D
                    if start + len > d_len {
                        return Min(None);
                    }
                    decoded.extend_from_slice(&d_str[start..start + len]);
                    pointer_count += 1;
                } else {
                    return Min(None);
                }
            } else {
                // v == empty_c, but we already filtered those out
                return Min(None);
            }
        }

        // Check decoded string matches the source string
        if decoded != self.string {
            return Min(None);
        }

        // Compute cost: |D| + |C| + (h-1) * pointer_count
        let cost = d_len + c_len + (self.pointer_cost - 1) * pointer_count;
        Min(Some(cost))
    }
}

crate::declare_variants! {
    default MinimumExternalMacroDataCompression => "(alphabet_size + 1) ^ string_length * (alphabet_size + 1 + string_length * (string_length + 1) / 2) ^ string_length",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Issue #441 example: alphabet {a,b,c,d,e,f} (6), s="abcdefabcdefabcdef" (18), h=2.
    // Optimal: D="abcdef"(6), C = ptr(0,6) ptr(0,6) ptr(0,6), cost = 6+3+(2-1)*3 = 12.
    // Solved via ILP reduction (brute force infeasible at this size).
    //
    // Config encoding (2*18 = 36 slots):
    // D-slots: [0,1,2,3,4,5, 6,6,...,6] (6 symbols + 12 empty, empty=alphabet_size=6)
    // C-slots: [ptr(0,6), ptr(0,6), ptr(0,6), 6,6,...,6] (3 pointers + 15 empty)
    // ptr(0,6) index: start=0, len=6 → index 5 → encoded as 6+1+5 = 12
    let s: Vec<usize> = (0..6).cycle().take(18).collect();
    let mut optimal_config = vec![0, 1, 2, 3, 4, 5];
    optimal_config.extend(vec![6; 12]); // empty D-slots
    optimal_config.extend(vec![12, 12, 12]); // 3 pointers to D[0..6]
    optimal_config.extend(vec![6; 15]); // empty C-slots
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_external_macro_data_compression",
        instance: Box::new(MinimumExternalMacroDataCompression::new(6, s, 2)),
        optimal_config,
        optimal_value: serde_json::json!(12),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_external_macro_data_compression.rs"]
mod tests;

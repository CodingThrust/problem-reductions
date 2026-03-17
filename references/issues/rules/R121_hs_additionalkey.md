---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hitting Set to Additional Key"
labels: rule
assignees: ''
canonical_source_name: 'Hitting Set'
canonical_target_name: 'Additional Key'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Hitting Set
**Target:** Additional Key
**Motivation:** Establishes NP-completeness of ADDITIONAL KEY via polynomial-time reduction from HITTING SET. This reduction shows that determining whether a relational schema admits a candidate key beyond a given set of known keys is computationally intractable. The result has implications for automated database normalization and schema design, since checking completeness of key enumeration is as hard as solving HITTING SET.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.232

## GJ Source Entry

> [SR27] ADDITIONAL KEY
> INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, a subset R ⊆ A, and a set K of keys for the relational scheme <R,F>.
> QUESTION: Does R have a key not already contained in K, i.e., is there an R' ⊆ R such that R' ∉ K, (R',R) ∈ F*, and for no R'' ⊆ R' is (R'',R) ∈ F*?
> Reference: [Beeri and Bernstein, 1978]. Transformation from HITTING SET.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Hitting Set instance (S, C, K) where S = {s_1, ..., s_n} is a universe, C = {c_1, ..., c_m} is a collection of subsets of S, and K is a positive integer, construct an Additional Key instance <A, F, R, K_known> as follows:

1. **Attribute set construction:** Create one attribute for each element of the universe: A = {a_{s_1}, ..., a_{s_n}} plus additional auxiliary attributes. Let R = A (the relation scheme is over all attributes).

2. **Functional dependencies:** For each subset c_j = {s_{i_1}, ..., s_{i_t}} in C, create functional dependencies that encode the covering constraint. Specifically, any subset of attributes that "hits" c_j (includes at least one a_{s_i} for s_i in c_j) can determine the auxiliary attributes associated with c_j through the functional dependency system.

3. **Known keys:** The set K_known contains all the keys already discovered. These are constructed to correspond to the subsets of S that are NOT hitting sets for C, or to known hitting sets that we want to exclude.

4. **Encoding of the hitting set condition:** The functional dependencies are designed so that a subset H ⊆ A corresponds to a key for <R, F> if and only if the corresponding elements form a hitting set for C (i.e., H intersects every c_j). The key property (H determines all of R via F*) maps to the hitting set property (H hits every subset in C).

5. **Known keys exclusion:** The set K_known is populated with known hitting sets (translated to attribute subsets), so the question "does R have an additional key not in K_known?" becomes "is there a hitting set not already in the known list?"

6. **Correctness (forward):** If there exists a hitting set H for C not corresponding to any key in K_known, then the corresponding attribute subset is a key for <R, F> not in K_known.

7. **Correctness (reverse):** If there is an additional key K' not in K_known, the corresponding universe elements form a hitting set for C not already enumerated.

**Time complexity of reduction:** O(poly(n, m, |K_known|)) to construct the attribute set, functional dependencies, and known key set.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `universe_size` of source Hitting Set instance (|S|)
- m = `num_sets` of source Hitting Set instance (|C|)
- k = |K_known| (number of already-known keys/hitting sets)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_attributes` | O(`universe_size` + `num_sets`) |
| `num_dependencies` | O(`universe_size` * `num_sets`) |
| `num_known_keys` | k (passed through from input) |

**Derivation:**
- Attributes: one per universe element plus auxiliary attributes for encoding subset constraints
- Functional dependencies: encode the membership relationships between universe elements and collection subsets
- Known keys: directly translated from the given set of known hitting sets

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a HittingSet instance to AdditionalKey, solve by brute-force enumeration of attribute subsets to find keys, check for keys not in K_known, extract solution, verify as hitting set on source
- Test with a case where exactly one hitting set exists and is already in K_known (answer: NO)
- Test with a case where multiple hitting sets exist and only some are in K_known (answer: YES)
- Verify that non-hitting-set subsets do not form keys under the constructed functional dependencies

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Hitting Set):**
Universe S = {s_1, s_2, s_3, s_4, s_5, s_6} (n = 6)
Collection C (6 subsets):
- c_1 = {s_1, s_2, s_3}
- c_2 = {s_2, s_4}
- c_3 = {s_3, s_5}
- c_4 = {s_4, s_5, s_6}
- c_5 = {s_1, s_6}
- c_6 = {s_2, s_5, s_6}

Known hitting sets (translated to K_known): {{s_2, s_3, s_6}, {s_2, s_5, s_1}}

Question: Is there a hitting set not in the known set?

**Constructed target instance (AdditionalKey):**
Attribute set A = {a_1, a_2, a_3, a_4, a_5, a_6, b_1, b_2, b_3, b_4, b_5, b_6}
(6 universe attributes + 6 auxiliary attributes for each subset constraint)

Functional dependencies F: for each subset c_j, the attributes corresponding to elements in c_j collectively determine auxiliary attribute b_j:
- {a_1} -> {b_1}, {a_2} -> {b_1}, {a_3} -> {b_1} (from c_1)
- {a_2} -> {b_2}, {a_4} -> {b_2} (from c_2)
- {a_3} -> {b_3}, {a_5} -> {b_3} (from c_3)
- {a_4} -> {b_4}, {a_5} -> {b_4}, {a_6} -> {b_4} (from c_4)
- {a_1} -> {b_5}, {a_6} -> {b_5} (from c_5)
- {a_2} -> {b_6}, {a_5} -> {b_6}, {a_6} -> {b_6} (from c_6)

R = A (full attribute set)
Known keys K_known = {{a_2, a_3, a_6}, {a_2, a_5, a_1}} (corresponding to known hitting sets)

**Solution mapping:**
Consider the candidate hitting set H = {s_2, s_3, s_4, s_6}:
- c_1 = {s_1, s_2, s_3}: s_2 in H
- c_2 = {s_2, s_4}: s_2, s_4 in H
- c_3 = {s_3, s_5}: s_3 in H
- c_4 = {s_4, s_5, s_6}: s_4, s_6 in H
- c_5 = {s_1, s_6}: s_6 in H
- c_6 = {s_2, s_5, s_6}: s_2, s_6 in H
All subsets are hit.

This corresponds to key K' = {a_2, a_3, a_4, a_6}, which:
- Is not in K_known (neither {a_2, a_3, a_6} nor {a_2, a_5, a_1})
- Determines all auxiliary attributes: b_1 via a_2, b_2 via a_2, b_3 via a_3, b_4 via a_4, b_5 via a_6, b_6 via a_2
- Therefore K' is a key for <R, F>

Answer: YES, there exists an additional key {a_2, a_3, a_4, a_6} not in K_known.

**Reverse mapping:**
Key {a_2, a_3, a_4, a_6} maps to hitting set {s_2, s_3, s_4, s_6}, verifying that this is a valid hitting set not in the known list.


## References

- **[Beeri and Bernstein, 1978]**: [`Beeri1978`] C. Beeri and P. A. Bernstein (1978). "Computational problems related to the design of normal form relational schemes". *ACM Transactions on Database Systems*, 4(1), pp. 30-59.

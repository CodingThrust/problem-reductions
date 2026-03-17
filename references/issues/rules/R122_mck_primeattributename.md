---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Minimum Cardinality Key to Prime Attribute Name"
labels: rule
assignees: ''
canonical_source_name: 'Minimum Cardinality Key'
canonical_target_name: 'Prime Attribute Name'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Minimum Cardinality Key
**Target:** Prime Attribute Name
**Motivation:** Establishes NP-completeness of PRIME ATTRIBUTE NAME via polynomial-time reduction from MINIMUM CARDINALITY KEY. This reduction shows that even the simpler-sounding question "does attribute x belong to some candidate key?" is as hard as finding a minimum-size key. The result implies that determining whether a given attribute is prime (i.e., participates in at least one candidate key) is computationally intractable, with direct consequences for database normalization algorithms that need to distinguish prime from non-prime attributes.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.232

## GJ Source Entry

> [SR28] PRIME ATTRIBUTE NAME
> INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a specified name x E A.
> QUESTION: Is x a "prime attribute name" for <A,F>, i.e., is there a key K for <A,F> such that x E K?
> Reference: [Lucchesi and Osborne, 1977]. Transformation from MINIMUM CARDINALITY KEY.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Minimum Cardinality Key instance <A, F, M> (asking whether there exists a key of cardinality at most M), construct a Prime Attribute Name instance <A', F', x> as follows:

1. **Extended attribute set:** Create a new attribute x_new not in A. Set A' = A ∪ {x_new}.

2. **Extended functional dependencies:** Keep all functional dependencies from F. Add new functional dependencies that make x_new behave as a "budget counter": x_new is designed so that it participates in a key K' for <A', F'> if and only if there exists a key K for <A, F> with |K| <= M.

3. **Construction of the budget encoding:** Introduce M additional auxiliary attributes {d_1, ..., d_M}. Set A' = A ∪ {x_new, d_1, ..., d_M}. Add functional dependencies:
   - For each original attribute a_i in A and each d_j: ({x_new, d_j}, {a_i}) -- this encodes that x_new together with budget attributes can derive everything.
   - More precisely, the dependencies are structured so that: if K is a key of A with |K| <= M, then {x_new} ∪ K (padded to exactly M attributes using dummy attributes d_j) forms a key of A' containing x_new.

4. **The specified attribute:** Set x = x_new as the query attribute.

5. **Correctness (forward):** If there exists a key K for <A, F> with |K| <= M, then x_new can be included in a key for <A', F'> by combining x_new with the attributes of K (and padding with dummy attributes if needed). This key contains x_new, so x_new is a prime attribute.

6. **Correctness (reverse):** If x_new is a prime attribute for <A', F'>, then there exists some key K' containing x_new. By the construction of F', the non-dummy, non-x_new attributes in K' must form a key for the original <A, F>, and their count is at most M (since x_new and the dummies account for the rest). Hence a key of cardinality at most M exists for <A, F>.

**Time complexity of reduction:** O(|A| * M + |F|) to construct the extended attribute set and functional dependencies.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_attributes` of source Minimum Cardinality Key instance (|A|)
- f = `num_dependencies` of source instance (|F|)
- M = `budget` of source instance

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_attributes` | `num_attributes` + `budget` + 1 |
| `num_dependencies` | `num_dependencies` + O(`num_attributes` * `budget`) |

**Derivation:**
- Attributes: original n plus M dummy attributes plus 1 query attribute = n + M + 1
- Functional dependencies: original f plus new dependencies linking x_new and dummies to original attributes
- The query attribute x_new is fixed

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumCardinalityKey instance to PrimeAttributeName, solve by enumerating all candidate keys of the extended schema, check if x_new appears in any, extract solution, verify key cardinality bound on source
- Test with a schema having a unique small key: the corresponding x_new should be prime
- Test with a schema where the minimum key has size larger than M: x_new should NOT be prime
- Verify that dummy attributes do not create spurious keys

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumCardinalityKey):**
Attribute set A = {a, b, c, d, e, f, g} (7 attributes)
Functional dependencies F:
- {a, b} -> {c}
- {c, d} -> {e}
- {a, d} -> {f}
- {b, e} -> {g}
- {f, g} -> {a}

Budget M = 3

Question: Is there a key of cardinality at most 3?

Analysis: Consider K = {a, b, d}:
- {a, b} -> {c} (derive c)
- {c, d} -> {e} (derive e, since c and d are known)
- {a, d} -> {f} (derive f)
- {b, e} -> {g} (derive g, since b and e are known)
- Closure of {a, b, d} = {a, b, c, d, e, f, g} = A
- K = {a, b, d} is a key of cardinality 3 = M. Answer: YES.

**Constructed target instance (PrimeAttributeName):**
Extended attribute set A' = {a, b, c, d, e, f, g, x_new, d_1, d_2, d_3} (11 attributes)

Extended functional dependencies F' = F ∪ {
- {x_new, d_1} -> {a}, {x_new, d_1} -> {b}, ..., {x_new, d_1} -> {g}  (x_new + any dummy determines all originals)
- {x_new, d_2} -> {a}, ..., {x_new, d_2} -> {g}
- {x_new, d_3} -> {a}, ..., {x_new, d_3} -> {g}
- Additional structural dependencies linking original keys to x_new
}

Query attribute: x = x_new

**Solution mapping:**
Since {a, b, d} is a key for <A, F> with |{a, b, d}| = 3 = M, we can construct a key for <A', F'> that includes x_new: K' = {x_new, a, b, d}. Under the extended dependencies, K' determines all of A' (x_new and the original attributes are in K' or derivable; dummy attributes d_1, d_2, d_3 are handled by additional dependencies).

Therefore x_new is prime (it appears in key K').

**Reverse mapping:**
From the prime attribute answer YES and the key K' = {x_new, a, b, d}, extract the original attributes: {a, b, d}. This is a key for <A, F> of cardinality 3 <= M = 3.


## References

- **[Lucchesi and Osborne, 1977]**: [`Lucchesi and Osborne1977`] Claudio L. Lucchesi and S. L. Osborne (1977). "Candidate keys for relations". *Journal of Computer and System Sciences*, 17(2), pp. 270-279.

---
name: Rule
about: Propose a new reduction rule
title: "[Rule] COMPARATIVE CONTAINMENT (with equal weights) to COMPARATIVE VECTOR INEQUALITIES"
labels: rule
assignees: ''
canonical_source_name: 'Comparative Containment'
canonical_target_name: 'Comparative Vector Inequalities'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** COMPARATIVE CONTAINMENT (with equal weights)
**Target:** COMPARATIVE VECTOR INEQUALITIES
**Motivation:** Establishes NP-completeness of COMPARATIVE VECTOR INEQUALITIES via polynomial-time reduction from COMPARATIVE CONTAINMENT (with equal weights). The reduction, due to Plaisted (1976), encodes set containment as componentwise vector dominance: each subset of a universe is represented by its characteristic binary vector, and the containment relation Y ⊆ R_i becomes the componentwise inequality of the characteristic vectors. This bridges the gap between set-based containment problems (SP10) and vector-based comparison problems (MP13) in Garey & Johnson's classification.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.248

## GJ Source Entry

> [MP13] COMPARATIVE VECTOR INEQUALITIES
> INSTANCE: Sets X = {x̄_1,x̄_2,...,x̄_k} and Y = {ȳ_1,ȳ_2,...,ȳ_l} of m-tuples of integers.
> QUESTION: Is there an m-tuple z̄ of integers such that the number of m-tuples x̄_i satisfying x̄_i ≥ z̄ is at least as large as the number of m-tuples ȳ_j satisfying ȳ_j ≥ z̄, where two m-tuples ū and v̄ satisfy ū ≥ v̄ if and only if no component of ū is less than the corresponding component of v̄?
> Reference: [Plaisted, 1976]. Transformation from COMPARATIVE CONTAINMENT (with equal weights).
> Comment: Remains NP-complete even if all components of the x̄_i and ȳ_j are required to belong to {0,1}.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a COMPARATIVE CONTAINMENT instance with equal weights — universe X = {x_1, ..., x_n}, collections R = {R_1, ..., R_k} and S = {S_1, ..., S_l} of subsets of X, all with weight 1 — construct a COMPARATIVE VECTOR INEQUALITIES instance as follows:

1. **Dimension:** Set m = n (one component per element of the universe X).

2. **Encoding subsets as vectors:** For each subset T ⊆ X, define its characteristic vector χ(T) ∈ {0,1}^n where χ(T)[j] = 1 if x_j ∈ T, and 0 otherwise.

3. **X-vectors (from R):** For each R_i ∈ R, create vector x̄_i = χ(R_i). This gives k vectors in {0,1}^n.

4. **Y-vectors (from S):** For each S_j ∈ S, create vector ȳ_j = χ(S_j). This gives l vectors in {0,1}^n.

5. **Correctness:** The key observation is that set containment Y ⊆ T is equivalent to componentwise vector dominance χ(T) ≥ χ(Y). Specifically:
   - A candidate subset Y ⊆ X corresponds to a candidate z̄ = χ(Y) ∈ {0,1}^n.
   - Y ⊆ R_i ⟺ every element of Y is in R_i ⟺ for all j, if χ(Y)[j] = 1 then χ(R_i)[j] = 1 ⟺ χ(R_i) ≥ χ(Y) ⟺ x̄_i ≥ z̄.
   - Similarly, Y ⊆ S_j ⟺ ȳ_j ≥ z̄.
   - With equal weights (all 1), the COMPARATIVE CONTAINMENT question asks: is there Y ⊆ X such that |{i : Y ⊆ R_i}| ≥ |{j : Y ⊆ S_j}|?
   - This becomes: is there z̄ ∈ {0,1}^n such that |{i : x̄_i ≥ z̄}| ≥ |{j : ȳ_j ≥ z̄}|?
   - Which is exactly the COMPARATIVE VECTOR INEQUALITIES question (restricted to {0,1} components).

6. **Solution extraction:** If z̄ is a solution to the COMPARATIVE VECTOR INEQUALITIES instance, then Y = {x_j : z̄[j] = 1} is a solution to the COMPARATIVE CONTAINMENT instance.

*Note: The reduction produces a {0,1}-restricted instance of COMPARATIVE VECTOR INEQUALITIES, which by the GJ comment is already NP-complete. This establishes the full NP-completeness of the general integer case as well.*

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |X| = `universe_size` of source COMPARATIVE CONTAINMENT instance
- k = |R| = `num_r_sets` (number of R-collection subsets)
- l = |S| = `num_s_sets` (number of S-collection subsets)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `dimension`                | `universe_size` (= n)            |
| `num_x_vectors`            | `num_r_sets` (= k)               |
| `num_y_vectors`            | `num_s_sets` (= l)               |

**Derivation:** Each subset maps to one binary vector of dimension n. The number of vectors equals the number of subsets. Total construction size is O((k + l) * n), which is linear in the input size of the COMPARATIVE CONTAINMENT instance.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a COMPARATIVE CONTAINMENT instance with equal weights, reduce to COMPARATIVE VECTOR INEQUALITIES, solve target with BruteForce (enumerate all z̄ ∈ {0,1}^m), extract solution, verify on source.
- Verify that the characteristic vector encoding preserves containment: for each R_i and candidate Y, check Y ⊆ R_i ⟺ χ(R_i) ≥ χ(Y).
- Test with small instances (3-4 elements, 2-3 sets) where the answer is known.
- Test both YES and NO instances to verify equivalence in both directions.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (COMPARATIVE CONTAINMENT with equal weights):**

Universe X = {a, b, c, d} (n = 4, indices 0..3)

R = { R_1 = {a, b, c}, R_2 = {a, b}, R_3 = {b, c, d} }  (k = 3, all weights = 1)
S = { S_1 = {a, b, c, d}, S_2 = {b, c}, S_3 = {c, d} }  (l = 3, all weights = 1)

Question: Is there Y ⊆ X such that |{i : Y ⊆ R_i}| ≥ |{j : Y ⊆ S_j}|?

**Constructed COMPARATIVE VECTOR INEQUALITIES instance:**

Dimension m = 4

Characteristic vectors (positions: a=0, b=1, c=2, d=3):
X-vectors (from R):
- x̄_1 = χ({a,b,c}) = (1, 1, 1, 0)
- x̄_2 = χ({a,b})   = (1, 1, 0, 0)
- x̄_3 = χ({b,c,d}) = (0, 1, 1, 1)

Y-vectors (from S):
- ȳ_1 = χ({a,b,c,d}) = (1, 1, 1, 1)
- ȳ_2 = χ({b,c})     = (0, 1, 1, 0)
- ȳ_3 = χ({c,d})     = (0, 0, 1, 1)

**Solution:**

Try z̄ = (0, 1, 0, 0), corresponding to Y = {b}.

Check x̄_i ≥ z̄:
- x̄_1 = (1,1,1,0) ≥ (0,1,0,0)? 1≥0, 1≥1, 1≥0, 0≥0 → YES
- x̄_2 = (1,1,0,0) ≥ (0,1,0,0)? 1≥0, 1≥1, 0≥0, 0≥0 → YES
- x̄_3 = (0,1,1,1) ≥ (0,1,0,0)? 0≥0, 1≥1, 1≥0, 1≥0 → YES
X-count: 3

Check ȳ_j ≥ z̄:
- ȳ_1 = (1,1,1,1) ≥ (0,1,0,0)? YES
- ȳ_2 = (0,1,1,0) ≥ (0,1,0,0)? YES
- ȳ_3 = (0,0,1,1) ≥ (0,1,0,0)? 0≥0, 0≥1? → NO
Y-count: 2

Comparison: 3 ≥ 2? YES

**Verification on source:**
Y = {b}:
- Y ⊆ R_1 = {a,b,c}? YES
- Y ⊆ R_2 = {a,b}? YES
- Y ⊆ R_3 = {b,c,d}? YES
R-count: 3

- Y ⊆ S_1 = {a,b,c,d}? YES
- Y ⊆ S_2 = {b,c}? YES
- Y ⊆ S_3 = {c,d}? NO
S-count: 2

3 ≥ 2? YES — consistent with the reduced instance.


## References

- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264–267. IEEE Computer Society.

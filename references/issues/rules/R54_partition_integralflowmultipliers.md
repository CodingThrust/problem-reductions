---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to INTEGRAL FLOW WITH MULTIPLIERS"
labels: rule
assignees: ''
canonical_source_name: 'Partition'
canonical_target_name: 'Integral Flow with Multipliers'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** PARTITION
**Target:** INTEGRAL FLOW WITH MULTIPLIERS
**Motivation:** Establishes NP-completeness of INTEGRAL FLOW WITH MULTIPLIERS via polynomial-time reduction from PARTITION. The multipliers make the flow conservation constraints non-standard, which is precisely what encodes the subset-sum structure of PARTITION. Without multipliers (h(v)=1 for all v), the problem reduces to standard max-flow solvable in polynomial time.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND33, p.215

## GJ Source Entry

> [ND33] INTEGRAL FLOW WITH MULTIPLIERS
> INSTANCE: Directed graph G=(V,A), specified vertices s and t, multiplier h(v)∈Z^+ for each v∈V-{s,t}, capacity c(a)∈Z^+ for each a∈A, requirement R∈Z^+.
> QUESTION: Is there a flow function f: A->Z_0^+ such that
> (1) f(a)<=c(a) for all a∈A,
> (2) for each v∈V-{s,t}, Sum_{(u,v)∈A} h(v)*f((u,v)) = Sum_{(v,u)∈A} f((v,u)), and
> (3) the net flow into t is at least R?
> Reference: [Sahni, 1974]. Transformation from PARTITION.
> Comment: Can be solved in polynomial time by standard network flow techniques if h(v)=1 for all v∈V-{s,t}. Corresponding problem with non-integral flows allowed can be solved by linear programming.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a PARTITION instance with multiset A = {a_1, a_2, ..., a_n} of positive integers with total sum S, construct an INTEGRAL FLOW WITH MULTIPLIERS instance as follows (based on Sahni, 1974):

1. **Vertices:** Create a directed graph with vertices s, t, and intermediate vertices v_1, v_2, ..., v_n.

2. **Arcs from s:** For each i = 1, ..., n, add an arc (s, v_i) with capacity c(s, v_i) = 1.

3. **Arcs to t:** For each i = 1, ..., n, add an arc (v_i, t) with capacity c(v_i, t) = a_i.

4. **Multipliers:** For each intermediate vertex v_i, set the multiplier h(v_i) = a_i. This means the generalized conservation constraint at v_i is:
   h(v_i) * f(s, v_i) = f(v_i, t), i.e., a_i * f(s, v_i) = f(v_i, t).

5. **Requirement:** Set R = S/2 (the required net flow into t).

6. **Correctness (forward):** If A has a balanced partition A_1 (with sum S/2), for each a_i in A_1 set f(s, v_i) = 1, f(v_i, t) = a_i; for each a_i not in A_1 set f(s, v_i) = 0, f(v_i, t) = 0. The conservation constraint a_i * f(s, v_i) = f(v_i, t) is satisfied at every v_i. The net flow into t is sum of a_i for i in A_1 = S/2 = R.

7. **Correctness (reverse):** If a feasible integral flow exists with net flow >= R = S/2 into t, the conservation constraints force f(v_i, t) = a_i * f(s, v_i). Since c(s, v_i) = 1, f(s, v_i) in {0, 1}. The net flow into t is sum of a_i * f(s, v_i) >= S/2. Since the total of all a_i is S, and each contributes either 0 or a_i, the set {a_i : f(s, v_i) = 1} has sum >= S/2 and the complementary set has sum <= S/2, giving a balanced partition.

**Key invariant:** The multiplier h(v_i) = a_i combined with unit capacity on the source arcs encodes the binary include/exclude decision. The flow requirement R = S/2 encodes the partition balance condition.

**Time complexity of reduction:** O(n) to construct the graph.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements in the PARTITION instance
- S = sum of all elements

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `n + 2` |
| `num_arcs` | `2 * n` |
| `requirement` (R) | `S / 2` |

**Derivation:** The graph has n + 2 vertices (s, t, and n intermediate vertices) and 2n arcs (one from s to each v_i and one from each v_i to t). The flow requirement is S/2.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a PARTITION instance to IntegralFlowWithMultipliers, solve target with BruteForce (enumerate integer flow assignments), extract solution, verify on source
- Test with known YES instance: A = {1, 2, 3, 4, 5, 5} with S = 20; balanced partition exists ({1,4,5} and {2,3,5})
- Test with known NO instance: A = {1, 2, 3, 7} with S = 13 (odd, no balanced partition)
- Compare with known results from literature

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {2, 3, 4, 5, 6, 4} with S = 24, S/2 = 12.
A valid partition: A_1 = {2, 4, 6} (sum = 12), A_2 = {3, 5, 4} (sum = 12).

**Constructed target instance (IntegralFlowWithMultipliers):**
- Vertices: s, v_1, v_2, v_3, v_4, v_5, v_6, t (8 vertices)
- Arcs and capacities:
  - (s, v_1): c = 1; (s, v_2): c = 1; (s, v_3): c = 1; (s, v_4): c = 1; (s, v_5): c = 1; (s, v_6): c = 1
  - (v_1, t): c = 2; (v_2, t): c = 3; (v_3, t): c = 4; (v_4, t): c = 5; (v_5, t): c = 6; (v_6, t): c = 4
- Multipliers: h(v_1) = 2, h(v_2) = 3, h(v_3) = 4, h(v_4) = 5, h(v_5) = 6, h(v_6) = 4
- Requirement: R = 12

**Solution mapping:**
- Partition A_1 = {a_1, a_3, a_5} = {2, 4, 6}: set f(s, v_1) = 1, f(s, v_3) = 1, f(s, v_5) = 1
- Partition A_2 = {a_2, a_4, a_6} = {3, 5, 4}: set f(s, v_2) = 0, f(s, v_4) = 0, f(s, v_6) = 0
- Flow on arcs to t: f(v_1, t) = 2*1 = 2, f(v_3, t) = 4*1 = 4, f(v_5, t) = 6*1 = 6
- All others: f(v_2, t) = 0, f(v_4, t) = 0, f(v_6, t) = 0
- Net flow into t: 2 + 0 + 4 + 0 + 6 + 0 = 12 = R
- Conservation at each v_i: h(v_i)*f(s,v_i) = f(v_i,t) holds


## References

- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262-279.

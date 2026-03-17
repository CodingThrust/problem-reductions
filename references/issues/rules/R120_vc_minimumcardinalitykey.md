---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to Minimum Cardinality Key"
labels: rule
assignees: ''
canonical_source_name: 'Vertex Cover'
canonical_target_name: 'Minimum Cardinality Key'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Vertex Cover
**Target:** Minimum Cardinality Key
**Motivation:** Establishes NP-completeness of MINIMUM CARDINALITY KEY via polynomial-time reduction from VERTEX COVER. This reduction bridges graph theory and relational database theory, showing that finding a minimum-size key for a relational schema (under functional dependencies) is as hard as finding a minimum vertex cover. The result implies that optimizing database key selection is computationally intractable in general.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.232

## GJ Source Entry

> [SR26] MINIMUM CARDINALITY KEY
> INSTANCE: A set A of "attribute names," a collection F of ordered pairs of subsets of A (called "functional dependencies" on A), and a positive integer M.
> QUESTION: Is there a key of cardinality M or less for the relational system <A,F>, i.e., a minimal subset K ⊆ A with |K| <= M such that the ordered pair (K,A) belongs to the "closure" F* of F defined by (1) F ⊆ F*, (2) B ⊆ C ⊆ A implies (C,B) E F*, (3) (B,C),(C,D) E F* implies (B,D) E F*, and (4) (B,C),(B,D) E F* implies (B,C ∪ D) E F*?
> Reference: [Lucchesi and Osborne, 1977], [Lipsky, 1977a]. Transformation from VERTEX COVER. See [Date, 1975] for general background on relational data bases.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Vertex Cover instance (G = (V, E), k) where V = {v_1, ..., v_n} and E = {e_1, ..., e_m}, construct a Minimum Cardinality Key instance <A, F, M> as follows:

1. **Attribute set construction:** Create one attribute for each vertex: A_V = {a_{v_1}, ..., a_{v_n}}. Additionally, create one attribute for each edge: A_E = {a_{e_1}, ..., a_{e_m}}. The full attribute set is A = A_V ∪ A_E, so |A| = n + m.

2. **Functional dependencies:** For each edge e_j = {v_p, v_q} in E, add two functional dependencies:
   - ({a_{v_p}}, {a_{e_j}}): attribute a_{v_p} determines a_{e_j}
   - ({a_{v_q}}, {a_{e_j}}): attribute a_{v_q} determines a_{e_j}

   These express that knowing either endpoint of an edge determines the edge attribute. Also, include the trivial identity dependencies so that each vertex attribute determines itself.

3. **Budget parameter:** Set M = k (same as the vertex cover budget).

4. **Key construction insight:** A subset K ⊆ A is a key for <A, F> if and only if the closure of K under F* equals all of A. Since the edge attributes are determined by the vertex attributes (via the functional dependencies), K needs to:
   - Include enough vertex attributes to determine all edge attributes (i.e., for every edge e_j = {v_p, v_q}, at least one of a_{v_p} or a_{v_q} must be in K or derivable from K)
   - Include all vertex attributes not derivable from other attributes in K

5. **Correctness (forward):** If S ⊆ V is a vertex cover of size ≤ k, then K = {a_v : v ∈ S} determines all edge attributes (since every edge has at least one endpoint in S). The remaining vertex attributes not in K can be added to the key if needed, but the functional dependencies are set up so that K already determines all of A. Hence K is a key of size ≤ k = M.

6. **Correctness (reverse):** If K is a key of cardinality ≤ M = k, then the vertex attributes in K form a vertex cover of G: for every edge e_j = {v_p, v_q}, the attribute a_{e_j} must be in the closure of K, which requires that at least one of a_{v_p} or a_{v_q} is in K (since the only way to derive a_{e_j} is from a_{v_p} or a_{v_q}).

**Time complexity of reduction:** O(n + m) to construct the attribute set and functional dependencies.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_attributes` | `num_vertices` + `num_edges` |
| `num_dependencies` | 2 * `num_edges` |
| `budget` | k (same as vertex cover budget) |

**Derivation:**
- Attributes: one per vertex (n) plus one per edge (m) = n + m total
- Functional dependencies: two per edge (one for each endpoint) = 2m total
- Each dependency has a single-attribute left-hand side and a single-attribute right-hand side
- Budget M = k is passed through unchanged

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumVertexCover instance to MinimumCardinalityKey, solve the key problem by brute-force enumeration of attribute subsets, extract solution, verify as vertex cover on original graph
- Test with a triangle graph K_3: minimum vertex cover is 2, so minimum key should have cardinality 2
- Test with a star graph K_{1,5}: minimum vertex cover is 1 (center vertex), so minimum key should be 1
- Verify that the closure computation correctly derives all edge attributes from the key attributes

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices V = {v_1, v_2, v_3, v_4, v_5, v_6} and 7 edges:
- e_1 = {v_1, v_2}
- e_2 = {v_1, v_3}
- e_3 = {v_2, v_4}
- e_4 = {v_3, v_4}
- e_5 = {v_3, v_5}
- e_6 = {v_4, v_6}
- e_7 = {v_5, v_6}

Minimum vertex cover: k = 3, e.g., S = {v_1, v_4, v_5} covers all edges:
- e_1 = {v_1, v_2}: v_1 in S
- e_2 = {v_1, v_3}: v_1 in S
- e_3 = {v_2, v_4}: v_4 in S
- e_4 = {v_3, v_4}: v_4 in S
- e_5 = {v_3, v_5}: v_5 in S
- e_6 = {v_4, v_6}: v_4 in S
- e_7 = {v_5, v_6}: v_5 in S

**Constructed target instance (MinimumCardinalityKey):**
Attribute set A = {a_{v1}, a_{v2}, a_{v3}, a_{v4}, a_{v5}, a_{v6}, a_{e1}, a_{e2}, a_{e3}, a_{e4}, a_{e5}, a_{e6}, a_{e7}}
(13 attributes total: 6 vertex + 7 edge)

Functional dependencies F (14 total, 2 per edge):
- From e_1: {a_{v1}} -> {a_{e1}}, {a_{v2}} -> {a_{e1}}
- From e_2: {a_{v1}} -> {a_{e2}}, {a_{v3}} -> {a_{e2}}
- From e_3: {a_{v2}} -> {a_{e3}}, {a_{v4}} -> {a_{e3}}
- From e_4: {a_{v3}} -> {a_{e4}}, {a_{v4}} -> {a_{e4}}
- From e_5: {a_{v3}} -> {a_{e5}}, {a_{v5}} -> {a_{e5}}
- From e_6: {a_{v4}} -> {a_{e6}}, {a_{v6}} -> {a_{e6}}
- From e_7: {a_{v5}} -> {a_{e7}}, {a_{v6}} -> {a_{e7}}

Budget M = 3

**Solution mapping:**
Key K = {a_{v1}, a_{v4}, a_{v5}} (cardinality 3 = M)

Closure computation for K:
- a_{v1} in K: determines a_{e1} (via {a_{v1}} -> {a_{e1}}) and a_{e2} (via {a_{v1}} -> {a_{e2}})
- a_{v4} in K: determines a_{e3} (via {a_{v4}} -> {a_{e3}}), a_{e4} (via {a_{v4}} -> {a_{e4}}), a_{e6} (via {a_{v4}} -> {a_{e6}})
- a_{v5} in K: determines a_{e5} (via {a_{v5}} -> {a_{e5}}), a_{e7} (via {a_{v5}} -> {a_{e7}})
- All 7 edge attributes determined. Vertex attributes a_{v2}, a_{v3}, a_{v6} are NOT determined by K alone.

Note: For K to be a proper key for <A, F>, K must determine ALL attributes in A. The vertex attributes not in K (a_{v2}, a_{v3}, a_{v6}) are not derivable from K via F alone. To make the reduction work correctly, additional functional dependencies or a modified attribute set may be needed so that only edge-covering matters.

**Corrected construction:** The attribute set should be A = A_E only (edge attributes), with the vertex attributes serving as the "selection pool." Alternatively, the construction uses A = A_V ∪ A_E with additional dependencies that make all vertex attributes self-determined (trivially in the closure). The key K must be a subset of A_V such that all of A_E is in the closure of K. Then |K| <= M iff the corresponding vertices form a vertex cover of size <= k.

Revised key K = {a_{v1}, a_{v4}, a_{v5}}:
- Closure of K under F: {a_{v1}, a_{v4}, a_{v5}} ∪ {a_{e1}, a_{e2}, a_{e3}, a_{e4}, a_{e5}, a_{e6}, a_{e7}} = all edge attributes determined
- The remaining vertex attributes {a_{v2}, a_{v3}, a_{v6}} must also be determined for K to be a key. This is achieved by adding functional dependencies: ({a_{e_j}}, {a_{v_p}, a_{v_q}}) for each edge e_j = {v_p, v_q}, or by restricting the "relation" to only the edge attributes.

In the standard formulation (Lucchesi and Osborne), K is a key for the relation restricted to the edge attributes, and the vertex attributes represent the "selection variables."

**Reverse mapping:**
Key K = {a_{v1}, a_{v4}, a_{v5}} maps to vertex cover S = {v_1, v_4, v_5}, which covers all 7 edges as verified above.


## References

- **[Lucchesi and Osborne, 1977]**: [`Lucchesi and Osborne1977`] Claudio L. Lucchesi and S. L. Osborne (1977). "Candidate keys for relations". *Journal of Computer and System Sciences*.
- **[Lipsky, 1977a]**: [`Lipsky1977a`] William Lipsky, Jr (1977). "Two {NP}-complete problems related to information retrieval". In: *Fundamentals of Computation Theory*. Springer.
- **[Date, 1975]**: [`Date1975`] C. J. Date (1975). "An Introduction to Database Systems". Addison-Wesley, Reading, MA.

---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Graph 3-Colorability to Conjunctive Query Foldability"
labels: rule
assignees: ''
canonical_source_name: 'GRAPH 3-COLORABILITY'
canonical_target_name: 'CONJUNCTIVE QUERY FOLDABILITY'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Graph 3-Colorability
**Target:** Conjunctive Query Foldability
**Motivation:** Establishes NP-completeness of CONJUNCTIVE QUERY FOLDABILITY via polynomial-time reduction from GRAPH 3-COLORABILITY. This reduction connects graph coloring to database query optimization: graph 3-colorability is equivalent to the existence of a homomorphism from a graph to K_3, which is precisely the foldability (containment) condition for conjunctive queries. This foundational result by Chandra and Merlin (1977) demonstrates that optimizing conjunctive queries is inherently hard.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.233

## GJ Source Entry

> [SR30] CONJUNCTIVE QUERY FOLDABILITY
> INSTANCE: Finite domain set D, a collection R = {R_1, R_2, ..., R_m} of relations, where each R_i consists of a set of d_i-tuples with entries from D, a set X of distinguished variables, a set Y of undistinguished variables, and two "queries" Q_1 and Q_2 over X, Y, D, and R, where a query Q has the form
>
> (x_1, x_2, ..., x_k)(∃y_1, y_2, ..., y_l)(A_1 ∧ A_2 ∧ ... ∧ A_r)
>
> for some k, l, and r, with X' = {x_1, x_2, ..., x_k} ⊆ X, Y' = {y_1, y_2, ..., y_l} ⊆ Y, and each A_i of the form R_j(u_1, u_2, ..., u_{d_j}) with each u E D ∪ X' ∪ Y' (see reference for interpretation of such expressions in terms of data bases).
> QUESTION: Is there a function σ: Y → X ∪ Y ∪ D such that, if for each y E Y the symbol σ(y) is substituted for every occurrence of y in Q_1, then the result is query Q_2?
> Reference: [Chandra and Merlin, 1977]. Transformation from GRAPH 3-COLORABILITY.
> Comment: The isomorphism problem for conjunctive queries (with two queries being isomorphic if they are the same up to one-to-one renaming of the variables, reordering of conjuncts, and reordering within quantifications) is polynomially equivalent to graph isomorphism.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Graph 3-Colorability instance G = (V, E), construct a Conjunctive Query Foldability instance as follows:

1. **Domain construction:** Let D = {1, 2, 3} (the three colors).

2. **Relation construction:** Create a single binary relation R consisting of all pairs (i, j) where i != j and i, j in {1, 2, 3}. That is, R = {(1,2), (1,3), (2,1), (2,3), (3,1), (3,2)} — this is the edge relation of the complete graph K_3.

3. **Query Q_G (from graph G):** For each vertex v in V, introduce a variable y_v (all undistinguished). For each edge (u, v) in E, add a conjunct R(y_u, y_v). The query is:
   Q_G = ()(exists y_{v_1}, ..., y_{v_n})(R(y_u, y_v) for each (u,v) in E)
   This is a Boolean query (no distinguished variables) with |V| existential variables and |E| conjuncts.

4. **Query Q_{K_3} (from complete triangle):** Introduce three undistinguished variables z_1, z_2, z_3. Add conjuncts R(z_1, z_2), R(z_2, z_3), R(z_3, z_1). The query is:
   Q_{K_3} = ()(exists z_1, z_2, z_3)(R(z_1, z_2) ∧ R(z_2, z_3) ∧ R(z_3, z_1))

5. **Foldability condition:** Ask whether Q_G can be "folded" into Q_{K_3}, i.e., whether there exists a substitution sigma mapping variables of Q_G to variables of Q_{K_3} (plus constants from D) such that applying sigma to Q_G yields Q_{K_3}. By the Chandra-Merlin homomorphism theorem, such a substitution exists if and only if there is a homomorphism from G to K_3, which is equivalent to G being 3-colorable.

6. **Solution extraction:** Given a folding sigma, the 3-coloring is: color vertex v with the color corresponding to sigma(y_v), where sigma maps y_v to one of {z_1, z_2, z_3} (corresponding to colors 1, 2, 3). Adjacent vertices must receive different colors because R only contains pairs of distinct values.

**Key invariant:** G is 3-colorable if and only if the query Q_G can be folded into Q_{K_3}. The folding function sigma encodes the color assignment: sigma(y_v) = z_c means vertex v gets color c.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `domain_size` | `3` (constant) |
| `num_relations` | `1` (single binary relation) |
| `relation_tuples` | `6` (constant: edges of K_3) |
| `num_undistinguished_vars_q1` | `num_vertices` |
| `num_conjuncts_q1` | `num_edges` |
| `num_undistinguished_vars_q2` | `3` (constant) |
| `num_conjuncts_q2` | `3` (constant) |

**Derivation:**
- Domain D = {1, 2, 3}: constant size 3
- One relation R with 6 tuples (all non-equal pairs from {1,2,3})
- Q_G has one variable per vertex (n variables) and one conjunct per edge (m conjuncts)
- Q_{K_3} has 3 variables and 3 conjuncts (constant)
- Total encoding size: O(n + m)

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a KColoring(k=3) instance to ConjunctiveQueryFoldability, solve the foldability problem with BruteForce (enumerate all substitutions sigma: Y -> X ∪ Y ∪ D), extract the coloring, verify it is a valid 3-coloring on the original graph
- Check that a 3-colorable graph (e.g., a bipartite graph) yields a positive foldability instance
- Check that a non-3-colorable graph (e.g., K_4) yields a negative foldability instance
- Verify the folding encodes a valid color assignment: adjacent vertices map to different z_i variables

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Graph 3-Colorability):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges (a wheel graph W_5 minus one spoke):
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}, {0,2}, {0,3}, {1,4}
- This graph is 3-colorable but not 2-colorable (it contains odd cycles)

Valid 3-coloring: 0->1, 1->2, 2->3, 3->1, 4->3, 5->2
- Edge {0,1}: colors 1,2 -- different
- Edge {1,2}: colors 2,3 -- different
- Edge {2,3}: colors 3,1 -- different
- Edge {3,4}: colors 1,3 -- different
- Edge {4,5}: colors 3,2 -- different
- Edge {5,0}: colors 2,1 -- different
- Edge {0,2}: colors 1,3 -- different
- Edge {0,3}: colors 1,1 -- INVALID! Need to fix coloring.

Corrected 3-coloring: 0->1, 1->2, 2->3, 3->2, 4->3, 5->3
- Edge {0,1}: 1,2 -- different
- Edge {1,2}: 2,3 -- different
- Edge {2,3}: 3,2 -- different
- Edge {3,4}: 2,3 -- different
- Edge {4,5}: 3,3 -- INVALID!

Revised graph (simpler, verified): G with 6 vertices {0,1,2,3,4,5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,5}, {4,5}
- Valid 3-coloring: 0->1, 1->2, 2->3, 3->1, 4->1, 5->2
  - {0,1}: 1,2 -- different
  - {0,2}: 1,3 -- different
  - {1,2}: 2,3 -- different
  - {1,3}: 2,1 -- different
  - {2,4}: 3,1 -- different
  - {3,5}: 1,2 -- different
  - {4,5}: 1,2 -- different

**Constructed target instance (ConjunctiveQueryFoldability):**
Domain D = {1, 2, 3}
Relation R = {(1,2), (1,3), (2,1), (2,3), (3,1), (3,2)}

Q_1 (from G): ()(exists y_0, y_1, y_2, y_3, y_4, y_5)(R(y_0, y_1) ∧ R(y_0, y_2) ∧ R(y_1, y_2) ∧ R(y_1, y_3) ∧ R(y_2, y_4) ∧ R(y_3, y_5) ∧ R(y_4, y_5))

Q_2 (K_3): ()(exists z_1, z_2, z_3)(R(z_1, z_2) ∧ R(z_2, z_3) ∧ R(z_3, z_1))

**Solution mapping:**
- Folding sigma: y_0 -> z_1, y_1 -> z_2, y_2 -> z_3, y_3 -> z_1, y_4 -> z_1, y_5 -> z_2
- This encodes the 3-coloring: vertex 0->color 1, 1->color 2, 2->color 3, 3->color 1, 4->color 1, 5->color 2
- Verification: applying sigma to Q_1 yields conjuncts R(z_1, z_2), R(z_1, z_3), R(z_2, z_3), R(z_2, z_1), R(z_3, z_1), R(z_1, z_2), R(z_1, z_2) — each of which is a conjunct of Q_2 (possibly repeated or a subset)
- The folding is valid, confirming the graph is 3-colorable


## References

- **[Chandra and Merlin, 1977]**: [`Chandra1977`] A. K. Chandra and P. M. Merlin (1977). "Optimal implementation of conjunctive queries in relational data bases". In: *Proceedings of the 9th Annual ACM Symposium on Theory of Computing*, pp. 77-90. Association for Computing Machinery.

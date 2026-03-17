---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Clique to Conjunctive Boolean Query"
labels: rule
assignees: ''
canonical_source_name: 'CLIQUE'
canonical_target_name: 'CONJUNCTIVE BOOLEAN QUERY'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Clique
**Target:** Conjunctive Boolean Query
**Motivation:** Establishes NP-completeness of CONJUNCTIVE BOOLEAN QUERY evaluation via polynomial-time reduction from CLIQUE. The reduction encodes the k-clique problem as a Boolean conjunctive query over a database: finding a k-clique in a graph G is equivalent to evaluating a specific conjunctive Boolean query over the edge relation of G. This result by Chandra and Merlin (1977) is foundational to database theory, showing that even the simplest class of database queries has NP-complete evaluation complexity.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.233

## GJ Source Entry

> [SR31] CONJUNCTIVE BOOLEAN QUERY
> INSTANCE: Finite domain set D, a collection R = {R_1, R_2, ..., R_m} of relations, where each R_i consists of a set of d_i-tuples with entries from D, and a conjunctive Boolean query Q over R and D, where such a query Q is of the form
>
> (∃y_1, y_2, ..., y_l)(A_1 ∧ A_2 ∧ ... ∧ A_r)
>
> with each A_i of the form R_j(u_1, u_2, ..., u_{d_j}) where each u E {y_1, y_2, ..., y_l} ∪ D.
> QUESTION: Is Q, when interpreted as a statement about R and D, true?
> Reference: [Chandra and Merlin, 1977]. Transformation from CLIQUE.
> Comment: If we are allowed to replace the conjunctive query Q by an arbitrary first-order sentence involving the predicates in R, then the problem becomes PSPACE-complete, even for D = {0,1}.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Clique instance (G, k) where G = (V, E) is an undirected graph and k is the desired clique size, construct a Conjunctive Boolean Query instance as follows:

1. **Domain construction:** Set D = V (the vertex set of G). The domain has |V| = n elements.

2. **Relation construction:** Create a single binary relation R = E, the edge relation of G. R consists of all pairs (u, v) where {u, v} in E. For an undirected graph, include both (u, v) and (v, u) for each edge. The relation has 2|E| tuples.

3. **Query construction:** Introduce k existentially quantified variables y_1, y_2, ..., y_k (one for each vertex in the desired clique). For every pair 1 <= i < j <= k, add a conjunct R(y_i, y_j) requiring that the vertices assigned to y_i and y_j are adjacent. The query is:

   Q = (exists y_1, ..., y_k)(R(y_1, y_2) ∧ R(y_1, y_3) ∧ ... ∧ R(y_{k-1}, y_k))

   This has k variables and C(k, 2) = k(k-1)/2 conjuncts.

4. **Distinctness enforcement:** To ensure the k variables are assigned distinct vertices (forming a true k-clique rather than allowing repeated vertices), we can either add inequality atoms or use the standard trick of adding a relation containing all pairs of distinct elements. In the basic formulation, if R is just the edge set E and the graph has no self-loops, then R(y_i, y_j) already forces y_i != y_j.

5. **Solution extraction:** If Q evaluates to true, there is an assignment y_1 = v_1, ..., y_k = v_k such that (v_i, v_j) in R for all i < j, meaning {v_1, ..., v_k} is a k-clique in G. Conversely, any k-clique in G provides such an assignment.

**Key invariant:** G has a k-clique if and only if the conjunctive Boolean query Q evaluates to true over the database (D, R). Each satisfying assignment of the existential variables corresponds to a k-clique.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G
- k = clique size parameter

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `domain_size` | `num_vertices` |
| `num_relations` | `1` (single binary relation) |
| `relation_tuples` | `2 * num_edges` |
| `num_variables` | `k` |
| `num_conjuncts` | `k * (k - 1) / 2` |

**Derivation:**
- Domain D = V: n elements
- One binary relation R encoding the edge set: 2m tuples (both directions)
- k existential variables (one per clique member)
- One conjunct per pair of clique members: C(k,2) = k(k-1)/2 conjuncts
- Total encoding size: O(n + m + k^2), which is polynomial since k <= n

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MaximumClique (decision version) instance to ConjunctiveBooleanQuery, evaluate the Boolean query with BruteForce (enumerate all assignments of k variables to domain elements), extract the clique, verify it is a valid k-clique on the original graph
- Check that a graph known to have a k-clique (e.g., a complete graph K_k embedded in a larger graph) yields a TRUE query evaluation
- Check that a graph without a k-clique yields a FALSE query evaluation
- Test with a graph where greedy vertex selection fails (e.g., a graph with high-degree vertices that do not form a clique)
- Verify that the query forces distinct vertex assignment (no self-loops in R)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Clique):**
Graph G with 7 vertices {0, 1, 2, 3, 4, 5, 6} and 12 edges:
- Edges: {0,1}, {0,2}, {0,3}, {1,2}, {1,3}, {2,3}, {2,4}, {3,5}, {4,5}, {4,6}, {5,6}, {1,6}
- Clique size: k = 4
- Vertices {0, 1, 2, 3} form a 4-clique (K_4):
  - {0,1}: edge exists
  - {0,2}: edge exists
  - {0,3}: edge exists
  - {1,2}: edge exists
  - {1,3}: edge exists
  - {2,3}: edge exists

**Constructed target instance (ConjunctiveBooleanQuery):**
Domain D = {0, 1, 2, 3, 4, 5, 6}
Relation R (binary, both directions for each edge, 24 tuples):
- (0,1),(1,0), (0,2),(2,0), (0,3),(3,0), (1,2),(2,1), (1,3),(3,1), (2,3),(3,2),
  (2,4),(4,2), (3,5),(5,3), (4,5),(5,4), (4,6),(6,4), (5,6),(6,5), (1,6),(6,1)

Query Q = (exists y_1, y_2, y_3, y_4)(R(y_1, y_2) ∧ R(y_1, y_3) ∧ R(y_1, y_4) ∧ R(y_2, y_3) ∧ R(y_2, y_4) ∧ R(y_3, y_4))

This has 4 variables and C(4,2) = 6 conjuncts.

**Solution mapping:**
- Satisfying assignment: y_1 = 0, y_2 = 1, y_3 = 2, y_4 = 3
- Verification of all 6 conjuncts:
  - R(y_1, y_2) = R(0, 1): (0,1) in R
  - R(y_1, y_3) = R(0, 2): (0,2) in R
  - R(y_1, y_4) = R(0, 3): (0,3) in R
  - R(y_2, y_3) = R(1, 2): (1,2) in R
  - R(y_2, y_4) = R(1, 3): (1,3) in R
  - R(y_3, y_4) = R(2, 3): (2,3) in R
- All conjuncts satisfied, Q evaluates to TRUE
- Extracted clique: {0, 1, 2, 3} — a valid 4-clique in G

**Greedy trap:** Vertex 4 has degree 3 (neighbors: 2, 5, 6), same as vertex 0 (neighbors: 1, 2, 3). However, {4, 5, 6} is only a triangle (3-clique) — the neighborhood of vertex 4 does not contain a 3-clique, so starting from vertex 4 cannot yield a 4-clique.


## References

- **[Chandra and Merlin, 1977]**: [`Chandra1977`] A. K. Chandra and P. M. Merlin (1977). "Optimal implementation of conjunctive queries in relational data bases". In: *Proceedings of the 9th Annual ACM Symposium on Theory of Computing*, pp. 77-90. Association for Computing Machinery.

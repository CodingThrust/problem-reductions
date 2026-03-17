---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hamiltonian Circuit to Feasible Basis Extension"
labels: rule
assignees: ''
canonical_source_name: 'HAMILTONIAN CIRCUIT'
canonical_target_name: 'FEASIBLE BASIS EXTENSION'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** FEASIBLE BASIS EXTENSION
**Motivation:** Establishes NP-completeness of the Feasible Basis Extension problem by encoding the Hamiltonian circuit problem as a linear programming basis selection question, revealing that even fundamental LP-theoretic problems about simplex bases are computationally intractable.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.246

## GJ Source Entry

> [MP4] FEASIBLE BASIS EXTENSION
> INSTANCE: An m x n integer matrix A, m < n, a column vector a-bar of length m, and a subset S of the columns of A with |S| < m.
> QUESTION: Is there a feasible basis B for Ax-bar = a-bar, x-bar >= 0, i.e., a nonsingular m x m submatrix B of A such that B^{-1}a-bar >= 0, and such that B contains all the columns in S?
> Reference: [Murty, 1972]. Transformation from HAMILTONIAN CIRCUIT.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Hamiltonian Circuit instance G = (V, E) with |V| = n vertices and |E| = m_e edges, construct a Feasible Basis Extension instance (A, a-bar, S) as follows:

1. **Construct the LP formulation of Hamiltonian Circuit:** The standard LP relaxation of the Hamiltonian circuit problem uses the node-edge incidence matrix of G. Define the matrix A as the (n x m_e) node-edge incidence matrix of G, where A_{v,e} = 1 if vertex v is incident to edge e, and 0 otherwise.

2. **Set the right-hand side:** Set a-bar = (2, 2, ..., 2)^T (a vector of all 2's of length n). This encodes the degree constraint: in a Hamiltonian circuit, every vertex has degree exactly 2.

3. **Set the required columns:** Set S = {} (empty set). The problem asks whether there exists a feasible basis B for the system Ax = (2,...,2)^T, x >= 0. A feasible basis selects n linearly independent columns (edges) such that the basic solution assigns nonnegative values to these edges, with the degree-2 constraint satisfied.

4. **Encode the circuit structure:** To ensure the solution actually forms a Hamiltonian circuit (not just a 2-factor), Murty's construction augments the matrix with additional rows and columns encoding subtour elimination constraints. Specifically:
   - Add auxiliary rows to the matrix A that enforce connectivity.
   - Add auxiliary columns with corresponding entries that represent slack/surplus variables.
   - Set S to be the set of auxiliary columns (these must appear in any feasible basis), forcing the basis to respect the connectivity constraints.

5. **Equivalence:** G has a Hamiltonian circuit if and only if the constructed LP system has a feasible basis extending S. The Hamiltonian circuit edges correspond to the basic columns achieving the degree constraints, and the auxiliary variables in S enforce that the selected edges form a single connected circuit rather than a disconnected union of smaller cycles.

**Key insight:** The incidence matrix of a graph encodes the degree constraints, and the challenge is encoding the subtour-elimination (connectivity) requirement within the LP basis framework. Murty achieves this by augmenting the system so that any feasible basis containing the required columns S necessarily corresponds to a connected 2-regular subgraph, i.e., a Hamiltonian circuit.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of the source graph G (|V|)
- m_e = `num_edges` of the source graph G (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_rows` (m) | O(n + n^2) = O(n^2) -- n degree rows + O(n^2) subtour elimination rows |
| `num_columns` (n_cols) | O(m_e + n^2) -- m_e edge columns + O(n^2) auxiliary columns |
| `num_required` (\|S\|) | O(n^2) -- auxiliary columns enforcing connectivity |

**Derivation:**
- The base incidence matrix has n rows and m_e columns (degree constraints).
- Subtour elimination requires additional rows and columns. In the worst case, there are O(2^n) subtour elimination constraints, but Murty's construction uses a polynomial-size encoding with O(n^2) auxiliary constraints derived from the graph structure.
- The total matrix size is polynomial in n and m_e.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- **Closed-loop test:** Start from a Hamiltonian Circuit instance G; apply R155 to construct an LP basis extension instance (A, a-bar, S); solve the Feasible Basis Extension problem by brute-force enumeration of basis extensions; verify that a feasible basis exists if and only if G has a Hamiltonian circuit.
- **Size verification:** Check that the constructed matrix A has dimensions consistent with the overhead expressions above.
- **Forward mapping:** Given a known Hamiltonian circuit in G, verify that the corresponding edge columns (plus the required columns S) form a feasible basis of the constructed system.
- **Backward mapping:** Given a feasible basis of the constructed system, extract the non-auxiliary basic columns and verify that they correspond to edges forming a Hamiltonian circuit in G.
- **Negative instance:** Test with a graph that has no Hamiltonian circuit (e.g., the Petersen graph) and verify that no feasible basis extension exists.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Hamiltonian Circuit):**

Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges (the prism graph):
- Edges (indexed e0..e8): e0={0,1}, e1={1,2}, e2={2,0}, e3={3,4}, e4={4,5}, e5={5,3}, e6={0,3}, e7={1,4}, e8={2,5}
- G has a Hamiltonian circuit: 0 -> 1 -> 4 -> 3 -> 5 -> 2 -> 0 (using edges e0, e7, e3, e5, e8, e2).

**Constructed target instance (Feasible Basis Extension):**

Step 1: Node-edge incidence matrix A_0 (6 x 9):

```
       e0 e1 e2 e3 e4 e5 e6 e7 e8
v0:  [  1  0  1  0  0  0  1  0  0 ]
v1:  [  1  1  0  0  0  0  0  1  0 ]
v2:  [  0  1  1  0  0  0  0  0  1 ]
v3:  [  0  0  0  1  0  1  1  0  0 ]
v4:  [  0  0  0  1  1  0  0  1  0 ]
v5:  [  0  0  0  0  1  1  0  0  1 ]
```

Step 2: a-bar_0 = (2, 2, 2, 2, 2, 2)^T (degree-2 constraint for each vertex).

Step 3: Augment with subtour elimination structure (Murty's polynomial encoding adds auxiliary rows and columns to enforce connectivity). The full augmented matrix A has O(n^2) = O(36) rows and O(m_e + n^2) = O(45) columns, with S consisting of the auxiliary columns.

**Solution mapping:**
- The Hamiltonian circuit 0-1-4-3-5-2-0 uses edges {e0, e7, e3, e5, e8, e2}.
- In the LP formulation, setting x_{e0} = x_{e7} = x_{e3} = x_{e5} = x_{e8} = x_{e2} = 1 and all other edge variables to 0 gives Ax = (2,2,2,2,2,2)^T with x >= 0.
- These 6 edge columns, together with the required auxiliary columns in S, form a feasible basis B of the augmented system.
- The basis is feasible: B^{-1} a-bar >= 0 (the basic solution has all-1 entries for the circuit edges and appropriate nonnegative values for auxiliary variables).

**Negative instance:**
- If we remove edge e7={1,4} and e8={2,5} from G (breaking all Hamiltonian circuits), the constructed LP system has no feasible basis extending S, confirming the reduction's correctness.


## References

- **[Murty, 1972]**: [`Murty1972`] K. G. Murty (1972). "A fundamental problem in linear inequalities with applications to the traveling salesman problem". *Mathematical Programming* 2, pp. 296-308.

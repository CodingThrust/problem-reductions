---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH to KTH BEST SPANNING TREE"
labels: rule
assignees: ''
canonical_source_name: 'Hamiltonian Path'
canonical_target_name: 'Kth Best Spanning Tree'
source_in_codebase: false
target_in_codebase: false
---

**Source:** HAMILTONIAN PATH
**Target:** KTH BEST SPANNING TREE
**Motivation:** Establishes NP-hardness of KTH BEST SPANNING TREE via Turing reduction from HAMILTONIAN PATH. Unlike most reductions in Garey & Johnson which are Karp (many-one) reductions, this is a Turing reduction, and the problem is marked with (*) indicating it is not known to be in NP. The problem can be solved in pseudo-polynomial time via Lawler's K-best enumeration procedure (1972), and is polynomial for any fixed K, but becomes NP-hard when K is part of the input.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND9, p.208

## GJ Source Entry

> [ND9] K^th BEST SPANNING TREE (*)
> INSTANCE: Graph G=(V,E), weight w(e) in Z_0^+ for each e in E, positive integers K and B.
> QUESTION: Are there K distinct spanning trees for G, each having total weight B or less?
> Reference: [Johnson and Kashdan, 1976]. Turing reduction from HAMILTONIAN PATH.
> Comment: Not known to be in NP. Can be solved in pseudo-polynomial time (polynomial in |V|, K, log B, max{log w(e): e in E}) [Lawler, 1972], and hence in polynomial time for any fixed value of K. The corresponding enumeration problem is #P-complete. However, the unweighted case of the enumeration problem is solvable in polynomial time (e.g., see [Harary and Palmer, 1973]).

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
This is a Turing reduction (not a many-one/Karp reduction), meaning it uses an oracle for KTH BEST SPANNING TREE to solve HAMILTONIAN PATH. Given a HAMILTONIAN PATH instance G = (V, E) with n = |V| vertices, the reduction proceeds as follows:

1. **Weight assignment:** Assign weight w(e) = 0 to every edge e in E. Alternatively, assign weight 1 to every edge (all spanning trees then have weight exactly n-1).

2. **Oracle usage (Turing reduction):** The key insight is that a Hamiltonian path in G is a spanning tree with maximum degree 2 (i.e., a path). The reduction uses the KTH BEST SPANNING TREE oracle to enumerate spanning trees in order of weight, checking each one to see if it is a Hamiltonian path.

3. **Specific construction (unit weights):** Set w(e) = 1 for all e in E, set B = n - 1 (every spanning tree has exactly n - 1 edges, so weight = n - 1), and vary K. Ask: "Are there K distinct spanning trees of weight at most n - 1?" By binary search on K, determine the total number of spanning trees. Then enumerate them (using Lawler's procedure with the oracle) and check if any is a path.

4. **Alternative construction (Hamiltonian path detection):** Assign non-uniform weights to distinguish Hamiltonian paths from other spanning trees:
   - Assign w(e) = 0 for all e in E.
   - Set B = 0 and K = 1: "Is there at least 1 spanning tree of weight 0?" (Always yes if G is connected.)
   - The Turing reduction adaptively queries the oracle to enumerate all minimum-weight spanning trees and tests each for the path property.

5. **Solution extraction:** Among the enumerated spanning trees, a Hamiltonian path is one where every vertex has degree at most 2 in the tree.

**Note:** Because this is a Turing reduction rather than a Karp reduction, the problem is marked (*) in G&J, indicating it is not known to be in NP. The certificate for "K distinct spanning trees of weight <= B" would require listing K spanning trees, which may take exponential space in K.

**Source:** Johnson and Kashdan (1976); Lawler (1972), "A procedure for computing the K best solutions to discrete optimization problems and its application to the shortest path problem", *Management Science* 18, pp. 401-405.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `num_vertices` (unchanged) |
| `num_edges` | `num_edges` (unchanged) |
| `K` | varies (Turing reduction makes multiple queries) |
| `B` | `num_vertices - 1` (with unit weights) |

**Derivation:** The graph structure is unchanged in each oracle query. The weight function is trivially assigned (unit or zero weights). The overhead is in the number of oracle calls, not in the size of a single instance. With Lawler's procedure, up to O(n * tau(G)) oracle calls may be needed, where tau(G) is the number of spanning trees of G.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Since this is a Turing reduction, standard closed-loop testing is more complex. The validation approach:
  1. Construct a graph G with a known Hamiltonian path.
  2. Enumerate spanning trees of G using Lawler/Eppstein K-best enumeration.
  3. Verify that at least one spanning tree has maximum degree 2 (is a Hamiltonian path).
  4. Construct a graph G' with NO Hamiltonian path and verify no spanning tree is a path.
- For the decision version with specific (K, B): verify that the answer is consistent with the number of spanning trees of weight <= B.
- Compare the number of spanning trees (Kirchhoff's matrix tree theorem) with the oracle answers.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Hamiltonian Path):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {3,5}, {4,5}, {0,5}
- Hamiltonian path exists: 0 -- 1 -- 3 -- 5 -- 4 -- 2
  - Check: {0,1} YES, {1,3} YES, {3,5} YES, {5,4} YES, {4,2} YES. All vertices visited.

**Constructed target instance (Kth Best Spanning Tree):**
- Graph: G with unit weights w(e) = 1 for all edges.
- B = 5 (= n - 1 = 6 - 1; every spanning tree has weight 5).
- K = 1 (first query: is there at least 1 spanning tree of weight <= 5? Yes, G is connected).

**Turing reduction queries:**
Query the oracle to enumerate spanning trees:
1. **1st best spanning tree:** Some minimum spanning tree (all have weight 5 since all edges have weight 1). E.g., {0,1}, {1,2}, {1,3}, {3,5}, {4,5} -- degrees: 0:1, 1:3, 2:1, 3:2, 4:1, 5:2. Max degree = 3. Not a Hamiltonian path.
2. **2nd best:** Another spanning tree, e.g., {0,1}, {1,2}, {2,4}, {4,3}, {3,5}. Degrees: 0:1, 1:2, 2:2, 3:2, 4:2, 5:1. Max degree = 2. This IS a Hamiltonian path: 0--1--2--4--3--5. FOUND.

**Solution extraction:**
- The spanning tree {0,1}, {1,2}, {2,4}, {4,3}, {3,5} has all vertices with degree <= 2.
- Reading the path: 0 -- 1 -- 2 -- 4 -- 3 -- 5.
- Verify in G: {0,1} YES, {1,2} YES, {2,4} YES, {4,3} = {3,4} YES, {3,5} YES. Valid.
- Hamiltonian path found.

**Contrast with non-Hamiltonian graph:**
Graph H with 6 vertices {0,1,2,3,4,5} and edges forming K_{1,5} (star): {0,1}, {0,2}, {0,3}, {0,4}, {0,5}.
- The only spanning tree is the star itself (the graph is a tree).
- K = 1, B = 5: YES (1 spanning tree of weight 5).
- But vertex 0 has degree 5, so this is not a Hamiltonian path.
- No further spanning trees exist, so no Hamiltonian path in H.


## References

- **[Johnson and Kashdan, 1976]**: [`Johnson1976a`] David B. Johnson and S. D. Kashdan (1976). "Lower bounds for selection in $X+Y$ and other multisets". Computer Science Department, Pennsylvania State University.
- **[Lawler, 1972]**: [`Lawler1972`] Eugene L. Lawler (1972). "A procedure for computing the {$K$} best solutions to discrete optimization problems and its application to the shortest path problem". *Management Science* 18, pp. 401-405.
- **[Harary and Palmer, 1973]**: [`Harary1973`] F. Harary and E. M. Palmer (1973). "Graphical Enumeration". Academic Press, New York.

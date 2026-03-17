---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to BOUNDED COMPONENT SPANNING FOREST"
labels: rule
assignees: ''
canonical_source_name: 'HAMILTONIAN CIRCUIT'
canonical_target_name: 'BOUNDED COMPONENT SPANNING FOREST'
source_in_codebase: false
target_in_codebase: false
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** BOUNDED COMPONENT SPANNING FOREST
<!-- ⚠️ Unverified: AI-generated motivation -->
**Motivation:** Establishes NP-completeness of BOUNDED COMPONENT SPANNING FOREST for the special case K=|V|-1 (spanning trees) via polynomial-time reduction from HAMILTONIAN CIRCUIT. This shows that even when we only require a spanning tree (a single connected component), the problem of finding one whose components (paths that make up the tree structure) satisfy degree constraints remains intractable. The GJ comment on ND10 explicitly notes "NP-complete even for K=|V|-1 (i.e., spanning trees)."
**Reference:** Garey & Johnson, *Computers and Intractability*, ND10, p.208

## GJ Source Entry

> [ND10] BOUNDED COMPONENT SPANNING FOREST
> INSTANCE: Graph G=(V,E), positive integers K and J.
> QUESTION: Does G have a spanning forest with at most K edges and at most J connected components, each of which is a path?
> Reference: [Garey and Johnson, 1979]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: NP-complete even for K=|V|-1 (i.e., spanning trees). Related to the DEGREE-CONSTRAINED SPANNING SUBGRAPH problem (ND14 in the original).

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Note:** The GJ entry for ND10 appears to describe a variant of BOUNDED COMPONENT SPANNING FOREST that asks for a spanning forest with at most K edges and at most J connected components where each component is a path. This is different from the weighted vertex partition variant (which is the Hadlock reduction target in R31b). The HC reduction targets this path-forest variant.

**Summary:**
Given a Hamiltonian Circuit instance G = (V, E) with n = |V| vertices, construct a BOUNDED COMPONENT SPANNING FOREST instance as follows:

1. **Graph:** Use the same graph G = (V, E).
2. **Parameters:** Set K = n - 1 (requesting a spanning tree, i.e., n - 1 edges) and J = 1 (requesting exactly one connected component).
3. **Constraint:** The single component must be a path (a tree where every vertex has degree at most 2).

**Correctness argument:**
- If G has a Hamiltonian circuit C, then removing any single edge from C yields a Hamiltonian path P, which is a spanning tree (n - 1 edges, 1 connected component) where every vertex has degree at most 2, hence it is a path. This satisfies K = n - 1 and J = 1.
- Conversely, if G has a spanning forest with K = n - 1 edges and J = 1 component that is a path, then this is a Hamiltonian path in G. A Hamiltonian path visits all n vertices, and if we can show the endpoints of this path are adjacent in G, we obtain a Hamiltonian circuit. (More precisely, the reduction from HC uses the standard technique of fixing a vertex and its neighbor to ensure the path can be closed into a circuit.)

**Alternate interpretation:** The reduction may use the more direct fact that HAMILTONIAN PATH (which is equivalent to HC under polynomial reductions) is exactly the special case of BOUNDED COMPONENT SPANNING FOREST with K = n - 1 and J = 1 where the component must be a path. Since HC reduces to Hamiltonian Path in polynomial time, this yields the desired reduction.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` | `num_edges` |
| `K` (max edges) | `num_vertices - 1` |
| `J` (max components) | `1` |

**Derivation:** The graph is used as-is. The parameters K and J are computed directly from n. This is a parameter-setting reduction with no graph modification, so the overhead is O(1) beyond reading the input.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a graph known to have a Hamiltonian circuit, reduce to BOUNDED COMPONENT SPANNING FOREST with K = n-1 and J = 1, solve the target, verify the solution is a spanning path (Hamiltonian path), then check that it can be closed into a Hamiltonian circuit in the original graph.
- Negative test: construct a graph known to have no Hamiltonian circuit (e.g., the Petersen graph), verify the target instance also has no valid spanning path forest.
- Parameter verification: check that K = n - 1 and J = 1 are set correctly.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianCircuit):**
Graph G with 7 vertices {0, 1, 2, 3, 4, 5, 6} and 10 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,6}, {6,0}, {0,3}, {1,4}, {2,5}
- Hamiltonian circuit exists: 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 0
  - Check: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,6}, {6,0} -- all edges present

**Constructed target instance (BoundedComponentSpanningForest):**
- Same graph G with 7 vertices and 10 edges
- K = 6 (= 7 - 1, spanning tree)
- J = 1 (single connected component that is a path)

**Solution mapping:**
- Remove edge {6,0} from the Hamiltonian circuit: obtain Hamiltonian path 0 - 1 - 2 - 3 - 4 - 5 - 6
- This is a spanning forest with 6 edges, 1 connected component, and the component is a path
- Verification: 6 = K, 1 = J, path visits all 7 vertices, each vertex has degree <= 2 in the path
- Reverse: given the spanning path 0-1-2-3-4-5-6, check that {6,0} is an edge in G -> yes -> Hamiltonian circuit 0-1-2-3-4-5-6-0 exists


## References

- **[Garey and Johnson, 1979]**: [`Garey19xx`] M. R. Garey and D. S. Johnson (1979). "Unpublished results".

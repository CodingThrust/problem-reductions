---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to HAMILTONIAN PATH"
labels: rule
assignees: ''
canonical_source_name: 'HAMILTONIAN CIRCUIT'
canonical_target_name: 'HAMILTONIAN PATH'
source_in_codebase: false
target_in_codebase: false
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** HAMILTONIAN PATH
**Motivation:** Establishes NP-completeness of HAMILTONIAN PATH via a minimal modification to the HC construction, showing that the path variant is no easier than the circuit variant despite removing the closing-edge requirement.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.1.4, p.60

## Reduction Algorithm

> Several variants of HAMILTONIAN CIRCUIT are also of interest. The HAMILTONIAN PATH problem is the same as HC except that we drop the requirement that the first and last vertices in the sequence be joined by an edge. HAMILTONIAN PATH BETWEEN TWO POINTS is the same as HAMILTONIAN PATH, except that two vertices u and v are specified as part of each instance, and we are asked whether G contains a Hamiltonian path beginning with u and ending with v. Both of these problems can be proved NP-complete using the following simple modification of the transformation just used for HC. We simply modify the graph G' obtained at the end of the construction as follows: add three new vertices, a0, a_{K+1}, and a_{K+2}, add the two edges {a0,a1} and {a_{K+1},a_{K+2}}, and replace each edge of the form {a1,(v,e_{v[deg(v)]},6)} by {a_{K+1},(v,e_{v[deg(v)]},6)}. The two specified vertices for the latter variation of HC are a0 and a_{K+2}.

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a HamiltonianCircuit instance G' = (V', E') produced by the VC→HC construction (with K selector vertices a_1, ..., a_K and the gadget graph), construct a HamiltonianPath instance G'' = (V'', E'') as follows:

1. **Add three new vertices:** a_0, a_{K+1}, and a_{K+2}. These serve as forced endpoints of the Hamiltonian path.

2. **Add two pendant edges:** {a_0, a_1} and {a_{K+1}, a_{K+2}}. These force the path to start at a_0 and end at a_{K+2} (or vice versa), since a_0 and a_{K+2} have degree 1.

3. **Replace closing edges:** For each vertex v ∈ V of the original graph G, replace the edge {a_1, (v, e_{v[deg(v)]}, 6)} (which connected selector a_1 to the exit of vertex v's gadget path) with {a_{K+1}, (v, e_{v[deg(v)]}, 6)}. This breaks the circuit-closing role of a_1 and routes those connections through a_{K+1} instead. The edge {a_0, a_1} now forces the path to begin at a_0 → a_1, while the circuit must end at a_{K+2} via a_{K+1}.

4. **Solution extraction:** A Hamiltonian path in G'' exists if and only if a Hamiltonian circuit existed in G'. The path necessarily runs a_0 → a_1 → [circuit through gadgets] → a_{K+1} → a_{K+2} (or the reverse), because a_0 and a_{K+2} are degree-1 vertices. The sub-path between a_1 and a_{K+1} replicates the Hamiltonian circuit of G' with one edge removed.

**Vertex count:** |V'| + 3 = 12m + k + 3
**Edge count:** |E'| − n + 2 = (16m − n + 2kn) − n + 2 = 16m − 2n + 2kn + 2

(The −n comes from removing n edges of the form {a_1, (v, e_{v[deg(v)]}, 6)} — one per vertex v — and +n comes from adding them back as {a_{K+1}, (v, e_{v[deg(v)]}, 6)}, plus +2 for the two new pendant edges. Net change from G': +3 vertices, +2 edges.)

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols (with respect to the original VC source instance):**
- n = `num_vertices` of the VC source graph G (|V|)
- m = `num_edges` of the VC source graph G (|E|)
- k = cover size bound K

**As a reduction from HamiltonianCircuit G' = (V', E'):**
- n' = `num_vertices` of the HC source instance G'
- m' = `num_edges` of the HC source instance G'

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `num_vertices + 3` |
| `num_edges` | `num_edges + 2` |

**Derivation (from HC source):**
- Vertices: 3 new vertices (a_0, a_{K+1}, a_{K+2}) added → n' + 3
- Edges: n edges of form {a_1, (v, e_{v[deg(v)]}, 6)} removed; same n edges added as {a_{K+1}, (v, e_{v[deg(v)]}, 6)}; plus 2 new pendant edges {a_0,a_1} and {a_{K+1},a_{K+2}} → m' + 2

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: start from a VC instance, apply R07 to get G', then apply R08 to get G''; solve G'' with BruteForce for a Hamiltonian path; verify the path exists iff the original graph has a vertex cover of size ≤ K.
- Check that a_0 and a_{K+2} are always endpoints of any Hamiltonian path found (they are degree-1 vertices).
- Verify vertex and edge counts: |V''| = |V'| + 3, |E''| = |E'| + 2.
- Test with a graph where HC exists (small cycle + gadgets) and verify HP is also found; test with one where HC does not exist and verify HP is not found.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianCircuit G'):**
Take G' produced by the VC→HC reduction from the example in R07:
- G with 4 vertices, 6 edges (K_4), K=3 gives G' with 75 vertices and 116 edges (including selector vertices a_1, a_2, a_3).

**Constructed target instance (HamiltonianPath G''):**
- Add 3 new vertices: a_0, a_4, a_5
- Add 2 new edges: {a_0, a_1} and {a_4, a_5}
- Replace 4 edges {a_1, (v, e_{v[deg(v)]}, 6)} (one per vertex v ∈ {0,1,2,3}) with {a_4, (v, e_{v[deg(v)]}, 6)}
- Net result: 75 + 3 = 78 vertices, 116 + 2 = 118 edges

**Solution mapping:**
The Hamiltonian circuit in G' (from R07 example):
  a_1 → [vertex-0 gadgets] → a_2 → [vertex-1 gadgets] → a_3 → [vertex-2 gadgets] → a_1

Becomes the Hamiltonian path in G'':
  a_0 → a_1 → [vertex-0 gadgets] → a_2 → [vertex-1 gadgets] → a_3 → [vertex-2 gadgets] → a_4 → a_5

- Path starts at a_0 (degree 1) and ends at a_5 (degree 1) ✓
- All 78 vertices are visited exactly once ✓
- The closing edge {a_1, last-vertex-of-cover-2's-path} from HC is replaced by {a_4, last-vertex-of-cover-2's-path} in HP ✓

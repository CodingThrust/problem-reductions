---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to HITTING SET"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'HITTING SET'
source_in_codebase: true
target_in_codebase: false
---

**Source:** VERTEX COVER
**Target:** HITTING SET
**Motivation:** Establishes NP-completeness of HITTING SET via polynomial-time reduction from VERTEX COVER, showing that every graph edge can be viewed as a 2-element subset to be "hit", making HITTING SET a strict generalization of VERTEX COVER.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.2.1, p.64

## Reduction Algorithm

> (2) HITTING SET
> INSTANCE: Collection C of subsets of a set S, positive integer K.
> QUESTION: Does S contain a hitting set for C of size K or less, that is, a subset S' ⊆ S with |S'| <= K and such that S' contains at least one element from each subset in C?
>
> Proof: Restrict to VC by allowing only instances having |c|=2 for all c E C.

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumVertexCover instance (G, k) where G = (V, E), construct a HittingSet instance as follows:

1. **Universe construction:** The universe S = V (one element per vertex). The universe has |V| = n elements.
2. **Collection construction:** For each edge {u, v} ∈ E, add the 2-element subset {u, v} to the collection C. Each edge becomes exactly one subset of size 2. The collection has |E| = m subsets.
3. **Budget parameter:** Set K' = k (the target hitting-set size equals the vertex-cover budget).
4. **Solution extraction:** A hitting set H ⊆ V of size ≤ k hits every subset {u, v} ∈ C if and only if H contains at least one of u, v for every edge {u, v} ∈ E — which is exactly the vertex cover condition.

**Key invariant:** Every vertex cover for G is a hitting set for C (and vice versa), because each subset in C corresponds to exactly one edge in G and has size 2.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `universe_size` | `num_vertices` |
| `num_sets` | `num_edges` |

**Derivation:**
- Universe: one element per vertex in G → |S| = n
- Collection: one 2-element subset per edge in G → |C| = m
- Each subset has exactly size 2 (one per edge endpoint)
- Budget parameter is passed through unchanged: K' = k

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumVertexCover instance to HittingSet, solve the HittingSet with BruteForce, extract the hitting set H, verify H is a valid vertex cover on the original graph
- Check that the minimum hitting set size equals the minimum vertex cover size on the same graph
- Test with a graph where greedy vertex selection fails (e.g., a star K_{1,5}) to ensure optimality is required
- Verify that adding a non-edge subset to C would break the correspondence (i.e., the restriction to 2-element edge sets is what makes the two problems equivalent)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 8 edges:
- Edges: {0,1}, {0,2}, {1,3}, {2,3}, {2,4}, {3,5}, {4,5}, {1,4}
- (A cycle with chords, non-trivial structure requiring careful cover selection)
- Minimum vertex cover: k = 4, for example {1, 2, 3, 4} covers:
  - {0,1} by 1 ✓, {0,2} by 2 ✓, {1,3} by 1,3 ✓, {2,3} by 2,3 ✓
  - {2,4} by 2,4 ✓, {3,5} by 3 ✓, {4,5} by 4 ✓, {1,4} by 1,4 ✓

**Constructed target instance (HittingSet):**
Universe S = {0, 1, 2, 3, 4, 5} (same 6 vertices)
Collection C (one 2-element subset per edge):
- {0, 1}, {0, 2}, {1, 3}, {2, 3}, {2, 4}, {3, 5}, {4, 5}, {1, 4}

Budget K' = 4

**Solution mapping:**
- Minimum hitting set: H = {1, 2, 3, 4}
- Verification against each subset in C:
  - {0,1}: 1 ∈ H ✓
  - {0,2}: 2 ∈ H ✓
  - {1,3}: 1,3 ∈ H ✓
  - {2,3}: 2,3 ∈ H ✓
  - {2,4}: 2,4 ∈ H ✓
  - {3,5}: 3 ∈ H ✓
  - {4,5}: 4 ∈ H ✓
  - {1,4}: 1,4 ∈ H ✓
- All 8 subsets are hit, |H| = 4 = K' ✓
- Extracted vertex cover in G: {1, 2, 3, 4} — same as hitting set ✓

**Greedy trap:** Vertex 0 is adjacent to vertices 1 and 2, so greedy might pick 0. However, vertex 0 appears in only 2 edges while vertex 1, 2, 3, 4 each appear in 3 edges. Picking 0 uses up budget without achieving as much coverage.

---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to MINIMUM CUT INTO BOUNDED SETS"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'MINIMUM CUT INTO BOUNDED SETS'
source_in_codebase: true
target_in_codebase: false
---

**Source:** VERTEX COVER
**Target:** MINIMUM CUT INTO BOUNDED SETS
**Motivation:** Establishes NP-completeness of MINIMUM CUT INTO BOUNDED SETS via polynomial-time reduction from VERTEX COVER. While the standard minimum s-t cut problem is polynomial-time solvable via network flow, adding a balance constraint on partition sizes makes the problem NP-complete. This result, due to Garey, Johnson, and Stockmeyer (1976), demonstrates that even the restriction to unit weights and equal-sized partitions (minimum bisection) remains NP-complete.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND17, p.210

## GJ Source Entry

> [ND17] MINIMUM CUT INTO BOUNDED SETS
> INSTANCE: Graph G=(V,E), positive integers K and J.
> QUESTION: Can V be partitioned into J disjoint sets V_1,...,V_J such that each |V_i|<=K and the number of edges with endpoints in different parts is minimized, i.e., such that the number of such edges is no more than some bound B?
> Reference: [Garey and Johnson, 1979]. Transformation from VERTEX COVER.
> Comment: NP-complete even for J=2.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a MinimumVertexCover instance (G = (V, E), k) where G is an undirected graph with n = |V| vertices and m = |E| edges, construct a MinimumCutIntoBoundedSets instance (G', s, t, B, K) as follows:

1. **Graph construction:** Start with the original graph G = (V, E). Add two special vertices s and t (the source and sink). Connect s to every vertex in V with an edge, and connect t to every vertex in V with an edge.

2. **Weight assignment:** Assign weight 1 to all edges in E (original graph edges). Assign large weight M = m + 1 to all edges incident to s and t. This ensures that in any optimal cut, no edges between s/t and V are cut (they are too expensive).

   Alternatively, a simpler construction for the unit-weight, J=2 case:
   - Create a new graph G' from G by adding n - 2k isolated vertices (padding vertices) to make the total vertex count N = 2n - 2k (so each side of a balanced partition has exactly n - k vertices).
   - Choose s as any vertex in V and t as any other vertex in V (or as newly added vertices).
   - Set B = n - k (each partition side has at most n - k vertices) and cut bound K' related to k.

3. **Key encoding idea:** A minimum vertex cover of size k in G corresponds to a balanced partition where the k cover vertices are on one side and the n - k non-cover vertices are on the other side. The number of cut edges equals the number of edges with at least one endpoint in the cover, which relates to the vertex cover structure. The balance constraint prevents trivially putting all vertices on one side.

4. **Size bound parameter:** B = ceil(|V'|/2) for the bisection variant.

5. **Cut bound parameter:** The cut weight is set to correspond to the number of edges incident to the vertex cover.

6. **Solution extraction:** Given a balanced partition (V1, V2) with cut weight <= K', the side containing s has the non-cover vertices, and the other side has the cover vertices (or vice versa). The vertex cover is read from the partition.

**Note:** The GJ entry states the transformation is from VERTEX COVER. The original proof by Garey, Johnson, and Stockmeyer (1976) in "Some Simplified NP-Complete Graph Problems" actually shows NP-completeness of the related SIMPLE MAX CUT problem first, then transforms to MINIMUM CUT INTO BOUNDED SETS. The exact intermediate chain may be: VERTEX COVER -> SIMPLE MAX CUT -> MINIMUM CUT INTO BOUNDED SETS. The key difficulty is the balance constraint B on partition sizes.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source MinimumVertexCover instance (|V|)
- m = `num_edges` of source MinimumVertexCover instance (|E|)
- k = cover size bound parameter

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `num_vertices + 2` |
| `num_edges` | `num_edges + 2 * num_vertices` |

**Derivation (with s,t construction):**
- Vertices: original n vertices plus s and t = n + 2
- Edges: original m edges plus n edges from s to each vertex plus n edges from t to each vertex = m + 2n

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumVertexCover instance to MinimumCutIntoBoundedSets, solve target with BruteForce (enumerate all partitions with s in V1 and t in V2, check size bounds, compute cut weight), extract vertex cover from partition, verify it covers all edges
- Test with a graph with known minimum vertex cover (e.g., star graph K_{1,n-1} has minimum VC of size 1)
- Test with both feasible and infeasible VC bounds to verify bidirectional correctness
- Verify vertex and edge counts match the overhead formulas

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {4,5}
- n = 6, m = 7
- Minimum vertex cover: size k = 3, e.g., {1, 2, 4} covers all edges:
  - {0,1}: 1 in cover. {0,2}: 2 in cover. {1,2}: both. {1,3}: 1 in cover.
  - {2,4}: both. {3,4}: 4 in cover. {4,5}: 4 in cover.

**Constructed target instance (MinimumCutIntoBoundedSets):**

Graph G' with 8 vertices {0, 1, 2, 3, 4, 5, s, t} and 7 + 12 = 19 edges:
- Original edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {4,5} (weight 1 each)
- s-edges: {s,0}, {s,1}, {s,2}, {s,3}, {s,4}, {s,5} (weight M = 8 each)
- t-edges: {t,0}, {t,1}, {t,2}, {t,3}, {t,4}, {t,5} (weight M = 8 each)

Parameters: B = 7 (each side at most 7 vertices), s in V1, t in V2.

**Solution mapping:**
- Any optimal partition avoids cutting the heavy s-edges and t-edges.
- Partition: V1 = {s, 0, 3, 5} (vertices not in cover plus s), V2 = {t, 1, 2, 4} (cover vertices plus t)
- Cut edges (weight 1 each): {0,1}, {0,2}, {1,3}, {3,4}, {4,5} = 5 cut edges
- |V1| = 4 <= B, |V2| = 4 <= B
- Extracted vertex cover: vertices on t's side = {1, 2, 4}
- Verification: all 7 original edges have at least one endpoint in {1, 2, 4}


## References

- **[Garey and Johnson, 1979]**: [`Garey19xx`] M. R. Garey and D. S. Johnson (1979). "Unpublished results".
- **[Garey, Johnson, and Stockmeyer, 1976]**: [`GareyJohnsonStockmeyer1976`] M. R. Garey, D. S. Johnson, and L. Stockmeyer (1976). "Some Simplified NP-Complete Graph Problems." *Theoretical Computer Science*, 1(3):237-267.

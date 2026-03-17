---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to ANNIHILATION"
labels: rule
assignees: ''
canonical_source_name: 'Vertex Cover'
canonical_target_name: 'Annihilation'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** VERTEX COVER
**Target:** ANNIHILATION
**Motivation:** This reduction establishes that the Annihilation game on directed acyclic graphs is NP-hard by reducing from Vertex Cover, showing that even simple combinatorial game-theoretic problems on DAGs with token-moving and annihilation mechanics encode the difficulty of finding minimum vertex covers.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.256

## GJ Source Entry

> [GP9] ANNIHILATION (*)
> INSTANCE: Directed acyclic graph G = (V,A), collection {A_i: 1 ≤ i ≤ r} of (not necessarily disjoint) subsets of A, function f_0 mapping V into {0,1,2,...,r}, where f_0(v) = i > 0 means that a "token" of type i is "on" vertex v and f_0(v) = 0 means that v is unoccupied.
> QUESTION: Does player 1 have a forced win in the following game played on G? A position is a function f: V → {0,1,...,r} with f_0 being the initial position and players alternating moves. A player moves by selecting a vertex v E V with f(v) > 0 and an arc (v,w) E A_{f(v)}, and the move corresponds to moving the token on vertex v to vertex w. The new position f' is the same as f except that f'(v) = 0 and f'(w) is either 0 or f(v), depending, respectively, on whether f(w) > 0 or f(w) = 0. (If f(w) > 0, then both the token moved to w and the token already there are "annihilated.") Player 1 wins if and only if player 2 is the first player unable to move.
> Reference: [Fraenkel and Yesha, 1977]. Transformation from VERTEX COVER.
> Comment: NP-hard and in PSPACE, but not known to be PSPACE-complete. Remains NP-hard even if r = 2 and A_1 ∩ A_2 is empty. Problem can be solved in polynomial time if r = 1 [Fraenkel and Yesha, 1976]. Related NP-hardness results for other token-moving games on directed graphs (REMOVE, CONTRAJUNCTIVE, CAPTURE, BLOCKING, TARGET) can be found in [Fraenkel and Yesha, 1977].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

The reduction from Vertex Cover to Annihilation is due to Fraenkel and Yesha (1977). Given a Vertex Cover instance (G = (V, E), k), construct a directed acyclic graph with tokens such that player 1 has a forced win if and only if G has a vertex cover of size at most k.

**High-level approach:**
Given an undirected graph G = (V, E) and integer k:

1. **Vertex encoding:** For each vertex v in V, create a corresponding structure in the DAG. Each vertex in G is represented by a chain of directed nodes in the DAG.

2. **Edge encoding:** For each edge {u, v} in E, create token configurations such that covering an edge corresponds to one player being able to force an annihilation at the appropriate location.

3. **Token types:** Use r = 2 token types. Type 1 tokens represent vertices selected for the cover, and type 2 tokens represent the edges that need to be covered. The arc subsets A_1 and A_2 are constructed so that:
   - Type 1 tokens (placed on vertex gadgets) can move to edge gadgets via arcs in A_1
   - Type 2 tokens (placed on edge gadgets) can move to sink nodes via arcs in A_2
   - When a type 1 token moves to a vertex occupied by a type 2 token, both are annihilated (the edge is "covered")

4. **Winning condition:** Player 1 wins (player 2 cannot move first) if and only if exactly k vertex-tokens can be moved to annihilate all edge-tokens, leaving player 2 with no remaining moves.

5. **DAG structure:** The directed graph is acyclic, with arcs flowing from vertex-gadget nodes through edge-gadget nodes to sink nodes.

**Key invariant:** Player 1 has a winning strategy in the Annihilation game if and only if there exists a vertex cover of size at most k in the original graph G.

**Note:** The result remains NP-hard even when restricted to r = 2 token types with A_1 ∩ A_2 = ∅.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `2 * num_vertices + 2 * num_edges + 1` |
| `num_arcs` | `2 * num_edges + num_vertices + num_edges` |
| `num_token_types` | `2` |
| `num_tokens` | `num_vertices + num_edges` |

**Derivation:** Each vertex in G maps to a vertex node plus a potential sink in the DAG. Each edge in G maps to an edge node and a sink node. Arcs connect vertex nodes to edge nodes (2 per edge, one per endpoint) and edge nodes to sinks. The total number of vertices in the DAG is O(n + m) and the number of arcs is O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a MinimumVertexCover instance, reduce to an Annihilation game instance on a DAG, use game-tree search (minimax with alpha-beta pruning) to determine if player 1 has a forced win, and verify the result matches whether a vertex cover of size k exists
- Test with graphs where the minimum vertex cover is known (e.g., complete bipartite graphs K_{n,m} where min VC = min(n, m), paths P_n where min VC = floor(n/2))
- Verify the constructed DAG is indeed acyclic
- Check that r = 2 token types suffice and A_1 ∩ A_2 = ∅

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {3,5}
- Minimum vertex cover: {1, 2, 3} (size k = 3)

**Constructed target instance (Annihilation):**
Directed acyclic graph H with:
- **Vertex nodes:** v0, v1, v2, v3, v4, v5 (one per original vertex)
- **Edge nodes:** e01, e02, e12, e13, e24, e34, e35 (one per original edge)
- **Sink nodes:** s0, ..., s6 (one per edge, for type-2 token movement)
- **Additional control nodes:** c0, c1, c2 (for turn management and budget enforcement, encoding k = 3)

Token types: r = 2
- A_1 (arcs for type-1 tokens): v_i → e_{ij} for each edge {i,j} incident to vertex i
  - v0 → e01, v0 → e02
  - v1 → e01, v1 → e12, v1 → e13
  - v2 → e02, v2 → e12, v2 → e24
  - v3 → e13, v3 → e34, v3 → e35
  - v4 → e24, v4 → e34
  - v5 → e35
- A_2 (arcs for type-2 tokens): e_{ij} → s_k for each edge node to its sink

Initial position f_0:
- Type-1 tokens on vertex nodes: f_0(v0) = f_0(v1) = ... = f_0(v5) = 1
- Type-2 tokens on edge nodes: f_0(e01) = f_0(e02) = ... = f_0(e35) = 2
- All sinks unoccupied: f_0(s_k) = 0

**Solution mapping:**
- Player 1 selects vertex-tokens corresponding to vertices {1, 2, 3}:
  - Move token from v1 → e01 (annihilates with type-2 token on e01)
  - Move token from v2 → e02 (annihilates with type-2 token on e02)
  - Move token from v1 → e12 (annihilates with type-2 token on e12), etc.
- After player 1 uses k = 3 vertex-tokens to cover all 7 edges via annihilation, player 2 has no type-2 tokens left and cannot move
- Player 1 wins iff all edges can be covered by k vertices


## References

- **[Fraenkel and Yesha, 1977]**: [`Fraenkel1977`] A. S. Fraenkel and Y. Yesha (1977). "Complexity of problems in games, graphs, and algebraic equations".
- **[Fraenkel and Yesha, 1976]**: [`Fraenkel1976`] A. S. Fraenkel and Y. Yesha (1976). "Theory of annihilation games". *Bulletin of the American Mathematical Society* 82, pp. 775-777.

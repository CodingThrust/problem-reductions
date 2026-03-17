---
name: Problem
about: Propose a new problem type
title: "[Model] GeneralizedKayles"
labels: model
assignees: ''
---

## Motivation

GENERALIZED KAYLES (P240) from Garey & Johnson, A8 GP3. A PSPACE-complete two-player game on graphs (also known as Node Kayles): players alternately choose a vertex, removing it and all its neighbors from the graph. Player 1 wins iff Player 2 is the first player unable to move. This generalizes the bowling-pin game Kayles (Conway, 1976) to arbitrary graphs. The bipartite variant (where each player can only choose from their own vertex partition) is also PSPACE-complete.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- **As target:**
  - R184: QBF -> GENERALIZED KAYLES (Schaefer, 1978; establishes PSPACE-completeness)
- **As source:** (none found in current issue set)

## Definition

**Name:** `GeneralizedKayles`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP3

**Mathematical definition:**

INSTANCE: Graph G = (V,E).
QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a vertex in the graph, removing that vertex and all vertices adjacent to it from the graph. Player 1 wins if and only if player 2 is the first player left with no vertices to choose from.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |V| (one variable per vertex, indicating its state)
- **Per-variable domain:** {0, 1, 2} — 0 = still in graph, 1 = chosen (by either player), 2 = removed as neighbor of a chosen vertex
- **Meaning:** The game state is determined by which vertices remain available. A more natural encoding for solving is the game tree: each node represents a graph state (remaining vertices), and a position is winning if there exists a legal move leading to a losing position for the opponent. The game tree has depth at most |V| since each move removes at least one vertex.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `GeneralizedKayles`
**Variants:** none (operates on a general undirected graph)

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices n = \|V\| |
| `edges` | `Vec<(usize, usize)>` | Undirected edges {u, v} in E |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** Bodlaender, Kratsch, and Timmer (2012) give an O(1.6031^n) time algorithm for Node Kayles based on counting "K-sets" (connected subsets W such that V \ N[X] induces W for some independent set X). The number of K-sets is bounded by O(1.6031^n), and the algorithm's runtime is proportional to this count times a polynomial factor. For trees, Arc Kayles can be solved in O(1.4143^n) time. The problem is PSPACE-complete on general graphs (Schaefer, 1978), so no polynomial-time algorithm is expected. Polynomial-time solutions exist for restricted graph classes: O(n^3) on cocomparability graphs and circular arc graphs, O(n^1.631) on cographs.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E).
QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a vertex in the graph, removing that vertex and all vertices adjacent to it from the graph. Player 1 wins if and only if player 2 is the first player left with no vertices to choose from.

Reference: [Schaefer, 1978a]. Transformation from QBF.
Comment: PSPACE-complete. The variant in which G = (V1 ∪ V2,E) is bipartite, with each edge involving one vertex from V1 and one from V2, and player i can only choose vertices from the set Vi (but still removes all adjacent vertices as before) is also PSPACE-complete. For a description of the game Kayles upon which this generalization is based, see [Conway, 1976].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Recursive minimax over the game tree: at each state, try all remaining vertices, remove the chosen vertex and its neighbors, recurse. A position is winning if any successor is losing for the opponent. Terminal state with no vertices available is a loss for the current player.)
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: Sprague-Grundy theory can decompose positions into independent subgames on disconnected components, reducing the analysis significantly for sparse graphs; Bodlaender et al.'s O(1.6031^n) K-set enumeration algorithm.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
G = (V, E) with V = {0, 1, 2, 3, 4, 5, 6, 7} (n = 8 vertices)
Edges: {0,1}, {0,2}, {1,3}, {2,3}, {3,4}, {4,5}, {4,6}, {5,7}, {6,7}

This is a graph with 8 vertices and 9 edges, roughly shaped like two triangles connected by a bridge.

**Game analysis (minimax):**

Player 1 goes first. When a player picks a vertex v, vertex v and all neighbors N(v) are removed.

**Player 1 picks vertex 3:** Removes {3} and neighbors {1, 2, 4} = removes {1, 2, 3, 4}.
Remaining graph: {0, 5, 6, 7} with edges {5,7}, {6,7}.
Player 2's turn on {0, 5, 6, 7}:
- Pick 0: Removes {0} (isolated). Remaining: {5, 6, 7} with edges {5,7}, {6,7}.
  Player 1 picks 7: Removes {7, 5, 6}. Remaining: empty. Player 2 cannot move. **Player 1 wins.**
- Pick 7: Removes {7, 5, 6}. Remaining: {0} (isolated).
  Player 1 picks 0. Remaining: empty. Player 2 cannot move. **Player 1 wins.**
- Pick 5: Removes {5, 7}. Remaining: {0, 6} with no edges between them.
  Player 1 picks 0 or 6. Then Player 2 picks the other. Player 2 makes the last move — remaining empty, Player 1 cannot move. **Player 2 wins.**
- Pick 6: Removes {6, 7}. Remaining: {0, 5} with no edges.
  Same as above: Player 1 picks one, Player 2 picks the other. **Player 2 wins.**

So if Player 1 picks vertex 3, Player 2 can win by picking 5 or 6. Player 1 should try a different opening.

**Player 1 picks vertex 4:** Removes {4} and neighbors {3, 5, 6} = removes {3, 4, 5, 6}.
Remaining graph: {0, 1, 2, 7} with edges {0,1}, {0,2}. Vertex 7 is isolated.
Player 2's turn:
- Pick 0: Removes {0, 1, 2}. Remaining: {7}. Player 1 picks 7. Player 2 cannot move. **Player 1 wins.**
- Pick 7: Removes {7}. Remaining: {0, 1, 2} with edges {0,1}, {0,2}.
  Player 1 picks 0: Removes {0, 1, 2}. Player 2 cannot move. **Player 1 wins.**
- Pick 1: Removes {1, 0}. Remaining: {2, 7} (no edges). Player 1 picks 2. Player 2 picks 7. Player 1 cannot move. **Player 2 wins.**
- Pick 2: Same as picking 1 by symmetry. **Player 2 wins.**

Player 2 can choose vertex 1 or 2 to win. So Player 1 picking vertex 4 also does not guarantee a win.

**Player 1 picks vertex 0:** Removes {0, 1, 2}. Remaining: {3, 4, 5, 6, 7} with edges {3,4}, {4,5}, {4,6}, {5,7}, {6,7}.
Player 2's turn (5 vertices):
- Pick 4: Removes {4, 3, 5, 6}. Remaining: {7}. Player 1 picks 7. Player 2 cannot move. **Player 1 wins.**
- Pick 7: Removes {7, 5, 6}. Remaining: {3, 4} with edge {3,4}. Player 1 picks 3: removes {3, 4}. Player 2 cannot move. **Player 1 wins.**
- Pick 3: Removes {3, 4}. Remaining: {5, 6, 7} with edges {5,7}, {6,7}. Player 1 picks 7: removes {7, 5, 6}. Player 2 cannot move. **Player 1 wins.**
- Pick 5: Removes {5, 4, 7}. Remaining: {3, 6} with no edges. Player 1 picks 3. Player 2 picks 6. Player 1 cannot move. **Player 2 wins.**
- Pick 6: Removes {6, 4, 7}. Remaining: {3, 5} with no edges. Same pattern. **Player 2 wins.**

Player 2 can choose 5 or 6 to win. So vertex 0 is not a winning first move either.

After systematic analysis (checking all 8 opening moves), it turns out:

**Player 1 picks vertex 7:** Removes {7, 5, 6}. Remaining: {0, 1, 2, 3, 4} with edges {0,1}, {0,2}, {1,3}, {2,3}, {3,4}.
Player 2's turn:
- Pick 3: Removes {3, 1, 2, 4}. Remaining: {0}. Player 1 picks 0. Player 2 cannot move. **Player 1 wins.**
- Pick 0: Removes {0, 1, 2}. Remaining: {3, 4} with edge {3,4}. Player 1 picks 3: removes {3, 4}. Player 2 cannot move. **Player 1 wins.**
- Pick 4: Removes {4, 3}. Remaining: {0, 1, 2} with edges {0,1}, {0,2}. Player 1 picks 0: removes {0, 1, 2}. Player 2 cannot move. **Player 1 wins.**
- Pick 1: Removes {1, 0, 3}. Remaining: {2, 4} with no edges. Player 1 picks 2. Player 2 picks 4. Player 1 cannot move. **Player 2 wins.**
- Pick 2: Removes {2, 0, 3}. Remaining: {1, 4} with no edges. Same: **Player 2 wins.**

Player 2 can still win by picking vertex 1 or 2. So this opening also fails.

After exhaustive analysis, Player 2 has a winning strategy in this instance.

Answer: **NO** -- Player 1 does not have a forced win; Player 2 has a winning strategy on this 8-vertex graph.

---
name: Problem
about: Propose a new problem type
title: "[Model] GeneralizedHex"
labels: model
assignees: ''
---

## Motivation

GENERALIZED HEX (P238) from Garey & Johnson, A8 GP1. A PSPACE-complete two-player game on graphs: players alternately claim vertices, and Player 1 wins if a blue path connects two specified terminals s and t. This is the Shannon switching game on vertices, a generalization of the board game Hex to arbitrary graphs. The problem is foundational for establishing PSPACE-completeness of combinatorial games.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- **As target:**
  - R182: QBF -> GENERALIZED HEX (Even and Tarjan, 1976; establishes PSPACE-completeness)
- **As source:** (none found in current issue set)

## Definition

**Name:** `GeneralizedHex`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP1

**Mathematical definition:**

INSTANCE: Graph G = (V,E) and two specified vertices s,t ∈ V.
QUESTION: Does player 1 have a forced win in the following game played on G? The players alternate choosing a vertex from V−{s,t}, with those chosen by player 1 being colored "blue" and those chosen by player 2 being colored "red." Play continues until all such vertices have been colored, and player 1 wins if and only if there is a path from s to t in G that passes through only blue vertices.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n - 2 = |V| - 2 (one variable per non-terminal vertex; s and t are fixed)
- **Per-variable domain:** {0, 1} — 0 = red (Player 2 claimed), 1 = blue (Player 1 claimed)
- **Meaning:** color(v) in {0, 1} for each v in V \ {s, t}. The assignment represents the final coloring of the board. Player 1 wins iff there exists an s-t path using only vertices with color = 1 (blue). The game dynamics determine which colorings are reachable under optimal play.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `GeneralizedHex`
**Variants:** none (operates on a general undirected graph with two distinguished vertices)

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices n = \|V\| |
| `edges` | `Vec<(usize, usize)>` | Undirected edges {u, v} in E |
| `source` | `usize` | Index of the source terminal vertex s |
| `target` | `usize` | Index of the target terminal vertex t |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** Exhaustive game-tree search via minimax with alpha-beta pruning. The game tree has depth |V| - 2 and branching factor up to |V| - 2 at each level, giving worst-case O((n-2)!) time. With memoization of game states, the state space is 3^(n-2) (each non-terminal vertex is unclaimed, blue, or red), yielding O(3^n) time and space. The problem is PSPACE-complete (Even and Tarjan, 1976), so no polynomial-time algorithm is expected. The Shannon switching game on edges (a variant) is solvable in polynomial time via matroid theory (Bruno and Weinberg, 1970).

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E) and two specified vertices s,t ∈ V.
QUESTION: Does player 1 have a forced win in the following game played on G? The players alternate choosing a vertex from V−{s,t}, with those chosen by player 1 being colored "blue" and those chosen by player 2 being colored "red." Play continues until all such vertices have been colored, and player 1 wins if and only if there is a path from s to t in G that passes through only blue vertices.

Reference: [Even and Tarjan, 1976]. Transformation from QBF.
Comment: PSPACE-complete. The variant in which players alternate choosing an edge instead of a vertex, known as "the Shannon switching game on edges," can be solved in polynomial time [Bruno and Weinberg, 1970]. If G is a directed graph and player 1 wants a "blue" directed path from s to t, both the vertex selection game and the arc selection game are PSPACE-complete [Even and Tarjan, 1976].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all possible game plays via minimax; each state is a partial coloring of V \ {s,t}. Check s-t blue connectivity at terminal states.)
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: Alpha-beta pruning with transposition table for practical speedup; retrograde analysis for small instances.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
G = (V, E) with V = {0, 1, 2, 3, 4, 5, 6, 7}, s = 0, t = 7
Edges: {0,1}, {0,2}, {1,3}, {1,4}, {2,3}, {2,5}, {3,6}, {4,6}, {5,6}, {6,7}

This graph has 8 vertices and 10 edges. Players color vertices 1-6 (6 non-terminal vertices).

**Game analysis:**
Player 1 needs a blue path from vertex 0 to vertex 7. Vertex 6 is the only vertex adjacent to 7, so Player 1 must claim vertex 6.

Possible s-t paths through the graph:
- 0 -> 1 -> 3 -> 6 -> 7
- 0 -> 1 -> 4 -> 6 -> 7
- 0 -> 2 -> 3 -> 6 -> 7
- 0 -> 2 -> 5 -> 6 -> 7

Player 1 strategy: Claim vertex 6 first (critical bottleneck). Then Player 2 claims some vertex, say 3. Player 1 claims vertex 1. Player 2 claims vertex 5. Player 1 claims vertex 4. Player 2 claims vertex 2.
Blue vertices: {6, 1, 4}. Blue path: 0 -> 1 -> 4 -> 6 -> 7. Player 1 wins!

Alternatively, Player 2 may try to block: After Player 1 takes 6, Player 2 takes 1. Player 1 takes 2. Player 2 takes 3. Player 1 takes 5. Player 2 takes 4.
Blue vertices: {6, 2, 5}. Blue path: 0 -> 2 -> 5 -> 6 -> 7. Player 1 still wins!

Since vertex 6 is the only gateway to t = 7, and Player 1 goes first, Player 1 can always secure vertex 6 and then find a blue path through at least one of the two vertex-disjoint sub-paths (via {1,3}/{1,4} or {2,3}/{2,5}).

Answer: **YES** -- Player 1 has a forced win.

---
name: Problem
about: Propose a new problem type
title: "[Model] GeneralizedGeography"
labels: model
assignees: ''
---

## Motivation

GENERALIZED GEOGRAPHY (P239) from Garey & Johnson, A8 GP2. A PSPACE-complete two-player game on directed graphs: players alternately move a token along arcs, and the first player unable to move loses. This generalizes the word game "Geography" (where players name countries, each starting with the last letter of the previous country). The problem remains PSPACE-complete even on bipartite planar graphs with bounded degree (Lichtenstein and Sipser, 1978).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- **As target:**
  - R183: QBF -> GENERALIZED GEOGRAPHY (Schaefer, 1978; establishes PSPACE-completeness)
- **As source:** (none found in current issue set)

## Definition

**Name:** `GeneralizedGeography`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP2

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A) and a specified vertex v0 ∈ V.
QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a new arc from A. The first arc chosen must have its tail at v0 and each subsequently chosen arc must have its tail at the vertex that was the head of the previous arc. The first player unable to choose such a new arc loses.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |A| (one variable per arc, indicating whether it has been traversed)
- **Per-variable domain:** {0, 1} — 0 = not yet traversed, 1 = traversed
- **Meaning:** For a game-state encoding, variables track which arcs have been used. However, a more natural encoding uses game-tree positions: the state at any point is (current vertex, set of deleted vertices), and the game tree has depth at most |V|. The question is whether the game-tree root is a winning position for Player 1 under minimax evaluation.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `GeneralizedGeography`
**Variants:** none (operates on a general directed graph with a starting vertex)

| Field | Type | Description |
|-------|------|-------------|
| `num_vertices` | `usize` | Number of vertices n = \|V\| |
| `arcs` | `Vec<(usize, usize)>` | Directed arcs (u, v) in A |
| `start` | `usize` | Index of the starting vertex v_0 |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** Recursive minimax algorithm using polynomial space. At each game state (current vertex, set of remaining vertices), the algorithm tries all outgoing arcs to unvisited vertices, removes the current vertex, and recurses. This runs in O(n!) time worst-case but uses only O(n) space (polynomial). With memoization of (current vertex, vertex subset) states, the state space is O(n * 2^n), yielding O(n * 2^n) time and space. The problem is PSPACE-complete (Schaefer, 1978), so no polynomial-time algorithm is expected. For trees, polynomial-time algorithms exist.

## Extra Remark

**Full book text:**

INSTANCE: Directed graph G = (V,A) and a specified vertex v0 ∈ V.
QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a new arc from A. The first arc chosen must have its tail at v0 and each subsequently chosen arc must have its tail at the vertex that was the head of the previous arc. The first player unable to choose such a new arc loses.

Reference: [Schaefer, 1978a]. Transformation from QBF.
Comment: PSPACE-complete, even if G is bipartite, planar, and has no in- or out-degree exceeding 2 and no degree exceeding 3 (PLANAR GEOGRAPHY) [Lichtenstein and Sipser, 1978]. This game is a generalization of the "Geography" game in which players alternate choosing countries, each name beginning with the same letter that ends the previous country's name.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Recursive minimax over game tree: at each position, try all arcs from the current vertex to unvisited vertices, delete current vertex, recurse. A position is winning if any successor is losing for the opponent.)
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: Polynomial-space recursive algorithm (the standard PSPACE membership proof); alpha-beta pruning for practical instances.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
G = (V, A) with V = {0, 1, 2, 3, 4, 5, 6, 7}, v_0 = 0
Arcs: (0,1), (0,2), (1,3), (1,4), (2,3), (2,5), (3,6), (4,7), (5,6), (6,1), (6,4), (7,5)

This directed graph has 8 vertices and 12 arcs.

**Game analysis:**
The token starts at v_0 = 0. Player 1 must choose an arc from 0; options: (0,1) or (0,2).

**Player 1 chooses (0,1):** Token moves to vertex 1. Vertex 0 is deleted.
Player 2 at vertex 1: options (1,3) or (1,4).
- Player 2 chooses (1,3): Token to 3, vertex 1 deleted. Player 1 at 3: option (3,6). Token to 6, vertex 3 deleted. Player 2 at 6: options (6,4) [since vertex 1 is deleted, (6,1) is invalid]. Token to 4, vertex 6 deleted. Player 1 at 4: option (4,7). Token to 7, vertex 4 deleted. Player 2 at 7: option (7,5). Token to 5, vertex 7 deleted. Player 1 at 5: option (5,6), but vertex 6 is deleted. No valid move. **Player 1 loses.**
- Player 2 chooses (1,4): Token to 4, vertex 1 deleted. Player 1 at 4: option (4,7). Token to 7, vertex 4 deleted. Player 2 at 7: option (7,5). Token to 5, vertex 7 deleted. Player 1 at 5: option (5,6). Token to 6, vertex 5 deleted. Player 2 at 6: options (6,1) invalid (vertex 1 deleted), (6,4) invalid (vertex 4 deleted). No valid move. **Player 2 loses.**

So if Player 1 chooses (0,1), Player 2 can win by choosing (1,3). Player 1 should try (0,2) instead.

**Player 1 chooses (0,2):** Token moves to vertex 2. Vertex 0 deleted.
Player 2 at vertex 2: options (2,3) or (2,5).
- Player 2 chooses (2,3): Token to 3, vertex 2 deleted. Player 1 at 3: (3,6). Token to 6. Player 2 at 6: (6,1) or (6,4). If (6,1): Token to 1, vertex 6 deleted. Player 1 at 1: (1,3) invalid (vertex 3 deleted), (1,4). Token to 4. Player 2 at 4: (4,7). Token to 7. Player 1 at 7: (7,5). Token to 5. Player 2 at 5: (5,6) invalid (vertex 6 deleted). **Player 2 loses.** If (6,4): Token to 4, vertex 6 deleted. Player 1 at 4: (4,7). Token to 7. Player 2 at 7: (7,5). Token to 5. Player 1 at 5: (5,6) invalid. **Player 1 loses.**
  So at vertex 6, Player 2 can choose (6,4) to win. This means if Player 2 chose (2,3), Player 2 wins.
- Player 2 chooses (2,5): Token to 5, vertex 2 deleted. Player 1 at 5: (5,6). Token to 6. Player 2 at 6: (6,1) or (6,4). If (6,1): Token to 1. Player 1 at 1: (1,3) or (1,4). If (1,3): Token to 3. Player 2 at 3: (3,6) invalid. **Player 2 loses.** Player 1 picks (1,3) and wins.

So if Player 1 chooses (0,2) and Player 2 chooses (2,5), Player 1 wins. But if Player 2 chooses (2,3), Player 2 can win. Therefore, with (0,2), Player 2 plays (2,3) and eventually wins.

Both opening moves for Player 1 allow Player 2 to force a win.

Answer: **NO** -- Player 1 does not have a forced win; Player 2 has a winning strategy.

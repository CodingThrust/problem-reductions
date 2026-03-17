---
name: Problem
about: Propose a new problem type
title: "[Model] AlternatingMaximumWeightedMatching"
labels: model
assignees: ''
---

## Motivation

ALTERNATING MAXIMUM WEIGHTED MATCHING (P245) from Garey & Johnson, A8 GP8. A PSPACE-complete two-player game problem where players alternately select edges of a weighted graph subject to matching constraints. Despite maximum weighted matching being solvable in polynomial time, the alternating (game) version is PSPACE-complete.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R189 (QBF to ALTERNATING MAXIMUM WEIGHTED MATCHING) -- transformation from QBF establishes PSPACE-completeness
- **As source:** None found in the current issue set

## Definition

**Name:** <!-- ⚠️ Unverified --> `AlternatingMaximumWeightedMatching`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Alternating Maximum Weighted Matching (also: Weighted Matching Game)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP8

**Mathematical definition:**

INSTANCE: Graph G = (V,E), a weight w(e) ∈ Z+ for each e ∈ E, and a bound B ∈ Z+.
QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a new edge from E, subject to the constraint that no edge can share an endpoint with any of the already chosen edges. If the sum of the weights of the edges chosen ever exceeds B, player 1 wins.

The problem is a satisfaction (decision) problem: given the game setup, determine whether player 1 has a forced winning strategy.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |E| (one binary variable per edge)
- **Per-variable domain:** binary {0, 1} -- whether edge e ∈ E is chosen during the game
- **Meaning:** A configuration (x_0, ..., x_{|E|-1}) represents a subset of edges that could be selected during the game. The configuration must form a valid matching (no two selected edges share an endpoint). The ordering of selections matters for the game semantics, but the final set of chosen edges determines the outcome. The game tree explores all possible alternating choices.

**Note:** The game-tree nature means that the "variables" are better understood as a sequence of moves rather than a static assignment. The brute-force evaluation explores the full game tree.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `AlternatingMaximumWeightedMatching<G, W>`
**Variants:** Parameterized by graph type G (SimpleGraph) and weight type W (i32)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `G` | The underlying graph G = (V, E) |
| `weights` | `Vec<W>` | Edge weights w(e) ∈ Z+ for each edge, indexed by edge index |
| `bound` | `W` | The threshold B: player 1 wins if total weight exceeds B |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** PSPACE-complete (Dobkin and Ladner, 1978; transformation from QBF).
- **Best known exact algorithm:** Full game-tree search (minimax). The game tree has depth at most |E| (each move selects one edge), and branching factor at most |E|. Worst case: O(|E|!) time. With alpha-beta pruning, practical performance can be significantly better but worst case remains exponential.
- **Polynomial-time solvable special case:** The non-game version (maximum weighted matching without alternation) is solvable in polynomial time, e.g., O(|V|^3) via the Blossom algorithm (Edmonds, 1965; Lawler, 1976).
- **Space complexity:** PSPACE-complete means it can be solved in polynomial space using alternating computation. A minimax game-tree search uses O(|E|) space (depth of recursion).
- **References:**
  - [Dobkin and Ladner, 1978] D. Dobkin and R. E. Ladner (1978). "Private communication". PSPACE-completeness via QBF reduction.
  - [Lawler, 1976a] E. L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Polynomial-time maximum weighted matching.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a game-theoretic extension of:** Maximum Weighted Matching (the corresponding optimization problem is polynomial-time solvable)
- **Known special cases:** For trees or paths, the game may be solvable more efficiently, but general graphs yield PSPACE-completeness
- **Related problems:** Node Kayles, Generalized Geography, and other PSPACE-complete two-player games from GJ Appendix A8

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), a weight w(e) ∈ Z+ for each e ∈ E, and a bound B ∈ Z+.
QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a new edge from E, subject to the constraint that no edge can share an endpoint with any of the already chosen edges. If the sum of the weights of the edges chosen ever exceeds B, player 1 wins.

Reference: [Dobkin and Ladner, 1978]. Transformation from QBF.
Comment: PSPACE-complete, even though the corresponding weighted matching problem can be solved in polynomial time (e.g., see [Lawler, 1976a]).

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all possible game trees by exhaustive minimax search: at each position, try all legal edge selections, alternate between players, and determine if player 1 has a forced win. The configuration space is the set of all possible edge subsets forming matchings, but game-tree search prunes infeasible branches.
- [ ] It can be solved by reducing to integer programming. Not directly applicable due to game-tree structure (alternating quantifiers).
- [ ] Other: Alpha-beta pruning can improve practical performance of the minimax search. For small graphs, the game tree is manageable.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Graph G with 8 vertices {0, 1, 2, 3, 4, 5, 6, 7} and 10 edges with weights:
- e0 = {0, 1}, w = 3
- e1 = {0, 2}, w = 5
- e2 = {1, 3}, w = 4
- e3 = {2, 3}, w = 2
- e4 = {2, 4}, w = 6
- e5 = {3, 5}, w = 1
- e6 = {4, 5}, w = 7
- e7 = {4, 6}, w = 3
- e8 = {5, 7}, w = 4
- e9 = {6, 7}, w = 5

Bound B = 12

**Game analysis:**
Players alternate choosing edges that form a valid matching (no shared endpoints).

Player 1 strategy: choose e4 = {2,4} (weight 6).
- Now vertices 2 and 4 are saturated. Remaining available edges: e0={0,1}(3), e2={1,3}(4), e5={3,5}(1), e8={5,7}(4), e9={6,7}(5).
Player 2 must choose an edge. Suppose player 2 picks e5 = {3,5} (weight 1, total = 7).
- Now vertices 2,3,4,5 are saturated. Remaining: e0={0,1}(3), e9={6,7}(5).
Player 1 picks e9 = {6,7} (weight 5, total = 12). Total weight = 6 + 1 + 5 = 12, which equals but does not exceed B = 12. Player 1 does NOT win yet.
Player 2 picks e0 = {0,1} (weight 3, total = 15 > 12). Now total exceeds B, so player 1 wins!

But player 2 would avoid moves that cause player 1 to win. The full game tree must be searched to determine the true outcome. With B = 12, player 1 can force a win by choosing high-weight edges early, since the maximum matching weight (e.g., {e1, e2, e6, e9} = 5+4+7+5 = 21) far exceeds B.

Answer: YES, player 1 has a forced win with B = 12 on this graph.

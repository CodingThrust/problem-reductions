---
name: Problem
about: Propose a new problem type
title: "[Model] Annihilation"
labels: model
assignees: ''
---

## Motivation

ANNIHILATION (P246) from Garey & Johnson, A8 GP9. A combinatorial game on directed acyclic graphs where players move tokens along arcs and tokens are annihilated upon collision. The problem is NP-hard (and in PSPACE), connecting classical graph covering problems to game-theoretic complexity.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R190 (VERTEX COVER to ANNIHILATION) -- transformation from Vertex Cover establishes NP-hardness
- **As source:** None found in the current issue set

## Definition

**Name:** <!-- ⚠️ Unverified --> `Annihilation`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Annihilation Game (also: Token Annihilation Game on DAGs)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP9

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A), collection {Ai: 1 ≤ i ≤ r} of (not necessarily disjoint) subsets of A, function f0 mapping V into {0,1,2,...,r}, where f0(v) = i > 0 means that a "token" of type i is "on" vertex v and f0(v) = 0 means that v is unoccupied.
QUESTION: Does player 1 have a forced win in the following game played on G? A position is a function f: V → {0,1,...,r} with f0 being the initial position and players alternating moves. A player moves by selecting a vertex v ∈ V with f(v) > 0 and an arc (v,w) ∈ Af(v), and the move corresponds to moving the token on vertex v to vertex w. The new position f' is the same as f except that f'(v) = 0 and f'(w) is either 0 or f(v), depending, respectively, on whether f(w) > 0 or f(w) = 0. (If f(w) > 0, then both the token moved to w and the token already there are "annihilated.") Player 1 wins if and only if player 2 is the first player unable to move.

The problem is a satisfaction (decision) problem: determine whether player 1 has a forced winning strategy.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |V| (one variable per vertex, encoding the token state at each vertex)
- **Per-variable domain:** {0, 1, ..., r} -- the token type currently on the vertex (0 = unoccupied)
- **Meaning:** A game position f: V → {0, 1, ..., r} describes which vertices hold which token types. The initial position f_0 is given, and the game evolves through player moves. The brute-force approach explores the full game tree of positions reachable from f_0.

**Note:** The game-tree structure means the problem involves alternating choices rather than a single static assignment. The game state space has at most (r+1)^|V| positions.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `Annihilation`
**Variants:** None (the graph is directed and unweighted; token types and arc subsets are stored directly)

| Field | Type | Description |
|-------|------|-------------|
| `vertices` | `usize` | Number of vertices |V| |
| `arcs` | `Vec<(usize, usize)>` | Directed arcs A of the DAG, each (u, v) |
| `arc_subsets` | `Vec<Vec<usize>>` | Collection {A_1, ..., A_r}: arc_subsets[i] lists arc indices in A_{i+1} |
| `initial_position` | `Vec<usize>` | f_0: initial_position[v] = token type on vertex v (0 = empty) |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-hard and in PSPACE (Fraenkel and Yesha, 1977). Not known to be PSPACE-complete. Remains NP-hard even if r = 2 and A_1 ∩ A_2 = ∅.
- **Best known exact algorithm:** Game-tree search (minimax). For r = 1 (single token type, impartial game), the game is solvable in polynomial time O(n^6) using Sprague-Grundy theory, where n = |V| (Fraenkel and Yesha, 1976/1982). For r ≥ 2, the problem is NP-hard and requires exponential-time game-tree exploration in the worst case.
- **Space complexity:** In PSPACE; the game tree can be explored with O(|V| * log(r+1)) space per recursion level and game depth bounded by the number of tokens.
- **Later results:** Fraenkel and Goldschmidt (1987) showed the general problem (on cyclic graphs) is PSPACE-hard. For acyclic graphs, the problem was shown PSPACE-complete by Fraenkel and Goldschmidt (1987).
- **References:**
  - [Fraenkel and Yesha, 1976] A. S. Fraenkel and Y. Yesha (1976). "Theory of annihilation games". *Bulletin of the American Mathematical Society* 82, pp. 775-777. Polynomial-time algorithm for r = 1.
  - [Fraenkel and Yesha, 1977] A. S. Fraenkel and Y. Yesha (1977). "Complexity of problems in games, graphs, and algebraic equations". NP-hardness via vertex cover reduction.
  - [Fraenkel and Yesha, 1982] A. S. Fraenkel and Y. Yesha (1982). "Theory of annihilation games--I". *Journal of Combinatorial Theory, Series B* 33, pp. 60-86. O(n^6) algorithm for r = 1.
  - [Fraenkel and Goldschmidt, 1987] A. S. Fraenkel and O. Goldschmidt (1987). PSPACE-hardness for general graphs, PSPACE-completeness for acyclic graphs.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **Special case r = 1:** Solvable in polynomial time via Sprague-Grundy theory (impartial game on DAG)
- **Special case r = 2, A_1 ∩ A_2 = ∅:** Still NP-hard (Fraenkel and Yesha, 1977)
- **Related games:** REMOVE (moved token is removed instead of annihilated), CONTRAJUNCTIVE, CAPTURE, BLOCKING, TARGET -- all shown NP-hard by Fraenkel and Yesha (1977)
- **Generalization:** On cyclic graphs (not DAGs), the problem becomes PSPACE-hard

## Extra Remark

**Full book text:**

INSTANCE: Directed acyclic graph G = (V,A), collection {Ai: 1 ≤ i ≤ r} of (not necessarily disjoint) subsets of A, function f0 mapping V into {0,1,2,...,r}, where f0(v) = i > 0 means that a "token" of type i is "on" vertex v and f0(v) = 0 means that v is unoccupied.
QUESTION: Does player 1 have a forced win in the following game played on G? A position is a function f: V → {0,1,...,r} with f0 being the initial position and players alternating moves. A player moves by selecting a vertex v ∈ V with f(v) > 0 and an arc (v,w) ∈ Af(v), and the move corresponds to moving the token on vertex v to vertex w. The new position f' is the same as f except that f'(v) = 0 and f'(w) is either 0 or f(v), depending, respectively, on whether f(w) > 0 or f(w) = 0. (If f(w) > 0, then both the token moved to w and the token already there are "annihilated.") Player 1 wins if and only if player 2 is the first player unable to move.

Reference: [Fraenkel and Yesha, 1977]. Transformation from VERTEX COVER.
Comment: NP-hard and in PSPACE, but not known to be PSPACE-complete. Remains NP-hard even if r = 2 and A1 ∩ A2 is empty. Problem can be solved in polynomial time if r = 1 [Fraenkel and Yesha, 1976]. Related NP-hardness results for other token-moving games on directed graphs (REMOVE, CONTRAJUNCTIVE, CAPTURE, BLOCKING, TARGET) can be found in [Fraenkel and Yesha, 1977].

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all game positions reachable from f_0 via game-tree search (minimax). At each position, try all legal moves (select a token, move it along a valid arc, handle annihilation), alternate between players, and determine if player 1 has a forced win. For r = 1, Sprague-Grundy values can be computed in polynomial time.
- [ ] It can be solved by reducing to integer programming. Not directly applicable due to game-tree structure (alternating quantifiers).
- [ ] Other: For r = 1 (impartial game), use Sprague-Grundy theory for polynomial-time solution. For r >= 2, no polynomial-time algorithm is known unless P = NP.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

Directed acyclic graph G with 8 vertices {0, 1, 2, 3, 4, 5, 6, 7} and 10 arcs:
- a0 = (0, 2), a1 = (0, 3), a2 = (1, 2), a3 = (1, 4)
- a4 = (2, 5), a5 = (3, 5), a6 = (3, 6), a7 = (4, 6)
- a8 = (5, 7), a9 = (6, 7)

Token types: r = 2
- A_1 (arcs for type-1 tokens): {a0, a1, a4, a5, a8} = {(0,2), (0,3), (2,5), (3,5), (5,7)}
- A_2 (arcs for type-2 tokens): {a2, a3, a6, a7, a9} = {(1,2), (1,4), (3,6), (4,6), (6,7)}
- A_1 ∩ A_2 = ∅

Initial position f_0:
- f_0(0) = 1 (type-1 token on vertex 0)
- f_0(1) = 2 (type-2 token on vertex 1)
- f_0(3) = 1 (type-1 token on vertex 3)
- f_0(4) = 2 (type-2 token on vertex 4)
- f_0(2) = f_0(5) = f_0(6) = f_0(7) = 0 (unoccupied)

Total tokens: 4 (two of each type).

**Game play analysis:**
Player 1 moves first. Available moves: move any token.
- Move type-1 token from 0 → 2 (via a0): f(0)=0, f(2)=1 (vertex 2 was empty)
- Move type-1 token from 0 → 3 (via a1): f(0)=0, but f(3)=1 already -- cannot, vertex 3 has a type-1 token, and moving type-1 to type-1 causes annihilation: f(3)=0
- Move type-1 token from 3 → 5 (via a5): f(3)=0, f(5)=1

Player 1 strategy: move token from 0 → 2 (f becomes: 0:0, 1:2, 2:1, 3:1, 4:2, rest 0).
Player 2 moves: move type-2 from 1 → 2 (via a2): vertex 2 has type-1, annihilation! f(1)=0, f(2)=0.
Now: 3:1, 4:2, rest 0.
Player 1: move type-1 from 3 → 5 (via a5): f(3)=0, f(5)=1.
Player 2: move type-2 from 4 → 6 (via a7): f(4)=0, f(6)=2.
Player 1: move type-1 from 5 → 7 (via a8): f(5)=0, f(7)=1.
Player 2: move type-2 from 6 → 7 (via a9): vertex 7 has type-1, annihilation! f(6)=0, f(7)=0.
All tokens annihilated. Player 1's turn: no tokens to move. Player 1 cannot move, but the question is who is "first unable to move." After player 2's last move, it is player 1's turn and player 1 cannot move, so player 2 wins.

With different strategies, player 1 may or may not have a forced win. The full game tree must be explored. Answer depends on complete minimax analysis.

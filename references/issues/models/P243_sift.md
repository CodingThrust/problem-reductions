---
name: Problem
about: Propose a new problem type
title: "[Model] Sift"
labels: model
assignees: ''
---

## Motivation

SIFT (P243) from Garey & Johnson, A8 GP6. A PSPACE-complete two-player combinatorial game played on two collections of subsets, where players alternately choose elements and the game ends when one collection is fully "hit." This problem captures the complexity of two-player set-intersection games and provides a natural combinatorial abstraction of QBF evaluation.

<!-- ⚠️ Unverified: AI-collected rule associations -->

**Associated reduction rules:**
- **As target:**
  - R187: QBF -> SIFT (Schaefer, 1978a)
- **As source:** (none known in GJ appendix)

## Definition

**Name:** <!-- ⚠️ Unverified --> `Sift`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Sift; also: Set Intersection Game, Two-Collection Hitting Game
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP6

**Mathematical definition:**

INSTANCE: Two collections A and B of subsets of a finite set X, with A and B having no subsets in common.
QUESTION: Does player 1 have a forced win in the following game played on A, B, and X? Players alternate choosing an element from X until the set X' of all elements chosen so far either intersects all the subsets in A or intersects all the subsets in B. Player 1 wins if and only if the final set X' of chosen elements intersects all the subsets in B and, if player 1 made the last move, does not intersect all subsets in A.

The problem asks whether player 1 has a winning strategy in this two-collection set-hitting game.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |X| binary variables (one per element of the ground set X)
- **Per-variable domain:** binary {0, 1} -- whether element x in X has been chosen (by either player)
- **Meaning:** Variable x_i = 1 if element x_i has been chosen. The game state is described by the set X' of chosen elements plus whose turn it is. The game terminates when X' intersects all subsets in A or all subsets in B. Player 1 wins if X' intersects all of B (and, if player 1 moved last, does not intersect all of A). The strategic question is whether player 1 can force a winning termination state.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `Sift`
**Variants:** none (no graph/weight parameterization)

| Field | Type | Description |
|-------|------|-------------|
| `elements` | `Vec<Element>` | Ground set X of elements |
| `collection_a` | `Vec<Vec<usize>>` | Collection A of subsets of X (indices into `elements`) |
| `collection_b` | `Vec<Vec<usize>>` | Collection B of subsets of X (indices into `elements`) |

**Invariant:** A and B must have no subsets in common (A intersection B = empty as set families).

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The answer is true if player 1 has a forced win, false otherwise.
- The game has a complex winning condition involving both collections A and B and the parity of the last move.
- Key getter methods needed: `num_elements()` (= |X|), `num_sets_a()` (= |A|), `num_sets_b()` (= |B|).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** PSPACE-complete (Schaefer, 1978a; transformation from QBF).
- **Best known exact algorithm:** Game-tree search (minimax). The game tree has depth at most |X| (each turn one element is chosen). At level d, the branching factor is |X| - d (remaining unchosen elements). The game terminates early when one of the collections is fully hit, so typical game trees are smaller than the worst case. With memoization of game states (the set X' of chosen elements), the state space is at most O(2^|X|). Total time: O(2^|X| * (|A| + |B|)) to evaluate termination conditions at each state.
- **Space complexity:** O(|X|) with depth-first search, O(2^|X|) with memoization.
- **References:**
  - T.J. Schaefer (1978). "Complexity of some two-person perfect-information games." *Journal of Computer and System Sciences* 16, pp. 185-225.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** General two-player combinatorial games
- **Known special cases:** When A = empty (no losing condition for player 1 on last move), the game simplifies to a pure hitting-set race. When all subsets in A and B have size 1, the game reduces to a simple element-claiming game.
- **Relationship to other problems:** The game generalizes Maker-Breaker games on hypergraphs. Player 1 acts as the "Maker" trying to hit all of B, while the presence of A introduces a "Breaker"-like constraint.

## Extra Remark

**Full book text:**

INSTANCE: Two collections A and B of subsets of a finite set X, with A and B having no subsets in common.
QUESTION: Does player 1 have a forced win in the following game played on A, B, and X? Players alternate choosing an element from X until the set X' of all elements chosen so far either intersects all the subsets in A or intersects all the subsets in B. Player 1 wins if and only if the final set X' of chosen elements intersects all the subsets in B and, if player 1 made the last move, does not intersect all subsets in A.

Reference: [Schaefer, 1978a]. Transformation from QBF.
Comment: PSPACE-complete.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate the game tree using minimax: at each state, the active player picks an unchosen element, update X', check termination conditions (X' hits all of A or all of B), evaluate the winning condition. Total time O(2^|X| * (|A| + |B|)).
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Reduce to QBF and use a QBF solver; game-tree search with alpha-beta pruning.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Sift instance with 8 elements, 3 subsets in A, and 4 subsets in B:**

Ground set: X = {x_1, x_2, x_3, x_4, x_5, x_6, x_7, x_8}

Collection A (3 subsets):
- A_1 = {x_1, x_2}
- A_2 = {x_3, x_4}
- A_3 = {x_5, x_6}

Collection B (4 subsets):
- B_1 = {x_1, x_3, x_5}
- B_2 = {x_2, x_4, x_7}
- B_3 = {x_3, x_6, x_8}
- B_4 = {x_4, x_5, x_7}

Verification: A and B have no subsets in common (A_1 = {x_1, x_2}, A_2 = {x_3, x_4}, A_3 = {x_5, x_6} are all distinct from B_1 through B_4).

**Game play (one scenario):**

Turn 1 (P1): Picks x_3. X' = {x_3}.
  - Hits: A_2 (via x_3), B_1 (via x_3), B_3 (via x_3). Not all of A or B hit. Continue.

Turn 2 (P2): Picks x_1. X' = {x_3, x_1}.
  - Hits: A_1 (via x_1), A_2 (via x_3), B_1 (via x_1 and x_3). Not all of A hit (A_3 needs x_5 or x_6). Continue.

Turn 3 (P1): Picks x_7. X' = {x_3, x_1, x_7}.
  - Hits: B_2 (via x_7), B_4 (via x_7). Now B_1, B_2, B_3 (x_3), B_4 all hit? B_3 needs x_6 or x_8 too -- x_3 already hits B_3. Actually B_3 = {x_3, x_6, x_8}: x_3 in X', so B_3 is hit. B_4 = {x_4, x_5, x_7}: x_7 in X', hit. B_2 = {x_2, x_4, x_7}: x_7, hit.
  - All of B hit! Check: Player 1 made last move. Does X' intersect all of A? A_1 hit (x_1), A_2 hit (x_3), A_3 = {x_5, x_6}: neither in X'. A_3 NOT hit.
  - X' intersects all of B AND does not intersect all of A. Player 1 WINS!

**Verification:**
- Player 1's strategy: pick x_3 first (hitting B_1, B_3, A_2), then pick x_7 (hitting B_2, B_4) on turn 3. After 3 turns with any P2 choice, all of B is hit while A_3 remains unhit.
- Player 2 cannot prevent this because x_3 alone hits B_1 and B_3, and x_7 alone hits B_2 and B_4, and neither x_3 nor x_7 is in A_3.

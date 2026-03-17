---
name: Problem
about: Propose a new problem type
title: "[Model] AlternatingHittingSet"
labels: model
assignees: ''
---

## Motivation

ALTERNATING HITTING SET (P244) from Garey & Johnson, A8 GP7. A PSPACE-complete two-player combinatorial game where players alternately choose elements from a ground set to cover subsets, and the player whose choice completes coverage of all subsets loses. This is a "last player loses" game on set systems, notable for being PSPACE-complete even when all subsets have at most 2 elements -- a restriction where the non-game version (ordinary Hitting Set) is polynomial-time solvable.

<!-- ⚠️ Unverified: AI-collected rule associations -->

**Associated reduction rules:**
- **As target:**
  - R188: QBF -> ALTERNATING HITTING SET (Schaefer, 1978a)
- **As source:** (none known in GJ appendix)

## Definition

**Name:** <!-- ⚠️ Unverified --> `AlternatingHittingSet`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Alternating Hitting Set; also: Competitive Hitting Set Game, Set Cover Avoidance Game
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP7

**Mathematical definition:**

INSTANCE: A collection C of subsets of a basic set B.
QUESTION: Does player 1 have a forced win in the following game played on C and B? Players alternate choosing a new element of B until, for each c in C, some member of c has been chosen. The player whose choice causes this to happen loses.

The problem asks whether player 1 has a winning strategy in this "last-to-complete-loses" game.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |B| binary variables (one per element of the basic set B)
- **Per-variable domain:** binary {0, 1} -- whether element b in B has been chosen
- **Meaning:** Variable x_b = 1 if element b has been chosen (by either player). The game state is described by the set of chosen elements. The game terminates when every subset c in C has at least one chosen member. The player who makes the move that causes this full-coverage condition to hold LOSES. Player 1 wins if they can force player 2 into making the completing move.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `AlternatingHittingSet`
**Variants:** none (no graph/weight parameterization)

| Field | Type | Description |
|-------|------|-------------|
| `elements` | `Vec<Element>` | Basic set B of elements |
| `subsets` | `Vec<Vec<usize>>` | Collection C of subsets of B (indices into `elements`) |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The answer is true if player 1 has a forced win (can force player 2 to make the completing move), false otherwise.
- The "last player loses" rule is the defining feature -- this inverts the typical game-playing objective.
- Key getter methods needed: `num_elements()` (= |B|), `num_sets()` (= |C|), `max_set_size()` (maximum |c| for c in C).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** PSPACE-complete (Schaefer, 1978a; transformation from QBF).
- **Restricted cases:**
  - PSPACE-complete even if no set in C contains more than 2 elements. This is remarkable because the non-game Hitting Set problem with sets of size at most 2 reduces to 2-SAT and is solvable in polynomial time.
  - If the roles of winner and loser are reversed (player whose move completes coverage WINS), the problem is PSPACE-complete even if no set has more than 3 elements.
- **Best known exact algorithm:** Game-tree search (minimax with "last-to-cover loses" evaluation). The game tree has depth at most |B| (each turn one element is chosen from the remaining unchosen elements). The branching factor is |B| - d at depth d. With memoization of game states (the set of chosen elements), the state space is at most O(2^|B|). At each state, checking the termination condition (all subsets hit) takes O(|C| * max-set-size). Total time: O(2^|B| * |C|).
- **Space complexity:** O(|B|) with depth-first search, O(2^|B|) with memoization.
- **References:**
  - T.J. Schaefer (1978). "Complexity of some two-person perfect-information games." *Journal of Computer and System Sciences* 16, pp. 185-225. PSPACE-completeness proof and size-restricted results.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** General two-player combinatorial games; related to Poset Games (PSPACE-complete)
- **Known special cases:** 2-Alternating Hitting Set (sets of size at most 2, still PSPACE-complete); 3-Alternating Hitting Set with reversed roles (PSPACE-complete)
- **Relationship to other problems:** The non-game version is the classical Hitting Set problem (NP-complete in general, polynomial for bounded set sizes). The game version adds the alternation and "last-player-loses" rule, elevating the complexity to PSPACE. Related to Maker-Breaker games and the Set Coincidence Game.

## Extra Remark

**Full book text:**

INSTANCE: A collection C of subsets of a basic set B.
QUESTION: Does player 1 have a forced win in the following game played on C and B? Players alternate choosing a new element of B until, for each c in C, some member of c has been chosen. The player whose choice causes this to happen loses.

Reference: [Schaefer, 1978a]. Transformation from QBF.
Comment: PSPACE-complete even if no set in C contains more than two elements, a subcase of the original HITTING SET problem that can be solved in polynomial time. If the roles of winner and loser are reversed, the problem is PSPACE-complete even if no set in C contains more than three elements.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate the game tree using minimax with the "last-to-cover loses" termination rule. At each state, the active player picks an unchosen element, updates the coverage, and checks if all subsets are now hit. If so, the active player loses. Otherwise, recurse. Total time O(2^|B| * |C|) with memoization.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Reduce to QBF and use a QBF solver; game-tree search with alpha-beta pruning.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Alternating Hitting Set instance with 8 elements and 6 subsets (all of size at most 2):**

Basic set: B = {b_1, b_2, b_3, b_4, b_5, b_6, b_7, b_8}

Subsets (6 subsets, each of size 2):
- c_1 = {b_1, b_2}
- c_2 = {b_3, b_4}
- c_3 = {b_5, b_6}
- c_4 = {b_1, b_4}
- c_5 = {b_2, b_5}
- c_6 = {b_3, b_6}

(Note: b_7 and b_8 are "safe" elements -- choosing them does not hit any unhit subset, acting as delay moves.)

**Game play analysis:**

The game ends when all 6 subsets have at least one member chosen. The player who makes the move that completes this coverage LOSES.

**Scenario 1:** Player 1's strategy -- delay completion.
- Turn 1 (P1): Picks b_7 (safe -- hits no subset). Coverage: none.
- Turn 2 (P2): Picks some element, say b_1. Hits c_1 (via b_1) and c_4 (via b_1). Coverage: {c_1, c_4}.
- Turn 3 (P1): Picks b_8 (safe). Coverage: {c_1, c_4}.
- Turn 4 (P2): Picks b_3. Hits c_2 (via b_3) and c_6 (via b_3). Coverage: {c_1, c_2, c_4, c_6}.
- Turn 5 (P1): Picks b_5. Hits c_3 (via b_5) and c_5 (via b_5). Coverage: {c_1, c_2, c_3, c_4, c_5, c_6} = ALL subsets!
  - Player 1 made the completing move, so Player 1 LOSES. Bad strategy!

**Scenario 2:** Player 1 tries a different approach.
- Turn 1 (P1): Picks b_1. Hits c_1 and c_4. Coverage: {c_1, c_4}.
- Turn 2 (P2): Picks b_7 (safe delay). Coverage: {c_1, c_4}.
- Turn 3 (P1): Picks b_3. Hits c_2 and c_6. Coverage: {c_1, c_2, c_4, c_6}.
- Turn 4 (P2): Picks b_8 (safe delay). Coverage: {c_1, c_2, c_4, c_6}.
- Turn 5 (P1): Picks b_2. Hits c_5 (b_2 in c_5). Coverage: {c_1, c_2, c_4, c_5, c_6}. c_3 = {b_5, b_6} not yet hit. Continue.
- Turn 6 (P2): Forced to pick from {b_4, b_5, b_6}. If P2 picks b_5: hits c_3. Coverage = all 6 subsets. P2 made the completing move -- P2 LOSES. Player 1 WINS!
  - But P2 can pick b_4 instead (c_2 already hit, so no new coverage). Coverage still {c_1, c_2, c_4, c_5, c_6}.
- Turn 7 (P1): Picks from {b_5, b_6}. If P1 picks b_5: hits c_3. All subsets hit. P1 LOSES.
  - If P1 picks b_6: hits c_3 (b_6 in c_3). All subsets hit. P1 LOSES.
  - Player 1 is forced to complete coverage and loses!

The full game tree must be evaluated by minimax to determine the winner. With 8 elements, the tree has at most 8! = 40320 leaves (reduced by early termination), and exhaustive search determines whether player 1 has a forced win.

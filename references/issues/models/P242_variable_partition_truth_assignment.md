---
name: Problem
about: Propose a new problem type
title: "[Model] VariablePartitionTruthAssignment"
labels: model
assignees: ''
---

## Motivation

VARIABLE PARTITION TRUTH ASSIGNMENT (P242) from Garey & Johnson, A8 GP5. A PSPACE-complete two-player game problem where players alternately choose variables from a set (in any order), and the truth assignment is determined by which player chose each variable. This problem generalizes Sequential Truth Assignment by allowing free variable selection order, and is notable for remaining PSPACE-complete even with positive-only clauses.

<!-- ⚠️ Unverified: AI-collected rule associations -->

**Associated reduction rules:**
- **As target:**
  - R186: QBF -> VARIABLE PARTITION TRUTH ASSIGNMENT (Schaefer, 1978a)
- **As source:** (none known in GJ appendix)

## Definition

**Name:** <!-- ⚠️ Unverified --> `VariablePartitionTruthAssignment`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Variable Partition Truth Assignment; also: Partitioned Variables QBF Game, Unordered Formula Game
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP5

**Mathematical definition:**

INSTANCE: A set U of variables and a collection C of clauses over U.
QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate choosing a variable from U until all variables have been chosen. Player 1 wins if and only if a satisfying truth assignment for C is obtained by setting "true" all variables chosen by player 1 and setting "false" all variables chosen by player 2.

The problem asks whether player 1 has a winning strategy in this variable-selection game.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |U| variables, each with a ternary state during the game: {unchosen, chosen-by-P1, chosen-by-P2}. As a final assignment, each variable is binary {true, false}.
- **Per-variable domain:** In the game formulation, each variable ultimately has domain {0, 1} -- true if chosen by player 1, false if chosen by player 2.
- **Meaning:** The configuration encodes a strategy tree: at each game state, the active player selects an unchosen variable, and the final partition determines the truth assignment. Player 1 wins if the resulting assignment satisfies all clauses.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `VariablePartitionTruthAssignment`
**Variants:** none (no graph/weight parameterization)

| Field | Type | Description |
|-------|------|-------------|
| `variables` | `Vec<Variable>` | Set of Boolean variables U = {u_1, ..., u_n} |
| `clauses` | `Vec<Vec<Literal>>` | Collection C of clauses, each a disjunction of literals |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The answer is true if player 1 has a forced win, false otherwise.
- Unlike Sequential Truth Assignment (GP4), players can choose ANY unchosen variable on their turn, not just the next in sequence.
- The truth value of a variable is determined by WHO chose it, not by what value they assign: player 1's choices become true, player 2's choices become false.
- Key getter methods needed: `num_vars()` (= n = |U|), `num_clauses()` (= |C|).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** PSPACE-complete (Schaefer, 1978a; transformation from QBF).
- **Restricted cases:**
  - PSPACE-complete even if each clause consists only of un-negated (positive) literals. This is a notable hardness result since positive SAT is trivially solvable.
  - Analogous results for several other games played on logical expressions (Schaefer, 1978a).
- **Best known exact algorithm:** Game-tree search over all possible variable-selection orderings. The game tree has depth n (one variable chosen per turn). At each node, the active player can choose from any remaining unchosen variable, giving branching factor up to n, n-1, ..., 1 at successive levels. The total number of game positions is at most n! (all permutations of variable choices), but many are equivalent. With memoization of game states (sets of chosen variables + whose turn), the state space is O(2^n) distinct game positions, each requiring O(m) work to evaluate clause satisfaction. Total time: O(2^n * m).
- **Space complexity:** O(2^n) with memoization, or O(n) with depth-first search (but exponential time).
- **References:**
  - T.J. Schaefer (1978). "Complexity of some two-person perfect-information games." *Journal of Computer and System Sciences* 16, pp. 185-225. PSPACE-completeness proof and positive-clause restriction result.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** QBF (generalized game-theoretic formulation)
- **Known special cases:** Positive Variable Partition Truth Assignment (clauses with un-negated literals only, still PSPACE-complete)
- **Relationship to other games:** Generalizes Sequential Truth Assignment (GP4) by removing the fixed variable ordering. This is Schaefer's "unordered formula game" or "Partitioned Variables" game variant.

## Extra Remark

**Full book text:**

INSTANCE: A set U of variables and a collection C of clauses over U.
QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate choosing a variable from U until all variables have been chosen. Player 1 wins if and only if a satisfying truth assignment for C is obtained by setting "true" all variables chosen by player 1 and setting "false" all variables chosen by player 2.

Reference: [Schaefer, 1978a]. Transformation from QBF.
Comment: PSPACE-complete, even if each clause consists only of un-negated literals (i.e., contains no literals of the form u-bar for u in U). Analogous results for several other games played on logical expressions can be found in the reference.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all possible game plays (permutations of variable selections) using minimax game-tree search. At each node, evaluate whether the active player can force a win. With memoization, O(2^n * m) time.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Reduce to QBF and use a QBF solver; game-tree search with alpha-beta pruning and symmetry-breaking.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Variable Partition Truth Assignment instance with 6 variables and 6 clauses:**

Variables: U = {u_1, u_2, u_3, u_4, u_5, u_6}
- Player 1 goes first, then players alternate choosing variables.
- Variables chosen by player 1 are set to TRUE; variables chosen by player 2 are set to FALSE.
- Each player chooses 3 variables total (n/2 = 3 each).

Clauses (positive-only, 6 clauses):
- c_1 = (u_1 or u_2 or u_3)
- c_2 = (u_2 or u_4 or u_5)
- c_3 = (u_3 or u_5 or u_6)
- c_4 = (u_1 or u_4 or u_6)
- c_5 = (u_1 or u_5)
- c_6 = (u_2 or u_6)

**Analysis:**
Since clauses are positive-only, a clause is satisfied iff at least one variable in the clause is chosen by player 1 (set to true). Player 1 wins if for every clause, at least one variable in that clause was chosen by player 1.

Player 1 must pick 3 out of 6 variables such that every clause is "hit" by at least one of player 1's choices -- regardless of which 3 variables player 2 chooses (equivalently, regardless of which 3 variables player 1 does NOT get).

**Player 1's strategy:**
- Player 1 first picks u_1. This hits c_1, c_4, c_5.
- Player 2 picks some variable, say u_2. This "removes" u_2 from player 1's potential hits.
- Player 1 picks u_5. This hits c_2, c_3, c_5 (c_5 already hit).
- Player 2 picks some variable, say u_6. Removes u_6.
- Player 1 picks u_3. This hits c_1 (already), c_3 (already).
  - Remaining: c_6 = (u_2 or u_6). u_2 chosen by P2 (false), u_6 chosen by P2 (false). c_6 = F.
  - Player 1 loses this line!

- Player 1 adjusts: picks u_1 first, then u_2 (after P2's move), then u_5 or u_6.
  - If P2 picks u_5 after u_1: Player 1 picks u_2. P2 picks u_6. Player 1 picks u_3.
    - c_6 = (u_2 or u_6) = T or F = T. c_2 = (u_2 or u_4 or u_5): u_2 = T. c_3 = (u_3 or u_5 or u_6): u_3 = T. All satisfied!

The full game tree (with 6! / (3!*3!) = 20 distinct partitions, but ~720 play orderings) must be evaluated by minimax to determine if player 1 has a forced win regardless of player 2's choices.

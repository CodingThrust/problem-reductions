---
name: Problem
about: Propose a new problem type
title: "[Model] SequentialTruthAssignment"
labels: model
assignees: ''
---

## Motivation

SEQUENTIAL TRUTH ASSIGNMENT (P241) from Garey & Johnson, A8 GP4. A PSPACE-complete two-player game problem where players alternately assign truth values to a fixed sequence of Boolean variables, and player 1 wins if the resulting assignment satisfies all clauses. This problem is the game-theoretic analogue of QBF with fixed variable ordering, and is a foundational problem for proving PSPACE-completeness of combinatorial games.

<!-- ⚠️ Unverified: AI-collected rule associations -->

**Associated reduction rules:**
- **As target:**
  - R185: QBF -> SEQUENTIAL TRUTH ASSIGNMENT (Stockmeyer and Meyer, 1973)
- **As source:** (none known in GJ appendix)

## Definition

**Name:** <!-- ⚠️ Unverified --> `SequentialTruthAssignment`
**Canonical name:** <!-- ⚠️ Unverified: web search --> Sequential Truth Assignment; also: Formula Game, Ordered QBF Game
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP4

**Mathematical definition:**

INSTANCE: A sequence U = <u_1, u_2, ..., u_n> of variables and a collection C of clauses over U (as in an instance of SATISFIABILITY).
QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate assigning truth values to the variables in U, with player 1 assigning a value to u_{2i-1} and player 2 assigning a value to u_{2i} on their i-th turns. Player 1 wins if and only if the resulting truth assignment satisfies all clauses in C.

The problem asks whether player 1 (the existential player) has a winning strategy in this two-player game.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |U| binary variables (one per variable in the sequence)
- **Per-variable domain:** binary {0, 1} -- the truth value assigned to variable u_i (0 = false, 1 = true)
- **Meaning:** Variable x_i = 1 if u_i is assigned true. The configuration (x_1, ..., x_n) encodes a complete truth assignment. However, unlike SAT, the problem is not about finding a single satisfying assignment but about whether player 1 has a strategy that guarantees satisfaction for all possible moves by player 2. The "configuration" in the game-theoretic sense is a strategy tree, not a single assignment.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SequentialTruthAssignment`
**Variants:** none (no graph/weight parameterization)

| Field | Type | Description |
|-------|------|-------------|
| `variables` | `Vec<Variable>` | Ordered sequence of Boolean variables U = <u_1, ..., u_n> |
| `clauses` | `Vec<Vec<Literal>>` | Collection C of clauses, each clause is a disjunction of literals |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The answer is true if player 1 has a forced win, false otherwise.
- Player 1 controls odd-indexed variables (u_1, u_3, u_5, ...), player 2 controls even-indexed variables (u_2, u_4, u_6, ...).
- Key getter methods needed: `num_vars()` (= n = |U|), `num_clauses()` (= |C|).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** PSPACE-complete (Stockmeyer and Meyer, 1973; transformation from QBF).
- **Restricted cases:**
  - PSPACE-complete even if each clause has at most 3 literals (analogous to 3-SAT restriction).
  - Solvable in polynomial time if no clause has more than 2 literals (Schaefer, 1978b), analogous to 2-SAT being in P.
- **Best known exact algorithm:** The game can be solved by minimax game-tree search. The game tree has depth n (one level per variable) and branching factor 2 at each level, giving a game tree of size O(2^n). With alpha-beta pruning, the best-case time is O(2^(n/2)), but the worst-case remains O(2^n). Since the problem is PSPACE-complete, no polynomial-time algorithm is expected unless P = PSPACE.
- **Space complexity:** Polynomial space suffices (depth-first game-tree search uses O(n) space), consistent with membership in PSPACE.
- **References:**
  - L.J. Stockmeyer, A.R. Meyer (1973). "Word problems requiring exponential time." *Proc. 5th Ann. ACM Symp. on Theory of Computing*, pp. 1-9. Original PSPACE-completeness proof.
  - T.J. Schaefer (1978). "The complexity of satisfiability problems." *Proc. 10th Annual ACM STOC*, pp. 216-226. Polynomial-time solvability for 2-literal restriction.

## Specialization

<!-- ⚠️ Unverified: AI-identified relationship -->

- **This is a special case of:** QBF (with alternating quantifiers in a fixed order matching the variable sequence)
- **Known special cases:** 3-Sequential Truth Assignment (clauses limited to 3 literals, still PSPACE-complete); 2-Sequential Truth Assignment (clauses limited to 2 literals, polynomial-time solvable)
- **Relationship to other games:** This is Schaefer's "ordered formula game" (G_omega). The Variable Partition Truth Assignment (GP5) generalizes this by allowing free choice of variable order.

## Extra Remark

**Full book text:**

INSTANCE: A sequence U = <u1,u2,...,un> of variables and a collection C of clauses over U (as in an instance of SATISFIABILITY).
QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate assigning truth values to the variables in U, with player 1 assigning a value to u2i-1 and player 2 assigning a value to u2i on their ith turns. Player 1 wins if and only if the resulting truth assignment satisfies all clauses in C.

Reference: [Stockmeyer and Meyer, 1973]. Transformation from QBF.
Comment: PSPACE-complete, even if each clause in C has only three literals. Solvable in polynomial time if no clause has more than two literals [Schaefer, 1978b].

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate the full game tree (depth n, branching factor 2) using minimax: at each odd level player 1 chooses the value maximizing satisfiability, at each even level player 2 minimizes. Total time O(2^n).
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Alpha-beta pruning on the game tree (best case O(2^(n/2))); reduction to QBF and using a QBF solver.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Sequential Truth Assignment instance with 6 variables and 7 clauses:**

Variable sequence: U = <u_1, u_2, u_3, u_4, u_5, u_6>
- Player 1 controls: u_1, u_3, u_5 (odd-indexed)
- Player 2 controls: u_2, u_4, u_6 (even-indexed)

Clauses (7 clauses):
- c_1 = (u_1 or u_2 or u_3)
- c_2 = (not u_1 or u_4 or u_5)
- c_3 = (u_2 or not u_3 or u_6)
- c_4 = (not u_2 or u_3 or not u_6)
- c_5 = (u_1 or not u_4 or u_6)
- c_6 = (not u_3 or u_5 or not u_6)
- c_7 = (u_3 or u_4 or not u_5)

**Game tree analysis (partial):**
- Turn 1: Player 1 assigns u_1. Suppose player 1 picks u_1 = T.
- Turn 1: Player 2 assigns u_2. Suppose player 2 picks u_2 = F (adversarial).
- Turn 2: Player 1 assigns u_3. Player 1 picks u_3 = T (to satisfy c_1 = T or F or T = T, and help c_4 and c_7).
- Turn 2: Player 2 assigns u_4. Player 2 picks u_4 = F (adversarial).
- Turn 3: Player 1 assigns u_5. Player 1 picks u_5 = T (to satisfy c_2 = F or F or T = T, and c_6).
- Turn 3: Player 2 assigns u_6. Player 2 picks u_6 = T (to try to falsify c_4 or c_6).
- Final assignment: (T, F, T, F, T, T)
  - c_1 = T or F or T = T
  - c_2 = F or F or T = T
  - c_3 = F or F or T = T
  - c_4 = T or T or F = T
  - c_5 = T or T or T = T
  - c_6 = F or T or F = T
  - c_7 = T or F or F = T
  - All satisfied! Player 1 wins with this line of play.

The full game tree has 2^6 = 64 leaves. Minimax evaluation over all branches determines whether player 1 has a forced win regardless of player 2's choices.

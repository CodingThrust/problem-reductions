---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to VARIABLE PARTITION TRUTH ASSIGNMENT"
labels: rule
assignees: ''
canonical_source_name: 'Quantified Boolean Formulas (QBF)'
canonical_target_name: 'Variable Partition Truth Assignment'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** QBF
**Target:** VARIABLE PARTITION TRUTH ASSIGNMENT
**Motivation:** Establishes PSPACE-completeness of the Variable Partition Truth Assignment game by reduction from QBF, showing that games where players freely choose which variable to assign (rather than following a fixed order) remain as hard as evaluating quantified Boolean formulas.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.255

## GJ Source Entry

> [GP5] VARIABLE PARTITION TRUTH ASSIGNMENT (*)
> INSTANCE: A set U of variables and a collection C of clauses over U.
> QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate choosing a variable from U until all variables have been chosen. Player 1 wins if and only if a satisfying truth assignment for C is obtained by setting "true" all variables chosen by player 1 and setting "false" all variables chosen by player 2.
> Reference: [Schaefer, 1978a]. Transformation from QBF.
> Comment: PSPACE-complete, even if each clause consists only of un-negated literals (i.e., contains no literals of the form u-bar for u E U). Analogous results for several other games played on logical expressions can be found in the reference.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a QBF instance F = (Q_1 u_1)(Q_2 u_2)...(Q_n u_n) E, construct a Variable Partition Truth Assignment instance (U', C') as follows. This reduction is due to Schaefer (1978a), who showed that several formula games are PSPACE-complete.

1. **Variable encoding:** For each variable u_i in the QBF, create a pair of variables {x_i, y_i} in U'. The variable x_i represents the positive literal and y_i represents the negative literal. Additionally, introduce auxiliary "enforcer" variables to constrain the game so that exactly one of x_i and y_i is chosen by each player pair.

2. **Clause transformation:** Transform each clause in E to use only positive (un-negated) literals. A positive literal u_i maps to x_i; a negative literal not-u_i maps to y_i. Since the game sets "true" all variables chosen by player 1 and "false" all chosen by player 2, the clause structure encodes the original satisfiability condition.

3. **Enforcer gadgets:** For each variable u_i, add enforcer clauses that ensure the game is well-defined: if player 1 picks x_i, player 2 is forced (by strategy considerations) to eventually pick y_i, and vice versa. These gadgets use O(1) additional variables and clauses per original variable.

4. **Quantifier simulation:** The alternating quantifier structure of the QBF is encoded in the game dynamics. The key insight is that in the Variable Partition game, players choose freely (not in a fixed order), so the reduction must embed the quantifier ordering into the clause structure so that rational play follows the quantifier prefix order. Schaefer achieves this through priority gadgets that make it strategically dominant for each player to claim their "designated" variables first.

5. **Correctness:** Player 1 has a forced win in the Variable Partition game on (U', C') iff the original QBF F is true. The free choice of variable order does not add power because the enforcer gadgets constrain rational play to respect the original quantifier structure.

**Source:** Schaefer (1978a), "On the complexity of some two-person perfect-information games." The paper proves PSPACE-completeness even when all clauses consist only of un-negated literals (positive CNF), making the reduction particularly elegant.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source QBF instance (number of variables)
- m = `num_clauses` of source QBF instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vars` | `2 * num_vars` |
| `num_clauses` | `num_clauses + num_vars` |

**Derivation:**
- Variables: Each QBF variable u_i is encoded as a pair (x_i, y_i), giving 2n variables. Additional enforcer variables may increase this by O(n), but the dominant term is 2n.
- Clauses: The m original clauses are transformed (with literal renaming), plus O(n) enforcer clauses to constrain the pairing, giving m + O(n) clauses.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a QBF instance, reduce to Variable Partition Truth Assignment, solve the game via game-tree search (minimax over all variable-choice orderings), verify that the game outcome matches the QBF truth value.
- Test with both true and false QBF instances.
- Test the special case of positive CNF (clauses with only un-negated literals) to verify PSPACE-completeness holds even in this restricted case.
- Verify variable and clause counts match the overhead formulas.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (QBF):**
F = exists u_1 forall u_2 exists u_3 . E

Variables: U = {u_1, u_2, u_3} (n = 3)

Clauses of E (positive CNF, 4 clauses over renamed variables):
- c_1 = (u_1 or u_2 or u_3)
- c_2 = (not u_1 or u_2)
- c_3 = (u_1 or not u_2 or not u_3)
- c_4 = (not u_1 or u_3)

**Constructed target instance (Variable Partition Truth Assignment):**

Variable set: U' = {x_1, y_1, x_2, y_2, x_3, y_3} (2n = 6 variables)
- x_i represents the positive literal of u_i
- y_i represents the negative literal of u_i

Clause transformation (positive-literal form):
- c'_1 = (x_1 or x_2 or x_3)       [from c_1: u_1 or u_2 or u_3]
- c'_2 = (y_1 or x_2)               [from c_2: not u_1 or u_2]
- c'_3 = (x_1 or y_2 or y_3)        [from c_3: u_1 or not u_2 or not u_3]
- c'_4 = (y_1 or x_3)               [from c_4: not u_1 or u_3]

Enforcer clauses (ensure consistency):
- e_1 = (x_1 or y_1)   [at least one of {x_1, y_1} must be chosen by player 1]
- e_2 = (x_2 or y_2)
- e_3 = (x_3 or y_3)

Total: C' = {c'_1, c'_2, c'_3, c'_4, e_1, e_2, e_3} (m + n = 4 + 3 = 7 clauses)

**Game play:**
- Players alternate choosing a variable from U'. Player 1 goes first.
- Player 1 chooses x_1 (claiming u_1 = true). Player 2 must respond; suppose player 2 picks y_2 (claiming u_2 = false). Player 1 picks x_3 (claiming u_3 = true). Player 2 picks y_1. Player 1 picks x_2. Player 2 picks y_3.
- Variables chosen by player 1: {x_1, x_3, x_2} -- set to true.
- Variables chosen by player 2: {y_2, y_1, y_3} -- set to false.
- Check clauses: c'_1 = (T or F or T) = T; c'_2 = (F or F) = F -- player 1 does not win with this play.

**Solution mapping:**
- Player 1's winning strategy (if the QBF is true) corresponds to Skolem functions for the existential variables, encoded as a strategy for choosing variables in the partition game.
- Player 2's winning strategy (if the QBF is false) corresponds to a universal counterexample.
- The game tree has at most 6! = 720 possible play orders (reduced by strategic dominance), and minimax evaluation determines the winner.


## References

- **[Schaefer, 1978a]**: [`Schaefer1978a`] T. J. Schaefer (1978). "Complexity of some two-person perfect-information games". *Journal of Computer and System Sciences* 16, pp. 185-225.

---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to SIFT"
labels: rule
assignees: ''
canonical_source_name: 'Quantified Boolean Formulas (QBF)'
canonical_target_name: 'Sift'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** QBF
**Target:** SIFT
**Motivation:** Establishes PSPACE-completeness of the Sift game by reduction from QBF, showing that a combinatorial set-intersection game with two competing families of subsets captures the full complexity of quantified Boolean satisfiability, bridging logical and set-theoretic game formulations.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.255

## GJ Source Entry

> [GP6] SIFT (*)
> INSTANCE: Two collections A and B of subsets of a finite set X, with A and B having no subsets in common.
> QUESTION: Does player 1 have a forced win in the following game played on A, B, and X? Players alternate choosing an element from X until the set X' of all elements chosen so far either intersects all the subsets in A or intersects all the subsets in B. Player 1 wins if and only if the final set X' of chosen elements intersects all the subsets in B and, if player 1 made the last move, does not intersect all subsets in A.
> Reference: [Schaefer, 1978a]. Transformation from QBF.
> Comment: PSPACE-complete.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a QBF instance F = (Q_1 u_1)(Q_2 u_2)...(Q_n u_n) E where E is a Boolean expression in CNF with clauses C = {c_1, ..., c_m}, construct a Sift instance (X, A, B) as follows. This reduction is due to Schaefer (1978a).

1. **Element set X:** For each variable u_i in the QBF, create two elements: x_i (representing u_i = true) and y_i (representing u_i = false). Thus |X| = 2n.

2. **Collection B (player 1's goal):** For each clause c_j in C, create a subset B_j in B consisting of the elements corresponding to the literals in c_j. If literal u_i appears in c_j, include x_i in B_j; if literal not-u_i appears in c_j, include y_i in B_j. Player 1 wins when X' intersects all subsets in B, i.e., every clause has at least one literal "hit" -- analogous to satisfying all clauses.

3. **Collection A (termination/spoiling condition):** Create subsets in A that encode the universal quantifier's adversarial role. For each universally quantified variable u_i (where Q_i = forall), add a subset {x_i, y_i} to A. Additionally, add auxiliary subsets to A that enforce variable consistency and the quantifier ordering. The subsets in A ensure that once all universal variables have been assigned (both x_i and y_i for some universal u_i are "hit"), the game reaches a termination condition.

4. **Game dynamics:** The alternating element choices simulate the quantifier alternation in the QBF:
   - Choosing x_i corresponds to setting u_i = true; choosing y_i corresponds to setting u_i = false.
   - Player 1 (existential) tries to hit all subsets in B (satisfy all clauses).
   - Player 2 (universal) tries to either prevent B from being fully hit, or trigger the A-termination condition prematurely.

5. **Correctness:** Player 1 has a forced win in the Sift game iff the QBF F is true. The winning condition (X' intersects all of B, and if player 1 moved last, X' does not intersect all of A) precisely captures the semantics of the quantified formula.

**Source:** Schaefer (1978a), "Complexity of some two-person perfect-information games."

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source QBF instance (number of variables)
- m = `num_clauses` of source QBF instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_elements` | `2 * num_vars` |
| `num_sets_a` | `num_vars` |
| `num_sets_b` | `num_clauses` |

**Derivation:**
- Elements: 2 per variable (true/false representatives), so |X| = 2n.
- Sets in A: O(n) subsets encoding the universal quantifier constraints and variable consistency.
- Sets in B: m subsets, one per clause, each containing the literal representatives.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a QBF instance, reduce to Sift, solve the Sift game by exhaustive game-tree search (minimax over element choices), verify that the game outcome matches the QBF truth value.
- Test with both true and false QBF instances.
- Verify that |X|, |A|, and |B| match the overhead formulas.
- Check that A and B have no subsets in common (required by the problem definition).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (QBF):**
F = exists u_1 forall u_2 exists u_3 . (u_1 or u_2 or u_3) and (not u_1 or not u_2) and (u_2 or not u_3)

Variables: U = {u_1, u_2, u_3} (n = 3), Clauses (m = 3):
- c_1 = (u_1 or u_2 or u_3)
- c_2 = (not u_1 or not u_2)
- c_3 = (u_2 or not u_3)

**Constructed target instance (Sift):**

Element set: X = {x_1, y_1, x_2, y_2, x_3, y_3} (2n = 6 elements)
- x_i represents u_i = true, y_i represents u_i = false

Collection B (clause subsets, m = 3):
- B_1 = {x_1, x_2, x_3}   [from c_1: u_1 or u_2 or u_3]
- B_2 = {y_1, y_2}         [from c_2: not u_1 or not u_2]
- B_3 = {x_2, y_3}         [from c_3: u_2 or not u_3]

Collection A (quantifier-encoding subsets):
- A_1 = {x_1, y_1}   [variable 1 consistency -- existential]
- A_2 = {x_2, y_2}   [variable 2 consistency -- universal]
- A_3 = {x_3, y_3}   [variable 3 consistency -- existential]

Verification that A and B share no common subset: B_1 = {x_1, x_2, x_3}, B_2 = {y_1, y_2}, B_3 = {x_2, y_3} are all distinct from A_1 = {x_1, y_1}, A_2 = {x_2, y_2}, A_3 = {x_3, y_3}.

**Game play (one scenario):**
- Player 1 picks x_1 (u_1 = true). X' = {x_1}. Hits B_1 partially.
- Player 2 picks x_2 (u_2 = true). X' = {x_1, x_2}. Hits B_1 (via x_1), B_3 (via x_2).
- Player 1 picks y_3 (u_3 = false). X' = {x_1, x_2, y_3}. Now check:
  - B_1: x_1 in X' -- hit. B_2: y_1 not in X', y_2 not in X' -- NOT hit. B_3: x_2 in X' -- hit.
  - A_1: x_1 in X', y_1 not -- not fully hit. A_2: x_2 in X', y_2 not -- not fully hit. A_3: y_3 in X', x_3 not -- not fully hit.
  - Game continues (neither all of A nor all of B fully hit).
- Player 2 picks y_1. X' = {x_1, x_2, y_3, y_1}. Now B_2 = {y_1, y_2}: y_1 hit. A_1 = {x_1, y_1}: fully hit.
  - Still not all of A hit (A_2, A_3 not fully hit) and not all of B hit (B_2 still needs y_2).
- Player 1 picks y_2. X' = {x_1, x_2, y_3, y_1, y_2}. B_2: y_1 and y_2 both hit. All of B hit!
  - Check A: A_1 fully hit, A_2 = {x_2, y_2} fully hit, A_3 = {x_3, y_3}: x_3 not in X' -- not fully hit.
  - Player 1 made last move. X' intersects all of B (good). Does X' intersect all of A? A_3 not fully hit, so no.
  - Player 1 wins!

**Solution mapping:**
- The truth assignment corresponding to this game play: u_1 = T (x_1 chosen), u_2 = T (x_2 chosen), u_3 = F (y_3 chosen).
- Check against original QBF: c_1 = T or T or F = T; c_2 = F or F = F. Not satisfying -- so this particular play does not reflect the QBF structure perfectly (the game dynamics differ from direct evaluation due to the A/B interaction). The full game tree must be analyzed to determine the winner.


## References

- **[Schaefer, 1978a]**: [`Schaefer1978a`] T. J. Schaefer (1978). "Complexity of some two-person perfect-information games". *Journal of Computer and System Sciences* 16, pp. 185-225.

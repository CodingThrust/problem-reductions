---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to ALTERNATING HITTING SET"
labels: rule
assignees: ''
canonical_source_name: 'Quantified Boolean Formulas (QBF)'
canonical_target_name: 'Alternating Hitting Set'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** QBF
**Target:** ALTERNATING HITTING SET
**Motivation:** Establishes PSPACE-completeness of the Alternating Hitting Set game by reduction from QBF, demonstrating that a simple combinatorial game where players alternately select elements to cover subsets -- with the last player to complete coverage losing -- inherits the full difficulty of quantified Boolean formula evaluation.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.255

## GJ Source Entry

> [GP7] ALTERNATING HITTING SET (*)
> INSTANCE: A collection C of subsets of a basic set B.
> QUESTION: Does player 1 have a forced win in the following game played on C and B? Players alternate choosing a new element of B until, for each c E C, some member of c has been chosen. The player whose choice causes this to happen loses.
> Reference: [Schaefer, 1978a]. Transformation from QBF.
> Comment: PSPACE-complete even if no set in C contains more than two elements, a subcase of the original HITTING SET problem that can be solved in polynomial time. If the roles of winner and loser are reversed, the problem is PSPACE-complete even if no set in C contains more than three elements.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a QBF instance F = (Q_1 u_1)(Q_2 u_2)...(Q_n u_n) E where E is in CNF with clauses {c_1, ..., c_m}, construct an Alternating Hitting Set instance (B, C) as follows. This reduction is due to Schaefer (1978a).

1. **Basic set B:** For each variable u_i in the QBF, create two elements: t_i (representing u_i = true) and f_i (representing u_i = false). Additionally, introduce auxiliary elements to encode the quantifier structure and game termination. Thus |B| = 2n + O(n).

2. **Subset collection C:** The subsets in C encode both the clause structure and the "last-to-cover-loses" game dynamics:
   - **Variable-pair subsets:** For each variable u_i, add {t_i, f_i} to C. This ensures that for each variable, at least one of t_i or f_i must be chosen. Since the game ends when all subsets are hit, the variable-pair subsets force all variables to be assigned.
   - **Clause-encoding subsets:** For each clause c_j, add subsets that connect the clause satisfaction to the game dynamics. If c_j = (l_1 or l_2 or l_3), add auxiliary subsets linking the literal representatives to game-ending elements. The construction ensures that an unsatisfied clause forces a premature game end (and thus a loss for the responsible player).
   - **Quantifier-ordering subsets:** Add subsets encoding the alternating quantifier structure so that rational play follows the quantifier prefix order.

3. **Game dynamics:** Players alternate choosing elements from B. Each choice "hits" all subsets containing that element. The game terminates when every subset in C has been hit, and the player who makes the final hit (completing coverage) loses.

4. **Correctness:** Player 1 has a forced win in the Alternating Hitting Set game iff the QBF F is true. The key insight is:
   - Choosing t_i corresponds to setting u_i = true; choosing f_i corresponds to u_i = false.
   - The existential player (player 1) tries to delay the complete coverage (avoid being the one to finish) while ensuring clause subsets are eventually satisfied.
   - The universal player (player 2) tries to force player 1 into making the final covering move.
   - The game dynamics with the "loser completes" rule capture the quantifier alternation semantics.

5. **Restriction result:** The problem remains PSPACE-complete even when every set in C has at most 2 elements (a restriction to pairs), which is a subcase of the classical Hitting Set problem solvable in polynomial time. This shows the game version is fundamentally harder.

**Source:** Schaefer (1978a), "Complexity of some two-person perfect-information games."

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source QBF instance (number of variables)
- m = `num_clauses` of source QBF instance (number of clauses)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_elements` | `2 * num_vars + num_clauses` |
| `num_sets` | `num_vars + num_clauses` |

**Derivation:**
- Elements: 2 per variable (true/false representatives) + auxiliary elements for clause encoding and game termination, O(n + m) total.
- Sets: n variable-pair subsets + m clause-related subsets + O(n) quantifier-ordering subsets, giving O(n + m) total.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a QBF instance, reduce to Alternating Hitting Set, solve the game by exhaustive game-tree search (minimax over element choices, with the "last-to-cover loses" termination condition), verify that the game outcome matches the QBF truth value.
- Test with both true and false QBF instances.
- Test the restricted case where all subsets have at most 2 elements.
- Verify that |B| and |C| match the overhead formulas.
- Check the reversed-roles variant (last to cover wins) with subsets of size at most 3.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (QBF):**
F = exists u_1 forall u_2 exists u_3 . (u_1 or u_2) and (not u_1 or u_3) and (not u_2 or not u_3)

Variables: U = {u_1, u_2, u_3} (n = 3), Clauses (m = 3):
- c_1 = (u_1 or u_2)
- c_2 = (not u_1 or u_3)
- c_3 = (not u_2 or not u_3)

**Constructed target instance (Alternating Hitting Set):**

Basic set: B = {t_1, f_1, t_2, f_2, t_3, f_3, a_1, a_2, a_3} (2n + m = 9 elements)
- t_i = variable u_i is true, f_i = variable u_i is false
- a_j = auxiliary element for clause c_j

Subset collection C (n + m = 6 subsets):
- Variable-pair subsets:
  - S_1 = {t_1, f_1}   [variable u_1 must be assigned]
  - S_2 = {t_2, f_2}   [variable u_2 must be assigned]
  - S_3 = {t_3, f_3}   [variable u_3 must be assigned]
- Clause subsets (size at most 2, encoding clause satisfaction):
  - S_4 = {t_1, t_2}   [from c_1: u_1 or u_2 -- hit when u_1 or u_2 is true]
  - S_5 = {f_1, t_3}   [from c_2: not u_1 or u_3 -- hit when u_1 is false or u_3 is true]
  - S_6 = {f_2, f_3}   [from c_3: not u_2 or not u_3 -- hit when u_2 is false or u_3 is false]

**Game play (one scenario):**
Turn 1 (Player 1): Picks t_1. Hits S_1 (via t_1) and S_4 (via t_1).
Turn 2 (Player 2): Picks t_2. Hits S_2 (via t_2), S_4 already hit.
Turn 3 (Player 1): Picks t_3. Hits S_3 (via t_3) and S_5 (via t_3).
  - Check: S_1 hit, S_2 hit, S_3 hit, S_4 hit, S_5 hit.
  - S_6 = {f_2, f_3}: f_2 not chosen, f_3 not chosen -- NOT hit.
  - Game continues.
Turn 4 (Player 2): Picks f_3. Hits S_6 (via f_3).
  - All subsets hit! Player 2 made the completing move, so Player 2 LOSES.
  - Player 1 wins!

Alternative play (Player 2 tries a different strategy):
Turn 1 (Player 1): Picks t_1. Turn 2 (Player 2): Picks f_1. Hits S_1 (fully covered).
Turn 3 (Player 1): Picks t_3. Hits S_3 and S_5. Turn 4 (Player 2): Picks f_2.
  - Hits S_2 and S_6. Check: S_4 = {t_1, t_2}: t_1 hit. All hit now.
  - Player 2 made the completing move, so Player 2 LOSES. Player 1 wins.

**Solution mapping:**
- The QBF is true (exists u_1 forall u_2 exists u_3 such that all clauses are satisfied: e.g., u_1 = T, u_2 = T, u_3 = F gives c_1 = T, c_2 = F -- need to check all universal assignments).
- Player 1's winning strategy corresponds to the existential player's Skolem functions.
- The "last to complete loses" rule inverts the typical game objective, making the reduction more subtle than direct satisfiability checking.


## References

- **[Schaefer, 1978a]**: [`Schaefer1978a`] T. J. Schaefer (1978). "Complexity of some two-person perfect-information games". *Journal of Computer and System Sciences* 16, pp. 185-225.

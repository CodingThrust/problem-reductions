---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to SEQUENTIAL TRUTH ASSIGNMENT"
labels: rule
assignees: ''
canonical_source_name: 'Quantified Boolean Formulas (QBF)'
canonical_target_name: 'Sequential Truth Assignment'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** QBF
**Target:** SEQUENTIAL TRUTH ASSIGNMENT
**Motivation:** Establishes PSPACE-completeness of the Sequential Truth Assignment game by reduction from QBF, demonstrating that even simple two-player formula games with fixed variable ordering inherit the full difficulty of quantified boolean satisfiability.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.254-255

## GJ Source Entry

> [GP4] SEQUENTIAL TRUTH ASSIGNMENT (*)
> INSTANCE: A sequence U = <u_1,u_2,...,u_n> of variables and a collection C of clauses over U (as in an instance of SATISFIABILITY).
> QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate assigning truth values to the variables in U, with player 1 assigning a value to u_{2i-1} and player 2 assigning a value to u_{2i} on their i^{th} turns. Player 1 wins if and only if the resulting truth assignment satisfies all clauses in C.
> Reference: [Stockmeyer and Meyer, 1973]. Transformation from QBF.
> Comment: PSPACE-complete, even if each clause in C has only three literals. Solvable in polynomial time if no clause has more than two literals [Schaefer, 1978b].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a QBF instance F = (Q_1 u_1)(Q_2 u_2)...(Q_n u_n) E where E is a Boolean expression in CNF and each Q_i is either forall or exists, construct a Sequential Truth Assignment instance (U', C') as follows:

1. **Variable reordering:** Rewrite the QBF into an equivalent form where quantifiers strictly alternate between exists and forall. If the QBF has k quantifier blocks, introduce dummy variables as needed so that the total number of variables n' is even and the quantifier prefix has the form exists-forall-exists-forall-... This can be done in polynomial time by padding with fresh variables that appear in no clause (they have no effect on satisfiability).

2. **Variable sequence:** Define the ordered sequence U' = <u'_1, u'_2, ..., u'_{n'}> where odd-indexed variables u'_{2i-1} correspond to existentially quantified variables and even-indexed variables u'_{2i} correspond to universally quantified variables. This maps the existential player to player 1 (who assigns u'_{2i-1}) and the universal player to player 2 (who assigns u'_{2i}).

3. **Clause construction:** Set C' = the clauses of E (possibly extended with dummy literals from step 1 to maintain 3-CNF form if desired). The clauses are over the same variables, now interpreted as a sequential game.

4. **Correctness:** Player 1 has a forced win in the Sequential Truth Assignment game on (U', C') if and only if the original QBF F is true. This is because:
   - Player 1 choosing values for odd-indexed variables corresponds to the existential quantifier choosing satisfying assignments.
   - Player 2 choosing values for even-indexed variables corresponds to the universal quantifier trying all possible assignments.
   - Player 1 wins (all clauses satisfied) iff there exists a strategy that works against all adversarial choices, which is exactly the semantics of QBF.

5. **Solution extraction:** A winning strategy for player 1 in the game directly gives a Skolem function witnessing the truth of the QBF. Conversely, if the QBF is false, player 2 has a spoiling strategy.

**Source:** Stockmeyer and Meyer (1973), "Word problems requiring exponential time." The reduction is essentially the observation that QBF evaluation is equivalent to a two-player game with alternating quantifiers mapped to alternating moves.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vars` of source QBF instance (number of variables)
- m = `num_clauses` of source QBF instance (number of clauses in the matrix E)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vars` | `num_vars` (at most `num_vars + 1` with padding) |
| `num_clauses` | `num_clauses` |

**Derivation:**
- Variables: The QBF variables map one-to-one to the game variable sequence. At most one dummy variable is added to ensure an even count, so n' <= n + 1.
- Clauses: The clauses of E are carried over directly without modification.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a QBF instance with alternating quantifiers, reduce to Sequential Truth Assignment, solve the game using minimax/game-tree search on the target, verify that the game outcome (player 1 wins or loses) matches the truth value of the original QBF.
- Test with both true and false QBF instances (e.g., a QBF that is true because exists-player can force satisfaction, and one that is false because forall-player can spoil).
- Verify variable count and clause count of the target match the overhead formulas.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (QBF):**
F = exists u_1 forall u_2 exists u_3 forall u_4 exists u_5 forall u_6 . E

Variables: U = {u_1, u_2, u_3, u_4, u_5, u_6} (n = 6)

Clauses of E (7 clauses):
- c_1 = (u_1 or u_2 or u_3)
- c_2 = (not u_1 or u_4 or u_5)
- c_3 = (u_2 or not u_3 or u_6)
- c_4 = (not u_2 or u_3 or not u_6)
- c_5 = (u_1 or not u_4 or u_6)
- c_6 = (not u_1 or u_2 or not u_5)
- c_7 = (u_3 or u_4 or not u_6)

**Constructed target instance (Sequential Truth Assignment):**

Variable sequence: U' = <u_1, u_2, u_3, u_4, u_5, u_6>
- Player 1 assigns: u_1 (turn 1), u_3 (turn 2), u_5 (turn 3)
- Player 2 assigns: u_2 (turn 1), u_4 (turn 2), u_6 (turn 3)

Clauses: C' = {c_1, c_2, c_3, c_4, c_5, c_6, c_7} (same 7 clauses)

**Solution mapping:**
- The QBF quantifier prefix exists u_1 forall u_2 exists u_3 forall u_4 exists u_5 forall u_6 maps directly to the game turn structure: player 1 controls existential variables (u_1, u_3, u_5) and player 2 controls universal variables (u_2, u_4, u_6).
- Suppose player 1 uses the strategy: u_1 = T, then regardless of u_2, set u_3 = T, regardless of u_4, set u_5 = T.
  - If player 2 picks u_2 = F, u_4 = F, u_6 = F: assignment = (T, F, T, F, T, F)
    - c_1: T or F or T = T; c_2: F or F or T = T; c_3: F or F or F = F -- not all satisfied.
  - Player 1 must adapt: if u_2 = F, set u_3 = T; if u_4 = F, set u_5 = T. After u_6 is revealed, check.
- Whether the QBF is true determines whether player 1 has a forced win. The game tree has 2^6 = 64 leaves, and minimax evaluation determines the outcome.
- A winning strategy for player 1 (if one exists) corresponds to Skolem functions: u_1 = f(), u_3 = g(u_2), u_5 = h(u_2, u_4), witnessing the QBF's truth.


## References

- **[Stockmeyer and Meyer, 1973]**: [`Stockmeyer and Meyer1973`] Larry J. Stockmeyer and Albert R. Meyer (1973). "Word problems requiring exponential time". In: *Proc. 5th Ann. ACM Symp. on Theory of Computing*, pp. 1-9. Association for Computing Machinery.
- **[Schaefer, 1978b]**: [`Schaefer1978b`] T. J. Schaefer (1978). "The complexity of satisfiability problems". In: *Proceedings of the 10th Annual ACM Symposium on Theory of Computing*, pp. 216-226. Association for Computing Machinery.

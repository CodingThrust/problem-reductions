---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to EQUILIBRIUM POINT"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability'
canonical_target_name: 'Equilibrium Point'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT
**Target:** EQUILIBRIUM POINT
**Motivation:** Establishes NP-completeness of the EQUILIBRIUM POINT problem via polynomial-time reduction from 3SAT. The reduction encodes each Boolean variable as a player with range set {0, 1} and constructs product polynomials so that a Nash-like equilibrium (where each player maximizes their payoff given others' choices) exists if and only if the 3SAT formula is satisfiable. This result, due to Sahni (1974), shows that finding equilibria of discrete games with product-polynomial payoffs is computationally intractable even with binary strategy sets.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.252

## GJ Source Entry

> [AN15] EQUILIBRIUM POINT
> INSTANCE: Set x = {x_1,x_2,...,x_n} of variables, collection {F_i: 1 ≤ i ≤ n} of product polynomials over X and the integers, and a finite "range-set" M_i ⊆ Z for 1 ≤ i ≤ n.
> QUESTION: Does there exist a sequence y_1,y_2,...,y_n of integers, with y_i E M_i, such that for 1 ≤ i ≤ n and all y E M_i,
>
> F_i(y_1,y_2,...,y_{i-1},y_i,y_{i+1},...,y_n) ≥ F_i(y_1,y_2,...,y_{i-1},y,y_{i+1},...,y_n)?
>
> Reference: [Sahni, 1974]. Transformation from 3SAT.
> Comment: Remains NP-complete even if M_i = {0,1} for 1 ≤ i ≤ n.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3SAT instance with variables x_1, ..., x_n and clauses C_1, ..., C_m (each clause has exactly 3 literals), construct an EQUILIBRIUM POINT instance as follows:

1. **Variables and range sets:** For each Boolean variable x_i, create an equilibrium variable x_i with range set M_i = {0, 1}. The value 1 represents TRUE and 0 represents FALSE.

2. **Product polynomials:** For each variable x_i, define the product polynomial F_i as a product of clause-satisfaction terms. For each clause C_j in which x_i appears (either positively or negatively), construct a factor:
   - If x_i appears positively in C_j = (x_i ∨ l_2 ∨ l_3), the factor contributed by C_j is: (x_i + val(l_2) + val(l_3)), where val(l_k) evaluates the literal l_k given the other variables' values. This is arranged so that the factor is maximized when the clause is satisfied.
   - The product polynomial F_i is the product of such factors over all clauses involving x_i, designed so that F_i is maximized at x_i's current value if and only if all clauses involving x_i are satisfied by the overall assignment.

3. **Correctness:** An equilibrium point (y_1, ..., y_n) exists (where no player can improve by unilateral deviation) if and only if the assignment x_i = y_i satisfies all clauses. If any clause is unsatisfied, at least one variable in that clause can improve its payoff by flipping, contradicting equilibrium.

4. **Solution extraction:** Given an equilibrium (y_1, ..., y_n), the satisfying assignment is x_i = y_i for all i.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of variables in 3SAT instance
- m = number of clauses in 3SAT instance

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_variables`           | n                                |
| `num_polynomials`         | n                                |
| `max_range_set_size`      | 2 (binary: {0, 1})              |
| `max_polynomial_degree`   | m (at most m factors per product)|

**Derivation:** Each 3SAT variable maps to one equilibrium variable with binary range. Each product polynomial has at most m factors (one per clause involving that variable). The total description size is O(n + m) since each clause contributes factors to at most 3 polynomials.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3SAT instance, reduce to EQUILIBRIUM POINT with M_i = {0, 1}, enumerate all 2^n assignments, verify that the equilibria correspond exactly to satisfying assignments.
- Check that each equilibrium variable has range set {0, 1}.
- Edge cases: test with a single clause (x_1 ∨ x_2 ∨ x_3) having 7 satisfying assignments; test with an unsatisfiable formula (e.g., all 8 clauses on 3 variables) having no equilibrium.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3
Clauses: C_1 = (x_1 ∨ x_2 ∨ x_3), C_2 = (¬x_1 ∨ ¬x_2 ∨ x_3)
(n = 3 variables, m = 2 clauses)

**Constructed EQUILIBRIUM POINT instance:**
- Variables: x_1, x_2, x_3 with M_1 = M_2 = M_3 = {0, 1}
- Product polynomials (simplified conceptual form):
  - F_1: depends on clauses C_1 (x_1 positive) and C_2 (x_1 negative). F_1 is constructed so that x_1's best response given (x_2, x_3) satisfies both clauses if possible.
  - F_2: depends on clauses C_1 (x_2 positive) and C_2 (x_2 negative). Similarly constructed.
  - F_3: depends on clauses C_1 (x_3 positive) and C_2 (x_3 positive). F_3 favors x_3 = 1.

**Solution:**
The assignment (x_1, x_2, x_3) = (1, 0, 1) satisfies both clauses:
- C_1 = (1 ∨ 0 ∨ 1) = TRUE
- C_2 = (0 ∨ 1 ∨ 1) = TRUE

At this assignment, no variable can improve its payoff by flipping (since all clauses are already satisfied), so (1, 0, 1) is an equilibrium point.

**Solution extraction:**
Satisfying assignment: x_1 = TRUE, x_2 = FALSE, x_3 = TRUE ✓


## References

- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262–279.

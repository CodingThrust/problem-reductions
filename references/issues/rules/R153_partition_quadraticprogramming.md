---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Quadratic Programming"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION'
canonical_target_name: 'QUADRATIC PROGRAMMING'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Partition (not in codebase)
**Target:** Quadratic Programming (not in codebase)
**Motivation:** This reduction establishes the NP-hardness of quadratic programming (with indefinite objectives) by encoding the PARTITION problem, which asks whether a set of integers can be split into two equal-sum halves. The reduction shows that even optimizing a quadratic objective over a linearly constrained polytope is intractable when the quadratic form is not positive semidefinite, bridging combinatorial number theory to nonlinear optimization.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.245

## GJ Source Entry

> [MP2] QUADRATIC PROGRAMMING (*)
> INSTANCE: Finite set X of pairs (x-bar, b), where x-bar is an m-tuple of rational numbers and b is a rational number, two m-tuples c-bar and d-bar of rational numbers, and a rational number B.
> QUESTION: Is there an m-tuple y-bar of rational numbers such that x-bar·y-bar <= b for all (x-bar, b) E X and such that sum_{i=1}^{m} (c_i y_i^2 + d_i y_i) >= B, where c_i, y_i, and d_i denote the i^th components of c-bar, y-bar, and d-bar respectively?
> Reference: [Sahni, 1974]. Transformation from PARTITION.
> Comment: Not known to be in NP, unless the c_i's are all non-negative [Klee, 1978]. If the constraints are quadratic and the objective function is linear (the reverse of the situation above), then the problem is also NP-hard [Sahni, 1974]. If we add to this last problem the requirement that all entries of y-bar be integers, then the problem becomes undecidable [Jeroslow, 1973].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a PARTITION instance with n positive integers a_1, ..., a_n and total sum S = Σ a_i, construct a quadratic programming instance as follows:

1. **Variables:** Create n rational variables y_1, ..., y_n.

2. **Linear constraints (enforce binary behavior via quadratics in objective):** Impose box constraints:
   0 <= y_i <= 1 for each i = 1, ..., n.
   Also add the constraint: Σ_{i=1}^{n} a_i · y_i <= S/2.
   And: Σ_{i=1}^{n} a_i · (1 - y_i) <= S/2, which simplifies to Σ_{i=1}^{n} a_i · y_i >= S/2.
   Together these force Σ a_i y_i = S/2.

3. **Quadratic objective:** Set c_i = -a_i and d_i = a_i for each i. Then the objective is:
   Σ_{i=1}^{n} (c_i y_i^2 + d_i y_i) = Σ_{i=1}^{n} a_i (y_i - y_i^2) = Σ_{i=1}^{n} a_i · y_i(1 - y_i).
   This expression is non-negative and equals zero if and only if every y_i ∈ {0, 1}. When all y_i are binary, the equality constraint Σ a_i y_i = S/2 is exactly the PARTITION condition. Set B = 0.

   Alternatively, to force integrality: set c_i = -M (large negative) and d_i = M, so that the objective Σ M · y_i(1 - y_i) acts as a penalty pushing y_i toward {0, 1}. But the simpler formulation uses the equality constraint plus B = 0.

4. **Correctness:** A partition exists (a subset summing to S/2) if and only if there exists a feasible point y-bar with all y_i ∈ {0, 1} satisfying Σ a_i y_i = S/2. The quadratic objective attains its maximum of 0 only at binary points (since a_i > 0 and y_i(1-y_i) >= 0 on [0,1]). With B = 0, the QP is feasible iff such a binary point exists, i.e., iff PARTITION has a solution.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements (`num_items` of source PARTITION instance)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vars` (m)            | `num_items` (= n)                |
| `num_constraints`         | `2 * num_items + 2` (= 2n + 2)  |

**Derivation:** One QP variable per partition element. Box constraints contribute 2n inequalities (0 <= y_i <= 1), and the two half-sum constraints contribute 2 more. The quadratic and linear coefficient vectors each have n entries. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to QP, solve (enumerate binary points satisfying the equality constraint), and verify the solution maps back to a valid balanced partition.
- Correctness check: confirm that the quadratic objective equals 0 at the solution (all variables binary) and that the sum constraint is satisfied.
- Infeasible case: test with an odd total sum S (no partition possible); verify QP has no feasible binary point achieving objective >= B = 0 with the equality constraint.
- Edge cases: test with n = 2 (trivial partition if a_1 = a_2), test with all elements equal.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**

A = {2, 3, 5, 4, 6} (n = 5 elements)
Total sum S = 2 + 3 + 5 + 4 + 6 = 20
Target half-sum S/2 = 10.
A balanced partition exists: A' = {4, 6} (sum = 10) and A \ A' = {2, 3, 5} (sum = 10).

**Constructed Quadratic Programming instance:**

Variables: y_1, y_2, y_3, y_4, y_5 (rationals).

Linear constraints (12 rows):
- 0 <= y_i <= 1 for i = 1..5 (10 bound constraints)
- 2y_1 + 3y_2 + 5y_3 + 4y_4 + 6y_5 <= 10
- -2y_1 - 3y_2 - 5y_3 - 4y_4 - 6y_5 <= -10

These two together force 2y_1 + 3y_2 + 5y_3 + 4y_4 + 6y_5 = 10.

Quadratic objective:
c-bar = (-2, -3, -5, -4, -6), d-bar = (2, 3, 5, 4, 6), B = 0.
Objective = Σ a_i · y_i(1 - y_i) = 2y_1(1-y_1) + 3y_2(1-y_2) + 5y_3(1-y_3) + 4y_4(1-y_4) + 6y_5(1-y_5).

**Solution:**

y = (0, 0, 0, 1, 1):
- Sum check: 2(0) + 3(0) + 5(0) + 4(1) + 6(1) = 10 = S/2 ✓
- Objective: 2·0 + 3·0 + 5·0 + 4·0 + 6·0 = 0 >= B = 0 ✓

All bounds satisfied ✓.

**Solution extraction:**
y = (0, 0, 0, 1, 1) → A' = {a_4, a_5} = {4, 6} (sum = 10), A \ A' = {2, 3, 5} (sum = 10). Balanced ✓


## References

- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262–279.
- **[Klee, 1978]**: [`Klee1978`] Victor Klee (1978). "Private communication".
- **[Jeroslow, 1973]**: [`Jeroslow1973`] Robert G. Jeroslow (1973). "There cannot be any algorithm for integer programming with quadratic constraints". *Operations Research* 21, pp. 221–224.

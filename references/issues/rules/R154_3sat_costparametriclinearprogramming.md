---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Cost-Parametric Linear Programming"
labels: rule
assignees: ''
canonical_source_name: '3SAT'
canonical_target_name: 'COST-PARAMETRIC LINEAR PROGRAMMING'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3SAT (KSatisfiability in codebase)
**Target:** Cost-Parametric Linear Programming (not in codebase)
**Motivation:** This reduction proves that cost-parametric linear programming is NP-complete, showing that sensitivity analysis of LP cost vectors — a natural question arising from first-order error analysis — is computationally intractable. Even for any fixed perturbation radius q > 0, determining whether a cost perturbation can change the optimal LP value beyond a threshold is as hard as 3SAT.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.245

## GJ Source Entry

> [MP3] COST-PARAMETRIC LINEAR PROGRAMMING
> INSTANCE: Finite set X of pairs (x-bar, b), where x-bar is an m-tuple of integers and b is an integer, a set J ⊆ {1, 2, ..., m}, and a positive rational number q.
> QUESTION: Is there an m-tuple c-bar with rational entries such that (c-bar·c-bar)^{1/2} <= q and such that, if Y is the set of all m-tuples y-bar with non-negative rational entries satisfying x-bar·y-bar >= b for all (x-bar, b) E X, then the minimum of sum_{j E J} c_j y_j over all y-bar E Y exceeds
> 1/2 max {|c_j|: j E J} + sum_{j E J} min {0, c_j} ?
> Reference: [Jeroslow, 1976]. Transformation from 3SAT.
> Comment: Remains NP-complete for any fixed q > 0. The problem arises from first order error analysis for linear programming.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3SAT instance with n variables x_1, ..., x_n and m clauses C_1, ..., C_m, construct a cost-parametric LP instance as follows:

1. **LP feasible region:** Construct linear constraints that encode the clause structure of the 3SAT formula. For each Boolean variable x_i, introduce LP variables y_i (representing x_i) and y_{n+i} (representing NOT x_i) with the linking constraint y_i + y_{n+i} = 1, plus non-negativity y_i >= 0, y_{n+i} >= 0.

2. **Clause constraints:** For each clause C_j = (l_{j1} OR l_{j2} OR l_{j3}), add a constraint that the sum of the LP variables corresponding to the three literals is >= 1. For example, (x_1 OR NOT x_2 OR x_3) becomes y_1 + y_{n+2} + y_3 >= 1.

3. **Index set J and cost perturbation:** Set J to be a subset of variable indices corresponding to the key LP variables. The cost vector c-bar acts as a perturbation with Euclidean norm bounded by q. Jeroslow's construction arranges the LP so that a cost perturbation c-bar within the q-ball can make the LP optimal value exceed the threshold (½ max|c_j| + Σ min{0, c_j}) if and only if the underlying 3SAT formula is satisfiable.

4. **Threshold condition:** The threshold ½ max{|c_j| : j ∈ J} + Σ_{j ∈ J} min{0, c_j} is designed so that when the LP has a vertex corresponding to a satisfying assignment, there exists a cost perturbation that drives the objective above this value. If no satisfying assignment exists, no perturbation within the q-ball can exceed the threshold.

5. **Correctness:** The 3SAT formula is satisfiable if and only if a cost vector c-bar with ||c-bar|| <= q exists such that the minimum of Σ c_j y_j over the feasible region Y exceeds the threshold. The reduction runs in polynomial time since the LP constraints are directly derived from the clause structure.

**Note:** The precise encoding follows Jeroslow's bracketing technique, which frames discrete feasibility as a gap between two continuous LP bounds. The full construction details are in [Jeroslow, 1976].

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of Boolean variables (`num_variables` of source 3SAT instance)
- m = number of clauses (`num_clauses` of source 3SAT instance)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_lp_vars` (m in GJ)   | `2 * num_variables` (= 2n)       |
| `num_constraints`         | `num_variables + num_clauses` (= n + m) |
| `index_set_size` (\|J\|)  | at most `2 * num_variables` (= 2n)|

**Derivation:** Two LP variables per Boolean variable (y_i and y_{n+i} for complementary literals). One linking constraint per variable (y_i + y_{n+i} = 1) plus one constraint per clause. The scalar q and index set J are fixed parameters. Construction is O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a small 3SAT instance, reduce to a Cost-Parametric LP instance, verify the structure (constraint matrix, index set J, threshold formula) matches the 3SAT clauses.
- Satisfiable case: for a satisfiable 3SAT formula, verify that a cost vector c-bar with ||c-bar|| <= q exists such that the LP minimum exceeds the threshold.
- Unsatisfiable case: for an unsatisfiable 3SAT formula, verify that no cost perturbation within the q-ball can make the LP minimum exceed the threshold.
- Edge cases: test with a single clause (m=1), test with q very small and very large.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT / KSatisfiability<K3>):**

Variables: x_1, x_2, x_3 (n = 3)
Clauses (m = 2):
- C_1: (x_1 OR NOT x_2 OR x_3)
- C_2: (NOT x_1 OR x_2 OR NOT x_3)

Satisfying assignment: x_1 = TRUE, x_2 = TRUE, x_3 = TRUE.
(C_1: TRUE OR FALSE OR TRUE = TRUE, C_2: FALSE OR TRUE OR FALSE = TRUE.)

**Constructed Cost-Parametric LP instance:**

LP variables: y_1, y_2, y_3, y_4, y_5, y_6 (2n = 6 variables).
- y_1 ~ x_1, y_4 ~ NOT x_1
- y_2 ~ x_2, y_5 ~ NOT x_2
- y_3 ~ x_3, y_6 ~ NOT x_3

Constraints:
- Linking: y_1 + y_4 = 1, y_2 + y_5 = 1, y_3 + y_6 = 1
- Clause C_1: y_1 + y_5 + y_3 >= 1
- Clause C_2: y_4 + y_2 + y_6 >= 1
- Non-negativity: y_i >= 0 for all i

Index set J = {1, 2, 3, 4, 5, 6}, q > 0 (e.g., q = 1).

Feasible region Y: all non-negative (y_1, ..., y_6) satisfying the linking and clause constraints.

**Verification sketch:**
The vertices of Y correspond to binary assignments of (x_1, x_2, x_3). At vertex y = (1, 1, 1, 0, 0, 0) (the satisfying assignment), a cost perturbation c-bar with ||c-bar|| <= 1 can be chosen to make the LP optimal value exceed the threshold, confirming satisfiability.

For an unsatisfiable formula, no vertex would satisfy all clause constraints, so the feasible region's vertex structure prevents any cost perturbation from exceeding the threshold.


## References

- **[Jeroslow, 1976]**: [`Jeroslow1976`] Robert G. Jeroslow (1976). "Bracketing discrete problems by two problems of linear optimization". In: *Proceedings of the First Symposium on Operations Research (at Heidelberg)*, pp. 205–216. Verlag Anton Hain.

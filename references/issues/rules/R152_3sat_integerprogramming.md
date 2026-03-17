---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Integer Programming"
labels: rule
assignees: ''
canonical_source_name: '3SAT'
canonical_target_name: 'INTEGER PROGRAMMING'
source_in_codebase: true
target_in_codebase: true
milestone: 'Garey & Johnson'
---

**Source:** 3SAT (KSatisfiability in codebase)
**Target:** Integer Programming (ILP in codebase)
**Motivation:** This reduction establishes the NP-completeness of integer programming by encoding Boolean satisfiability constraints as linear inequalities over binary variables. It is one of Karp's original 21 reductions (1972) and serves as the foundational link between logic and mathematical programming, enabling every SAT instance to be solved via ILP solvers.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.245

## GJ Source Entry

> [MP1] INTEGER PROGRAMMING
> INSTANCE: Finite set X of pairs (x-bar, b), where x-bar is an m-tuple of integers and b is an integer, an m-tuple c-bar of integers, and an integer B.
> QUESTION: Is there an m-tuple y-bar of integers such that x-bar·y-bar <= b for all (x-bar, b) E X and such that c-bar·y-bar >= B (where the dot-product u-bar·v-bar of two m-tuples u-bar = (u_1, u_2, ..., u_m) and v-bar = (v_1, v_2, ..., v_m) is given by sum_{i=1}^{m} u_i v_i)?
> Reference: [Karp, 1972], [Borosh and Treybig, 1976]. Transformation from 3SAT. The second reference proves membership in NP.
> Comment: NP-complete in the strong sense. Variant in which all components of y-bar are required to belong to {0,1} (ZERO-ONE INTEGER PROGRAMMING) is also NP-complete, even if each b, all components of each x-bar, and all components of c-bar are required to belong to {0,1}. Also NP-complete are the questions of whether a y-bar with non-negative integer entries exists such that x-bar·y-bar = b for all (x-bar, b) E X, and the question of whether there exists any y-bar with integer entries such that x-bar·y-bar >= 0 for all (x-bar, b) E X [Sahni, 1974].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3SAT instance with n Boolean variables x_1, ..., x_n and m clauses C_1, ..., C_m, construct a 0-1 integer programming instance as follows:

1. **Variables:** Create n integer variables y_1, ..., y_n, each constrained to {0, 1}. Setting y_i = 1 represents x_i = TRUE and y_i = 0 represents x_i = FALSE. The binary constraints are encoded as: 0 <= y_i and y_i <= 1 for each i.

2. **Clause constraints:** For each clause C_j, construct one linear inequality. For a clause such as (x_a OR NOT x_b OR x_c), create the constraint:
   y_a + (1 - y_b) + y_c >= 1.
   In general, for a clause with positive literals x_i contribute y_i, and negated literals NOT x_i contribute (1 - y_i). The sum of these terms must be >= 1.
   After rearranging, the constraint becomes: for each clause C_j, let P_j be the set of variables appearing positively and N_j be the set appearing negatively. Then:
   Σ_{i ∈ P_j} y_i - Σ_{i ∈ N_j} y_i >= 1 - |N_j|.
   Equivalently (in Ax <= b form): Σ_{i ∈ N_j} y_i - Σ_{i ∈ P_j} y_i <= |N_j| - 1.

3. **Objective:** The objective function is trivial (e.g., c-bar = 0, B = 0), since we only need feasibility. Alternatively, set c-bar = (1, 1, ..., 1) and B = 0, which is always satisfiable if the constraints hold.

4. **Correctness:** A satisfying assignment for the 3SAT formula exists if and only if the constructed ILP system has a feasible 0-1 solution. Each clause constraint ensures at least one literal evaluates to TRUE.

**Matrix formulation (Ax <= b form):**
Construct a (2n + m) × n matrix A and vector b:
- Rows 1..n: y_i <= 1 (upper bound constraints)
- Rows n+1..2n: -y_i <= 0 (non-negativity constraints)
- Rows 2n+1..2n+m: for clause j, a_{j,i} = 1 if variable i appears negated in clause j, a_{j,i} = -1 if variable i appears unnegated, and a_{j,i} = 0 otherwise. The RHS b_j = |N_j| - 1.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of Boolean variables (`num_variables` of source 3SAT instance)
- m = number of clauses (`num_clauses` of source 3SAT instance)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vars`                | `num_variables` (= n)            |
| `num_constraints`         | `2 * num_variables + num_clauses` (= 2n + m) |

**Derivation:** One ILP variable per Boolean variable (n total). Each variable contributes 2 bound constraints (0 <= y_i <= 1), and each clause contributes 1 linear inequality. Total constraints = 2n + m. Construction time is O(n + m) since each clause has at most 3 literals.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a small 3SAT instance (KSatisfiability<K3>), reduce to ILP<bool>, solve with BruteForce (`find_satisfying`), and verify that the ILP solution maps back to a satisfying assignment for the original 3SAT formula.
- Correctness check: confirm that every clause constraint is satisfied (sum of active literals >= 1) and that all variables are binary.
- Unsatisfiable case: construct an unsatisfiable 3SAT instance (e.g., x AND NOT x), reduce, verify ILP has no feasible solution.
- Edge cases: test with a single clause, test with all-positive and all-negative literal clauses.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT / KSatisfiability<K3>):**

Variables: x_1, x_2, x_3, x_4 (n = 4)
Clauses (m = 3):
- C_1: (x_1 OR NOT x_2 OR x_3)
- C_2: (NOT x_1 OR x_2 OR NOT x_4)
- C_3: (x_2 OR x_3 OR x_4)

Satisfying assignment: x_1 = TRUE, x_2 = TRUE, x_3 = TRUE, x_4 = FALSE.

**Constructed ILP<bool> instance:**

Variables: y_1, y_2, y_3, y_4, each in {0, 1}.

Bound constraints (8 rows):
- y_1 <= 1, y_2 <= 1, y_3 <= 1, y_4 <= 1
- -y_1 <= 0, -y_2 <= 0, -y_3 <= 0, -y_4 <= 0

Clause constraints (3 rows, in Ax <= b form):
- C_1: (x_1 OR NOT x_2 OR x_3) → y_2 - y_1 - y_3 <= 0
  (P_1 = {1, 3}, N_1 = {2}, |N_1| - 1 = 0)
- C_2: (NOT x_1 OR x_2 OR NOT x_4) → y_1 + y_4 - y_2 <= 1
  (P_2 = {2}, N_2 = {1, 4}, |N_2| - 1 = 1)
- C_3: (x_2 OR x_3 OR x_4) → -y_2 - y_3 - y_4 <= -1
  (P_3 = {2, 3, 4}, N_3 = {}, |N_3| - 1 = -1)

Objective: c-bar = (0, 0, 0, 0), B = 0 (trivially satisfiable).

Total constraints: 2(4) + 3 = 11.

**Solution:**

y = (1, 1, 1, 0):
- C_1: y_2 - y_1 - y_3 = 1 - 1 - 1 = -1 <= 0 ✓
- C_2: y_1 + y_4 - y_2 = 1 + 0 - 1 = 0 <= 1 ✓
- C_3: -y_2 - y_3 - y_4 = -1 - 1 - 0 = -2 <= -1 ✓

All bounds satisfied, objective 0 >= 0 ✓.

**Solution extraction:**
y = (1, 1, 1, 0) → x_1 = TRUE, x_2 = TRUE, x_3 = TRUE, x_4 = FALSE.
- C_1: TRUE OR NOT TRUE OR TRUE = TRUE ✓
- C_2: NOT TRUE OR TRUE OR NOT FALSE = TRUE ✓
- C_3: TRUE OR TRUE OR FALSE = TRUE ✓


## References

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Borosh and Treybig, 1976]**: [`Borosh1976`] I. Borosh and L. B. Treybig (1976). "Bounds on positive integral solutions of linear {Diophantine} equations". *Proceedings of the American Mathematical Society* 55, pp. 299–304.
- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262–279.

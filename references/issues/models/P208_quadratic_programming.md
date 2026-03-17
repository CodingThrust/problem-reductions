---
name: Problem
about: Propose a new problem type
title: "[Model] QuadraticProgramming(*)"
labels: model
assignees: ''
---

## Motivation

QUADRATIC PROGRAMMING (*) (P208) from Garey & Johnson, A6 MP2. A foundational NP-hard optimization problem that bridges linear programming and combinatorial optimization. The decision version asks whether a quadratic objective can exceed a threshold subject to linear constraints. NP-hardness arises even with a single negative eigenvalue in the quadratic form. Not known to be in NP in general (the (*) marker in GJ), unless all quadratic coefficients c_i are non-negative.

**Associated rules:**
- R153: PARTITION to QUADRATIC PROGRAMMING (source reduction establishing NP-hardness)

<!-- ⚠️ Unverified: AI-generated motivation and associated rules -->

## Definition

**Name:** `QuadraticProgramming`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP2

**Mathematical definition:**

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of rational numbers and b is a rational number, two m-tuples c̄ and d̄ of rational numbers, and a rational number B.
QUESTION: Is there an m-tuple ȳ of rational numbers such that x̄·ȳ ≤ b for all (x̄,b) ∈ X and such that Σᵢ₌₁ᵐ (cᵢyᵢ² + dᵢyᵢ) ≥ B, where cᵢ, yᵢ, and dᵢ denote the iᵗʰ components of c̄, ȳ, and d̄ respectively?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** m (one continuous variable per dimension of the decision vector ȳ)
- **Per-variable domain:** Rational numbers (in practice, discretized for brute-force search; e.g., fixed-point rationals over a bounded range)
- **Meaning:** y_i represents the i-th component of the decision vector. The feasible region is the polyhedron {ȳ : x̄·ȳ ≤ b for all (x̄,b) ∈ X}. The question is whether a point in this polyhedron achieves quadratic objective value ≥ B.

**Note:** Since the variables are continuous rationals, this problem does not fit the standard `dims()` framework (which assumes finite discrete domains). Implementation would require either: (a) discretization of the feasible region, (b) restriction to the PARTITION-reduction form where the objective forces binary solutions, or (c) a specialized solver interface. The codebase's `SatisfactionProblem` trait (`Metric = bool`) is appropriate for the decision version.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `QuadraticProgramming`
**Variants:** none (rational arithmetic; no graph or weight type parameters)

| Field            | Type              | Description                                                     |
|------------------|-------------------|-----------------------------------------------------------------|
| `num_vars`       | `usize`           | Number of variables m                                           |
| `constraints`    | `Vec<(Vec<f64>, f64)>` | Linear constraints: each (x̄, b) where x̄·ȳ ≤ b           |
| `quad_coeffs`    | `Vec<f64>`        | Quadratic coefficients c̄ = (c_1, ..., c_m) in objective        |
| `lin_coeffs`     | `Vec<f64>`        | Linear coefficients d̄ = (d_1, ..., d_m) in objective           |
| `threshold`      | `f64`             | Target value B (objective must be ≥ B)                          |

**Getter methods:**
- `num_vars()` → number of variables m
- `num_constraints()` → number of linear constraints |X|

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** For the general (indefinite/non-convex) case, the problem is NP-hard [Sahni, 1974] and not known to be in NP [Klee, 1978]. Exact algorithms use branch-and-bound or spatial branch-and-bound with convex relaxations. For the special case where all c_i ≥ 0 (convex), the problem is in P and solvable in polynomial time via interior-point methods (Kozlov, Tarasov, and Khachian, 1979; Ye and Tse, 1989). The convex case has complexity O(L² · m⁴) where L is the input bit length. For the non-convex decision version arising from PARTITION reduction, the complexity is dominated by the PARTITION problem itself, which has best known exact algorithm O*(2^(n/2)) via Horowitz-Sahni meet-in-the-middle [Horowitz & Sahni, 1974].

## Extra Remark

**Full book text:**

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of rational numbers and b is a rational number, two m-tuples c̄ and d̄ of rational numbers, and a rational number B.
QUESTION: Is there an m-tuple ȳ of rational numbers such that x̄·ȳ ≤ b for all (x̄,b) ∈ X and such that Σᵢ₌₁ᵐ (cᵢyᵢ² + dᵢyᵢ) ≥ B, where cᵢ, yᵢ, and dᵢ denote the iᵗʰ components of c̄, ȳ, and d̄ respectively?

Reference: [Sahni, 1974]. Transformation from PARTITION.
Comment: Not known to be in NP, unless the cᵢ's are all non-negative [Klee, 1978]. If the constraints are quadratic and the objective function is linear (the reverse of the situation above), then the problem is also NP-hard [Sahni, 1974]. If we add to this last problem the requirement that all entries of ȳ be integers, then the problem becomes undecidable [Jeroslow, 1973].

## How to solve

- [ ] It can be solved by (existing) bruteforce. (Requires discretization; not naturally suited to finite-domain enumeration since variables are continuous rationals.)
- [ ] It can be solved by reducing to integer programming. (The PARTITION-sourced instances can be converted to 0-1 ILP by enforcing binary variables, but general QP cannot be directly cast as ILP.)
- [x] Other: For convex instances (all c_i ≥ 0), use interior-point methods. For the non-convex decision version from PARTITION, enumerate binary assignments and check the equality constraint.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**

Variables: m = 4.
Constraints X (defining the feasible polyhedron):

| Constraint | x̄             | b   |
|------------|---------------|-----|
| 1          | (1, 0, 0, 0)  | 1   |
| 2          | (0, 1, 0, 0)  | 1   |
| 3          | (0, 0, 1, 0)  | 1   |
| 4          | (0, 0, 0, 1)  | 1   |
| 5          | (-1, 0, 0, 0) | 0   |
| 6          | (0, -1, 0, 0) | 0   |
| 7          | (0, 0, -1, 0) | 0   |
| 8          | (0, 0, 0, -1) | 0   |

These constrain 0 ≤ y_i ≤ 1 for all i.

Quadratic coefficients: c̄ = (-3, -5, -7, -1) (all negative → indefinite/non-convex).
Linear coefficients: d̄ = (3, 5, 7, 1).
Threshold: B = 0.

Objective: Σ cᵢyᵢ² + dᵢyᵢ = 3y₁(1-y₁) + 5y₂(1-y₂) + 7y₃(1-y₃) + 1·y₄(1-y₄).

This equals 0 at binary points and is positive at non-binary points in [0,1]⁴.

**Feasible assignment:**
ȳ = (1, 0, 1, 0):
- All constraints satisfied: 0 ≤ y_i ≤ 1 ✓
- Objective: 3·1·0 + 5·0·1 + 7·1·0 + 1·0·1 = 0 ≥ B = 0 ✓

Answer: YES — a feasible point meeting the threshold exists.

**Interpretation:** This QP instance encodes whether the set {3, 5, 7, 1} (sum = 16) can be partitioned into two subsets of equal sum 8. Indeed: {3, 5} and {7, 1} both sum to 8.

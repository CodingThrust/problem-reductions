---
name: Problem
about: Propose a new problem type
title: "[Model] CostParametricLinearProgramming"
labels: model
assignees: ''
---

## Motivation

COST-PARAMETRIC LINEAR PROGRAMMING (P209) from Garey & Johnson, A6 MP3. An NP-complete problem arising from first-order error analysis for linear programming: given a feasible LP and a perturbation ball of radius q around the cost vector, can a perturbation be found that shifts the optimal value beyond a specific threshold? This captures the computational difficulty of sensitivity analysis in LP. Remains NP-complete for any fixed q > 0.

**Associated rules:**
- R154: 3SAT to COST-PARAMETRIC LINEAR PROGRAMMING (source reduction establishing NP-completeness)

<!-- ⚠️ Unverified: AI-generated motivation and associated rules -->

## Definition

**Name:** `CostParametricLinearProgramming`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP3

**Mathematical definition:**

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of integers and b is an integer, a set J ⊆ {1,2,...,m}, and a positive rational number q.
QUESTION: Is there an m-tuple c̄ with rational entries such that (c̄·c̄)^½ ≤ q and such that, if Y is the set of all m-tuples ȳ with non-negative rational entries satisfying x̄·ȳ ≥ b for all (x̄,b) ∈ X, then the minimum of Σⱼ∈J cⱼyⱼ over all ȳ ∈ Y exceeds
½ max {|cⱼ|: j ∈ J} + Σⱼ∈J min {0,cⱼ} ?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** m (one rational-valued entry per component of the cost vector c̄ being searched for)
- **Per-variable domain:** Rational numbers (the "variables" here are the cost vector entries c_j, not the LP decision variables y_j). The cost vector is constrained to lie within a Euclidean ball of radius q.
- **Meaning:** c_j is the j-th component of the cost perturbation vector. The question asks whether any c̄ with ||c̄|| ≤ q can drive the LP minimum of Σⱼ∈J cⱼyⱼ above the threshold ½ max|cⱼ| + Σ min{0, cⱼ}. The LP feasible set Y = {ȳ ≥ 0 : x̄·ȳ ≥ b ∀(x̄,b) ∈ X} is data, not decision variables.

**Note:** This is a satisfaction/decision problem (`SatisfactionProblem` with `Metric = bool`). The "variables" being searched are the cost vector components c̄, not the LP variables ȳ. Implementation would require either: (a) discretizing the q-ball and enumerating cost vectors, or (b) encoding the problem as an optimization over the q-ball (which itself is a quadratic constraint). Brute-force enumeration is not straightforward due to continuous domains.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `CostParametricLinearProgramming`
**Variants:** none (integer constraint data, rational cost search space)

| Field            | Type                   | Description                                                          |
|------------------|------------------------|----------------------------------------------------------------------|
| `num_vars`       | `usize`                | Number of LP variables m (dimension of ȳ and c̄)                     |
| `constraints`    | `Vec<(Vec<i64>, i64)>` | Linear constraints: each (x̄, b) where x̄·ȳ ≥ b                     |
| `index_set`      | `Vec<usize>`           | Subset J ⊆ {1,...,m} — indices over which the cost is evaluated      |
| `perturbation_radius` | `f64`             | Positive rational q — Euclidean norm bound on c̄                     |

**Getter methods:**
- `num_vars()` → m, the number of LP variables
- `num_constraints()` → |X|, the number of linear constraints
- `index_set_size()` → |J|

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete [Jeroslow, 1976], remaining so for any fixed q > 0. Standard LP can be solved in polynomial time (Khachian 1979, Karmarkar 1984), but the parametric cost question — searching over all cost vectors in a ball — is NP-complete. The complexity of the parametric version is related to the number of breakpoints (slope changes) in the optimal cost as a function of the cost parameter, which can be exponential in the number of variables in the worst case [Carstensen, 1983; Murty, 1980]. For general instances, exact solution requires enumerating LP bases or vertices, giving worst-case exponential time in m.

## Extra Remark

**Full book text:**

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of integers and b is an integer, a set J ⊆ {1,2,...,m}, and a positive rational number q.
QUESTION: Is there an m-tuple c̄ with rational entries such that (c̄·c̄)^½ ≤ q and such that, if Y is the set of all m-tuples ȳ with non-negative rational entries satisfying x̄·ȳ ≥ b for all (x̄,b) ∈ X, then the minimum of Σⱼ∈J cⱼyⱼ over all ȳ ∈ Y exceeds

½ max {|cⱼ|: j ∈ J} + Σⱼ∈J min {0,cⱼ} ?

Reference: [Jeroslow, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete for any fixed q > 0. The problem arises from first order error analysis for linear programming.

## How to solve

- [ ] It can be solved by (existing) bruteforce. (Continuous cost-vector search space makes naive enumeration infeasible.)
- [ ] It can be solved by reducing to integer programming. (Not directly; the search is over a continuous Euclidean ball, though the LP vertices are discrete.)
- [x] Other: The problem can be approached by: (1) enumerating vertices of the LP feasible region Y, and for each vertex checking whether a cost perturbation in the q-ball makes that vertex optimal with value exceeding the threshold; (2) formulating as a second-order cone program (SOCP) or semidefinite program (SDP) relaxation.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**

Variables: m = 3.
Constraints X (defining feasible region Y = {ȳ ≥ 0 : x̄·ȳ ≥ b}):

| Constraint | x̄          | b   |
|------------|------------|-----|
| 1          | (1, 0, 0)  | 1   |
| 2          | (0, 1, 0)  | 1   |
| 3          | (0, 0, 1)  | 1   |
| 4          | (1, 1, 1)  | 4   |

These require y_1 ≥ 1, y_2 ≥ 1, y_3 ≥ 1, and y_1 + y_2 + y_3 ≥ 4.

Index set: J = {1, 2, 3} (all variables).
Perturbation radius: q = 1.

**Question:** Is there c̄ with ||c̄|| ≤ 1 such that min_{ȳ ∈ Y} (c₁y₁ + c₂y₂ + c₃y₃) exceeds ½ max{|c₁|, |c₂|, |c₃|} + min{0, c₁} + min{0, c₂} + min{0, c₃}?

**Analysis:**

The feasible region Y has a vertex at (1, 1, 2) (and permutations like (1, 2, 1), (2, 1, 1)).

Consider c̄ = (0, 0, -1) with ||c̄|| = 1 ≤ q ✓.
- min_{ȳ ∈ Y} (0·y₁ + 0·y₂ + (-1)·y₃) = -y₃ is minimized by minimizing y₃. With y₁ ≥ 1, y₂ ≥ 1, y₃ ≥ 1, and y₁+y₂+y₃ ≥ 4: set y₁ = y₂ = 1, y₃ = 2 → min value = -2.
- Threshold: ½ max{0, 0, 1} + min{0,0} + min{0,0} + min{0,-1} = ½ · 1 + 0 + 0 + (-1) = -0.5.
- Check: -2 > -0.5? NO.

Consider c̄ = (0, 0, 1) with ||c̄|| = 1 ≤ q ✓.
- min_{ȳ ∈ Y} y₃ is minimized at y₃ = 1 (set y₁ = y₂ = 1.5, y₃ = 1, or y₁ = 2, y₂ = 1, y₃ = 1). Min value = 1.
- Threshold: ½ · 1 + 0 + 0 + 0 = 0.5.
- Check: 1 > 0.5? YES ✓.

Answer: YES — the cost vector c̄ = (0, 0, 1) satisfies all conditions.

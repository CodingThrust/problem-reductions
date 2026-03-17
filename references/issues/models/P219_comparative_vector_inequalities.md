---
name: Problem
about: Propose a new problem type
title: "[Model] ComparativeVectorInequalities"
labels: model
assignees: ''
milestone: 'Garey & Johnson'
---

## Motivation

COMPARATIVE VECTOR INEQUALITIES (P219) from Garey & Johnson, A6 MP13. A classical NP-complete problem in mathematical programming: given two sets X and Y of integer m-tuples, decide whether there exists an integer m-tuple z such that at least as many vectors in X dominate z (componentwise) as vectors in Y dominate z. Introduced by Plaisted (1976), who proved NP-completeness via reduction from COMPARATIVE CONTAINMENT (with equal weights). The problem remains NP-complete even when all components are restricted to {0,1}. It captures a fundamental comparison principle over componentwise vector dominance and connects set-based containment problems to vector-based inequality problems.

<!-- ⚠️ Unverified: AI-generated motivation -->

**Associated reduction rules:**
- As target: R163 (COMPARATIVE CONTAINMENT (with equal weights) to COMPARATIVE VECTOR INEQUALITIES)

## Definition

**Name:** `ComparativeVectorInequalities`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Canonical name:** Comparative Vector Inequalities
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP13

**Mathematical definition:**

INSTANCE: Sets X = {x̄₁,x̄₂,...,x̄ₖ} and Y = {ȳ₁,ȳ₂,...,ȳₗ} of m-tuples of integers.
QUESTION: Is there an m-tuple z̄ of integers such that the number of m-tuples x̄ᵢ satisfying x̄ᵢ ≥ z̄ is at least as large as the number of m-tuples ȳⱼ satisfying ȳⱼ ≥ z̄, where two m-tuples ū and v̄ satisfy ū ≥ v̄ if and only if no component of ū is less than the corresponding component of v̄?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** m (one integer variable per component of the m-tuple z̄)
- **Per-variable domain:** integers (in the {0,1} restricted case, domain is {0, 1})
- **Meaning:** z_j = the j-th component of the candidate m-tuple z̄. The problem asks whether there exists an assignment of z̄ such that |{i : x̄ᵢ ≥ z̄}| ≥ |{j : ȳⱼ ≥ z̄}|, where x̄ᵢ ≥ z̄ means x̄ᵢ[c] ≥ z̄[c] for all components c = 1,...,m.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ComparativeVectorInequalities`
**Variants:** none (components are integers; in the {0,1} case, a specialization)

| Field | Type | Description |
|-------|------|-------------|
| `dimension` | `usize` | Dimension m of each tuple |
| `x_vectors` | `Vec<Vec<i64>>` | Set X: k vectors, each an m-tuple of integers |
| `y_vectors` | `Vec<Vec<i64>>` | Set Y: l vectors, each an m-tuple of integers |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** Brute-force enumeration. In the general integer case, the candidate z̄ can be restricted to values appearing in the input vectors (for each component, only values from the union of x̄ᵢ and ȳⱼ components are relevant thresholds). This gives at most (k + l)^m candidate z̄ vectors. For each candidate, checking dominance takes O((k + l) * m) time. Total: O((k + l)^m * (k + l) * m). In the {0,1} restricted case, there are 2^m candidate z̄ vectors, giving O(2^m * (k + l) * m). No specialized exact algorithm is known beyond this enumeration. The problem is NP-complete (Plaisted, 1976), remaining NP-complete even with {0,1} components (Garey & Johnson).

## Specialization

<!-- ⚠️ Unverified: AI-generated specialization -->

- The {0,1} restricted case (all components in {0,1}) remains NP-complete.
- When m is fixed, the problem is solvable in polynomial time (polynomial in k + l) since the number of candidate z̄ vectors is bounded by (k + l)^m.

## Extra Remark

**Full book text:**

INSTANCE: Sets X = {x̄₁,x̄₂,...,x̄ₖ} and Y = {ȳ₁,ȳ₂,...,ȳₗ} of m-tuples of integers.
QUESTION: Is there an m-tuple z̄ of integers such that the number of m-tuples x̄ᵢ satisfying x̄ᵢ ≥ z̄ is at least as large as the number of m-tuples ȳⱼ satisfying ȳⱼ ≥ z̄, where two m-tuples ū and v̄ satisfy ū ≥ v̄ if and only if no component of ū is less than the corresponding component of v̄?

Reference: [Plaisted, 1976]. Transformation from COMPARATIVE CONTAINMENT (with equal weights).
Comment: Remains NP-complete even if all components of the x̄ᵢ and ȳⱼ are required to belong to {0,1}.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all candidate z̄ vectors — restrict each component to values from the input. For each z̄, count how many x̄ᵢ ≥ z̄ vs ȳⱼ ≥ z̄.)
- [x] It can be solved by reducing to integer programming. (Binary variables for dominance indicators; linear constraints encoding componentwise comparison; objective or feasibility constraint on the count difference.)
- [ ] Other: (TBD)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input ({0,1} restricted case):**
Dimension m = 3

X = { x̄₁ = (1, 0, 1), x̄₂ = (1, 1, 0), x̄₃ = (0, 1, 1), x̄₄ = (1, 1, 1) }  (k = 4)
Y = { ȳ₁ = (1, 0, 0), ȳ₂ = (0, 1, 0), ȳ₃ = (1, 1, 0) }  (l = 3)

**Feasible assignment:**
Choose z̄ = (1, 0, 0).

Check x̄ᵢ ≥ z̄ (componentwise):
- x̄₁ = (1,0,1) ≥ (1,0,0)? 1≥1, 0≥0, 1≥0 → YES
- x̄₂ = (1,1,0) ≥ (1,0,0)? 1≥1, 1≥0, 0≥0 → YES
- x̄₃ = (0,1,1) ≥ (1,0,0)? 0≥1? → NO
- x̄₄ = (1,1,1) ≥ (1,0,0)? 1≥1, 1≥0, 1≥0 → YES
Count of x̄ᵢ dominating z̄: 3

Check ȳⱼ ≥ z̄ (componentwise):
- ȳ₁ = (1,0,0) ≥ (1,0,0)? 1≥1, 0≥0, 0≥0 → YES
- ȳ₂ = (0,1,0) ≥ (1,0,0)? 0≥1? → NO
- ȳ₃ = (1,1,0) ≥ (1,0,0)? 1≥1, 1≥0, 0≥0 → YES
Count of ȳⱼ dominating z̄: 2

Comparison: 3 ≥ 2? YES

Answer: YES — z̄ = (1, 0, 0) witnesses that the X-dominance count meets or exceeds the Y-dominance count.

**Verification that not all z̄ work:**
Try z̄ = (0, 0, 0):
All x̄ᵢ ≥ (0,0,0) → count = 4
All ȳⱼ ≥ (0,0,0) → count = 3
4 ≥ 3? YES (this also works)

Try z̄ = (1, 1, 0):
- x̄₁ = (1,0,1): 0≥1? NO
- x̄₂ = (1,1,0): YES
- x̄₃ = (0,1,1): 0≥1? NO
- x̄₄ = (1,1,1): YES
X-count: 2
- ȳ₁ = (1,0,0): 0≥1? NO
- ȳ₂ = (0,1,0): 0≥1? NO
- ȳ₃ = (1,1,0): YES
Y-count: 1
2 ≥ 1? YES

Try z̄ = (0, 1, 1):
- x̄₁ = (1,0,1): 0≥1? NO
- x̄₂ = (1,1,0): 0≥1? NO
- x̄₃ = (0,1,1): YES
- x̄₄ = (1,1,1): YES
X-count: 2
- ȳ₁ = (1,0,0): 0≥1? NO
- ȳ₂ = (0,1,0): 0≥1? NO
- ȳ₃ = (1,1,0): 0≥1? NO
Y-count: 0
2 ≥ 0? YES

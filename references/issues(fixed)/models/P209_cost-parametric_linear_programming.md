---
name: Problem
about: Propose a new problem type
title: "[Model] CostParametricLinearProgramming"
labels: model
assignees: ''
---

## Motivation

COST-PARAMETRIC LINEAR PROGRAMMING (P209) from Garey & Johnson, A6 MP3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP3

**Mathematical definition:**

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of integers and b is an integer, a set J ⊆ {1,2,...,m}, and a positive rational number q.
QUESTION: Is there an m-tuple c̄ with rational entries such that (c̄·c̄)^½ ≤ q and such that, if Y is the set of all m-tuples ȳ with non-negative rational entries satisfying x̄·ȳ ≥ b for all (x̄,b) ∈ X, then the minimum of Σⱼ∈J cⱼyⱼ over all ȳ ∈ Y exceeds
½ max {|cⱼ|: j ∈ J} + Σⱼ∈J min {0,cⱼ} ?

## Variables

- **Count:** (TBD)
- **Per-variable domain:** (TBD)
- **Meaning:** (TBD)

## Schema (data type)

**Type name:** (TBD)
**Variants:** (TBD)

| Field | Type | Description |
|-------|------|-------------|
| (TBD) | (TBD) | (TBD) |

## Complexity

- **Best known exact algorithm:** (TBD)

## Extra Remark

**Full book text:**

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of integers and b is an integer, a set J ⊆ {1,2,...,m}, and a positive rational number q.
QUESTION: Is there an m-tuple c̄ with rational entries such that (c̄·c̄)^½ ≤ q and such that, if Y is the set of all m-tuples ȳ with non-negative rational entries satisfying x̄·ȳ ≥ b for all (x̄,b) ∈ X, then the minimum of Σⱼ∈J cⱼyⱼ over all ȳ ∈ Y exceeds

½ max {|cⱼ|: j ∈ J} + Σⱼ∈J min {0,cⱼ} ?

Reference: [Jeroslow, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete for any fixed q > 0. The problem arises from first order error analysis for linear programming.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

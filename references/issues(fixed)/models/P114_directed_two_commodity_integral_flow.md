---
name: Problem
about: Propose a new problem type
title: "[Model] DirectedTwoCommodityIntegralFlow"
labels: model
assignees: ''
---

## Motivation

DIRECTED TWO-COMMODITY INTEGRAL FLOW (P114) from Garey & Johnson, A2 ND38. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND38

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s_1, s_2, t_1, and t_2, capacity c(a) ∈ Z^+ for each a ∈ A, requirements R_1,R_2 ∈ Z^+.
QUESTION: Are there two flow functions f_1,f_2: A → Z_0^+ such that
(1) for each a ∈ A, f_1(a)+f_2(a) ≤ c(a),
(2) for each v ∈ V − {s,t} and i ∈ {1,2}, flow f_i is conserved at v, and
(3) for i ∈ {1,2}, the net flow into t_i under flow f_i is at least R_i?

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

INSTANCE: Directed graph G = (V,A), specified vertices s_1, s_2, t_1, and t_2, capacity c(a) ∈ Z^+ for each a ∈ A, requirements R_1,R_2 ∈ Z^+.
QUESTION: Are there two flow functions f_1,f_2: A → Z_0^+ such that
(1) for each a ∈ A, f_1(a)+f_2(a) ≤ c(a),
(2) for each v ∈ V − {s,t} and i ∈ {1,2}, flow f_i is conserved at v, and
(3) for i ∈ {1,2}, the net flow into t_i under flow f_i is at least R_i?
Reference: [Even, Itai, and Shamir, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete even if c(a) = 1 for all a ∈ A and R_1 = 1. Variant in which s_1 = s_2, t_1 = t_2, and arcs can be restricted to carry only one specified commodity is also NP-complete (follows from [Even, Itai, and Shamir, 1976]). Corresponding M-commodity problem with non-integral flows allowed is polynomially equivalent to LINEAR PROGRAMMING for all M ≥ 2 [Itai, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

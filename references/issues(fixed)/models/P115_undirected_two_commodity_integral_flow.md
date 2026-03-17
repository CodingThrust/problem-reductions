---
name: Problem
about: Propose a new problem type
title: "[Model] UndirectedTwoCommodityIntegralFlow"
labels: model
assignees: ''
---

## Motivation

UNDIRECTED TWO-COMMODITY INTEGRAL FLOW (P115) from Garey & Johnson, A2 ND39. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND39

**Mathematical definition:**

INSTANCE: Graph G = (V,E), specified vertices s_1, s_2, t_1, and t_2, a capacity c(e) ∈ Z^+ for each e ∈ E, requirements R_1,R_2 ∈ Z^+.
QUESTION: Are there two flow functions f_1,f_2: {(u,v),(v,u): {u,v} ∈ E} → Z_0^+ such that
(1) for all {u,v} ∈ E and i ∈ {1,2}, either f_i((u,v)) = 0 or f_i((v,u)) = 0,
(2) for each {u,v} ∈ E,
max{f_1((u,v)),f_1((v,u))} + max{f_2((u,v)),f_2((v,u))} ≤ c({u,v}),
(3) for each v ∈ V − {s,t} and i ∈ {1,2}, flow f_i is conserved at v, and
(4) for i ∈ {1,2}, the net flow into t_i under flow f_i is at least R_i?

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

INSTANCE: Graph G = (V,E), specified vertices s_1, s_2, t_1, and t_2, a capacity c(e) ∈ Z^+ for each e ∈ E, requirements R_1,R_2 ∈ Z^+.
QUESTION: Are there two flow functions f_1,f_2: {(u,v),(v,u): {u,v} ∈ E} → Z_0^+ such that
(1) for all {u,v} ∈ E and i ∈ {1,2}, either f_i((u,v)) = 0 or f_i((v,u)) = 0,
(2) for each {u,v} ∈ E,
    max{f_1((u,v)),f_1((v,u))} + max{f_2((u,v)),f_2((v,u))} ≤ c({u,v}),
(3) for each v ∈ V − {s,t} and i ∈ {1,2}, flow f_i is conserved at v, and
(4) for i ∈ {1,2}, the net flow into t_i under flow f_i is at least R_i?
Reference: [Even, Itai, and Shamir, 1976]. Transformation from DIRECTED TWO-COMMODITY INTEGRAL FLOW.
Comment: Remains NP-complete even if c(e) = 1 for all e ∈ E. Solvable in polynomial time if c(e) is even for all e ∈ E. Corresponding problem with non-integral flows allowed can be solved in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

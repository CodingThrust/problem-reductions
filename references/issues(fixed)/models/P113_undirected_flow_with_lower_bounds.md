---
name: Problem
about: Propose a new problem type
title: "[Model] UndirectedFlowWithLowerBounds"
labels: model
assignees: ''
---

## Motivation

UNDIRECTED FLOW WITH LOWER BOUNDS (P113) from Garey & Johnson, A2 ND37. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND37

**Mathematical definition:**

INSTANCE: Graph G = (V,E), specified vertices s and t, capacity c(e) ∈ Z^+ and lower bound l(e) ∈ Z_0^+ for each e ∈ E, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: {(u,v),(v,u): {u,v} ∈ E} → Z_0^+ such that
(1) for all {u,v} ∈ E, either f((u,v)) = 0 or f((v,u)) = 0,
(2) for each e = {u,v} ∈ E, l(e) ≤ max{f((u,v)),f((v,u))} ≤ c(e),
(3) for each v ∈ V − {s,t}, flow is conserved at v, and
(4) the net flow into t is at least R?

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

INSTANCE: Graph G = (V,E), specified vertices s and t, capacity c(e) ∈ Z^+ and lower bound l(e) ∈ Z_0^+ for each e ∈ E, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: {(u,v),(v,u): {u,v} ∈ E} → Z_0^+ such that
(1) for all {u,v} ∈ E, either f((u,v)) = 0 or f((v,u)) = 0,
(2) for each e = {u,v} ∈ E, l(e) ≤ max{f((u,v)),f((v,u))} ≤ c(e),
(3) for each v ∈ V − {s,t}, flow is conserved at v, and
(4) the net flow into t is at least R?
Reference: [Itai, 1977]. Transformation from SATISFIABILITY.
Comment: Problem is NP-complete in the strong sense, even if non-integral flows are allowed. Corresponding problem for directed graphs can be solved in polynomial time, even if we ask that the total flow be R or less rather than R or more [Ford and Fulkerson, 1962] (see also [Lawler, 1976a]). The analogous DIRECTED M-COMMODITY FLOW WITH LOWER BOUNDS problem is polynomially equivalent to LINEAR PROGRAMMING for all M ≥ 2 if non-integral flows are allowed [Itai, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

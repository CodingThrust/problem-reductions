---
name: Problem
about: Propose a new problem type
title: "[Model] PathConstrainedNetworkFlow"
labels: model
assignees: ''
---

## Motivation

PATH CONSTRAINED NETWORK FLOW (P110) from Garey & Johnson, A2 ND34. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND34

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, a capacity c(a) ∈ Z^+ for each a ∈ A, a collection P of directed paths in G, and a requirement R ∈ Z^+.
QUESTION: Is there a function g: P → Z_0^+ such that if f: A → Z_0^+ is the flow function defined by f(a) = Σ_{p ∈ P(a)} g(p), where P(a) ⊆ P is the set of all paths in P containing the arc a, then f is such that
(1) f(a) ≤ c(a) for all a ∈ A,
(2) for each v ∈ V − {s,t}, flow is conserved at v, and
(3) the net flow into t is at least R?

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

INSTANCE: Directed graph G = (V,A), specified vertices s and t, a capacity c(a) ∈ Z^+ for each a ∈ A, a collection P of directed paths in G, and a requirement R ∈ Z^+.
QUESTION: Is there a function g: P → Z_0^+ such that if f: A → Z_0^+ is the flow function defined by f(a) = Σ_{p ∈ P(a)} g(p), where P(a) ⊆ P is the set of all paths in P containing the arc a, then f is such that
(1) f(a) ≤ c(a) for all a ∈ A,
(2) for each v ∈ V − {s,t}, flow is conserved at v, and
(3) the net flow into t is at least R?
Reference: [Prömel, 1978]. Transformation from 3SAT.
Comment: Remains NP-complete even if all c(a) = 1. The corresponding problem with non-integral flows is equivalent to LINEAR PROGRAMMING, but the question of whether the best rational flow fails to exceed the best integral flow is NP-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

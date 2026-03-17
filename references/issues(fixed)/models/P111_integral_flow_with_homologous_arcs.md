---
name: Problem
about: Propose a new problem type
title: "[Model] IntegralFlowWithHomologousArcs"
labels: model
assignees: ''
---

## Motivation

INTEGRAL FLOW WITH HOMOLOGOUS ARCS (P111) from Garey & Johnson, A2 ND35. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND35

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, capacity c(a) ∈ Z^+ for each a ∈ A, requirement R ∈ Z^+, set H ⊆ A × A of "homologous" pairs of arcs.
QUESTION: Is there a flow function f: A → Z_0^+ such that
(1) f(a) ≤ c(a) for all a ∈ A,
(2) for each v ∈ V − {s,t}, flow is conserved at v,
(3) for all pairs <a,a'> ∈ H, f(a) = f(a'), and
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

INSTANCE: Directed graph G = (V,A), specified vertices s and t, capacity c(a) ∈ Z^+ for each a ∈ A, requirement R ∈ Z^+, set H ⊆ A × A of "homologous" pairs of arcs.
QUESTION: Is there a flow function f: A → Z_0^+ such that
(1) f(a) ≤ c(a) for all a ∈ A,
(2) for each v ∈ V − {s,t}, flow is conserved at v,
(3) for all pairs <a,a'> ∈ H, f(a) = f(a'), and
(4) the net flow into t is at least R?
Reference: [Sahni, 1974]. Transformation from 3SAT.
Comment: Remains NP-complete if c(a) = 1 for all a ∈ A (by modifying the construction in [Even, Itai, and Shamir, 1976]). Corresponding problem with non-integral flows is polynomially equivalent to LINEAR PROGRAMMING [Itai, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

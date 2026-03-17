---
name: Problem
about: Propose a new problem type
title: "[Model] IntegralFlowWithMultipliers"
labels: model
assignees: ''
---

## Motivation

INTEGRAL FLOW WITH MULTIPLIERS (P109) from Garey & Johnson, A2 ND33. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND33

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, multiplier h(v) ∈ Z^+ for each v ∈ V − {s,t}, capacity c(a) ∈ Z^+ for each a ∈ A, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: A → Z_0^+ such that
(1) f(a) ≤ c(a) for all a ∈ A,
(2) for each v ∈ V − {s,t}, Σ_{(u,v) ∈ A} h(v)·f((u,v)) = Σ_{(v,u) ∈ A} f((v,u)), and
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

INSTANCE: Directed graph G = (V,A), specified vertices s and t, multiplier h(v) ∈ Z^+ for each v ∈ V − {s,t}, capacity c(a) ∈ Z^+ for each a ∈ A, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: A → Z_0^+ such that
(1) f(a) ≤ c(a) for all a ∈ A,
(2) for each v ∈ V − {s,t}, Σ_{(u,v) ∈ A} h(v)·f((u,v)) = Σ_{(v,u) ∈ A} f((v,u)), and
(3) the net flow into t is at least R?
Reference: [Sahni, 1974]. Transformation from PARTITION.
Comment: Can be solved in polynomial time by standard network flow techniques if h(v) = 1 for all v ∈ V − {s,t}. Corresponding problem with non-integral flows allowed can be solved by linear programming.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

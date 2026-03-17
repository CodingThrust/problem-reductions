---
name: Problem
about: Propose a new problem type
title: "[Model] IntegralFlowWithBundles"
labels: model
assignees: ''
---

## Motivation

INTEGRAL FLOW WITH BUNDLES (P112) from Garey & Johnson, A2 ND36. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND36

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s and t, "bundles" I_1,I_2,···,I_k ⊆ A such that ∪_{1 ≤ j ≤ k} I_j = A, bundle capacities c_1,c_2,···,c_k ∈ Z^+, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: A → Z_0^+ such that
(1) for 1 ≤ j ≤ k, Σ_{a ∈ I_j} f(a) ≤ c_j,
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

INSTANCE: Directed graph G = (V,A), specified vertices s and t, "bundles" I_1,I_2,···,I_k ⊆ A such that ∪_{1 ≤ j ≤ k} I_j = A, bundle capacities c_1,c_2,···,c_k ∈ Z^+, requirement R ∈ Z^+.
QUESTION: Is there a flow function f: A → Z_0^+ such that
(1) for 1 ≤ j ≤ k, Σ_{a ∈ I_j} f(a) ≤ c_j,
(2) for each v ∈ V − {s,t}, flow is conserved at v, and
(3) the net flow into t is at least R?
Reference: [Sahni, 1974]. Transformation from INDEPENDENT SET.
Comment: Remains NP-complete if all capacities are 1 and all bundles have two arcs. Corresponding problem with non-integral flows allowed can be solved by linear programming.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

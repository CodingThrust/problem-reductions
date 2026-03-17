---
name: Problem
about: Propose a new problem type
title: "[Model] DigraphDMorphism"
labels: model
assignees: ''
---

## Motivation

DIGRAPH D-MORPHISM (P64) from Garey & Johnson, A1.4 GT53. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT53

**Mathematical definition:**

INSTANCE: Directed graphs G = (V_1,A_1), H = (V_2,A_2).
QUESTION: Is there a D-morphism from G to H, i.e., a function f: V_1 → V_2 such that for all (u,v) ∈ A_1 either (f(u),f(v)) ∈ A_2 or (f(v),f(u)) ∈ A_2 and such that for all u ∈ V_1 and v' ∈ V_2 if (f(u),v') ∈ A_2 then there exists a v ∈ f^{-1}(v') for which (u,v) ∈ A_1?

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

INSTANCE: Directed graphs G = (V_1,A_1), H = (V_2,A_2).
QUESTION: Is there a D-morphism from G to H, i.e., a function f: V_1 → V_2 such that for all (u,v) ∈ A_1 either (f(u),f(v)) ∈ A_2 or (f(v),f(u)) ∈ A_2 and such that for all u ∈ V_1 and v' ∈ V_2 if (f(u),v') ∈ A_2 then there exists a v ∈ f^{-1}(v') for which (u,v) ∈ A_1?

Reference: [Fraenkel and Yesha, 1977]. Transformation from GRAPH GRUNDY NUMBERING.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

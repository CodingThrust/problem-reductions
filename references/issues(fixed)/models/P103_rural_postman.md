---
name: Problem
about: Propose a new problem type
title: "[Model] RuralPostman"
labels: model
assignees: ''
---

## Motivation

RURAL POSTMAN (P103) from Garey & Johnson, A2 ND27. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND27

**Mathematical definition:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z_0^+ for each e ∈ E, subset E' ⊆ E, bound B ∈ Z^+.
QUESTION: Is there a circuit in G that includes each edge in E' and that has total length no more than B?

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

INSTANCE: Graph G = (V,E), length l(e) ∈ Z_0^+ for each e ∈ E, subset E' ⊆ E, bound B ∈ Z^+.
QUESTION: Is there a circuit in G that includes each edge in E' and that has total length no more than B?
Reference: [Lenstra and Rinnooy Kan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete even if l(e) = 1 for all e ∈ E, as does the corresponding problem for directed graphs.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

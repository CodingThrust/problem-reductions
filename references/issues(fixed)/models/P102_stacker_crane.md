---
name: Problem
about: Propose a new problem type
title: "[Model] StackerCrane"
labels: model
assignees: ''
---

## Motivation

STACKER-CRANE (P102) from Garey & Johnson, A2 ND26. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND26

**Mathematical definition:**

INSTANCE: Mixed graph G = (V,A,E), length l(e) ∈ Z_0^+ for each e ∈ A ∪ E, bound B ∈ Z^+.
QUESTION: Is there a cycle in G that includes each directed edge in A at least once, traversing such edges only in the specified direction, and that has total length no more than B?

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

INSTANCE: Mixed graph G = (V,A,E), length l(e) ∈ Z_0^+ for each e ∈ A ∪ E, bound B ∈ Z^+.
QUESTION: Is there a cycle in G that includes each directed edge in A at least once, traversing such edges only in the specified direction, and that has total length no more than B?
Reference: [Frederickson, Hecht, and Kim, 1978]. Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete even if all edge lengths equal 1. The analogous path problem (with or without specified endpoints) is also NP-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

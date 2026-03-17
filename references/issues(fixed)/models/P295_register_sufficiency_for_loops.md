---
name: Problem
about: Propose a new problem type
title: "[Model] RegisterSufficiencyForLoops"
labels: model
assignees: ''
---

## Motivation

REGISTER SUFFICIENCY FOR LOOPS (P295) from Garey & Johnson, A11 PO3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO3

**Mathematical definition:**

INSTANCE: Set V of loop variables, a loop length N ∈ Z+, for each variable v ∈ V a start time s(v) ∈ Z0+ and a duration l(v) ∈ Z+, and a positive integer K.
QUESTION: Can the loop variables be safely stored in K registers, i.e., is their an assignment f: V→{1,2,...,K} such that if f(v) = f(u) for some u ≠ v ∈ V, then s(u) ≤ s(v) implies s(u) + l(u) ≤ s(v) and s(v) + l(v)(mod N) ≤ s(u)?

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

INSTANCE: Set V of loop variables, a loop length N ∈ Z+, for each variable v ∈ V a start time s(v) ∈ Z0+ and a duration l(v) ∈ Z+, and a positive integer K.
QUESTION: Can the loop variables be safely stored in K registers, i.e., is their an assignment f: V→{1,2,...,K} such that if f(v) = f(u) for some u ≠ v ∈ V, then s(u) ≤ s(v) implies s(u) + l(u) ≤ s(v) and s(v) + l(v)(mod N) ≤ s(u)?
Reference: [Garey, Johnson, Miller, and Papadimitriou, 1978]. Transformation from permutation generation.
Comment: Solvable in polynomial time for any fixed K.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

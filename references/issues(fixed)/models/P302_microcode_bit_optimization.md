---
name: Problem
about: Propose a new problem type
title: "[Model] MicrocodeBitOptimization"
labels: model
assignees: ''
---

## Motivation

MICROCODE BIT OPTIMIZATION (P302) from Garey & Johnson, A11 PO10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO10

**Mathematical definition:**

INSTANCE: Finite set A of "micro-commands," collection C = {C1,C2,...,Cm} of subsets of A called "micro-instructions," and a positive integer K.
QUESTION: Is there a K-bit instruction format for the given micro-instructions, i.e., is there a partition of A into disjoint subsets A1,A2,...,An such that no pair Ai,Cj have more than one element in common and such that ∑i=1n ⌈log2(|Ai|+1)⌉ ≤ K?

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

INSTANCE: Finite set A of "micro-commands," collection C = {C1,C2,...,Cm} of subsets of A called "micro-instructions," and a positive integer K.
QUESTION: Is there a K-bit instruction format for the given micro-instructions, i.e., is there a partition of A into disjoint subsets A1,A2,...,An such that no pair Ai,Cj have more than one element in common and such that ∑i=1n ⌈log2(|Ai|+1)⌉ ≤ K?
Reference: [Robertson, 1978]. Transformation from 3DM.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

---
name: Problem
about: Propose a new problem type
title: "[Model] BinPacking"
labels: model
assignees: ''
---

## Motivation

BIN PACKING (P149) from Garey & Johnson, A4 SR1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR1

**Mathematical definition:**

INSTANCE: Finite set U of items, a size s(u) ∈ Z+ for each u ∈ U, a positive integer bin capacity B, and a positive integer K.
QUESTION: Is there a partition of U into disjoint sets U1,U2,...,UK such that the sum of the sizes of the items in each Ui is B or less?

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

INSTANCE: Finite set U of items, a size s(u) ∈ Z+ for each u ∈ U, a positive integer bin capacity B, and a positive integer K.
QUESTION: Is there a partition of U into disjoint sets U1,U2,...,UK such that the sum of the sizes of the items in each Ui is B or less?
Reference: Transformation from PARTITION, 3-PARTITION.
Comment: NP-complete in the strong sense. NP-complete and solvable in pseudo-polynomial time for each fixed K ≥ 2. Solvable in polynomial time for any fixed B by exhaustive search.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

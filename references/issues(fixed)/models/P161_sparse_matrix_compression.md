---
name: Problem
about: Propose a new problem type
title: "[Model] SparseMatrixCompression"
labels: model
assignees: ''
---

## Motivation

SPARSE MATRIX COMPRESSION (P161) from Garey & Johnson, A4 SR13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR13

**Mathematical definition:**

INSTANCE: An m×n matrix A with entries aij ∈ {0,1}, 1 ≤ i ≤ m, 1 ≤ j ≤ n, and a positive integer K ≤ mn.
QUESTION: Is there a sequence (b1,b2,...,bn+K) of integers bi, each satisfying 0 ≤ bi ≤ m, and a function s: {1,2,...,m} → {1,2,...,K} such that, for 1 ≤ i ≤ m and 1 ≤ j ≤ n, the entry aij = 1 if and only if bs(i)+j−1 = i?

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

INSTANCE: An m×n matrix A with entries aij ∈ {0,1}, 1 ≤ i ≤ m, 1 ≤ j ≤ n, and a positive integer K ≤ mn.
QUESTION: Is there a sequence (b1,b2,...,bn+K) of integers bi, each satisfying 0 ≤ bi ≤ m, and a function s: {1,2,...,m} → {1,2,...,K} such that, for 1 ≤ i ≤ m and 1 ≤ j ≤ n, the entry aij = 1 if and only if bs(i)+j−1 = i?
Reference: [Even, Lichtenstein, and Shiloach, 1977]. Transformation from GRAPH 3-COLORABILITY.
Comment: Remains NP-complete for fixed K = 3.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

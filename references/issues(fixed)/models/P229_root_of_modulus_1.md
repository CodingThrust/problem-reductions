---
name: Problem
about: Propose a new problem type
title: "[Model] RootOfModulus1"
labels: model
assignees: ''
---

## Motivation

ROOT OF MODULUS 1 (P229) from Garey & Johnson, A7 AN10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN10

**Mathematical definition:**

INSTANCE: Ordered pairs (a[i], b[i]), 1 ≤ i ≤ n, of integers, with each b[i] ≥ 0.
QUESTION: Does the polynomial Σ_{i=1}^{n} a[i]·z^{b[i]} have a root on the complex unit circle, i.e., is there a complex number q with |q| = 1 such that Σ_{i=1}^{n} a[i]·q^{b[i]} = 0?

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

INSTANCE: Ordered pairs (a[i], b[i]), 1 ≤ i ≤ n, of integers, with each b[i] ≥ 0.
QUESTION: Does the polynomial Σ_{i=1}^{n} a[i]·z^{b[i]} have a root on the complex unit circle, i.e., is there a complex number q with |q| = 1 such that Σ_{i=1}^{n} a[i]·q^{b[i]} = 0?

Reference: [Plaisted, 1977b]. Transformation from 3SAT.
Comment: Not known to be in NP or co-NP.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

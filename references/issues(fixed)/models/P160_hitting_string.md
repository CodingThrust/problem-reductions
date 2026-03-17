---
name: Problem
about: Propose a new problem type
title: "[Model] HittingString"
labels: model
assignees: ''
---

## Motivation

HITTING STRING (P160) from Garey & Johnson, A4 SR12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR12

**Mathematical definition:**

INSTANCE: Finite set A of strings over {0,1,*}, all having the same length n.
QUESTION: Is there a string x ∈ {0,1}* with |x| = n such that for each string a ∈ A there is some i, 1 ≤ i ≤ n, for which the ith symbol of a and the ith symbol of x are identical?

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

INSTANCE: Finite set A of strings over {0,1,*}, all having the same length n.
QUESTION: Is there a string x ∈ {0,1}* with |x| = n such that for each string a ∈ A there is some i, 1 ≤ i ≤ n, for which the ith symbol of a and the ith symbol of x are identical?
Reference: [Fagin, 1974]. Transformation from 3SAT.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

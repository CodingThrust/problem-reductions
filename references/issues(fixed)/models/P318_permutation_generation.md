---
name: Problem
about: Propose a new problem type
title: "[Model] PermutationGeneration"
labels: model
assignees: ''
---

## Motivation

PERMUTATION GENERATION (P318) from Garey & Johnson, A12 MS6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS6

**Mathematical definition:**

INSTANCE: Permutation σ of the integers {1,2,...,N}, and a sequence S1,S2,...,Sm of subsets of {1,2,...,N}.
QUESTION: Can σ be expressed as a composition σ = σ1σ2 ··· σm, where for each i, 1 ≤ i ≤ m, σi is a permuation of {1,2,...,N} that leaves all elements in {1,2,...,N} − Si fixed?

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

INSTANCE: Permutation σ of the integers {1,2,...,N}, and a sequence S1,S2,...,Sm of subsets of {1,2,...,N}.
QUESTION: Can σ be expressed as a composition σ = σ1σ2 ··· σm, where for each i, 1 ≤ i ≤ m, σi is a permuation of {1,2,...,N} that leaves all elements in {1,2,...,N} − Si fixed?
Reference: [Garey, Johnson, Miller, Papadimitriou, 1978]. Transformation from X3C.
Comment: Solvable in polynomial time for any fixed N.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

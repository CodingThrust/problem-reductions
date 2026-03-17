---
name: Problem
about: Propose a new problem type
title: "[Model] SetSplitting"
labels: model
assignees: ''
---

## Motivation

SET SPLITTING (P131) from Garey & Johnson, A3 SP4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP4

**Mathematical definition:**

INSTANCE: Collection C of subsets of a finite set S.
QUESTION: Is there a partition of S into two subsets S_1 and S_2 such that no subset in C is entirely contained in either S_1 or S_2?

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

INSTANCE: Collection C of subsets of a finite set S.
QUESTION: Is there a partition of S into two subsets S_1 and S_2 such that no subset in C is entirely contained in either S_1 or S_2?
Reference: [Lovasz, 1973]. Transformation from NOT-ALL-EQUAL 3SAT. The problem is also known as HYPERGRAPH 2-COLORABILITY.
Comment: Remains NP-complete even if all c ∈ C have |c| ≤ 3. Solvable in polynomial time if all c ∈ C have |c| ≤ 2 (becomes GRAPH 2-COLORABILITY).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

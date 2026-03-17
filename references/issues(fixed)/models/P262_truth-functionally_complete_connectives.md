---
name: Problem
about: Propose a new problem type
title: "[Model] TruthFunctionallyCompleteConnectives"
labels: model
assignees: ''
---

## Motivation

TRUTH-FUNCTIONALLY COMPLETE CONNECTIVES (P262) from Garey & Johnson, A9 LO10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO10

**Mathematical definition:**

INSTANCE: Set U of variables, collection C of well-formed Boolean expressions over U.
QUESTION: Is C truth-functionally complete, i.e., is there a truth-functionally complete set of logical connectives (unary and binary operators) D={θ_1,θ_2,...,θ_k} such that for each θ_i∈D there is an expression E∈C and a substitution s:U→{a,b} for which s(E)≡aθ_ib or s(E)≡θ_ia (depending on whether θ_i is binary or unary)?

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

INSTANCE: Set U of variables, collection C of well-formed Boolean expressions over U.
QUESTION: Is C truth-functionally complete, i.e., is there a truth-functionally complete set of logical connectives (unary and binary operators) D={θ_1,θ_2,...,θ_k} such that for each θ_i∈D there is an expression E∈C and a substitution s:U→{a,b} for which s(E)≡aθ_ib or s(E)≡θ_ia (depending on whether θ_i is binary or unary)?
Reference: [Statman, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete even if |C|=2.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

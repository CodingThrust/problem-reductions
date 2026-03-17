---
name: Problem
about: Propose a new problem type
title: "[Model] FirstOrderSubsumption"
labels: model
assignees: ''
---

## Motivation

FIRST ORDER SUBSUMPTION (P270) from Garey & Johnson, A9 LO18. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO18

**Mathematical definition:**

INSTANCE: Finite set U of "variable symbols," finite set C of "function symbols," collection E={E_1,E_2,...,E_m} of expressions over U∪C, collection F={F_1,F_2,...,F_n} of expressions over C.
QUESTION: Is there a substitution mapping s that assigns to each u∈U an expression s(u) over C such that, if s(E_i) denotes the result of substituting for each occurrence in E_i of each u∈U the corresponding expression s(u), then {s(E_1),s(E_2),...,s(E_m)} is a subset of {F_1,F_2,...,F_n}?

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

INSTANCE: Finite set U of "variable symbols," finite set C of "function symbols," collection E={E_1,E_2,...,E_m} of expressions over U∪C, collection F={F_1,F_2,...,F_n} of expressions over C.
QUESTION: Is there a substitution mapping s that assigns to each u∈U an expression s(u) over C such that, if s(E_i) denotes the result of substituting for each occurrence in E_i of each u∈U the corresponding expression s(u), then {s(E_1),s(E_2),...,s(E_m)} is a subset of {F_1,F_2,...,F_n}?
Reference: [Baxter, 1976], [Baxter, 1977]. Transformation from 3SAT.
Comment: Remains NP-complete for any fixed n≥3, but is solvable in polynomial time for any fixed m.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

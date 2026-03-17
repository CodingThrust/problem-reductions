---
name: Problem
about: Propose a new problem type
title: "[Model] PartitionIntoPathsOfLength2"
labels: model
assignees: ''
---

## Motivation

PARTITION INTO PATHS OF LENGTH 2 (P10) from Garey & Johnson, Chapter 3, Section 3.3, p.76. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.3, p.76

**Mathematical definition:**

INSTANCE: Graph G = (V,E), with |V| = 3q for a positive integer q.
QUESTION: Is there a partition of V into q disjoint sets V_1, V_2, ..., V_q of three vertices each so that, for each V_t = {v_{t[1]}, v_{t[2]}, v_{t[3]}}, at least two of the three edges {v_{t[1]}, v_{t[2]}}, {v_{t[1]}, v_{t[3]}}, and {v_{t[2]}, v_{t[3]}} belong to E?

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

INSTANCE: Graph G = (V,E), with |V| = 3q for a positive integer q.
QUESTION: Is there a partition of V into q disjoint sets V_1, V_2, ..., V_q of three vertices each so that, for each V_t = {v_{t[1]}, v_{t[2]}, v_{t[3]}}, at least two of the three edges {v_{t[1]}, v_{t[2]}}, {v_{t[1]}, v_{t[3]}}, and {v_{t[2]}, v_{t[3]}} belong to E?

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

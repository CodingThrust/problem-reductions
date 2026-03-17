---
name: Problem
about: Propose a new problem type
title: "[Model] DomaticNumber"
labels: model
assignees: ''
---

## Motivation

DOMATIC NUMBER (P14) from Garey & Johnson, A1.1 GT3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT3

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is the domatic number of G at least K, i.e., can V be partitioned into k ≥ K disjoint sets V_1, V_2, . . . , V_k such that each V_i is a dominating set for G?

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

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is the domatic number of G at least K, i.e., can V be partitioned into k ≥ K disjoint sets V_1, V_2, . . . , V_k such that each V_i is a dominating set for G?
Reference: [Garey, Johnson, and Tarjan, 1976b]. Transformation from 3SAT. The problem is discussed in [Cockayne and Hedetniemi, 1975].
Comment: Remains NP-complete for any fixed K ≥ 3. (The domatic number is always at least 2 unless G contains an isolated vertex.)

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

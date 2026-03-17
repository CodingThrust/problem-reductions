---
name: Problem
about: Propose a new problem type
title: "[Model] ProductionPlanning"
labels: model
assignees: ''
---

## Motivation

PRODUCTION PLANNING (P205) from Garey & Johnson, A5 SS21. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS21

**Mathematical definition:**

INSTANCE: Number n ∈ Z+ of periods, for each period i, 1 ≤ i ≤ n, a demand ri ∈ Z0+, a production capacity ci ∈ Z0+, a production set-up cost bi ∈ Z0+, an incremental production cost coefficient pi ∈ Z0+, and an inventory cost coefficient hi ∈ Z0+, and an overall bound B ∈ Z+.
QUESTION: Do there exist production amounts xi ∈ Z0+ and associated inventory levels Ii = ∑'_{j=1}(xj−rj), 1 ≤ i ≤ n, such that all xi ≤ ci, all Ii ≥ 0, and
∑_{i=1}^{n}(pi·xi + hi·Ii) + ∑_{xi>0} bi ≤ B ?

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

INSTANCE: Number n ∈ Z+ of periods, for each period i, 1 ≤ i ≤ n, a demand ri ∈ Z0+, a production capacity ci ∈ Z0+, a production set-up cost bi ∈ Z0+, an incremental production cost coefficient pi ∈ Z0+, and an inventory cost coefficient hi ∈ Z0+, and an overall bound B ∈ Z+.
QUESTION: Do there exist production amounts xi ∈ Z0+ and associated inventory levels Ii = ∑'_{j=1}(xj−rj), 1 ≤ i ≤ n, such that all xi ≤ ci, all Ii ≥ 0, and

∑_{i=1}^{n}(pi·xi + hi·Ii) + ∑_{xi>0} bi ≤ B ?

Reference: [Lenstra, Rinnooy Kan, and Florian, 1978]. Transformation from PARTITION.

Comment: Solvable in pseudo-polynomial time, but remains NP-complete even if all demands are equal, all set-up costs are equal, and all inventory costs are 0. If all capacities are equal, the problem can be solved in polynomial time [Florian and Klein, 1971]. The cited algorithms can be generalized to allow for arbitrary monotone non-decreasing concave cost functions, if these can be computed in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

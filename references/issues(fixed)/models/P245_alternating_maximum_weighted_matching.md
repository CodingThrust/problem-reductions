---
name: Problem
about: Propose a new problem type
title: "[Model] AlternatingMaximumWeightedMatching"
labels: model
assignees: ''
---

## Motivation

ALTERNATING MAXIMUM WEIGHTED MATCHING (P245) from Garey & Johnson, A8 GP8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP8

**Mathematical definition:**

INSTANCE: Graph G = (V,E), a weight w(e) ∈ Z+ for each e ∈ E, and a bound B ∈ Z+.
QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a new edge from E, subject to the constraint that no edge can share an endpoint with any of the already chosen edges. If the sum of the weights of the edges chosen ever exceeds B, player 1 wins.

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

INSTANCE: Graph G = (V,E), a weight w(e) ∈ Z+ for each e ∈ E, and a bound B ∈ Z+.
QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a new edge from E, subject to the constraint that no edge can share an endpoint with any of the already chosen edges. If the sum of the weights of the edges chosen ever exceeds B, player 1 wins.

Reference: [Dobkin and Ladner, 1978]. Transformation from QBF.
Comment: PSPACE-complete, even though the corresponding weighted matching problem can be solved in polynomial time (e.g., see [Lawler, 1976a]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

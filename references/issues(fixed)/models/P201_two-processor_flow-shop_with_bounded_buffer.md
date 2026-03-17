---
name: Problem
about: Propose a new problem type
title: "[Model] TwoProcessorFlowShopWithBoundedBuffer"
labels: model
assignees: ''
---

## Motivation

TWO-PROCESSOR FLOW-SHOP WITH BOUNDED BUFFER (P201) from Garey & Johnson, A5 SS17. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS17

**Mathematical definition:**

INSTANCE: (Same as for FLOW-SHOP SCHEDULING with m = 2, with the addition of a "buffer bound" B ∈ Z0+.)
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and such that, for all u ≥ 0, the number of jobs j ∈ J for which both σ1(j) + l(t1[j]) ≤ u and σ2(j) > u does not exceed B?

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

INSTANCE: (Same as for FLOW-SHOP SCHEDULING with m = 2, with the addition of a "buffer bound" B ∈ Z0+.)
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and such that, for all u ≥ 0, the number of jobs j ∈ J for which both σ1(j) + l(t1[j]) ≤ u and σ2(j) > u does not exceed B?

Reference: [Papadimitriou and Kanellakis, 1978]. Transformation from NUMERICAL 3-DIMENSIONAL MATCHING.

Comment: NP-complete in the strong sense for any fixed B, 1 ≤ B < ∞. Solvable in polynomial time if B = 0 [Gilmore and Gomory, 1964] or if B ≥ |J|-1 [Johnson, 1954].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

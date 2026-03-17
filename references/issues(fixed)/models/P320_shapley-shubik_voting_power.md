---
name: Problem
about: Propose a new problem type
title: "[Model] ShapleyShubikVotingPower"
labels: model
assignees: ''
---

## Motivation

SHAPLEY-SHUBIK VOTING POWER (P320) from Garey & Johnson, A12 MS8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS8

**Mathematical definition:**

INSTANCE: Ordered set V = {v1,v2,...,vn} of voters, number of votes wi ∈ Z+ for each vi ∈ V, and a quota q ∈ Z+.
QUESTION: Does voter v1 have non-zero "Shapley-Shubik voting power," where the voting power p(v) for a voter v ∈ V is defined to be (1/n!) times the number of permutations π of {1,2,...,n} for which ∑i=1j−1 wπ(i) < q, ∑i=1j wπ(i) ≥ q, and v = vπ(j)?

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

INSTANCE: Ordered set V = {v1,v2,...,vn} of voters, number of votes wi ∈ Z+ for each vi ∈ V, and a quota q ∈ Z+.
QUESTION: Does voter v1 have non-zero "Shapley-Shubik voting power," where the voting power p(v) for a voter v ∈ V is defined to be (1/n!) times the number of permutations π of {1,2,...,n} for which ∑i=1j−1 wπ(i) < q, ∑i=1j wπ(i) ≥ q, and v = vπ(j)?
Reference: [Garey and Johnson, ——]. Transformation from PARTITION. The definition of voting power is from [Shapley and Shubik, 1954].
Comment: Determining the value of the Shapley-Shubik voting power for a given voter is #P-complete, but that value can be computed in pseudo-polynomial time by dynamic programming.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

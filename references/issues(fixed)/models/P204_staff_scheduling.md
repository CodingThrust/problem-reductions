---
name: Problem
about: Propose a new problem type
title: "[Model] StaffScheduling"
labels: model
assignees: ''
---

## Motivation

STAFF SCHEDULING (P204) from Garey & Johnson, A5 SS20. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS20

**Mathematical definition:**

INSTANCE: Positive integers m and k, a collection C of m-tuples, each having k 1's and m - k 0's (representing possible worker schedules), a "requirement" m-tuple R̄ of non-negative integers, and a number n of workers.
QUESTION: Is there a schedule f: C→Z0+ such that ∑_{c̄ ∈ C} f(c̄) ≤ n and such that ∑_{c̄ ∈ C} f(c̄)·c̄ ≥ R̄?

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

INSTANCE: Positive integers m and k, a collection C of m-tuples, each having k 1's and m - k 0's (representing possible worker schedules), a "requirement" m-tuple R̄ of non-negative integers, and a number n of workers.
QUESTION: Is there a schedule f: C→Z0+ such that ∑_{c̄ ∈ C} f(c̄) ≤ n and such that ∑_{c̄ ∈ C} f(c̄)·c̄ ≥ R̄?

Reference: [Garey and Johnson, ——] Transformation from X3C.

Comment: Solvable in polynomial time if every c̄ ∈ C has the cyclic one's property, i.e., has all its 1's occuring in consecutive positions with position 1 regarded as following position m [Bartholdi, Orlin, and Ratliff, 1977]. (This corresponds to workers who are available only for consecutive hours of the day, or days of the week.)

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

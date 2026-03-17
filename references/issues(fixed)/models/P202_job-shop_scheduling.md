---
name: Problem
about: Propose a new problem type
title: "[Model] JobShopScheduling"
labels: model
assignees: ''
---

## Motivation

JOB-SHOP SCHEDULING (P202) from Garey & Johnson, A5 SS18. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS18

**Mathematical definition:**

INSTANCE: Number m ∈ Z+ of processors, set J of jobs, each j ∈ J consisting of an ordered collection of tasks tk[j], 1 ≤ k ≤ nj, for each such task t a length l(t) ∈ Z0+ and a processor p(t) ∈ {1,2,...,m}, where p(tk[j]) ≠ p(tk+1[j]) for all j ∈ J and 1 ≤ k < nj, and a deadline D ∈ Z+.
QUESTION: Is there a job-shop schedule for J that meets the overall deadline, i.e., a collection of one-processor schedules σi mapping {t: p(t) = i} into Z0+, 1 ≤ i ≤ m, such that σi(t) > σi(t') implies σi(t) ≥ σi(t') + l(t), such that σ(tk+1[j]) ≥ σ(tk[j]) + l(tk[j]) (where the appropriate subscripts are to be assumed on σ) for all j ∈ J and 1 ≤ k < nj, and such that for all j ∈ J σ(tn_j[j]) + l(tn_j[j]) ≤ D (again assuming the appropriate subscript on σ)?

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

INSTANCE: Number m ∈ Z+ of processors, set J of jobs, each j ∈ J consisting of an ordered collection of tasks tk[j], 1 ≤ k ≤ nj, for each such task t a length l(t) ∈ Z0+ and a processor p(t) ∈ {1,2,...,m}, where p(tk[j]) ≠ p(tk+1[j]) for all j ∈ J and 1 ≤ k < nj, and a deadline D ∈ Z+.
QUESTION: Is there a job-shop schedule for J that meets the overall deadline, i.e., a collection of one-processor schedules σi mapping {t: p(t) = i} into Z0+, 1 ≤ i ≤ m, such that σi(t) > σi(t') implies σi(t) ≥ σi(t') + l(t), such that σ(tk+1[j]) ≥ σ(tk[j]) + l(tk[j]) (where the appropriate subscripts are to be assumed on σ) for all j ∈ J and 1 ≤ k < nj, and such that for all j ∈ J σ(tn_j[j]) + l(tn_j[j]) ≤ D (again assuming the appropriate subscript on σ)?

Reference: [Garey, Johnson, and Sethi, 1976]. Transformation from 3-PARTITION.

Comment: NP-complete in the strong sense for m = 2. Can be solved in polynomial time if m = 2 and nj ≤ 2 for all j ∈ J [Jackson, 1956]. NP-complete (in the ordinary sense) if m = 2 and nj ≤ 3 for all j ∈ J, or if m = 3 and nj ≤ 2 for all j ∈ J [Gonzalez and Sahni, 1978a]. All the above results continue to hold if "preemptive" schedules are allowed [Gonzalez and Sahni, 1978a]. If in the nonpreemptive case all tasks have the same length, the problem is NP-complete for m = 3 and open for m = 2 [Lenstra and Rinnooy Kan, 1978b].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumTardinessSequencing"
labels: model
assignees: ''
---

## Motivation

MINIMUM TARDINESS SEQUENCING (P7) from Garey & Johnson, Chapter 3, Section 3.2.3, p.73. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.2.3, p.73

**Mathematical definition:**

INSTANCE: A set T of "tasks," each t ∈ T having "length" 1 and a "deadline" d(t) ∈ Z+, a partial order < on T, and a non-negative integer K ≤ |T|.
QUESTION: Is there a "schedule" σ: T → {0,1,...,|T|−1} such that σ(t) ≠ σ(t') whenever t ≠ t', such that σ(t) < σ(t') whenever t < t', and such that |{t ∈ T: σ(t)+1 > d(t)}| ≤ K?

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

INSTANCE: A set T of "tasks," each t ∈ T having "length" 1 and a "deadline" d(t) ∈ Z+, a partial order < on T, and a non-negative integer K ≤ |T|.
QUESTION: Is there a "schedule" σ: T → {0,1,...,|T|−1} such that σ(t) ≠ σ(t') whenever t ≠ t', such that σ(t) < σ(t') whenever t < t', and such that |{t ∈ T: σ(t)+1 > d(t)}| ≤ K?

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

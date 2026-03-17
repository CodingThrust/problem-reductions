---
name: Problem
about: Propose a new problem type
title: "[Model] PathDistinguishers"
labels: model
assignees: ''
---

## Motivation

PATH DISTINGUISHERS (P71) from Garey & Johnson, A1.5 GT60. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT60

**Mathematical definition:**

INSTANCE: Acyclic directed graph G = (V,A), specified vertices s,t ∈ V, positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≤ K such that, for any pair p_1,p_2 of paths from s to t in G, there is some arc in A' that is in one of p_1 and p_2 but not both?

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

INSTANCE: Acyclic directed graph G = (V,A), specified vertices s,t ∈ V, positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≤ K such that, for any pair p_1,p_2 of paths from s to t in G, there is some arc in A' that is in one of p_1 and p_2 but not both?

Reference: [Maheshwari, 1976]. Transformation from VERTEX COVER.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

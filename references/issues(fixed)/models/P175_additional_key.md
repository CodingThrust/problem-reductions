---
name: Problem
about: Propose a new problem type
title: "[Model] AdditionalKey"
labels: model
assignees: ''
---

## Motivation

ADDITIONAL KEY (P175) from Garey & Johnson, A4 SR27. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR27

**Mathematical definition:**

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, a subset R ⊆ A, and a set K of keys for the relational scheme <R,F>.
QUESTION: Does R have a key not already contained in K, i.e., is there an R' ⊆ R such that R' ∉ K, (R',R) ∈ F*, and for no R'' ⊆ R' is (R'',R) ∈ F*?

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

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, a subset R ⊆ A, and a set K of keys for the relational scheme <R,F>.
QUESTION: Does R have a key not already contained in K, i.e., is there an R' ⊆ R such that R' ∉ K, (R',R) ∈ F*, and for no R'' ⊆ R' is (R'',R) ∈ F*?
Reference: [Beeri and Bernstein, 1978]. Transformation from HITTING SET.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

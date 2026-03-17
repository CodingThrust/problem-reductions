---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumCardinalityKey"
labels: model
assignees: ''
---

## Motivation

MINIMUM CARDINALITY KEY (P174) from Garey & Johnson, A4 SR26. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR26

**Mathematical definition:**

INSTANCE: A set A of "attribute names," a collection F of ordered pairs of subsets of A (called "functional dependencies" on A), and a positive integer M.
QUESTION: Is there a key of cardinality M or less for the relational system <A,F>, i.e., a minimal subset K ⊆ A with |K| ≤ M such that the ordered pair (K,A) belongs to the "closure" F* of F defined by (1) F ⊆ F*, (2) B ⊆ C ⊆ A implies (C,B) ∈ F*, (3) (B,C),(C,D) ∈ F* implies (B,D) ∈ F*, and (4) (B,C),(B,D) ∈ F* implies (B,C ∪ D) ∈ F*?

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

INSTANCE: A set A of "attribute names," a collection F of ordered pairs of subsets of A (called "functional dependencies" on A), and a positive integer M.
QUESTION: Is there a key of cardinality M or less for the relational system <A,F>, i.e., a minimal subset K ⊆ A with |K| ≤ M such that the ordered pair (K,A) belongs to the "closure" F* of F defined by (1) F ⊆ F*, (2) B ⊆ C ⊆ A implies (C,B) ∈ F*, (3) (B,C),(C,D) ∈ F* implies (B,D) ∈ F*, and (4) (B,C),(B,D) ∈ F* implies (B,C ∪ D) ∈ F*?
Reference: [Lucchesi and Osborne, 1977], [Lipsky, 1977a]. Transformation from VERTEX COVER. See [Date, 1975] for general background on relational data bases.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)

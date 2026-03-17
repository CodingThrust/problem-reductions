---
name: Rule
about: Propose a new reduction rule
title: "[Rule] EXACT COVER BY 3-SETS (X3C) to MINIMUM COVER"
labels: rule
assignees: ''
---

**Source:** EXACT COVER BY 3-SETS (X3C)
**Target:** MINIMUM COVER
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.2.1, p.64

## Reduction Algorithm

> (1) MINIMUM COVER
> INSTANCE: Collection C of subsets of a set S, positive integer K.
> QUESTION: Does C contain a cover for S of size K or less, that is, a subset C' ⊆ C with |C'| <= K and such that U_{c E C'} c = S?
>
> Proof: Restrict to X3C by allowing only instances having |c|=3 for all c E C and having K = |S|/3.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

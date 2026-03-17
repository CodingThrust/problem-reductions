---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to HITTING SET"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** HITTING SET
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.2.1, p.64

## Reduction Algorithm

> (2) HITTING SET
> INSTANCE: Collection C of subsets of a set S, positive integer K.
> QUESTION: Does S contain a hitting set for C of size K or less, that is, a subset S' ⊆ S with |S'| <= K and such that S' contains at least one element from each subset in C?
>
> Proof: Restrict to VC by allowing only instances having |c|=2 for all c E C.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

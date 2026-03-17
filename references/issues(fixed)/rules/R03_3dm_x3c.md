---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-DIMENSIONAL MATCHING (3DM) to EXACT COVER BY 3-SETS (X3C)"
labels: rule
assignees: ''
---

**Source:** 3-DIMENSIONAL MATCHING (3DM)
**Target:** EXACT COVER BY 3-SETS (X3C)
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.1.2, p.53

## Reduction Algorithm

> In proving NP-completeness results, the following slightly simpler and more general version of 3DM can often be used in its place:
>
> EXACT COVER BY 3-SETS (X3C)
> INSTANCE: A finite set X with |X|=3q and a collection C of 3-element subsets of X.
> QUESTION: Does C contain an exact cover for X, that is, a subcollection C' ⊆ C such that every element of X occurs in exactly one member of C'?
>
> Note that every instance of 3DM can be viewed as an instance of X3C, simply by regarding it as an unordered subset of W U X U Y, and the matchings for that 3DM instance will be in one-to-one correspondence with the exact covers for the X3C instance. Thus 3DM is just a restricted version of X3C, and the NP-completeness of X3C follows by a trivial transformation from 3DM.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to SET BASIS"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** SET BASIS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SP7, p.222

## GJ Source Entry

> [SP7] SET BASIS
> INSTANCE: Collection C of subsets of a finite set S, positive integer K≤|C|.
> QUESTION: Is there a collection B of subsets of S with |B|=K such that, for each c∈C, there is a subcollection of B whose union is exactly c?
> Reference: [Stockmeyer, 1975]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete if all c∈C have |c|≤3, but is trivial if all c∈C have |c|≤2.

## Reduction Algorithm

(TBD)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Stockmeyer, 1975]**: [`Stockmeyer1975`] Larry J. Stockmeyer (1975). "The set basis problem is {NP}-complete". IBM Research Center.
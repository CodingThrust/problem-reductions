---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to CYCLIC ORDERING"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** CYCLIC ORDERING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS2

## GJ Source Entry

> [MS2]  CYCLIC ORDERING
> INSTANCE:  Finite set A, collection C of ordered triples (a,b,c) of distinct elements from A.
> QUESTION:  Is there a one-to-one function f: A→{1,2,...,|A|} such that, for each (a,b,c) ∈ A, we have either f(a) < f(b) < f(c) or f(b) < f(c) < f(a) or f(c) < f(a) < f(b)?
> Reference:  [Galil and Megiddo, 1977]. Transformation from 3SAT.

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

- **[Galil and Megiddo, 1977]**: [`Galil1977b`] Z. Galil and N. Megiddo (1977). "Cyclic ordering is {NP}-complete". *Theoretical Computer Science* 5, pp. 179–182.
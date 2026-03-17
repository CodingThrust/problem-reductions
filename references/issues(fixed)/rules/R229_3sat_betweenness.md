---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SET SPLITTING to BETWEENNESS"
labels: rule
assignees: ''
---

**Source:** SET SPLITTING
**Target:** BETWEENNESS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS1

## GJ Source Entry

> [MS1]  BETWEENNESS
> INSTANCE:  Finite set A, collection C of ordered triples (a,b,c) of distinct elements from A.
> QUESTION:  Is there a one-to-one function f: A→{1,2,...,|A|} such that for each (a,b,c) ∈ C, we have either f(a) < f(b) < f(c) or f(c) < f(b) < f(a)?
> Reference:  [Opatrný, 1978]. Transformation from SET SPLITTING.

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

- **[Opatrný, 1978]**: [`Opatrny1978`] J. Opatrn{\'y} (1978). "Total ordering problem".
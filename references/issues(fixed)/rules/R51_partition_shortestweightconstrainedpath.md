---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to SHORTEST WEIGHT-CONSTRAINED PATH"
labels: rule
assignees: ''
---

**Source:** PARTITION
**Target:** SHORTEST WEIGHT-CONSTRAINED PATH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND30, p.214

## GJ Source Entry

> [ND30] SHORTEST WEIGHT-CONSTRAINED PATH
> INSTANCE: Graph G=(V,E), length l(e)∈Z^+, and weight w(e)∈Z^+ for each e∈E, specified vertices s,t∈V, positive integers K,W.
> QUESTION: Is there a simple path in G from s to t with total weight W or less and total length K or less?
> Reference: [Megiddo, 1977]. Transformation from PARTITION.
> Comment: Also NP-complete for directed graphs. Both problems are solvable in polynomial time if all weights are equal or all lengths are equal.

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

- **[Megiddo, 1977]**: [`Megiddo1977`] Nimrod Megiddo (1977). "".
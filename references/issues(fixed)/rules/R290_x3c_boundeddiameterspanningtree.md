---
name: Rule
about: Propose a new reduction rule
title: "[Rule] EXACT COVER BY 3-SETS to BOUNDED DIAMETER SPANNING TREE"
labels: rule
assignees: ''
---

**Source:** EXACT COVER BY 3-SETS
**Target:** BOUNDED DIAMETER SPANNING TREE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2.1 ND4

## GJ Source Entry

> [ND4] BOUNDED DIAMETER SPANNING TREE
> INSTANCE: Graph G = (V,E), weight w(e) ∈ Z+ for each e ∈ E, positive integer D ≤ |V|, positive integer B.
> QUESTION: Is there a spanning tree T for G such that the sum of the weights of the edges in T does not exceed B and such that T contains no simple path with more than D edges?
> Reference: [Garey and Johnson, ——]. Transformation from EXACT COVER BY 3-SETS.
> Comment: Remains NP-complete for any fixed D ≥ 4, even if all edge weights are either 1 or 2. Can be solved easily in polynomial time if D ≤ 3, or if all edge weights are equal.

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

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to RANDOMIZATION TEST FOR MATCHED PAIRS"
labels: rule
assignees: ''
---

**Source:** PARTITION
**Target:** RANDOMIZATION TEST FOR MATCHED PAIRS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS10

## GJ Source Entry

> [MS10]  RANDOMIZATION TEST FOR MATCHED PAIRS (*)
> INSTANCE:  Sequence (x1,y1),(x2,y2),...,(xn,yn) of ordered pairs of integers, nonnegative integer K.
> QUESTION:  Are there at least K subsets S ⊆ {1,2,...,n} for which
> ∑i∈S |xi−yi| ≤ ∑xi>yi (xi−yi) ?
> Reference:  [Shamos, 1976]. Transformation from PARTITION.
> Comment:  Not known to be in NP. The corresponding enumeration problem is #P-complete, but solvable in pseudo-polynomial time by dynamic programming.

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

- **[Shamos, 1976]**: [`Shamos1976`] Michael I. Shamos (1976). "Geometry and statistics: problems at the interface". In: *Algorithms and Complexity: New Directions and Recent Results*. Academic Press.
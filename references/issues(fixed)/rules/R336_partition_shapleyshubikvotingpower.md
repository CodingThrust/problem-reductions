---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to SHAPLEY-SHUBIK VOTING POWER"
labels: rule
assignees: ''
---

**Source:** PARTITION
**Target:** SHAPLEY-SHUBIK VOTING POWER
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS8

## GJ Source Entry

> [MS8]  SHAPLEY-SHUBIK VOTING POWER
> INSTANCE:  Ordered set V = {v1,v2,...,vn} of voters, number of votes wi ∈ Z+ for each vi ∈ V, and a quota q ∈ Z+.
> QUESTION:  Does voter v1 have non-zero "Shapley-Shubik voting power," where the voting power p(v) for a voter v ∈ V is defined to be (1/n!) times the number of permutations π of {1,2,...,n} for which ∑i=1j−1 wπ(i) < q, ∑i=1j wπ(i) ≥ q, and v = vπ(j)?
> Reference:  [Garey and Johnson, ——]. Transformation from PARTITION. The definition of voting power is from [Shapley and Shubik, 1954].
> Comment:  Determining the value of the Shapley-Shubik voting power for a given voter is #P-complete, but that value can be computed in pseudo-polynomial time by dynamic programming.

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
- **[Shapley and Shubik, 1954]**: [`Shapley and Shubik1954`] Lloyd S. Shapley and Martin Shubik (1954). "A method of evaluating the distribution of power in a committee system". *American Political Science Review* 48, pp. 787–792.
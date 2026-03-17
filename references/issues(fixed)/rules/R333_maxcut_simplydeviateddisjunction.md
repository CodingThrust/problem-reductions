---
name: Rule
about: Propose a new reduction rule
title: "[Rule] MAX CUT to SIMPLY DEVIATED DISJUNCTION"
labels: rule
assignees: ''
---

**Source:** MAX CUT
**Target:** SIMPLY DEVIATED DISJUNCTION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS14

## GJ Source Entry

> [MS14]  SIMPLY DEVIATED DISJUNCTION
> INSTANCE:  Collection M of m-tuples (Mi[1],Mi[2],...,Mi[m]), 1 ≤ i ≤ n, with each Mi[j] being either 0,1, or x.
> QUESTION:  Is there a partition of {1,2,...,m} into disjoint sets I,J and an assignment f: {1,2,...,m} → {0,1} such that, if Φ is the formula ⋁j∈I (M[j] = f(j)) and Ψ is the formula ⋁j∈J (M[j] = f(j)), then Φ and Ψ are simply deviated in M, i.e., the number of Mi ∈ M such that Φ and Ψ are both true for Mi times the number of Mi ∈ M such that Φ and Ψ are both false for Mi is larger than the number of Mi ∈ M such that Φ is true and Ψ is false for Mi times the number of Mi ∈ M such that Φ is false and Ψ is true for Mi? (The definition of "simply deviated" is from [Havránek, 1975].)
> Reference:  [Pudlák and Springsteel, 1975]. Transformation from MAX CUT.
> Comment:  Remains NP-complete even if f(j) = 1 for 1 ≤ j ≤ m. Solvable in polynomial time if each Mi[j] is either 0 or 1. See reference for additional related results.

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

- **[Havránek, 1975]**: [`Havranek1975`] T. Havr\'{a}nek (1975). "Statistical quantifiers in observational calculi: an application in {GUHA} methods". *Theory and Decision* 6, pp. 213–230.
- **[Pudlák and Springsteel, 1975]**: [`Pudlak1975b`] P. Pudl{\'a}k and F. N. Springsteel (1975). "Complexity in mechanized hypothesis formation".
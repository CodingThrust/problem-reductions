---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-DIMENSIONAL MATCHING to 3-MATROID INTERSECTION"
labels: rule
assignees: ''
---

**Source:** 3-DIMENSIONAL MATCHING
**Target:** 3-MATROID INTERSECTION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SP11, p.223

## GJ Source Entry

> [SP11] 3-MATROID INTERSECTION
> INSTANCE: Three matroids (E,F_1),(E,F_2),(E,F_3), positive integer K≤|E|. (A matroid (E,F) consists of a set E of elements and a non-empty family F of subsets of E such that (1) S∈F implies all subsets of S are in F and (2) if two sets S,S'∈F satisfy |S|=|S'|+1, then there exists an element e∈S−S' such that (S'∪{e})∈F.)
> QUESTION: Is there a subset E'⊆E such that |E'|=K and E'∈(F_1∩F_2∩F_3)?
> Reference: Transformation from 3DM.
> Comment: The related 2-MATROID INTERSECTION problem can be solved in polynomial time, even if the matroids are described by giving polynomial time algorithms for recognizing their members, and even if each element e∈E has a weight w(e)∈Z^+, with the goal being to find an E'∈(F_1∩F_2) having maximum total weight (e.g., see [Lawler, 1976a]).

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

- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.
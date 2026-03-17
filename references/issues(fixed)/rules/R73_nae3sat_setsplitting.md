---
name: Rule
about: Propose a new reduction rule
title: "[Rule] NOT-ALL-EQUAL 3SAT to SET SPLITTING"
labels: rule
assignees: ''
---

**Source:** NOT-ALL-EQUAL 3SAT
**Target:** SET SPLITTING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SP4, p.221

## GJ Source Entry

> [SP4] SET SPLITTING
> INSTANCE: Collection C of subsets of a finite set S.
> QUESTION: Is there a partition of S into two subsets S_1 and S_2 such that no subset in C is entirely contained in either S_1 or S_2?
> Reference: [Lovasz, 1973]. Transformation from NOT-ALL-EQUAL 3SAT. The problem is also known as HYPERGRAPH 2-COLORABILITY.
> Comment: Remains NP-complete even if all c∈C have |c|≤3. Solvable in polynomial time if all c∈C have |c|≤2 (becomes GRAPH 2-COLORABILITY).

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

- **[Lovasz, 1973]**: [`Lovasz1973`] Laszlo Lovasz (1973). "Coverings and colorings of hypergraphs". In: *Proceedings of the 4th Southeastern Conference on Combinatorics, Graph Theory, and Computing*, pp. 3–12. Utilitas Mathematica Publishing.
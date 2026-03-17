---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to MINIMUM SUM OF SQUARES"
labels: rule
assignees: ''
---

**Source:** PARTITION
**Target:** MINIMUM SUM OF SQUARES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SP19, p.225

## GJ Source Entry

> [SP19] MINIMUM SUM OF SQUARES
> INSTANCE: Finite set A, a size s(a)∈Z^+ for each a∈A, positive integers K≤|A| and J.
> QUESTION: Can A be partitioned into K disjoint sets A_1,A_2,…,A_K such that
> Σ_{i=1}^{K}(Σ_{a∈A_i} s(a))^2 ≤ J ?
> Reference: Transformation from PARTITION or 3-PARTITION.
> Comment: NP-complete in the strong sense. NP-complete in the ordinary sense and solvable in pseudo-polynomial time for any fixed K. Variants in which the bound K on the number of sets is replaced by a bound B on either the maximum set cardinality or the maximum total set size are also NP-complete in the strong sense [Wong and Yao, 1976]. In all these cases, NP-completeness is preserved if the exponent 2 is replaced by any fixed rational α>1.

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

- **[Wong and Yao, 1976]**: [`Wong and Yao1976`] C. K. Wong and A. C. Yao (1976). "A combinatorial optimization problem related to data set allocation". *Revue Francaise d'Automatique, Informatique, Recherche Operationnelle Ser. Bleue* 10(suppl.), pp. 83–95.
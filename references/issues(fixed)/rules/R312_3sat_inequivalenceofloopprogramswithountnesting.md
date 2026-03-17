---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to INEQUIVALENCE OF LOOP PROGRAMS WITHOUT NESTING"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** INEQUIVALENCE OF LOOP PROGRAMS WITHOUT NESTING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO14

## GJ Source Entry

> [PO14]  INEQUIVALENCE OF LOOP PROGRAMS WITHOUT NESTING
> INSTANCE:  Finite set X of variables, subset Y ⊆ X of input variables, specified output variable x0, two loop programs P1 and P2 without nested loops, i.e., sequences of instructions of the form "x←y," "x←x+1," "x←0," "loop x," and "end," where x,y ∈ X and each loop instruction is followed by a corresponding end instruction before any further loop instructions occur.
> QUESTION:  Is there an initial assignment f: Y→Z+ of integers to the input variables such that the two programs halt with different values for the output variable x0 (see references for details on the execution of such programs)?
> Reference:  [Constable, Hunt, and Sahni, 1974], [Tsichritzis, 1970]. Transformation from 3SAT.  The second reference proves membership in NP.
> Comment:  Problem becomes undecidable if nested loops are allowed (even for nesting of only depth 2) [Meyer and Ritchie, 1967].  Solvable in polynomial time if loop statements are not allowed [Tsichritzis, 1970].  See [Hunt, 1977] for a generalization of the main result.

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

- **[Constable, Hunt, and Sahni, 1974]**: [`Constable1974`] R. L. Constable and H. B. Hunt, III and S. Sahni (1974). "On the computational complexity of scheme equivalence". Cornell University.
- **[Tsichritzis, 1970]**: [`Tsichritzis1970`] Dennis Tsichritzis (1970). "The equivalence problem of simple programs". *Journal of the Association for Computing Machinery* 17, pp. 729–738.
- **[Meyer and Ritchie, 1967]**: [`Meyer1967`] Albert R. Meyer and Dennis M. Ritchie (1967). "The complexity of loop programs". In: *Proceedings of the 22nd National Conference of the ACM*, pp. 465–469. Thompson Book Co..
- **[Hunt, 1977]**: [`Hunt1977a`] Harry B. Hunt III (1977). "A complexity theory of computation structures: preliminary report".
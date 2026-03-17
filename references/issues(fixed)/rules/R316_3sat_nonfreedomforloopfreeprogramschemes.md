---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to NON-FREEDOM FOR LOOP-FREE PROGRAM SCHEMES"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** NON-FREEDOM FOR LOOP-FREE PROGRAM SCHEMES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO19

## GJ Source Entry

> [PO19]  NON-FREEDOM FOR LOOP-FREE PROGRAM SCHEMES
> INSTANCE:  Finite sets F and P of function and predicate symbols, set X of variables, and a loop-free monadic program scheme S over F,P, and X, where such a scheme consists of a sequence I1,I2,...,Im of instructions of the form "x←f(y)," "if p(x) then goto Ij else goto Ik," and "halt," with x ∈ X, f ∈ F, and p ∈ P, and must be such that no directed cycles occur in the corresponding "flow graph."
> QUESTION:  Is S non-free, i.e., is there a directed path in the flow graph for S that can never be followed in any computation, no matter what the interpretation of the functions and predicates in F and P and the initial values for the variables in X?
> Reference:  [Constable, Hunt, and Sahni, 1974]. Transformation from 3SAT.
> Comment:  Remains NP-complete for |X|=2.  If |X|=1, the problem is solvable in polynomial time.  If loops are allowed and |X| is arbitrary, the problem is undecidable [Paterson, 1967].

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
- **[Paterson, 1967]**: [`Paterson1967`] M. S. Paterson (1967). "Equivalence Problems in a Model of Computation". Cambridge University.
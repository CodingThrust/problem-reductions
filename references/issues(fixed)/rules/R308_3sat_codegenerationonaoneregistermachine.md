---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to CODE GENERATION ON A ONE-REGISTER MACHINE"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** CODE GENERATION ON A ONE-REGISTER MACHINE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO4

## GJ Source Entry

> [PO4]  CODE GENERATION ON A ONE-REGISTER MACHINE
> INSTANCE:  Directed acyclic graph G = (V,A) in which no vertex has out-degree larger than 2, and a positive integer K.
> QUESTION:  Is there a program with K or fewer instructions for computing all the root vertices of G (i.e., those with in-degree 0) on a one-register machine, starting with all the leaves of G (i.e., those with out-degree 0) in memory and using only LOAD, STORE, and OP instructions?  (A LOAD instruction copies a specified vertex into the register.  A STORE instruction copies the vertex in the register into memory.  A new vertex v can be computed by an OP instruction if the vertex u in the register is such that (v,u)∈A and, if there is another vertex u' such that (v,u')∈A, then u' is in memory.  Execution of the OP instruction replaces u by v in the register.  The computation of a new vertex is not completed until it is copied into memory by a STORE instruction.)
> Reference:  [Bruno and Sethi, 1976]. Transformation from 3SAT.
> Comment:  Remains NP-complete even if all vertices having in-degree larger than one have arcs only to leaves of G [Aho, Johnson, and Ullman, 1977a].  Solvable in polynomial time if G is a directed forest [Sethi and Ullman, 1970].

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

- **[Bruno and Sethi, 1976]**: [`Bruno1976`] J. Bruno and R. Sethi (1976). "Code generation for a one-register machine". *Journal of the Association for Computing Machinery* 23, pp. 502–510.
- **[Aho, Johnson, and Ullman, 1977a]**: [`Aho1977b`] A. V. Aho and S. C. Johnson and J. D. Ullman (1977). "Code generation for expressions with common subexpressions". *Journal of the Association for Computing Machinery* 24, pp. 146–160.
- **[Sethi and Ullman, 1970]**: [`Sethi1970`] R. Sethi and J. D. Ullman (1970). "The generation of optimal code for arithmetic expressions". *Journal of the Association for Computing Machinery* 17, pp. 715–728.
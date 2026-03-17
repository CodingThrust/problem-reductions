---
name: Rule
about: Propose a new reduction rule
title: "[Rule] FEEDBACK VERTEX SET to CODE GENERATION WITH UNLIMITED REGISTERS"
labels: rule
assignees: ''
---

**Source:** FEEDBACK VERTEX SET
**Target:** CODE GENERATION WITH UNLIMITED REGISTERS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO5

## GJ Source Entry

> [PO5]  CODE GENERATION WITH UNLIMITED REGISTERS
> INSTANCE:  Directed acyclic graph G = (V,A) in which no vertex has out-degree larger than 2, partition of A into disjoints sets L and R such that two arcs leaving the same vertex always belong to different sets, and a positive integer K.
> QUESTION:  Is there a program with K or fewer instructions for computing all the root vertices of G, starting with all the leaves of G stored in registers and using only instructions of the form "ri←rj" or "ri←ri op rj," i,j ∈ Z+, where a vertex v with out-degree 2 and outgoing arcs (v,u) ∈ L and (v,w) ∈ R can be computed only by an instruction ri←ri op rj when ri contains u and rj contains w?
> Reference:  [Aho, Johnson, and Ullman, 1977a]. Transformation from FEEDBACK VERTEX SET.
> Comment:  Remains NP-complete even if only leaves of G have in-degree exceeding 1.  The "commutative" variant in which instructions of the form "ri←rj op ri" are also allowed is NP-complete [Aho, Johnson, and Ullman, 1977b].  Both problems can be solved in polynomial time if G is a forest or if 3-address instructions "ri←rj op rk" are allowed [Aho, Johnson, and Ullman, 1977a].

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

- **[Aho, Johnson, and Ullman, 1977a]**: [`Aho1977b`] A. V. Aho and S. C. Johnson and J. D. Ullman (1977). "Code generation for expressions with common subexpressions". *Journal of the Association for Computing Machinery* 24, pp. 146–160.
- **[Aho, Johnson, and Ullman, 1977b]**: [`Aho1977b`] A. V. Aho and S. C. Johnson and J. D. Ullman (1977). "Code generation for expressions with common subexpressions". *Journal of the Association for Computing Machinery* 24, pp. 146–160.
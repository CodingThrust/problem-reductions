---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to CODE GENERATION WITH UNFIXED VARIABLE LOCATIONS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** CODE GENERATION WITH UNFIXED VARIABLE LOCATIONS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO8

## GJ Source Entry

> [PO8]  CODE GENERATION WITH UNFIXED VARIABLE LOCATIONS
> INSTANCE:  Sequence I = (I1,I2,...,In) of instructions, finite set V of variables, assignment g: I→I ∪ V, positive integers B and M.
> QUESTION:  Can the instructions in I be stored as one- and two-byte instructions and the variables stored among them so that the total memory required is at most M, i.e., is there a one-to-one function f: I ∪ V→{1,2,...,M} such that f(Ii) < f(Ij) whenever i < j and such that, for 1≤i≤n, either |f(Ii)−f(g(Ii))| < B or f(Ii)+1 is not in the range of f?
> Reference:  [Robertson, 1977]. Transformation from 3SAT.
> Comment:  Remains NP-complete even for certain fixed values of B, e.g., B = 31.  Solvable in polynomial time if V is empty.

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

- **[Robertson, 1977]**: [`Robertson1977`] E. L. Robertson (1977). "Code generation for short/long address machines". Mathematics Research Center, University of Wisconsin.
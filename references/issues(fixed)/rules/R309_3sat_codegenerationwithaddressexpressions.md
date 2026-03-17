---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to CODE GENERATION WITH ADDRESS EXPRESSIONS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** CODE GENERATION WITH ADDRESS EXPRESSIONS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO7

## GJ Source Entry

> [PO7]  CODE GENERATION WITH ADDRESS EXPRESSIONS
> INSTANCE:  Sequence I = (I1,I2,...,In) of instructions, for each Ii ∈ I an expression g(Ii) of the form "Ij," "Ij+k," "Ij−k," or "k" where Ij ∈ I and k ∈ Z+, and positive integers B,C, and M.
> QUESTION:  Can the instructions in I be stored as one- and two-byte instructions so that the total memory required is at most M, i.e., is there a one-to-one function f: I→{1,2,...,M} such that f(Ii) < f(Ij) whenever i < j and such that, if h(Ii) is defined to be f(Ij), f(Ij)±k, or k depending on whether g(Ii) is Ij, Ij±k, or k, then for each i, 1≤i≤n, either −C < f(Ii)−h(Ii) < B or f(Ii)+1 is not in the range of f?
> Reference:  [Szymanski, 1978]. Transformation from 3SAT.
> Comment:  Remains NP-complete for certain fixed values of B and C, e.g., 128 and 127 (much smaller values also are possible).  Solvable in polynomial time if no "pathological" expressions occur (see reference for details).

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

- **[Szymanski, 1978]**: [`Szymanski1978`] Thomas G. Szymanski (1978). "Assembling code for machines with span-dependent instructions". *Communications of the ACM* 21, pp. 300–308.
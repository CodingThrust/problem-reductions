---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to FAULT DETECTION IN LOGIC CIRCUITS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** FAULT DETECTION IN LOGIC CIRCUITS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS17

## GJ Source Entry

> [MS17]  FAULT DETECTION IN LOGIC CIRCUITS
> INSTANCE:  Directed acyclic graph G = (V,A) with a single vertex v* ∈ V having out-degree 0, an assignment f: (V−{v*}) → {I, and, or, not} such that f(v) = I implies v has in-degree 0, f(v) = not implies v has in-degree 1, and f(v) = and or f(v) = or implies v has in-degree 2, and a subset V' ⊆ V.
> QUESTION:  Can all single faults occurring at vertices of V' be detected by input-output experiments, i.e., regarding G as a logic circuit with input vertices I, output vertex v*, and logic gates for the functions "and," "or," and "not" at the specified vertices, is there for each v ∈ V' and x ∈ {T,F} an assignment of a value to each vertex in I of a value in {T,F} such that the output of the circuit for those input values differs from the output of the same circuit with the output of the gate at v "stuck-at" x?
> Reference:  [Ibarra and Sahni, 1975]. Transformation from 3SAT.
> Comment:  Remains NP-complete even if V' = V or if V' contains just a single vertex v with f(v) = I.

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

- **[Ibarra and Sahni, 1975]**: [`Ibarra1975c`] Oscar H. Ibarra and Sartaj K. Sahni (1975). "Polynomially complete fault detection problems". *IEEE Transactions on Computers* C-24, pp. 242–249.
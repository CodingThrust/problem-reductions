---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to STACKER-CRANE"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** STACKER-CRANE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND26, p.212

## GJ Source Entry

> [ND26] STACKER-CRANE
> INSTANCE: Mixed graph G=(V,A,E), length l(e)∈Z_0^+ for each e∈A∪E, bound B∈Z^+.
> QUESTION: Is there a cycle in G that includes each directed edge in A at least once, traversing such edges only in the specified direction, and that has total length no more than B?
> Reference: [Frederickson, Hecht, and Kim, 1978]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete even if all edge lengths equal 1. The analogous path problem (with or without specified endpoints) is also NP-complete.

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

- **[Frederickson, Hecht, and Kim, 1978]**: [`Frederickson1978`] G. N. Frederickson and M. S. Hecht and C. E. Kim (1978). "Approximation algorithms for some routing problems". *SIAM Journal on Computing* 7, pp. 178–193.
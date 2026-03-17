---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to LONGEST CIRCUIT"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** LONGEST CIRCUIT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND28, p.213

## GJ Source Entry

> [ND28] LONGEST CIRCUIT
> INSTANCE: Graph G=(V,E), length l(e)∈Z^+ for each e∈E, positive integer K.
> QUESTION: Is there a simple circuit in G of length K or more, i.e., whose edge lengths sum to at least K?
> Reference: Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete if l(e)=1 for all e∈E, as does the corresponding problem for directed circuits in directed graphs. The directed problem with all l(e)=1 can be solved in polynomial time if G is a "tournament" [Morrow and Goodman, 1976]. The analogous directed and undirected problems, which ask for a simple circuit of length K or less, can be solved in polynomial time (e.g., see [Itai and Rodeh, 1977b]), but are NP-complete if negative lengths are allowed.

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

- **[Morrow and Goodman, 1976]**: [`Morrow1976`] C. Morrow and S. Goodman (1976). "An efficient algorithm for finding a longest cycle in a tournament". In: *Proceedings of the 7th Southeastern Conference on Combinatorics, Graph Theory, and Computing*, pp. 453–462. Utilitas Mathematica Publishing.
- **[Itai and Rodeh, 1977b]**: [`Itai1977c`] Alon Itai and Michael Rodeh (1977). "Some matching problems". In: *Automata, Languages, and Programming*. Springer.
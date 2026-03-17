---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to RURAL POSTMAN"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** RURAL POSTMAN
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND27, p.213

## GJ Source Entry

> [ND27] RURAL POSTMAN
> INSTANCE: Graph G=(V,E), length l(e)∈Z_0^+ for each e∈E, subset E'⊆E, bound B∈Z^+.
> QUESTION: Is there a circuit in G that includes each edge in E' and that has total length no more than B?
> Reference: [Lenstra and Rinnooy Kan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete even if l(e)=1 for all e∈E, as does the corresponding problem for directed graphs.

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

- **[Lenstra and Rinnooy Kan, 1976]**: [`Lenstra1976`] Jan K. Lenstra and A. H. G. Rinnooy Kan (1976). "On general routing problems". *Networks* 6, pp. 273–280.
---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to OPTIMUM COMMUNICATION SPANNING TREE"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** OPTIMUM COMMUNICATION SPANNING TREE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND7, p.207

## GJ Source Entry

> [ND7] OPTIMUM COMMUNICATION SPANNING TREE
> INSTANCE: Complete graph G=(V,E), weight w(e)∈Z_0^+ for each e∈E, requirement r({u,v})∈Z_0^+ for each pair {u,v} of vertices from V, bound B∈Z_0^+.
> QUESTION: Is there a spanning tree T for G such that, if W({u,v}) denotes the sum of the weights of the edges on the path joining u and v in T, then
> ∑_{u,v∈V} [W({u,v})·r({u,v})] ≤ B ?
> Reference: [Johnson, Lenstra, and Rinnooy Kan, 1978]. Transformation from X3C.
> Comment: Remains NP-complete even if all requirements are equal. Can be solved in polynomial time if all edge weights are equal [Hu, 1974].

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

- **[Johnson, Lenstra, and Rinnooy Kan, 1978]**: [`Johnson1978b`] David S. Johnson and Jan K. Lenstra and Alexander H. G. Rinnooy Kan (1978). "The complexity of the network design problem". *Networks*.
- **[Hu, 1974]**: [`Hu1974`] Te C. Hu (1974). "Optimum communication spanning trees". *SIAM Journal on Computing* 3, pp. 188–195.
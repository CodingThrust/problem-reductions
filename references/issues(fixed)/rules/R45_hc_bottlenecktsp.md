---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to BOTTLENECK TRAVELING SALESMAN"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** BOTTLENECK TRAVELING SALESMAN
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND24, p.212

## GJ Source Entry

> [ND24] BOTTLENECK TRAVELING SALESMAN
> INSTANCE: Set C of m cities, distance d(c_i,c_j)∈Z^+ for each pair of cities c_i,c_j∈C, positive integer B.
> QUESTION: Is there a tour of C whose longest edge is no longer than B, i.e., a permutation <c_{π(1)},c_{π(2)},...,c_{π(m)}> of C such that d(c_{π(i)},c_{π(i+1)})≤B for 1≤i<m and such that d(c_{π(m)},c_{π(1)})≤B?
> Reference: Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete even if d(c_i,c_j)∈{1,2} for all c_i,c_j∈C. An important special case that is solvable in polynomial time can be found in [Gilmore and Gomory, 1964].

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

- **[Gilmore and Gomory, 1964]**: [`Gilmore1964`] P. C. Gilmore and R. E. Gomory (1964). "Sequencing a one state-variable machine: a solvable case of the traveling salesman problem". *Operations Research* 12, pp. 655–679.
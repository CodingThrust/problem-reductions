---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to TRAVELING SALESMAN"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** TRAVELING SALESMAN
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND22, p.211

## GJ Source Entry

> [ND22] TRAVELING SALESMAN
> INSTANCE: Set C of m cities, distance d(c_i,c_j)∈Z^+ for each pair of cities c_i,c_j∈C, positive integer B.
> QUESTION: Is there a tour of C having length B or less, i.e., a permutation <c_{π(1)},c_{π(2)},...,c_{π(m)}> of C such that
> (∑_{i=1}^{m-1} d(c_{π(i)},c_{π(i+1)})) + d(c_{π(m)},c_{π(1)}) ≤ B ?
> Reference: Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete even if d(c_i,c_j)∈{1,2} for all c_i,c_j∈C. Special cases that can be solved in polynomial time are discussed in [Gilmore and Gomory, 1964], [Garfinkel, 1977], and [Syslo, 1973]. The variant in which we ask for a tour with "mean arrival time" of B or less is also NP-complete [Sahni and Gonzalez, 1976].

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
- **[Garfinkel, 1977]**: [`Garfinkel1977`] R. S. Garfinkel (1977). "Minimizing wallpaper waste, {Part} 1: a class of traveling salesman problems". *Operations Research* 25, pp. 741–751.
- **[Syslo, 1973]**: [`Syslo1973`] Maciej M. Syslo (1973). "A new solvable case of the traveling salesman problem". *Mathematical Programming* 4, pp. 347–348.
- **[Sahni and Gonzalez, 1976]**: [`Gonzalez1976`] T. Gonzalez and S. Sahni (1976). "Open shop scheduling to minimize finish time". *Journal of the Association for Computing Machinery* 23, pp. 665–679.
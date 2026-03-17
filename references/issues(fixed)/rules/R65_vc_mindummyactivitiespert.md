---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND44, p.218

## GJ Source Entry

> [ND44] MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS
> INSTANCE: Directed acyclic graph G=(V,A) where vertices represent tasks and the arcs represent precedence constraints, and a positive integer K≤|V|.
> QUESTION: Is there a PERT network corresponding to G with K or fewer dummy activities, i.e., a directed acyclic graph G'=(V',A') where V'={v_i^−,v_i^+: v_i∈V} and {(v_i^−,v_i^+): v_i∈V}⊆A', and such that |A'|≤|V|+K and there is a path from v_i^+ to v_j^− in G' if and only if there is a path from v_i to v_j in G?
> Reference: [Krishnamoorthy and Deo, 1977b]. Transformation from VERTEX COVER.

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

- **[Krishnamoorthy and Deo, 1977b]**: [`Krishnamoorthy1977b`] M. S. Krishnamoorthy and N. Deo (1977). "Complexity of the minimum dummy activities problem in a {Pert} network". Computer Centre, Indian Institute of Technology.
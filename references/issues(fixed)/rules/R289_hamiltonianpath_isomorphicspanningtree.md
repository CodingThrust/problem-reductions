---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH to ISOMORPHIC SPANNING TREE"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN PATH
**Target:** ISOMORPHIC SPANNING TREE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2.1 ND8

## GJ Source Entry

> [ND8] ISOMORPHIC SPANNING TREE
> INSTANCE: Graph G = (V,E), tree T = (V_T,E_T).
> QUESTION: Does G contain a spanning tree isomorphic to T?
> Reference: Transformation from HAMILTONIAN PATH.
> Comment: Remains NP-complete even if (a) T is a path, (b) T is a full binary tree [Papadimitriou and Yannakakis, 1978], or if (c) T is a 3-star (that is, V_T = {v_0} ∪ {u_i,v_i,w_i: 1 ≤ i ≤ n}, E_T = {{v_0,u_i},{u_i,v_i},{v_i,w_i}: 1 ≤ i ≤ n}) [Garey and Johnson, ——]. Solvable in polynomial time by graph matching if G is a 2-star. For a classification of the complexity of this problem for other types of trees, see [Papadimitriou and Yannakakis, 1978].

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

- **[Papadimitriou and Yannakakis, 1978]**: [`Papadimitriou1978f`] Christos H. Papadimitriou and M. Yannakakis (1978). "On the complexity of minimum spanning tree problems".
- **[Garey and Johnson, ——]**: *(not found in bibliography)*
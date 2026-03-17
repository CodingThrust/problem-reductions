---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to Ensemble Computation"
labels: rule
assignees: ''
---

**Source:** Vertex Cover
**Target:** Ensemble Computation
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.6, p.66

## Reduction Algorithm

> ENSEMBLE COMPUTATION
> INSTANCE: A collection C of subsets of a finite set A and a positive integer J.
> QUESTION: Is there a sequence
>
> < z_1 = x_1 ∪ y_1, z_2 = x_2 ∪ y_2, . . . , z_j = x_j ∪ y_j >
>
> of j ≤ J union operations, where each x_i and y_i is either {a} for some a ∈ A or z_k for some k < i, such that x_i and y_i are disjoint for 1 ≤ i ≤ j and such that for every subset c ∈ C there is some z_i, 1 ≤ i ≤ j, that is identical to c ?
>
> Theorem 3.6 ENSEMBLE COMPUTATION is NP-complete.
> Proof: We transform VERTEX COVER to ENSEMBLE COMPUTATION. Let the graph G = (V,E) and the positive integer K ≤ |V| constitute an arbitrary instance of VC.
>
> The basic units of the instance of VC are the edges of G. Let a_0 be some new element not in V. The local replacement just substitutes for each edge {u,v} ∈ E the subset {a_0,u,v} ∈ C. The instance of ENSEMBLE COMPUTATION is completely specified by:
>
>     A = V ∪ {a_0}
>     C = {{a_0,u,v}: {u,v} ∈ E}
>     J = K + |E|
>
> It is easy to see that this instance can be constructed in polynomial time. We claim that G has a vertex cover of size K or less if and only if the desired sequence of j ≤ J operations exists for C.
>
> First, suppose V' is a vertex cover for G of size K or less. Since we can add additional vertices to V' and it will remain a vertex cover, there is no loss of generality in assuming that |V'| = K. Label the elements of V' as v_1,v_2, . . . , v_K and label the edges in E as e_1,e_2, . . . , e_m, where m = |E|. Since V' is a vertex cover, each edge e_j contains at least one element from V'. Thus we can write each e_j as e_j = {u_j,v_{r[j]}}, where r[j] is an integer satisfying 1 ≤ r[j] ≤ K. The following sequence of K + |E| = J operations is easily seen to have all the required properties:
>
>     < z_1 = {a_0} ∪ {v_1}, z_2 = {a_0} ∪ {v_2}, . . . , z_k = {a_0} ∪ {v_K},
>       z_{K+1} = {u_1} ∪ z_{r[1]}, z_{K+2} = {u_2} ∪ z_{r[2]}, . . . , z_J = {u_m} ∪ z_{r[m]} >
>
> Conversely, suppose S = < z_1 = x_1 ∪ y_1, . . . , z_j = x_j ∪ y_j > is the desired sequence of j ≤ J operations for the ENSEMBLE COMPUTATION instance. Furthermore, let us assume that S is the shortest such sequence for this instance and that, among all such minimum sequences, S contains the fewest possible operations of the form z_i = {u} ∪ {v} for u, v ∈ V. Our first claim is that S can contain no operations of this latter form. For suppose that z_i = {u} ∪ {v} with u,v ∈ V is included. Since {u,v} is not in C and since S has minimum length, we must have {u,v} ∈ E, and {a_0,u,v} = {a_0} ∪ z_i (or z_i ∪ {a_0}) must occur later in S. However, since {u,v} is a subset of only one member of C, z_i cannot be used in any other operation in this minimum length sequence. It follows that we can replace the two operations
>
>     z_i = {u} ∪ {v}  and  {a_0,u,v} = {a_0} ∪ z_i
>
> by
>
>     z_i = {a_0} ∪ {u}  and  {a_0,u,v} = {v} ∪ z_i
>
> thereby reducing the number of proscribed operations without lengthening the overall sequence, a contradiction to the choice of S. Hence S consists only of operations having one of the two forms, z_i = {a_0} ∪ {u} for u ∈ V or {a_0,u,v} = {v} ∪ z_i for {u,v} ∈ E (where we disregard the relative order of the two operands in each case). Because |C| = |E| and because every member of C contains three elements, S must contain exactly |E| operations of the latter form and exactly j−|E| ≤ J−|E| = K of the former. Therefore the set
>
>     V' = {u ∈ V: z_i = {a_0} ∪ {u} is an operation in S}
>
> contains at most K vertices from V and, as can be verified easily from the construction of C, must be a vertex cover for G. ∎

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

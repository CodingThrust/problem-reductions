---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-SATISFIABILITY (3SAT) to VERTEX COVER"
labels: rule
assignees: ''
---

**Source:** 3-SATISFIABILITY (3SAT)
**Target:** VERTEX COVER
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.3, p.54-56

## Reduction Algorithm

> Theorem 3.3 VERTEX COVER is NP-complete.
> Proof: It is easy to see that VC E NP since a nondeterministic algorithm need only guess a subset of vertices and check in polynomial time whether that subset contains at least one endpoint of every edge and has the appropriate size.
>
> We transform 3SAT to VERTEX COVER. Let U = {u1,u2, . . . , un} and C = {c1,c2, . . . , cm} be any instance of 3SAT. We must construct a graph G = (V,E) and a positive integer K <= |V| such that G has a vertex cover of size K or less if and only if C is satisfiable.
>
> As in the previous proof, the construction will be made up of several components. In this case, however, we will have only truth-setting components and satisfaction testing components, augmented by some additional edges for communicating between the various components.
>
> For each variable ui E U, there is a truth-setting component T_i = (V_i,E_i), with V_i = {ui,u-bar_i} and E_i = {{ui,u-bar_i}}, that is, two vertices joined by a single edge. Note that any vertex cover will have to contain at least one of ui and u-bar_i in order to cover the single edge in E_i.
>
> For each clause cj E C, there is a satisfaction testing component S_j = (V'_j,E'_j), consisting of three vertices and three edges joining them to form a triangle:
>
> V'_j = {a1[j],a2[j],a3[j]}
> E'_j = {{a1[j],a2[j]},{a1[j],a3[j]},{a2[j],a3[j]}}
>
> Note that any vertex cover will have to contain at least two vertices from V'_j in order to cover the edges in E'_j.
>
> The only part of the construction that depends on which literals occur in which clauses is the collection of communication edges. These are best viewed from the vantage point of the satisfaction testing components. For each clause cj E C, let the three literals in cj be denoted by xj, yj, and zj. Then the communication edges emanating from S_j are given by:
>
> E''_j = {{a1[j],xj},{a2[j],yj},{a3[j],zj}}
>
> The construction of our instance of VC is completed by setting K = n + 2m and G = (V,E), where
>
> V = (U (i=1 to n) V_i) U (U (j=1 to m) V'_j)
>
> and
>
> E = (U (i=1 to n) E_i) U (U (j=1 to m) E'_j) U (U (j=1 to m) E''_j)
>
> Figure 3.3 shows an example of the graph obtained when U = {u1,u2,u3,u4} and C = {{u1,u-bar_3,u-bar_4},{u-bar_1,u2,u-bar_4}}.
>
> It is easy to see how the construction can be accomplished in polynomial time. All that remains to be shown is that C is satisfiable if and only if G has a vertex cover of size K or less.
>
> First, suppose that V' ⊆ V is a vertex cover for G with |V'| <= K. By our previous remarks, V' must contain at least one vertex from each T_i and at least two vertices from each S_j. Since this gives a total of at least n+2m = K vertices, V' must in fact contain exactly one vertex from each T_i and exactly two vertices from each S_j. Thus we can use the way in which V' intersects each truth-setting component to obtain a truth assignment t: U->{T,F}. We merely set t(ui) = T if ui E V' and t(ui) = F if u-bar_i E V'. To see that this truth assignment satisfies each of the clauses cj E C, consider the three edges in E''_j. Only two of those edges can be covered by vertices from V'_j ∩ V', so one of them must be covered by a vertex from some V_i that belongs to V'. But that implies that the corresponding literal, either ui or u-bar_i, from clause cj is true under the truth assignment t, and hence clause cj is satisfied by t. Because this holds for every cj E C, it follows that t is a satisfying truth assignment for C.
>
> Conversely, suppose that t: U->{T,F} is a satisfying truth assignment for C. The corresponding vertex cover V' includes one vertex from each T_i and two vertices from each S_j. The vertex from T_i in V' is ui if t(ui) = T and is u-bar_i if t(ui) = F. This ensures that at least one of the three edges from each set E''_j is covered, because t satisfies each clause cj. Therefore we need only include in V' the endpoints from S_j of the other two edges in E''_j (which may or may not also be covered by vertices from truth-setting components), and this gives the desired vertex cover.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

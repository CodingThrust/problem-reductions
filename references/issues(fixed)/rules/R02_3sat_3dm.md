---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-SATISFIABILITY (3SAT) to 3-DIMENSIONAL MATCHING (3DM)"
labels: rule
assignees: ''
---

**Source:** 3-SATISFIABILITY (3SAT)
**Target:** 3-DIMENSIONAL MATCHING (3DM)
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.2, p.50-53

## Reduction Algorithm

> Theorem 3.2 3-DIMENSIONAL MATCHING is NP-complete.
> Proof: It is easy to see that 3DM E NP, since a nondeterministic algorithm need only guess a subset of q=|W|=|X|=|Y| triples from M and check in polynomial time that no two of the guessed triples agree in any coordinate.
>
> We will transform 3SAT to 3DM. Let U = {u1,u2, . . . , un} be the set of variables and C = {c1,c2, . . . , cm} be the set of clauses in an arbitrary instance of 3SAT. We must construct disjoint sets W, X, and Y, with |W| = |X| = |Y|, and a set M ⊆ W×X×Y such that M contains a matching if and only if C is satisfiable.
>
> The set M of ordered triples will be partitioned into three separate classes, grouped according to their intended function: "truth-setting and fan-out," "satisfaction testing," or "garbage collection."
>
> Each truth-setting and fan-out component corresponds to a single variable u E U, and its structure depends on the total number m of clauses in C. This structure is illustrated for the case of m=4 in Figure 3.2. In general, the truth-setting and fan-out component for a variable ui involves "internal" elements ai[j] E X and bi[j] E Y, 1 <= j <= m, which will not occur in any triples outside of this component, and "external" elements ui[j], u-bar_i[j] E W, 1 <= j <= m, which will occur in other triples. The triples making up this component can be divided into two sets:
>
> T^t_i = {(u-bar_i[j],ai[j],bi[j]): 1 <= j <= m}
> T^f_i = {(ui[j],ai[j+1],bi[j]): 1 <= j < m} U {(ui[m],ai[1],bi[m])}
>
> Since none of the internal elements {ai[j], bi[j]: 1 <= j <= m} will appear in any triples outside of T_i = T^t_i U T^f_i, it is easy to see that any matching M' will have to include exactly m triples from T_i, either all triples in T^t_i or all triples in T^f_i. Hence we can think of the component T_i as forcing a matching to make a choice between setting ui true and setting ui false. Thus, in general, a matching M' ⊆ M specifies a truth assignment for U, with the variable ui being set true if and only if M' ∩ T_i = T^t_i.
>
> Each satisfaction testing component in M corresponds to a single clause cj E C. It involves only two "internal" elements, s1[j] E X and s2[j] E Y, and external elements from {ui[j], u-bar_i[j]: 1 <= i <= n}, determined by which literals occur in clause cj. The set of triples making up this component is defined as follows:
>
> Cj = {(ui[j],s1[j],s2[j]): ui E cj} U {(u-bar_i[j],s1[j],s2[j]): u-bar_i E cj}
>
> Thus any matching M' ⊆ M will have to contain exactly one triple from Cj. This can only be done, however, if some ui[j] (or u-bar_i[j]) for a literal ui E cj (u-bar_i E cj) does not occur in the triples in T_i ∩ M', which will be the case if and only if the truth setting determined by M' satisfies clause cj.
>
> The construction is completed by means of one large "garbage collection" component G, involving internal elements g1[k] E X and g2[k] E Y, 1 <= k <= m(n-1), and external elements of the form ui[j] and u-bar_i[j] from W. It consists of the following set of triples:
>
> G = {(ui[j],g1[k],g2[k]),(u-bar_i[j],g1[k],g2[k]):
>          1 <= k <= m(n-1), 1 <= i <= n, 1 <= j <= m}
>
> Thus each pair g1[k], g2[k] must be matched with a unique ui[j] or u-bar_i[j] that does not occur in any triples of M'-G. There are exactly m(n-1) such "uncovered" external elements, and the structure of G insures that they can always be covered by choosing M' ∩ G appropriately. Thus G merely guarantees that, whenever a subset of M-G satisfies all the constraints imposed by the truth-setting and fan-out components, then that subset can be extended to a matching for M.
>
> To summarize, we set
>
> W = {ui[j], u-bar_i[j]: 1 <= i <= n, 1 <= j <= m}
> X = A U S1 U G1
>
> where
>      A  = {ai[j]: 1 <= i <= n, 1 <= j <= m}
>      S1 = {s1[j]: 1 <= j <= m}
>      G1 = {g1[j]: 1 <= j <= m(n-1)}
>
> Y = B U S2 U G2
>
> where
>      B  = {bi[j]: 1 <= i <= n, 1 <= j <= m}
>      S2 = {s2[j]: 1 <= j <= m}
>      G2 = {g2[j]: 1 <= j <= m(n-1)}
>
> and
>
> M = (U (i=1 to n) T_i) U (U (j=1 to m) Cj) U G
>
> Notice that every triple in M is an element of W×X×Y as required. Furthermore, since M contains only
>
>      2mn + 3m + 2m^2n(n-1)
>
> triples and since its definition in terms of the given 3SAT instance is quite direct, it is easy to see that M can be constructed in polynomial time.
>
> From the comments made during the description of M, it follows immediately that M cannot contain a matching unless C is satisfiable. We now must show that the existence of a satisfying truth assignment for C implies that M contains a matching.
>
> Let t: U->{T,F} be any satisfying truth assignment for C. We construct a matching M' ⊆ M as follows: For each clause cj E C, let zj E {ui, u-bar_i: 1 <= i <= n} ∩ cj be a literal that is set true by t (one must exist since t satisfies cj). We then set
>
> M' = (U_{t(ui)=T} T^t_i) U (U_{t(ui)=F} T^f_i) U (U (j=1 to m) {(zj[j],s1[j],s2[j])}) U G'
>
> where G' is an appropriately chosen subcollection of G that includes all the g1[k],g2[k], and remaining ui[j] and u-bar_i[j]. It is easy to verify that such a G' can always be chosen and that the resulting set M' is a matching.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

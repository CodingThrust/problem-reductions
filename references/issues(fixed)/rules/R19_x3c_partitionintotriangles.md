---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Exact Cover by 3-Sets to Partition Into Triangles"
labels: rule
assignees: ''
---

**Source:** Exact Cover by 3-Sets
**Target:** Partition Into Triangles
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.7, p.68

## Reduction Algorithm

> Another example of a polynomial time transformation using local replacement, this time from EXACT COVER BY 3-SETS, is the following:
>
> PARTITION INTO TRIANGLES
> INSTANCE: A graph G = (V,E), with |V| = 3q for a positive integer q.
> QUESTION: Is there a partition of V into q disjoint sets V_1, V_2, ..., V_q of three vertices each such that, for each V_i = {v_{i[1]}, v_{i[2]}, v_{i[3]}}, the three edges {v_{i[1]}, v_{i[2]}}, {v_{i[1]}, v_{i[3]}}, and {v_{i[2]}, v_{i[3]}} all belong to E?
>
> Theorem 3.7 PARTITION INTO TRIANGLES is NP-complete.
> Proof: We transform EXACT COVER BY 3-SETS to PARTITION INTO TRIANGLES. Let the set X with |X| = 3q and the collection C of 3-element subsets of X be an arbitrary instance of X3C. We shall construct a graph G = (V,E), with |V| = 3q', such that the desired partition exists for G if and only if C contains an exact cover.
>
> The basic units of the X3C instance are the 3-element subsets in C. The local replacement substitutes for each such subset c_i = {x_i, y_i, z_i} E C the collection E_i of 18 edges shown in Figure 3.8. Thus G = (V,E) is defined by
>
>   V = X U U_{i=1}^{|C|} {a_i[j]: 1 <= j <= 9}
>   E = U_{i=1}^{|C|} E_i
>
> Notice that the only vertices that appear in edges belonging to more than a single E_i are those that are in the set X. Notice also that |V| = |X| + 9|C| = 3q + 9|C| so that q' = q + 3|C|. It is not hard to see that this instance of PARTITION INTO TRIANGLES can be constructed in polynomial time from the X3C instance.
>
> If c_1, c_2, ..., c_q are the 3-element subsets from C in any exact cover for X, then the corresponding partition V = V_1 U V_2 U ... U V_{q'} of V is given by taking
>
>   {a_i[1], a_i[2], x_i},  {a_i[4], a_i[5], y_i}
>   {a_i[7], a_i[8], z_i},  {a_i[3], a_i[6], a_i[9]}
>
> from the vertices meeting E_i whenever c_i = {x_i, y_i, z_i} is in the exact cover, and by taking
>
>   {a_i[1], a_i[2], a_i[3]},  {a_i[4], a_i[5], a_i[6]},  {a_i[7], a_i[8], a_i[9]}
>
> from the vertices meeting E_i whenever c_i is not in the exact cover. This ensures that each element of X is included in exactly one 3-vertex subset in the partition.
>
> Conversely, if V = V_1 U V_2 U ... U V_{q'} is any partition of G into triangles, the corresponding exact cover is given by choosing those c_i E C such that {a_i[3], a_i[6], a_i[9]} = V_j for some j, 1 <= j <= q'. We leave to the reader the straightforward task of verifying that the two partitions we have constructed are as claimed.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

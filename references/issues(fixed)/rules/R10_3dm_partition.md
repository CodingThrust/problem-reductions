---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-DIMENSIONAL MATCHING (3DM) to PARTITION"
labels: rule
assignees: ''
---

**Source:** 3-DIMENSIONAL MATCHING (3DM)
**Target:** PARTITION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.5, p.60-62

## Reduction Algorithm

> Theorem 3.5 PARTITION is NP-complete
> Proof: It is easy to see that PARTITION E NP, since a nondeterministic algorithm need only guess a subset A' of A and check in polynomial time that the sum of the sizes of the elements in A' is the same as that for the elements in A-A'.
>
> We transform 3DM to PARTITION. Let the sets W,X,Y, with |W|=|X|=|Y|=q, and M ⊆ W×X×Y be an arbitrary instance of 3DM. Let the elements of these sets be denoted by
>
> W = {w1, w2, . . . , wq}
> X = {x1, x2, . . . , xq}
> Y = {y1, y2, . . . , yq}
>
> and
>
> M = {m1, m2, . . . , mk}
>
> where k = |M|. We must construct a set A, and a size s(a) E Z+ for each a E A, such that A contains a subset A' satisfying
>
>      sum_{a E A'} s(a) = sum_{a E A-A'} s(a)
>
> if and only if M contains a matching.
>
> The set A will contain a total of k+2 elements and will be constructed in two steps. The first k elements of A are {a_i: 1 <= i <= k}, where the element a_i is associated with the triple m_i E M. The size s(a_i) of a_i will be specified by giving its binary representation, in terms of a string of 0's and 1's divided into 3q "zones" of p = [log2(k+1)] bits each. Each of these zones is labeled by an element of W U X U Y, as shown in Figure 3.7.
>
> Figure 3.7 Labeling of the 3q "zones," each containing p = [log2(k+1)] bits of the binary representation for s(a), used in transforming 3DM to PARTITION.
>
> The representation for s(a_i) depends on the corresponding triple m_i = (w_{f(i)},x_{g(i)},y_{h(i)}) E M (where f, g, and h are just the functions that give the subscripts of the first, second, and third components for each m_i). It has a 1 in the rightmost bit position of the zones labeled by w_{f(i)}, x_{g(i)}, and y_{h(i)} and 0's everywhere else. Alternatively, we can write
>
>      s(a_i) = 2^{p(3q-f(i))} + 2^{p(2q-g(i))} + 2^{p(q-h(i))}
>
> Since each s(a_i) can be expressed in binary with no more than 3pq bits, it is clear that s(a_i) can be constructed from the given 3DM instance in polynomial time.
>
> The important thing to observe about this part of the construction is that, if we sum up all the entries in any zone, over all elements of {a_i: 1 <= i <= k}, the total can never exceed k = 2^p-1. Hence, in adding up sum_{a E A'} s(a) for any subset A' ⊆ {a_i: 1 <= i <= k}, there will never be any "carries" from one zone to the next. It follows that if we let
>
>      B = sum_{j=0}^{3q-1} 2^{pj}
>
> (which is the number whose binary representation has a 1 in the rightmost position of every zone), then any subset A' ⊆ {a_i: 1 <= i <= k} will satisfy
>
>      sum_{a E A'} s(a) = B
>
> if and only if M' = {m_i: a_i E A'} is a matching for M.
>
> The final step of the construction specifies the last two elements of A. These are denoted by b1 and b2 and have sizes defined by
>
>      s(b1) = 2(sum_{i=1}^{k} s(a_i)) - B
>
> and
>
>      s(b2) = (sum_{i=1}^{k} s(a_i)) + B
>
> Both of these can be specified in binary with no more than (3pq+1) bits and thus can be constructed in time polynomial in the size of the given 3DM instance.
>
> Now suppose we have a subset A' ⊆ A such that
>
>      sum_{a E A'} s(a) = sum_{a E A-A'} s(a)
>
> Then both of these sums must be equal to 2sum_{i=1}^{k} s(a_i), and one of the two sets, A' or A-A', contains b1 but not b2. It follows that the remaining elements of that set form a subset of {a_i: 1 <= i <= k} whose sizes sum to B, and hence, by our previous comments, that subset corresponds to a matching M' in M. Conversely, if M' ⊆ M is a matching, then the set {b1} U {a_i: m_i E M'} forms the desired set A' for the PARTITION instance. Therefore, 3DM oc PARTITION, and the theorem is proved.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Dimensional Matching (3DM) to Minimum Test Collection"
labels: rule
assignees: ''
---

**Source:** 3-Dimensional Matching (3DM)
**Target:** Minimum Test Collection
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.9, p.71

## Reduction Algorithm

> MINIMUM TEST COLLECTION
> INSTANCE: A finite set A of "possible diagnoses," a collection C of subsets of A, representing binary "tests," and a positive integer J ≤ |C|.
> QUESTION: Is there a subcollection C' ⊆ C with |C'| ≤ J such that, for every pair a_i,a_j of possible diagnoses from A, there is some test c ∈ C' for which |{a_i,a_j} ∩ c| = 1 (that is, a test c that "distinguishes" between a_i and a_j)?
>
> Theorem 3.9 MINIMUM TEST COLLECTION is NP-complete.
> Proof: We transform 3DM to this problem. Let the sets W, X, Y, with |W| = |X| = |Y| = q, and the collection M ⊆ W × X × Y constitute an arbitrary instance of 3DM.
>
> The basic units of the 3DM instance are the ordered triples in M. The local replacement substitutes for each m = (w,x,y) ∈ M the subset {w,x,y} ∈ C. The enforcer is provided by three additional elements, w_0, x_0, and y_0, not belonging to W ∪ X ∪ Y, and two additional tests, W ∪ {w_0} and X ∪ {x_0}. The complete MINIMUM TEST COLLECTION instance is defined by:
>
>     A = W ∪ X ∪ Y ∪ {w_0, x_0, y_0}
>     C = {{w,x,y}: (w,x,y) ∈ M} ∪ { W ∪ {w_0}, X ∪ {x_0}}
>     J = q + 2
>
> It is easy to see that this instance can be constructed in polynomial time from the given 3DM instance.
>
> Once again the enforcer places certain limitations on the form of the desired entity (in this case, the subcollection C' of tests). First, C' must contain both W ∪ {w_0} and X ∪ {x_0}, since they are the only tests that distinguish y_0 from w_0 and x_0. Then, since w_0, x_0, and y_0 are not contained in any other tests in C, each element of W ∪ X ∪ Y must be distinguished from the appropriate one of w_0, x_0, or y_0 by being included in some additional test c ∈ C'−{W ∪ {w_0}, X ∪ {x_0}}. At most J−2 = q such additional tests can be included. Because each of the remaining tests in C contains exactly one member from each of W, X, and Y, and because W, X, and Y are disjoint sets, having q members each, it follows that any such additional q tests in C' must correspond to q triples that form a matching for M. Conversely, given any matching for M, the corresponding q tests from C can be used to complete the desired collection of J = q+2 tests. Thus M contains a matching if and only if the required subcollection of tests from C exists. ∎

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

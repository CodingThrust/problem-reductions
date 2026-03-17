---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SATISFIABILITY (SAT) to 3-SATISFIABILITY (3SAT)"
labels: rule
assignees: ''
---

**Source:** SATISFIABILITY (SAT)
**Target:** 3-SATISFIABILITY (3SAT)
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.1, p.48-50

## Reduction Algorithm

> Theorem 3.1. 3-SATISFIABILITY is NP-complete.
> Proof: It is easy to see that 3SAT E NP since a nondeterministic algorithm need only guess a truth assignment for the variables and check in polynomial time whether that truth setting satisfies all the given three-literal clauses.
>
> We transform SAT to 3SAT. Let U = {u1,u2, . . . , un} be a set of variables and C = {c1,c2, . . . , cm} be a set of clauses making up an arbitrary instance of SAT. We shall construct a collection C' of three-literal clauses on a set U' of variables such that C' is satisfiable if and only if C is satisfiable.
>
> The construction of C' will merely replace each individual clause cj E C by an "equivalent" collection C'j of three-literal clauses, based on the original variables U and some additional variables U'j whose use will be limited to clauses in C'j. These will be combined by setting
>
> U' = U U {U (j=1 to m) U'j}
>
> and
>
> C' = U (j=1 to m) C'j
>
> Thus we only need to show how C'j and U'j can be constructed from cj.
>
> Let cj be given by {z1,z2, . . . , zk} where the zi's are all literals derived from the variables in U. The way in which C'j and U'j are formed depends on the value of k.
>
> Case 1. k=1.  U'j = {y1j, y2j}
>                C'j = {{z1,y1j,y2j},{z1,y1j,y2j-bar},{z1,y1j-bar,y2j},{z1,y1j-bar,y2j-bar}}
> Case 2. k=2.  U'j = {y1j},  C'j = {{z1,z2,y1j},{z1,z2,y1j-bar}}
> Case 3. k=3.  U'j = phi,  C'j = {{cj}}
> Case 4. k>3.  U'j = {y1j: 1 <= i <= k-3}
>                C'j = {{z1,z2,y1j}} U {{y-bar_j^i,z_{i+2},y_j^{i+1}}: 1 <= i <= k-4}
>                     U {{y-bar_j^{k-3},z_{k-1},zk}}
>
> To prove that this is indeed a transformation, we must show that the set C' of clauses is satisfiable if and only if C is. Suppose first that t: U->{T,F} is a truth assignment satisfying C. We show that t can be extended to a truth assignment t': U'->{T,F} satisfying C'. Since the variables in U'-U are partitioned into sets U'j and since the variables in each U'j occur only in clauses belonging to C'j, we need only show how t can be extended to the sets U'j one at a time, and in each case we need only verify that all the clauses in the corresponding C'j are satisfied. We can do this as follows: If U'j was constructed under either Case 1 or Case 2, then the clauses in C'j are already satisfied by t, so we can extend t arbitrarily to U'j, say by setting t'(y) = T for all y E U'j. If U'j was constructed under Case 3, then U'j is empty and the single clause in C'j is already satisfied by t. The only remaining case is Case 4, which corresponds to a clause {z1,z2, . . . , zk} from C with k>3. Since t is a satisfying truth assignment for C, there must be a least integer l such that the literal zl is set true under t. If l is either 1 or 2, then we set t'(y_j^i) = F for 1 <= i <= k-3. If l is either k-1 or k, then we set t'(y_j^i) = T for 1 <= i <= k-3. Otherwise we set t'(y_j^i) = T for 1 <= i <= l-2 and t'(y_j^i) = F for l-1 <= i <= k-3. It is easy to verify that these choices will insure that all the clauses in C'j will be satisfied, so all the clauses in C' will be satisfied by t'. Conversely, if t' is a satisfying truth assignment for C', it is easy to verify that the restriction of t' to the variables in U must be a satisfying truth assignment for C. Thus C' is satisfiable if and only if C is.
>
> To see that this transformation can be performed in polynomial time, it suffices to observe that the number of three-literal clauses in C' is bounded by a polynomial in mn. Hence the size of the 3SAT instance is bounded above by a polynomial function of the size of the SAT instance, and, since all details of the construction itself are straightforward, the reader should have no difficulty verifying that this is a polynomial transformation.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

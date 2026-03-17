---
name: Rule
about: Propose a new reduction rule
title: "[Rule] FEEDBACK VERTEX SET to CODE GENERATION FOR PARALLEL ASSIGNMENTS"
labels: rule
assignees: ''
---

**Source:** FEEDBACK VERTEX SET
**Target:** CODE GENERATION FOR PARALLEL ASSIGNMENTS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO6

## GJ Source Entry

> [PO6]  CODE GENERATION FOR PARALLEL ASSIGNMENTS
> INSTANCE:  Set V = {v1,v2,...,vn} of variables, set A = {A1,A2,...,An} of assignments, each Ai of the form "vi←op(Bi)" for some subset Bi ⊆ V, and a positive integer K.
> QUESTION:  Is there an ordering vπ(1),vπ(2),...,vπ(n) of V such that there are at most K values of i, 1≤i≤n, for which vπ(i) ∈ Bπ(j) for some j > i?
> Reference:  [Sethi, 1973]. Transformation from FEEDBACK VERTEX SET.
> Comment:  Remains NP-complete even if each Bi satisfies |Bi| ≤ 2.

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

- **[Sethi, 1973]**: [`Sethi1973`] R. Sethi (1973). "A note on implementing parallel assignment instructions". *Information Processing Letters* 2, pp. 91–95.
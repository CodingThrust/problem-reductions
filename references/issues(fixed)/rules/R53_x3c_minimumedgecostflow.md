---
name: Rule
about: Propose a new reduction rule
title: "[Rule] EXACT COVER BY 3-SETS to MINIMUM EDGE-COST FLOW"
labels: rule
assignees: ''
---

**Source:** EXACT COVER BY 3-SETS
**Target:** MINIMUM EDGE-COST FLOW
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND32, p.214

## GJ Source Entry

> [ND32] MINIMUM EDGE-COST FLOW
> INSTANCE: Directed graph G=(V,A), specified vertices s and t, capacity c(a)∈Z^+ and price p(a)∈Z_0^+ for each a∈A, requirement R∈Z^+, bound B∈Z^+.
> QUESTION: Is there a flow function f: A→Z_0^+ such that
> (1) f(a)≤c(a) for all a∈A,
> (2) for each v∈V−{s,t}, Σ_{(u,v)∈A} f((u,v)) = Σ_{(v,u)∈A} f((v,u)), i.e., flow is "conserved" at v,
> (3) Σ_{(u,t)∈A} f((u,t)) − Σ_{(t,u)∈A} f((t,u)) ≥ R, i.e., the net flow into t is at least R, and
> (4) if A'={a∈A: f(a)≠0}, then Σ_{a∈A'} p(a)≤B?
> Reference: [Even and Johnson, 1977]. Transformation from X3C.
> Comment: Remains NP-complete if c(a)=2 and p(a)∈{0,1} for all a∈A. Solvable in polynomial time if c(a)=1 for all a∈A [Even and Johnson, 1977] or if (4) is replaced by Σ_{a∈A} p(a)·f(a)≤B (e.g., see [Lawler, 1976a]). However, becomes NP-complete once more if (4) is replaced by Σ_{a∈A}(p_1(a)f(a)^2+p_2(a)f(a))≤B [Herrmann, 1973].

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

- **[Even and Johnson, 1977]**: [`Even1977a`] S. Even and D. S. Johnson (1977). "Unpublished results".
- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.
- **[Herrmann, 1973]**: [`Herrmann1973`] P. P. Herrmann (1973). "On reducibility among combinatorial problems". Project MAC, Massachusetts Institute of Technology.
---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SATISFIABILITY to UNDIRECTED FLOW WITH LOWER BOUNDS"
labels: rule
assignees: ''
---

**Source:** SATISFIABILITY
**Target:** UNDIRECTED FLOW WITH LOWER BOUNDS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND37, p.216

## GJ Source Entry

> [ND37] UNDIRECTED FLOW WITH LOWER BOUNDS
> INSTANCE: Graph G=(V,E), specified vertices s and t, capacity c(e)∈Z^+ and lower bound l(e)∈Z_0^+ for each e∈E, requirement R∈Z^+.
> QUESTION: Is there a flow function f: {(u,v),(v,u): {u,v}∈E}→Z_0^+ such that
> (1) for all {u,v}∈E, either f((u,v))=0 or f((v,u))=0,
> (2) for each e={u,v}∈E, l(e)≤max{f((u,v)),f((v,u))}≤c(e),
> (3) for each v∈V−{s,t}, flow is conserved at v, and
> (4) the net flow into t is at least R?
> Reference: [Itai, 1977]. Transformation from SATISFIABILITY.
> Comment: Problem is NP-complete in the strong sense, even if non-integral flows are allowed. Corresponding problem for directed graphs can be solved in polynomial time, even if we ask that the total flow be R or less rather than R or more [Ford and Fulkerson, 1962] (see also [Lawler, 1976a]). The analogous DIRECTED M-COMMODITY FLOW WITH LOWER BOUNDS problem is polynomially equivalent to LINEAR PROGRAMMING for all M≥2 if non-integral flows are allowed [Itai, 1977].

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

- **[Itai, 1977]**: [`Itai1977a`] Alon Itai (1977). "Two commodity flow". Dept. of Computer Science, Technion.
- **[Ford and Fulkerson, 1962]**: [`Ford1962`] L. R. Ford and D. R. Fulkerson (1962). "Flows in Networks". Princeton University Press, Princeton, NJ.
- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.
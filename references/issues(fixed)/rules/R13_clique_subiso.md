---
name: Rule
about: Propose a new reduction rule
title: "[Rule] CLIQUE to SUBGRAPH ISOMORPHISM"
labels: rule
assignees: ''
---

**Source:** CLIQUE
**Target:** SUBGRAPH ISOMORPHISM
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.2.1, p.64

## Reduction Algorithm

> (3) SUBGRAPH ISOMORPHISM
> INSTANCE: Two graphs, G = (V1,E1) and H = (V2,E2).
> QUESTION: Does G contain a subgraph isomorphic to H, that is, a subset V ⊆ V1 and a subset E ⊆ E1 such that |V|=|V2|, |E|=|E2|, and there exists a one-to-one function f: V2->V satisfying {u,v} E E2 if and only if {f(u),f(v)} E E?
>
> Proof: Restrict to CLIQUE by allowing only instances for which H is a complete graph, that is, E2 contains all possible edges joining two members of V2.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

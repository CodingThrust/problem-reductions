---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to HAMILTONIAN PATH"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** HAMILTONIAN PATH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.1.4, p.60

## Reduction Algorithm

> Several variants of HAMILTONIAN CIRCUIT are also of interest. The HAMILTONIAN PATH problem is the same as HC except that we drop the requirement that the first and last vertices in the sequence be joined by an edge. HAMILTONIAN PATH BETWEEN TWO POINTS is the same as HAMILTONIAN PATH, except that two vertices u and v are specified as part of each instance, and we are asked whether G contains a Hamiltonian path beginning with u and ending with v. Both of these problems can be proved NP-complete using the following simple modification of the transformation just used for HC. We simply modify the graph G' obtained at the end of the construction as follows: add three new vertices, a0, a_{K+1}, and a_{K+2}, add the two edges {a0,a1} and {a_{K+1},a_{K+2}}, and replace each edge of the form {a1,(v,e_{v[deg(v)]},6)} by {a_{K+1},(v,e_{v[deg(v)]},6)}. The two specified vertices for the latter variation of HC are a0 and a_{K+2}.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)

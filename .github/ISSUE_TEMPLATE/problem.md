---
name: Problem
about: Propose a new problem type
title: "[Model] Problem name"
labels: model
assignees: ''

---

## Definition

**Name:** <!-- e.g. MaximumIndependentSet. Use Maximum/Minimum prefix for optimization problems -->
**Reference:** <!-- URL or citation for the formal definition -->

<!--
Formal definition: input, feasibility constraints, and objective.
Define all symbols (e.g. G, V, E, S, K) before using them.

E.g. "Given an undirected graph G=(V,E) where V is the vertex set and E is the edge set,
find S ⊆ V such that no two vertices in S are adjacent, maximizing |S|."
-->

## Variables

<!--
How the problem maps to a configuration vector x = (x_0, ..., x_{n-1}).
Use symbols defined above.
-->

- **Count:** <!-- e.g. n = |V| (one variable per vertex) -->
- **Per-variable domain:** <!-- e.g. binary {0,1}, or {0,...,K-1} for K colors -->
- **Meaning:** <!-- e.g. x_i = 1 if vertex i ∈ S (selected in independent set) -->

## Schema (data type)

<!--
Describe the data fields that define a problem instance.
Connect fields to the symbols defined above.
-->

**Type name:** <!-- e.g. MaximumIndependentSet -->
**Variants:** <!-- e.g. graph topology (SimpleGraph, GridGraph, UnitDiskGraph), weighted or unweighted -->

| Field | Type | Description |
|-------|------|-------------|
| <!-- e.g. graph --> | <!-- e.g. SimpleGraph --> | <!-- e.g. the graph G=(V,E) --> |
| <!-- e.g. weights --> | <!-- e.g. Vec<W> --> | <!-- e.g. vertex weights w_i for each i ∈ V (weighted variant only) --> |

## Complexity

- **Decision complexity:** <!-- e.g. NP-complete, NP-hard, P, etc. -->
- **Best known exact algorithm:** <!-- e.g. O(1.1996^n) by Xiao & Nagamochi (2017), where n = |V| -->
- **Best known approximation:** <!-- e.g. no PTAS unless P=NP; or 2-approximation via greedy -->

## How to solve
<!--
Solver is required for reduction rule verification purpose.
- Can it be solved by (existing) bruteforce?
- Can it be solved by reducing the integer programming? If so, how to reduce?
- If none apply
-->

## Example Instance

<!--
A small but non-trivial instance with known optimal solution, for testing and the paper.
Should be large enough to exercise the problem's constraints meaningfully (avoid trivial cases like triangle graphs).
E.g. "Petersen graph: |V|=10, |E|=15, 3-regular. Optimal IS size = 4, and more details.."

This example will be shown in our paper, where you could find some references.
-->

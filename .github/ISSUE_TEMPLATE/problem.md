---
name: Problem
about: Propose a new problem type
title: "[Model] Problem name"
labels: model
assignees: ''

---

## Motivation

<!-- One sentence: why is this problem useful to include? E.g. "Widely used in network design and has known reductions to QUBO." -->

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
| <!-- e.g. weights --> | <!-- e.g. list of W --> | <!-- e.g. vertex weights w_i for each i ∈ V (weighted variant only) --> |

## Complexity

- **Best known exact algorithm:** <!-- e.g. O(1.1996^n) by Xiao & Nagamochi (2017), where n = |V| -->
- **References:** <!-- URL or citation for complexity results -->

## Extra Remark

<!--
Optional notes about the problem that are worth mentioning in the paper or rustdoc.
E.g. historical context, notable applications, relationship to other problems, or special properties.
-->

## Reduction Rule Crossref

<!--
At least one reduction rule (to or from this problem) must exist or be proposed,
so the new problem is connected to the reduction graph.
Link to existing rule issues or file new ones.
-->

- [ ] #issue-number <!-- e.g. [Rule] NewProblem to QUBO -->

## How to solve
<!--
Solver is required for reduction rule verification purpose.
-->
- [ ] It can be solved by (existing) brute force.
- [ ] It can be solved by reducing to ILP via #issue-number (please file a new rule issue if one does not exist).
- [ ] Other, refer to ...

## Example Instance

<!--
A small but non-trivial instance for testing and the paper.
Should be large enough to exercise the problem's constraints meaningfully (avoid trivial cases like triangle graphs).
E.g. "Petersen graph: |V|=10, |E|=15, 3-regular."

This example will be shown in our paper, where you could find some references.
-->

## Expected Outcome

<!--
Optimization: provide one optimal configuration and its objective value.
Satisfaction: provide one valid / satisfying configuration and a brief justification.
-->

## BibTeX

<!-- Machine-readable citation for the definition/complexity references. E.g.
```bibtex
@article{Author2021,
  title={Paper title},
  author={Last, First and Last, First},
  journal={Journal Name},
  year={2021},
  doi={10.xxxx/xxxxx}
}
```
-->

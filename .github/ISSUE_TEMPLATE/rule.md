---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Source to Target"
labels: rule
assignees: ''

---

## Source
<!-- e.g. MaximumIndependentSet. Browse existing problems: https://codingthrust.github.io/problem-reductions/ -->

## Target
<!-- e.g. QUBO -->

## Motivation
<!-- Why is this reduction useful? E.g.
- Connects orphan problem X to the main reduction graph
- Enables solving X on quantum annealers via QUBO formulation
- Well-known textbook reduction (Garey & Johnson, Karp's 21, etc.)
-->

## Reference
<!-- Author(s), title, journal/conference, year, and URL or DOI -->

## Reduction Algorithm

<!-- How to construct a Target instance from a Source instance.

1. Define notation: list all symbols for the source instance (e.g. G=(V,E), n=|V|, m=|E|)
   and the target instance (e.g. Q ∈ R^{n×n}).
2. Variable mapping: how source variables map to target variables.
3. Constraint/objective transformation: formulas, penalty terms, etc.

Solution extraction follows from the variable mapping, no need to describe separately.
-->

## Size Overhead

<!-- How large is the target instance relative to the source size?
Use the symbols defined in the Reduction Algorithm above. -->

| Target metric | Formula (using symbols above) |
|---------------|-------------------------------|
| <!-- e.g. number of variables --> | <!-- e.g. n = |V| --> |
| <!-- e.g. number of edges --> | <!-- e.g. m = |E| --> |

## Validation Method

<!-- How to verify the reduction is correct beyond closed-loop testing?
E.g.
- Generate ground truth from ProblemReductions.jl: https://github.com/GiggleLiu/ProblemReductions.jl
- Compare source/target instance pairs against known results
- Use external solver to cross-check
-->

## Example

<!-- A small but non-trivial worked example for the paper illustration.
Structure your example as follows:

1. **Source instance:** Describe the input (e.g. graph, formula, sequence).
2. **Construction:** Show how each step of the reduction algorithm transforms it.
3. **Target instance:** Show the resulting target problem data (e.g. QUBO matrix, ILP constraints).

Must be small enough for brute-force solving, but large enough to exercise the reduction meaningfully.
Please provide as many details as possible, because
1. this example will appear in the paper.
2. AI needs this information to generate example code and derive round-trip tests from it. You do **not** need to provide a solved witness manually.

Please check existing examples in our paper for references.
-->

## BibTeX

<!-- Machine-readable citation for the reference above. E.g.
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

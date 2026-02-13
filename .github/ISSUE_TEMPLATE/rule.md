---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Source to Target"
labels: rule
assignees: ''

---

**Source:** <!-- e.g. MaximumIndependentSet. Browse existing problems: https://codingthrust.github.io/problem-reductions/ -->
**Target:** <!-- e.g. QUBO -->
**Reference:** <!-- URL, paper, or textbook citation for this reduction -->

## Reduction Algorithm

<!-- How to construct a Target instance from a Source instance.

1. Define notation: list all symbols for the source instance (e.g. G=(V,E), n=|V|, m=|E|)
   and the target instance (e.g. Q ∈ R^{n×n}).
2. Variable mapping: how source variables map to target variables.
3. Constraint/objective transformation: formulas, penalty terms, etc.

Solution extraction follows from the variable mapping, no need to describe separately.
-->

## Size Overhead

<!-- How large is the target instance as a polynomial of the source size?
Use the symbols defined in the Reduction Algorithm above.
Also provide the code-level metric name (from the problem's `problem_size()` method). -->

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| <!-- e.g. num_vars --> | <!-- e.g. n = |V| --> |
| <!-- e.g. num_edges --> | <!-- e.g. m = |E| --> |

## Validation Method

<!-- How to verify the reduction is correct beyond closed-loop testing?
E.g.
- Generate ground truth from ProblemReductions.jl: https://github.com/GiggleLiu/ProblemReductions.jl
- Compare source/target instance pairs against known results
- Use external solver to cross-check
-->

## Example Source Instance

<!-- A small but non-trivial source instance for the paper illustration.
Must be small enough for brute-force solving, but large enough to exercise the reduction meaningfully. E.g. "petersen graph: |V|=10, |E|=15, 3-regular" should be perfect.
Please provide as many details as possible, because
1. this example will appear in the paper.
2. AI needs this information to generate example code, run it, and try to compare with what you provided.

Please check existing examples in our paper for references.
-->

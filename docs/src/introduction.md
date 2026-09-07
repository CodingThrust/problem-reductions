# Problem Reductions

Problem Reductions is a Rust library and command-line tool for NP-hard problems and the reductions between them. Each problem is a model with a configuration space and an objective. Each reduction is a registered rule that maps an instance of one problem to an instance of another and maps solutions back. Searching the reduction graph yields a route from a problem to a solver, such as integer linear programming, with recovery of a solution to the original instance. The catalog currently holds:

{{#include generated/catalog-counts.md}}

- [Atlas](index.html#atlas): every problem variant and reduction, with schemas and overheads
- [Paper](reductions.pdf): definitions, constructions, and proofs
- [Rust API](api/problemreductions/index.html): generated from source

This guide covers the `pred` CLI, the agent skills shipped with the repository, and the Rust library. Every page has a **Markdown** link for reading without a browser; the [Markdown index](markdown/index.md) lists all pages.

## Cite

```bibtex
@misc{pan2026problemreductionsscaleagentic,
  title         = {Problem Reductions at Scale: Agentic Integration of Computationally Hard Problems},
  author        = {Xi-Wei Pan and Shi-Wen An and Jin-Guo Liu},
  year          = {2026},
  eprint        = {2604.11535},
  archivePrefix = {arXiv},
  primaryClass  = {cs.AI},
  url           = {https://arxiv.org/abs/2604.11535},
}
```

## Research scope

The long-term goal is autonomous discovery of new reduction rules. Today the repository provides executable models, registered reductions, solver routing, and agent workflows for proposals, implementation, and review. A rule needs a mathematical argument as well as tests; passing finite examples does not establish a general proof.

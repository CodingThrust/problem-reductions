# Problem Reductions

**problem-reductions** is a rust library that provides implementations of various computational hard problems and reduction rules between them. It is designed for algorithm research, education, and industry applications.

## Reduction Graph

<script src="https://unpkg.com/elkjs@0.9.3/lib/elk.bundled.js"></script>
<script src="https://unpkg.com/cytoscape-elk@2.2.0/dist/cytoscape-elk.js"></script>
<script src="https://unpkg.com/cytoscape-svg@0.4.0/cytoscape-svg.js"></script>

<div id="cy-search">
  <input id="search-input" type="text" placeholder="Search problems...">
</div>
<div id="cy"></div>
<div id="cy-controls">
  <div id="legend">
    <span class="swatch" style="background:#c8f0c8;"></span>Graph
    <span class="swatch" style="background:#c8c8f0;"></span>Formula
    <span class="swatch" style="background:#f0c8c8;"></span>Set
    <span class="swatch" style="background:#f0f0a0;"></span>Algebraic
    <span class="swatch" style="background:#f0c8e0;"></span>Misc
    <span style="display:inline-block;width:20px;height:0;border-top:2px dashed #bbb;margin-left:10px;margin-right:3px;vertical-align:middle;"></span>Variant Cast
  </div>
  <div>
    <span id="instructions">Click a node to start path selection</span>
    <button id="clear-btn">Clear</button>
    <button id="download-svg-btn">Download SVG</button>
  </div>
</div>
<div id="cy-help">
  Click a problem node to expand/collapse its variants.
  Click a variant to filter its edges.
  Click two nodes to find a reduction path.
  Double-click for API docs (nodes) or source code (edges).
  Scroll to zoom, drag to pan.
</div>
<div id="cy-tooltip"></div>

You can also explore this graph from the terminal with the [CLI tool](./cli.md). For theoretical background and correctness proofs, see the [PDF manual](https://codingthrust.github.io/problem-reductions/reductions.pdf).

## Our Vision

Computational complexity theory has produced a rich body of polynomial-time reductions between NP-hard problems, yet these results largely remain confined to papers. The gap between theoretical algorithms and working software leads to two persistent inefficiencies:

- **Solver underutilization.** State-of-the-art solvers (SAT solvers, ILP solvers, QUBO annealers) each target a single problem formulation. In principle, any problem reducible to that formulation can leverage the same solver — but without a systematic reduction library, practitioners must re-derive and re-implement each transformation.
- **Redundant effort.** Problems that are polynomial-time equivalent are, from a computational standpoint, interchangeable. Without infrastructure connecting them, the same algorithmic insights are independently reimplemented across domains.

Our goal is to build a comprehensive, machine-readable reduction graph: a directed graph in which every node is a computational problem and every edge is a verified polynomial-time reduction. Given such a graph, one can automatically compose reduction paths to route any source problem to any reachable target solver.

A key enabler is AI-assisted implementation. We propose a pipeline of `algorithm → paper → software`, in which AI agents translate published reduction proofs into tested code. The critical question — can AI-generated reductions be trusted? — has a concrete answer: nearly all reductions admit **closed-loop verification**. A round-trip test reduces a source instance to a target, solves the target, extracts the solution back, and checks it against a direct solve of the source. This property makes correctness mechanically verifiable, independent of how the code was produced.

<div class="theme-light-only">

![](static/workflow-loop.svg)

</div>
<div class="theme-dark-only">

![](static/workflow-loop-dark.svg)

</div>

This library is the foundation of that effort: an open-source, extensible reduction graph with verified implementations, designed for contributions from both human researchers and AI agents.

## Call for Contributions

> **No programming experience required.** You contribute domain knowledge — we handle the implementation.

### How it works

1. **File an issue** — use the [Problem](https://github.com/CodingThrust/problem-reductions/issues/new?template=problem.md) or [Rule](https://github.com/CodingThrust/problem-reductions/issues/new?template=rule.md) template. Describe the problem or reduction you have in mind; the template guides you through the details.
2. **We implement it** — for reasonable requests, maintainers tag the issue `implement` and AI agents generate a tested implementation.
3. **We present it to you** — all issue contributors are invited to community calls (via [Zulip](https://problem-reductions.zulipchat.com/)), where maintainers walk through the implementation — documentation, CLI behavior, correctness — and you provide feedback.

### Authorship

Contribute 10 non-trivial reduction rules and you'll be added to the author list of the [paper](https://codingthrust.github.io/problem-reductions/reductions.pdf).

For manual implementation, see the [Design](./design.md#contributing) guide.

## License

MIT License

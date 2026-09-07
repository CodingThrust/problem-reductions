# Reduction graph

<script src="https://unpkg.com/cytoscape-svg@0.4.0/cytoscape-svg.js"></script>

<div id="cy-search">
  <input id="search-input" type="text" placeholder="Search problems..." aria-label="Search reduction graph">
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

For exact variants and structured output, use [CLI path queries](cli-commands.md#paths).

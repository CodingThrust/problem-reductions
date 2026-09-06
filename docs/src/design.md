# Architecture

The Rust library stores problem definitions, executable reductions, and their registry metadata. The CLI and MCP server expose that core to tools and agents.

| Location | Responsibility |
|---|---|
| `src/models/` | Models grouped by graph, formula, set, algebraic, or miscellaneous input |
| `src/rules/` | Reduction implementations and solution/value mappings |
| `src/registry/` | Concrete variant metadata and dynamic dispatch |
| `src/solvers/` | Exhaustive, ILP, specialized, and decision-search solvers |
| `problemreductions-cli/` | CLI and optional MCP server |
| `src/example_db/` | Canonical model and rule examples |
| `src/unit_tests/` | Tests mirroring the source tree |

## Module map


<div id="module-graph"></div>
<div id="mg-controls">
  <div id="mg-legend">
    <span class="swatch" style="background:#c8f0c8;"></span>Core
    <span class="swatch" style="background:#c8c8f0;"></span>Models
    <span class="swatch" style="background:#f0d8b0;"></span>Rules
    <span class="swatch" style="background:#b0e0f0;"></span>Registry
    <span class="swatch" style="background:#d0f0d0;"></span>Solvers
    <span class="swatch" style="background:#e0e0e0;"></span>Utilities
  </div>
</div>
<div id="mg-help">
  Click a module to expand/collapse its public items.
  Double-click to open rustdoc.
</div>
<div id="mg-tooltip"></div>

Read the [problem contract](design-problem.md), [reduction contracts](design-reductions.md), and [variant system](design-variants.md) before extending the library.

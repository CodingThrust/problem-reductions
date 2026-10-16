# Research website

The static Home/Atlas page and reduction-path explorer use the same Rust registry,
WASM mathematics and Typst articles as the guide and paper. Preserve their current
appearance. See [ENGINEERING.md](ENGINEERING.md) for pinned tools, production
verification, size budgets, deployment/rollback and outstanding acceptance gates;
[DATA_BOUNDARIES.md](DATA_BOUNDARIES.md) inventories runtime contracts.
[ACCESSIBILITY_CHECKS.md](ACCESSIBILITY_CHECKS.md) owns manual acceptance.

## Source layout

- `index.html`, `graph.html`: Home/Atlas and Graph entry markup and preloads.
- `assets/site.ts`, `site.css`: Home/Atlas navigation, search and reference pages.
- `assets/graph-startup.ts`, `graph.ts`, `graph-scene.ts`, `graph.css`: validated
  scene download, graph controller and native SVG rendering/gestures.
- `assets/path-worker.ts`: Rust/WASM search and comparison; cancellation terminates
  the worker, successful queries retain it, and results use an eight-entry cache.
- `assets/details.ts`, `details.css`: shared lazy article drawer/index and
  24-entry article cache. `graph-growth.mts` displays exact certificate degrees.
- `*.schema.json`, `assets/*-data.mts`: eleven contracts and their cross-field
  checks. `assets/*.validator.mjs`, matching declarations and `validator-runtime*.mjs`
  are committed generated outputs; edit schemas and regenerate, never edit guards.
- `assets/graph-layout.js`: Node-only options used by the Python build's layout
  generator; it is not a browser module.
- `assets/fonts/`: licensed WOFF2 fonts. Home/guide retain the original static DM
  Sans weights; Graph retains its variable font and smaller unhinted italic subset.
  The full hinted italic face remains separate to preserve both pages' rendering.
- `formulas/`: Typst inputs compiled to the homepage's equation SVGs.

No registry exports, generated articles, WASM binaries, screenshots or benchmark
samples belong here. Build outputs go into `book/` or a temporary verification
artifact, with assets under immutable `releases/<build-number>/assets/` URLs.

## Build and verify

```sh
npm ci
npm run generate:frontend-contracts # after changing contracts; includes Rust schemas
npm run check:frontend-contracts    # compare schemas, declarations, guards and helpers
npm run typecheck
npm run lint:frontend
npm run test:frontend-contracts
```

Use the full fresh production command in [ENGINEERING.md](ENGINEERING.md) before
shipping. For a preview, `make website` builds the main site and guide; serve
`book/` over HTTP. The complete verifier also assembles the PDF/API and checks
root/nested hosting, CSP, lazy delivery, mathematical workflows and failure recovery
in Chromium, Firefox and WebKit. Include the separately built Open Question Board
with `scripts/check_website.py --open-questions /path/to/board` when available;
CI requires it. Never mix files from different finalized releases.

The build chain is `build_website.py` (validated registry data, layout, Typst,
WASM), `build_frontend.mjs` (two-page Vite emission and size budget), then
`finalize_website.py` (release URLs and CSP). `check_website.py` orchestrates a fresh
artifact and records provenance. CI uses the same contracts and production tests.

## Evidence and measurements

The maintained size budgets and cold-start measurements live in
[ENGINEERING.md](ENGINEERING.md). Raw evidence stays outside Git.

The guide source is `docs/src/`; its reading theme is
`docs/src/static/docs-theme.css`. Articles come from `docs/paper/reductions.typ`
through `scripts/build_graph_details.py`, without a second mathematical source.
Refresh the real CLI recording with
`python scripts/record_cli_demo.py --pred target/debug/pred` after CLI changes.

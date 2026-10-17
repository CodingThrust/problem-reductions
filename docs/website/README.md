# Research website

The static Home/Atlas page and reduction-path explorer use the same Rust registry,
WASM mathematics and Typst articles as the guide and paper. Preserve their current
appearance. See [ENGINEERING.md](ENGINEERING.md) for pinned tools, production
verification, size budgets, deployment/rollback and outstanding acceptance gates;
[DATA_BOUNDARIES.md](DATA_BOUNDARIES.md) inventories runtime contracts.
[ACCESSIBILITY_CHECKS.md](ACCESSIBILITY_CHECKS.md) owns manual acceptance.

## Source layout

- `index.html`, `graph.html`: Home/Atlas and Graph entry markup and preloads.
- `src/pages/`: `site.ts` (Home/Atlas navigation, search, reference pages);
  `graph-startup.ts`, `graph.ts`, `graph-scene.ts` (validated scene download, graph
  controller, native SVG rendering/gestures); `details.ts` (shared lazy article
  drawer/index and 24-entry article cache).
- `src/workers/path-worker.ts`: Rust/WASM search and comparison; cancellation terminates
  the worker, successful queries retain it, and results use an eight-entry cache.
- `src/lib/`: `*-data.mts` boundary checks, `graph-growth.mts`
  (exact certificate degrees) and `graph-layout.js` (Node-only layout options for the
  Python build; not a browser module).
- `src/styles/`: `site.css`, `graph.css`, `details.css`.
- `*.schema.json`: JSON contracts. `src/generated/` holds the committed
  validators, declarations and `validator-runtime*.mjs`; edit schemas and regenerate,
  never edit guards.
- `assets/fonts/`: licensed WOFF2 fonts. Pages share variable DM Sans and Instrument
  Serif; Graph adds DM Mono.
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

The maintained size budgets and delivery checks live in
[ENGINEERING.md](ENGINEERING.md). Raw evidence stays outside Git.

The guide source is `docs/src/`; its reading theme is
`docs/src/static/docs-theme.css`. Articles come from `docs/paper/reductions.typ`
through `scripts/build_graph_details.py`, without a second mathematical source.
Refresh the real CLI recording with
`python scripts/record_cli_demo.py --pred target/debug/pred` after CLI changes.

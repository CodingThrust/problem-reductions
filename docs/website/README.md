# Research website

The public entry point presents Problem Reductions as infrastructure for autonomous
research. It includes a searchable problem atlas, pages for every registered
problem variant and reduction, and an interactive vertex-cover / independent-set
example. Current implementation capabilities and future research ambitions are
explicitly distinguished.

## Build and preview

```sh
make website
python3 -m http.server 3001 --bind 127.0.0.1 --directory book
```

Open <http://localhost:3001>. `make doc` also builds Rust API documentation.
The deployment workflow builds the PDF and API and combines everything in `book/`.
The fast `make website` preview includes the guide; API/PDF links require those
additional builds, as in deployment.

For CSS/JavaScript iteration after the registry exports have been generated:

```sh
python3 scripts/build_website.py
```

The builder overlays the website's `index.html` and `assets/` onto mdBook's output.
Existing documentation URLs, `introduction.html`, the paper, and Rust API paths
stay available. Running `mdbook build` by itself restores mdBook's homepage; run
the website builder afterward. The GitHub Pages workflow does this automatically.

## Design and data

- Deep forest surfaces, warm ivory text, and pale lime form the dark visual palette.
- The header, footer, and favicon use the original `docs/logo.svg` artwork, recolored
  at build time to match the dark palette. The canonical source logo stays unchanged.
- Self-hosted DM Sans and Instrument Serif include their OFL licenses in `assets/fonts/`.
- The graph is a generated SVG based on actual registry edges. Its homepage view
  groups variants by problem family and does not imply that arbitrary paths compose.
- Detail pages use exact variants. URLs encode problem names and variant keys,
  rather than transient registry indices.
- Counts, schemas, capabilities, and overhead expressions come from the same
  generated JSON used by the documentation and paper. Nothing is manually counted.
- The five-vertex example is illustrative; research activity, novelty, and live test
  results are never fabricated. A reduction's registration does not imply formal proof.
- Navigation, filters, search, clipboard actions, and the example are keyboard
  accessible. Layouts adapt to mobile; motion respects reduced-motion preferences.
- No frontend framework, third-party runtime, or external font request is required.

The static website does not yet provide agent execution, an experiment database,
or live research telemetry. The research section describes the intended loop and
links to the existing agent workflows.

## Browser checks

Build the full documentation with `make doc` so API link checks have their targets,
then run these commands with the preview server running:

```sh
uv run --no-project --with playwright python -m playwright install chromium
uv run --no-project --with playwright python scripts/test_website.py
```

Set `WEBSITE_BROWSER_CHANNEL=chrome` to use an installed Chrome instead, or
`WEBSITE_BASE_URL=http://localhost:3001/` to change the preview address. The suite
exercises search, filters, exact variants, deep links, keyboard navigation,
clipboard actions, the mathematical example, legacy docs, and mobile overflow.

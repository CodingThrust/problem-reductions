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

## Documentation

The guide has nine pages listed in `docs/src/SUMMARY.md`: an overview, the CLI
(quick start, command reference, reduction graph), agent skills, the Rust library
(getting started, API, design), and a research placeholder. Keep one example per
concept and link to `pred --help`, rustdoc, or `.claude/CLAUDE.md` instead of
restating them.

`docs/src/static/docs-theme.css` supplies the restrained dark reading theme;
`docs-theme.js` adds the GitHub and atlas links. Native mdBook search, code copying, and
sidebar navigation remain available. Both mdBook 0.4.37 (deployment) and 0.5.2
are supported.

The website builder emits `book/markdown/` from the same source pages, expanding
Rust and generated-output includes. The Markdown index is the entry point for
agents.

## CLI recording

`docs/src/static/cli-demo.cast` contains real PTY output from six successful
commands against the current CLI. `cli-demo.html` embeds a copy of that cast
and asciinema-player 3.8.0 for offline playback, with autoplay disabled; the
recording script refreshes both files. The bundled
player is Apache-2.0 licensed; its license is alongside the recording.

To refresh the recording after a CLI change, build the CLI and run:

```sh
python3 scripts/record_cli_demo.py --pred target/debug/pred
```

The script runs the commands listed in `docs/src/cli.md` in a temporary
directory, simulates typing, and fails if any command exits non-zero. Never
substitute a simulated successful run.

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

The guide uses short task pages grouped in `docs/src/SUMMARY.md`: start here,
agent workflows, CLI reference, Rust library, and internals. Keep one task or
contract per page and link prerequisites and next steps. Agent prompts name the
actual repository skill file; instructions distinguish implemented capabilities
from the research goal.

`docs/src/static/docs-theme.css` supplies the restrained dark reading theme;
`docs-theme.js` adds Markdown access and redirects old section links. Native
mdBook search, code copying, and sidebar navigation remain available. Both
mdBook 0.4.37 (deployment) and 0.5.2 are supported.

The website builder emits `book/markdown/` from the same source pages, expanding
Rust and generated-output includes. The Markdown index is the entry point for
agents. The builder preserves the Markdown index URL after mdBook’s HTML link rewriting.

To check the complete assembled documentation, build the API and PDF first:

```sh
make doc
make paper
cp docs/paper/reductions.pdf book/reductions.pdf
uv run --no-project --with playwright python scripts/test_documentation.py
```

The checks cover task navigation, search, clipboard, Markdown links and includes,
legacy section redirects, mobile layout, offline playback, and replaying the
recorded CLI workflow against its expected optimum.

## CLI recording

`docs/src/static/cli-demo.cast` contains real PTY output from six successful
commands against the current CLI. `cli-demo.html` embeds that cast and
asciinema-player 3.8.0 for offline playback, with autoplay disabled. The bundled
player is Apache-2.0 licensed; its license is alongside the recording.

To refresh the recording, use the `how-to-demo-cli` skill from
[qude-software-skills](https://github.com/QudeLeap/qude-software-skills/tree/main/skills/how-to-demo-cli).
Build the CLI, create an isolated temporary directory, and place a `pred` symlink
to the new executable there. Record the commands in `docs/src/cli-demo.md`, using
`./pred` in that directory, at 88 columns × 22 rows with `--theme nord`,
`--typing-speed 35`, `--step-pause 2.5`, and `--no-autoplay`.

Retain real output and exit codes; never substitute a simulated successful run.
The checked-in HTML adds the documentation palette and a compact embedded mode
to the skill's generated wrapper. Preserve those wrapper styles when refreshing.

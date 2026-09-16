# Research website

The public entry point presents Problem Reductions as infrastructure for autonomous
research. It includes a searchable problem atlas, pages for every registered
problem variant and reduction, and an interactive vertex-cover / independent-set
example. Current implementation capabilities and future research ambitions are
explicitly distinguished.

The standalone `graph.html` is the primary visual explorer: the graph owns the
workspace between a problem/rule browser and an always-visible detail panel.
The browser lists registered families, expandable variants, and exact directed
rules; its search and mode switch leave the graph viewport unchanged. Selecting
an item highlights its graph counterpart.
Drag the divider to resize the
panel, or focus it and use the arrow keys (Home/End select the width limits).
On narrow screens the panel sits below the graph.
The fCoSE overview is computed at build time and published with the graph data,
so reloads retain the same positions. Expanding variants relaxes the visible local
neighborhood; collapsing restores the overview. Labels
are culled by available screen space, with full variant labels reserved in layout.

Details compile all problem definitions and reduction rules directly from
`docs/paper/reductions.typ`, including their examples, footnotes and references.
Graph's inspector and Atlas problem/reduction pages use the same renderer and
article files. The inspector's “Open in Atlas” link retains exact variant
endpoints; family-level selections open the corresponding default record.
Selection stays at exact variant granularity: variant-specific content wins;
otherwise the parent problem or directed problem-pair content is used by default.
If neither exists, the panel explicitly reports missing documentation.
No web prose is maintained separately. `problem-def` accepts `variant: (...)`;
`reduction-rule` accepts `source-variant: (...)` and `target-variant: (...)` for
specialized content. Omit these to publish shared family content.

`scripts/build_graph_details.py` compiles once with Typst 0.15.1 and extracts
each article into the release's `assets/details/` directory. The URL index and
articles load on selection; the 24 most recently used articles are cached. References load
when expanded. Repeated SVG glyph definitions are shared within each article.
Generated content is not copied into the website source directory or committed.
Text reflows in the panel; equations and figures use
Typst's SVG rendering to preserve mathematical notation. The existing PDF build
uses the same source and its original print presentation. HTML export remains
experimental, so the build fails on export errors.

## Build and preview

Use mdBook 0.5.2 and Typst 0.15.1, matching the deployment workflow.

```sh
make website
python3 -m http.server 3001 --bind 127.0.0.1 --directory book
```

Open <http://localhost:3001>. `make doc` also builds Rust API documentation.
The deployment workflow builds the PDF and API and combines everything in `book/`.
The fast `make website` preview includes the guide; API/PDF links require those
additional builds, as in deployment.

Website assets are published under `book/releases/<build-number>/assets/`.
The builder rewrites their URLs together, so old open pages cannot mix new
documentation with old registry data. If an old release is no longer available,
reload the page. Do not overwrite files inside an existing release directory.
The homepage loads a small registry summary; full Atlas data and the graph's
local-layout library load only when needed.

Finalization runs after PDF/API assembly. It externalizes generated inline
scripts and applies a same-origin CSP (inline styles remain allowed for graph
and Typst layout). Typst articles containing active elements, event handlers,
or unsafe URLs fail the build. Deployment runs browser and build-security checks
before upload; only the deploy job has Pages/OIDC write permissions.

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
- Model and rule “Open implementation” links follow GitHub `main`, not a pinned
  commit. Source paths come from registry metadata and are checked against files
  during the build; renamed files require a website rebuild and deployment.
- Counts, schemas, capabilities, and overhead expressions come from the same
  generated JSON used by the documentation and paper. Nothing is manually counted.
- The five-vertex example is illustrative; research activity, novelty, and live test
  results are never fabricated. A reduction's registration does not imply formal proof.
- Navigation, filters, search, clipboard actions, and the example are keyboard
  accessible. Layouts adapt to mobile; motion respects reduced-motion preferences.
- No frontend framework or external font request is required. The graph uses
  self-hosted Cytoscape and fCoSE; dependency licenses are copied beside the scripts.

The static website does not yet provide agent execution, an experiment database,
or live research telemetry. The research section describes the intended loop and
links to the existing agent workflows.

## Browser checks

Build the documentation with `make doc` so API link checks have their targets,
then build the PDF and run these commands with the preview server running:

```sh
typst compile --root . docs/paper/reductions.typ book/reductions.pdf
uv run --no-project --with playwright==1.58.0 python -m playwright install chromium
uv run --no-project --with playwright==1.58.0 python scripts/test_website.py
```

Set `WEBSITE_BROWSER_CHANNEL=chrome` to use an installed Chrome instead, or
`WEBSITE_BASE_URL=http://localhost:3001/` to change the preview address. The suite
exercises search, filters, exact variants, deep links, keyboard navigation,
clipboard actions, the mathematical example, legacy docs, and mobile overflow.

## Documentation

Open questions has its own website navigation tab at `index.html#open-questions`.

The guide has seven pages listed in `docs/src/SUMMARY.md`: an overview, the CLI
(quick start, command reference), agent skills, the Rust library
(getting started, API, design). Keep one example per
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

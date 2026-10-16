# Research website

The public entry point presents Problem Reductions as infrastructure for autonomous
research. It includes a searchable problem atlas, pages for every registered
problem variant and reduction, and an interactive vertex-cover / independent-set
example. Current implementation capabilities and future research ambitions are
explicitly distinguished.

The standalone `graph.html` keeps the site's title bar and navigation above a
full-screen reduction-path explorer, with a
floating glass panel (a bottom sheet on phones), node popovers, and a zoom dock.
It opens on SAT → QUBO. Search by name or catalog alias, choose exact variants
with option pills, or use a node popover's Start and End actions.

Positions use the exported preset layout scaled by 1.6 and one overlap-removal
pass. No browser layout library loads. Labels are culled by available screen
space. A Web Worker first finds paths using the shared Rust path search, then
compares their composed parameter certificates using the shared polynomial
comparator. Routes appear immediately, with pending size badges; comparison
fills the badges and ranks the best and slower routes without changing the
selected route. Search more increases the limit from 20 to 50, 100, then 999.
Cancel terminates the worker in either phase, and results are cached by endpoints
and limit. Same-family variant conversions are grouped in the expanded steps;
copying a route preserves every exact variant for `pred reduce --via`.

Documentation opens on demand in a right drawer (a full-screen sheet on phones).
Use Details in a node popover, a problem name in the expanded steps, a connector's
rule button, or a selected route edge. Other edges are inert. Escape or the close
button dismisses the drawer. Articles, footnotes, and references come from
`docs/paper/reductions.typ` through the existing shared renderer. The drawer's
Open in Atlas link preserves exact variant endpoints. Variant-specific content
wins; otherwise the parent problem or directed problem-pair article is used.
The drawer reserves desktop graph space without moving the camera; step and
article problem links also frame and pulse the corresponding node.

`scripts/build_graph_details.py` compiles once with Typst 0.15.1 and extracts
each article into the release's `assets/details/` directory. The graph's existing
registry payload includes the URL index, avoiding a separate index request before
the first article. Entering the Atlas starts the shared index download in the
background, alongside any selected problem's metadata. Articles load on
selection; the 24 most recently used articles are cached. References load
when expanded. Repeated SVG glyph definitions are shared within each article.
Generated content is not copied into the website source directory or committed.
Text reflows in the panel; equations and figures use
Typst's SVG rendering to preserve mathematical notation. The existing PDF build
uses the same source and its original print presentation. HTML export remains
experimental, so the build fails on export errors.

## Performance measurements and architecture investigation

Measured on 15–16 October 2026 on a 32 GiB Apple M6, Node 24.21 and Chromium 145
(Playwright 1.58), using release WASM builds. The baseline already includes the
previous website fixes. [Raw samples and regression fingerprints](performance-results.json)
record the actual inputs, memory definitions and artifact hashes. Times below
are medians of three fresh processes, excluding enumeration, WASM initialization,
JavaScript parsing and file writes. The widest registered query's baseline has
one sample. These development measurements are not device or CDN guarantees.

| Case | Returned routes | Previous comparison | New browser comparison | WASM memory, before → after |
| --- | ---: | ---: | ---: | ---: |
| SAT → QUBO, limit 20 | 20 | 0.56 s | 0.10 s | 4.88 → 3.50 MiB |
| SAT → QUBO, limit 50 | 50 | 5.91 s | 0.79 s | 16.75 → 10.88 MiB |
| SAT → QUBO, limit 100 | 100 | 22.19 s | 1.30 s | 33.44 → 20.13 MiB |
| SAT → QUBO, limit 999 | 127 | 20.42 s | 1.26 s | 38.38 → 20.31 MiB |
| Factoring → ILP(f64, bool, general), limit 999 | 269 | 26.94 s | 2.61 s | 108.31 → 29.19 MiB |
| Synthetic monomial grid, limit 999 | 999 | 29.81 s | 1.93 s | 833.19 → 10.25 MiB |

The current registry has 257 exact variants and 344 directed edges. An exhaustive
non-Turing witness-path count found at most 269 routes for an endpoint pair; SAT → QUBO has
127. A limit of 999 does not manufacture additional routes. The separate scaling
fixture supplies 999 distinct two-hop routes with certificates `n^a*m^b`. Its
498,501 verdicts are independently checked against coordinatewise exponent
comparison: 261,261 dominated and 237,240 incomparable pairs, with one front path.

Across all seven benchmark cases, the complete compatible API preserves every
certificate, all **553,863 pair verdicts**, and every front exactly. The browser
summary agrees with that complete output. The fixture's response falls from
31.78 MB to 0.108 MB. WASM memory means linear-memory capacity after completion,
which records allocation high water rather than live object bytes. Peak process
RSS, including V8 and JavaScript, falls from a median 1,076 MiB to 111 MiB on
that fixture. The complete pair API remains available: its new 999-route run
uses 438 MiB of WASM memory and takes 4.74 s, so consumers needing only the front
and counts should use the summary API.

Reproduce the computation benchmarks after building the site:

```sh
node scripts/benchmark_website_paths.mjs book --repeat 3 --output /tmp/path-results.json
# Compare against an archived pre-change binding JS + raw WASM directory:
node scripts/benchmark_website_paths.mjs book --baseline /path/to/previous/assets --repeat 3
# A focused scaling run:
node scripts/benchmark_website_paths.mjs book --case synthetic-monomial-grid-999 --repeat 1
```

Each sample starts a real WASM engine in a separate process, with a 180 s timeout.
The script checks exact profile equality, preserves all previously decided pair
verdicts, compares full and summary results, and applies an independent oracle
to the monomial fixtures. No network or computation is mocked.

### What the profile changed

The website is a static client of generated Rust registry exports. The web crate
compiles the same expression, parameter-transform, polynomial, path-search and
comparison sources as the native library, without linking the model catalogue or
HiGHS into WASM. One worker handles symbolic computation; Cytoscape handles the
collapsed family graph; Typst articles are generated at build time.

Native stage samples put parsing and composing 100 routes at about **25 ms**,
expanding their final certificates at **3.8 s**, and the original comparison at
**11.3 s**. The batch contains 162 distinct polynomials and 25,390 terms; one
certificate has 6,545 terms. Consequently, parsing caches and shared-prefix
composition caches would have saved little. The changes address the measured
arithmetic and comparison work instead:

- Integer polynomial products use exact BigInt accumulation. Moderately dense
  exponent boxes use a mixed-radix coefficient array, bounded to 65,536 BigInts;
  sparse boxes and large exponents use a hash map. Fractional coefficients retain
  exact rational multiplication. This removes per-product monomial-tree creation
  and fraction reduction from the dense case. Both paths return the same canonical
  polynomial. The dense path improved the 100-route run by roughly 20% over the
  integer hash-map version in a separate three-sample comparison.
- Comparison keeps the upper support needed for nonnegative exponent directions.
  It removes coordinatewise dominated points and points covered by an axis
  simplex. Axis-simplex coverage constructs exact rational weights directly;
  other convex coverage retains the bounded exact Fourier–Motzkin solver. Tests
  include convex redundancy that previously exhausted the LP budget and a mixed
  support case that caught a regression during this investigation.
- Polynomial IDs use hash lookup. Positive hulls share their existing polynomial
  storage. Curve evaluations share immutable results. The witness catalogue is
  built once per batch/source coordinate set. Rebuilding it for every pair was a
  hidden cubic cost exposed by the genuine 999-route fixture.
- Witness support heights depend on exponents, so a failed height gap eliminates
  every scale variant immediately. Alternative scales are generated only when
  needed for signed validity. All original signed certificates remain in the
  batch; coordinate, escape, positivity and exact-substitution obligations remain.
- The browser compares groups with equal upper growth support, counts group pairs
  with their multiplicities, and keeps every original route/profile. It does not
  allocate the full pair matrix. Native proof output and the compatible full-pair
  WASM endpoint remain available. Moving JSON arrays into the response also avoids
  duplicating the complete legacy pair payload.
- The idle worker survives successful queries, preserving its loaded module and
  optimized code. Cancellation or failure still terminates it. Query results use
  an eight-entry LRU cache; growth summaries use weak references. The idle worker
  retains its memory high water until cancellation or navigation. Each comparison
  creates a fresh mathematical batch, so cached refutations cannot leak across
  different route limits.

Let `P` be routes, `G` distinct growth-support groups, `A,B` input term counts,
`T` output terms, `H` retained support points, and `d` coordinates:

| Choice | Work / storage | Assessment for this project |
| --- | --- | --- |
| BTreeMap rational convolution → hybrid integer convolution | Originally `AB` rational products plus tree insertions; now `AB` integer products with array indexing or expected constant-time hashing, then canonical output sorting. Dense scratch slots are bounded; coefficient precision remains arbitrary. | Adopted; changes representation without changing algebra. Worst-case expansion is still exponential. |
| Full pair records → grouped summary | `O(P²)` output/storage becomes `O(G²)` comparisons plus grouping and `O(P)` output; all-unique groups still require quadratic comparison work. Batch curve caches also depend on unique polynomials and curves. | Adopted for the UI. Complete proof/pair APIs retain their output costs. |
| Raw Newton support → reduced upper support | Axis filtering is linear in support size and dimension, apart from exact arithmetic; the remaining skyline costs `O(T log T + T H d)`. General LP elimination remains exponential and capped. | Adopted; every discarded point has convex or coordinatewise coverage. No approximate hull decisions. |
| Bounded Yen/BFS → sidetrack simple-path search | Current enumeration is roughly 10–35 ms for registered queries and 0.15–0.18 s for the 999-route fixture. More complex search could improve larger graphs but would need identical simple-path and tie-order behavior. | Keep Yen. Ordinary Eppstein allows cycles and is not a drop-in replacement. |
| Exact FM → exact simplex / floating candidate plus exact verification | These could avoid elimination-row explosion on large irregular supports; a floating answer alone cannot certify a comparison. New solver code/dependencies and WASM support would need validation. | Not adopted: all 390 Unknown pairs at 100 SAT routes involve unavailable certificates, rather than an unresolved numerical LP. Faster LP cannot invent missing contracts. |
| Single WASM worker → JS rewrite, server computation or worker pool | A JS rewrite duplicates the exact proof kernel. A server changes static/offline operation. A pool adds scheduling, data copies and memory; shared-memory WASM also needs cross-origin isolation headers. | Keep the shared Rust worker. Native code was slow too; the main gains came from algorithms and representation. Alternative runtime speedups were not assumed or fabricated. |

The integer representation follows the established
[Kronecker encoding approach for sparse/dense multiplication](https://arxiv.org/abs/1004.4548).
Research on [simple-path sidetrack algorithms](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ISAAC.2016.49)
clarifies the distinction from cycle-permitting k-shortest walks.
[CGAL's exact LP documentation](https://doc.cgal.org/latest/QP_solver/index.html)
provides a credible future solver direction if irregular support LPs become a
measured bottleneck. [V8's compilation pipeline](https://v8.dev/docs/wasm-compilation-pipeline)
explains the value of retaining the worker; [shared-memory requirements](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/crossOriginIsolated)
make a threaded migration a hosting and memory decision as well as a CPU decision.

### Rendering and loading alternatives

The fixed overlap-removal pass now runs in the layout generator, preserving the
published geometry and removing its repeated `O(80 F²)` browser work. Native
scene interaction still uses Cytoscape. Increasing the route limit changes the
comparison batch and route list, while the scene still has the same 218 family
nodes. The complete browser audit records
startup long tasks and pan frame intervals; cold startup on a 1.6 Mbps,
150 ms connection is still about seven seconds. Larger optimized WASM adds
roughly 27 KB of compressed transfer, so computation gains do not imply a
comparable network-startup gain. Production CDN compression was not benchmarked.
The slow scenario uses 4× main-thread throttling, which does not throttle workers
and is not equivalent to a physical phone.

A separate renderer-only probe uses the actual 218 family nodes, 305 family
edges, all labels, identical geometry/fonts and a 120-frame pan trajectory:

| Renderer-only probe | Cytoscape Canvas | Native SVG prototype |
| --- | ---: | ---: |
| Throttled cold ready time, one run | 4.67 s | 2.42 s |
| Resource transfer, uncompressed server | 805 KB | 370 KB |
| JavaScript heap after GC | 5.5 MiB | 1.4 MiB |
| DOM nodes | 26 | 976 |
| Pan frame p95 | about 26 ms | about 26 ms |

```sh
uv run --no-project --with playwright==1.58.0 python scripts/benchmark_website_renderers.py book --output /tmp/renderers.json
```

**SVG is a promising next renderer, not a proven application replacement.** This
probe omits route selection/styling, label culling, dragging, touch gestures,
popovers and drawers. Its JS heap measurement excludes native DOM storage. It
shows a download and scripting opportunity, not full UX parity or total memory
superiority. A local gzip measurement reduces the vendor script to 136 KB, so real CDN compression can reduce the download advantage. A full
migration should preserve the browser suite and be measured on physical mobile
hardware. WebGL is not automatically preferable: the current scene is small,
and [Cytoscape's own WebGL guidance](https://blog.js.cytoscape.org/2025/01/13/webgl-preview/)
describes its strongest case as substantially larger networks and notes style
limitations. No GPU speedup is claimed without a device-specific benchmark.

Other measured costs are smaller: JavaScript parsing of the actual 100-route
summary takes about 2 ms, while profile expansion and comparison take over a
second. Replacing JSON with a custom binary protocol is therefore lower priority
than the kernel changes. Asset hashes shared across releases and normal HTTP
Content-Encoding for WASM are worth evaluating against real CDN headers; the
current gzip stream is portable to the uncompressed local server, but its
synthetic Response lacks the original fetch URL used by V8's persistent code
cache ([V8 documentation](https://v8.dev/blog/wasm-code-caching)). No service worker or hosting change was added on an unmeasured assumption.

Unavailable size fields now display `?` and sort after known growth within the
candidate front; missing certificates are not treated as constant-size problems.
Copying or inspecting a route locks its selection while comparison completes.
The earlier Atlas changes remain: a lightweight family summary, lazy per-family
metadata, cached row elements, offscreen layout skipping, and a prefetched shared
documentation index. Those paths already measured around 0.11 s for Atlas opening
and under 5 ms for search updates in the throttled scenario.

## Build and preview

Use mdBook 0.5.2, Typst 0.15.1, and wasm-bindgen-cli 0.2.129, matching the deployment workflow.
Install the browser build tools once:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.129 --locked
```

```sh
make website
python3 -m http.server 3001 --bind 127.0.0.1 --directory book
```

Open <http://localhost:3001>. `make doc` also builds Rust API documentation.
If local requests take seconds or return proxy 503 responses, check the browser's
Network timings and bypass loopback traffic in the local proxy before benchmarking.
`make website`, `make doc`, and `make mdbook` share the `paper-data` prerequisite,
which exports the `docs/paper/data/examples.json` that the Typst source reads.
The deployment workflow builds the PDF and API and combines everything in `book/`.
The fast `make website` preview includes the guide; API/PDF links require those
additional builds, as in deployment.

Website assets are published under `book/releases/<build-number>/assets/`.
The builder rewrites their URLs together, so old open pages cannot mix new
documentation with old registry data. If an old release is no longer available,
reload the page. Do not overwrite files inside an existing release directory.
The homepage and Atlas list use a small registry summary. Full metadata loads
per problem family, including its variants and incident reductions; documentation
articles load only when selected.

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
stay available; the retired `reduction-graph.html` and `open-problems.html` redirect
to their website counterparts through `[output.html.redirect]` in `book.toml`.
Running `mdbook build` by itself restores mdBook's homepage; run
the website builder afterward. The GitHub Pages workflow does this automatically.

## Design and data

- Deep forest surfaces, warm ivory text, and pale lime form the dark visual palette.
- The header, footer, and favicon use the original `docs/logo.svg` artwork, recolored
  at build time to match the dark palette. The canonical source logo stays unchanged.
- Self-hosted DM Sans, DM Mono, and Instrument Serif include their OFL licenses
  in `assets/fonts/`. The graph page uses the prototype’s font weights.
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
- No frontend framework is required. The graph uses self-hosted Cytoscape, with
  its license copied beside the script. fCoSE runs only at build time.

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

The homepage's **Open questions** link opens `open-questions/`.
The board is generated from the registry and shared templates in
[`GiggleLiu/autoresearch-gadgets`](https://github.com/GiggleLiu/autoresearch-gadgets),
then included in the same Pages artifact. Legacy `#open-questions` and
`open-problems.html` links redirect to the board.

The deployment workflow reads that private repository's `main` branch with the
existing `PROJECT_READ_TOKEN` secret. That token needs **Contents: read** access
to `GiggleLiu/autoresearch-gadgets`; the default `GITHUB_TOKEN` cannot read another
private repository. Only the generated `website/dist/` is published. Registry
changes appear on the next documentation deployment; use **Run workflow** on
`main` to refresh the board without changing this repository. Feature-branch
runs build and verify the artifact without deploying it.

To include the board in a local preview, clone its source beside this repository,
build it with its committed lockfile, and run the website overlay after mdBook:

```sh
uv run --project ../autoresearch-gadgets/website --locked python ../autoresearch-gadgets/website/build.py
python3 scripts/build_website.py --open-questions ../autoresearch-gadgets/website/dist
```

Include the board when running the browser suite. Without `--open-questions`,
the builder previews the main website and guide only.

The guide has seven pages listed in `docs/src/SUMMARY.md`: an overview, the CLI
(quick start, command reference), agent skills, the Rust library
(getting started, API, design). Keep one example per
concept and link to `pred --help`, rustdoc, or `.claude/CLAUDE.md` instead of
restating them.

`docs/src/static/docs-theme.css` supplies the restrained dark reading theme;
`docs-theme.js` adds the GitHub and atlas links. Native mdBook search, code copying, and
sidebar navigation remain available. Both mdBook 0.4.37 and 0.5.2 (deployment)
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

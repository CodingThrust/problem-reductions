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
each article into the release's `assets/details/` directory. The documentation
URL/anchor index loads on the first drawer open, before looking up that article.
Entering the Atlas starts the shared index download in the
background, alongside any selected problem's metadata. Articles load on
selection; the 24 most recently used articles are cached. References load
when expanded. Repeated SVG glyph definitions are shared within each article.
Generated content is not copied into the website source directory or committed.
Text reflows in the panel; equations and figures use
Typst's SVG rendering to preserve mathematical notation. The existing PDF build
uses the same source and its original print presentation. HTML export remains
experimental, so the build fails on export errors.

## Performance measurements and architecture investigation

This section records the comparator and startup study before the Vite/TypeScript
migration. See [engineering checks](ENGINEERING.md) for current verification,
performance observations and outstanding acceptance checks.

Measured on 15–16 October 2026 on a 32 GiB Apple M6, Node 24.21 and Chromium 145
(Playwright 1.58), using release WASM builds. The baseline already includes the
previous website fixes. Raw samples and regression fingerprints are retained outside Git
and record the actual inputs, memory definitions and artifact hashes. The following
computation table reports medians of three fresh processes, excluding enumeration, WASM initialization,
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
HiGHS into WASM. One worker handles symbolic computation; native SVG handles the
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

### Cold Graph navigation

These are **complete application cold starts**, measured separately from the
100/999-route computation benchmarks above. The controlled slow profile reproduced
7.62 / 7.70 s median/p95 full startup, rather than
exactly ten seconds on the user's device. The unthrottled desktop results below
make the environment dependence explicit.

Each sample launches a new Chromium process, temporary profile and browser context,
clears the HTTP cache and starts a new WASM worker. Service workers are blocked;
loopback bypasses the local proxy. All runs use the actual built website and Rust
WASM, with real searches, route rendering and interaction. Browser traces are
recorded for every sample, at 1440 × 1000 and device scale factor 1. Default motion stays enabled.

The slow link shares **200,000 bytes/s and 150 ms response latency across all
requests**, including worker requests; the page CPU is throttled 4×. Workers are
not separately throttled, so this is not a physical-phone simulation. Each slow
cell reports **median / p95 across 20 successful samples**, using nearest rank p95.
Raw text delivery and HTTP gzip are measured independently under identical settings.

| Navigation-to-event or resource | Before, raw server | After, raw server |
| --- | ---: | ---: |
| Meaningful graph (s) | 6.67 / 6.72 | 1.40 / 1.49 |
| Verified search interaction (s) | 7.17 / 7.30 | 1.48 / 1.59 |
| Selectable routes (s) | 7.11 / 7.16 | 3.02 / 3.04 |
| Full readiness (s) | 7.62 / 7.70 | 3.58 / 3.61 |
| Blocking above 50 ms (ms) | 128 / 149 | 0 / 0 |
| Transferred (KiB) | 1206.9 / 1206.9 | 508.2 / 508.2 |
| Peak Chromium RSS sum (MiB) | 643.7 / 679.1 | 547.5 / 552.6 |
| Peak page JS heap (MiB) | 17.76 / 24.57 | 4.56 / 4.71 |

| Navigation-to-event or resource | Before, HTTP gzip | After, HTTP gzip |
| --- | ---: | ---: |
| Meaningful graph (s) | 2.62 / 2.65 | 0.87 / 1.00 |
| Verified search interaction (s) | 2.95 / 2.98 | 0.99 / 1.12 |
| Selectable routes (s) | 3.30 / 3.32 | 2.54 / 2.57 |
| Full readiness (s) | 3.83 / 3.86 | 3.11 / 3.16 |
| Blocking above 50 ms (ms) | 204 / 247 | 0 / 4 |
| Transferred (KiB) | 547.1 / 547.1 | 406.5 / 406.5 |
| Peak Chromium RSS sum (MiB) | 682.4 / 689.7 | 548.0 / 556.4 |
| Peak page JS heap (MiB) | 23.98 / 25.05 | 4.52 / 4.59 |

Meaningful graph means the graph and controls have rendered, followed by two
animation frames. Verified interaction fills the source field and checks the real
MIS suggestion while startup continues; this includes browser-driver latency and
is not an INP measurement. Selectable routes means real enumeration results have
painted, with comparison still allowed to be pending. Full readiness additionally
waits for the comparison, fonts and three stable viewport frames after the final
camera animation. Optional documentation is outside this initial-query milestone.
Browser FCP can show only site navigation and is not used as a
substitute for a meaningful Graph render.

For the **unthrottled loopback desktop** profile (10 new processes per cell):

| Navigation-to-event | Before | After |
| --- | ---: | ---: |
| Meaningful graph (s) | 0.11 / 0.14 | 0.06 / 0.07 |
| Verified search interaction (s) | 0.35 / 0.42 | 0.18 / 0.21 |
| Selectable routes (s) | 0.19 / 0.21 | 0.08 / 0.12 |
| Full readiness (s) | 0.74 / 0.77 | 0.66 / 0.68 |

The critical path is visible in a representative baseline trace: HTML completes at
0.20 s; the registry script downloads from
0.20–5.74 s,
Cytoscape from 0.20–6.39 s,
and preloaded WASM from 0.20–5.37 s.
Only then does the Graph script start at 6.40 s, install its
controls at 6.53 s and create the worker. The routes
paint at 7.11 s; comparison DOM finishes at
7.08 s; fonts/camera finish at 7.62 s.
The baseline's main-thread work could delay the route paint past the comparison DOM update.
**These download intervals overlap; adding them would invent startup time.**

Phase timings for the raw slow profile, again median / p95:

| Phase | Before | After |
| --- | ---: | ---: |
| Main JS parse (ms) | 35.8 / 39.7 | 23.1 / 25.4 |
| Main JS compile (ms) | 54.0 / 58.3 | 29.5 / 31.5 |
| Main JS execution (ms) | 1010.2 / 1039.5 | 61.3 / 67.6 |
| WASM download, overlapping (ms) | 5149 / 5174 | 2790 / 2814 |
| WASM compiler thread intervals (ms) | 171.3 / 192.4 | 179.8 / 192.8 |
| WASM synchronous instantiation (ms) | 0.048 / 0.064 | 0.058 / 0.100 |
| Engine ready from navigation (s) | 6.96 / 7.01 | 2.98 / 3.01 |
| Graph model/construction (ms) | 34.3 / 35.9 | 11.1 / 11.9 |
| Controls/initial viewport setup (ms) | 89.3 / 94.2 | 20.6 / 23.2 |
| Path enumeration, including API serialization (ms) | 4.5 / 5.2 | 4.7 / 5.1 |
| 20-route comparison, including API serialization (ms) | 65.4 / 68.0 | 69.6 / 75.4 |
| Comparison delivery, ranking and DOM update (ms) | 39.9 / 46.1 | 19.9 / 24.2 |
| Main layout (ms) | 32.6 / 36.2 | 143.8 / 152.1 |
| Main style recalculation (ms) | 15.8 / 18.2 | 49.8 / 55.5 |

The phase table includes both overlapping work and nested JS work; it must not be
summed into wall time. Parsing/compilation numbers cover main-thread trace events,
including the small measurement observer. WASM compilation includes lazy and
background compilation on multiple threads. The per-run machine-readable records
also contain compressed/decoded JS bytes, scene/worker JSON parsing and initialization
spans. After splitting metadata, scene JSON parsing is
0.60 / 0.90 ms and worker metadata JSON parsing is
0.30 / 0.40 ms. Before the split, metadata
was an executable object literal, included in JavaScript parsing/evaluation.

The worker's fetch-to-initialization span now includes waiting for a still-running
download: initialization starts earlier instead of waiting behind graph construction.
There is no separate global mathematical catalogue initialization; graph deserialization,
expression composition and certificate construction occur inside the measured Rust
API calls. React rendering, hydration and React effects are absent: this is plain
JavaScript with native SVG.

The implemented changes remove work and dependencies from the critical path:

- Native SVG replaces the 434 KB Cytoscape browser bundle and its canvas render
  loop. The build-time layout and overlap removal remain unchanged. Pan, wheel/dock
  zoom, node dragging, touch pinch/pan, label culling, routes, popovers and drawers
  are covered by actual browser interaction checks. Label metrics are cached, and
  unchanged classes no longer cause repeated DOM attribute writes during camera motion.
- The scene receives positions, names, variants, aliases and endpoint pairs only.
  Descriptions, the documentation URL/anchor index and mathematical edge contracts
  no longer travel in one executable registry script. The documentation index loads
  on the first drawer open; articles, references and proofs keep their existing lazy
  renderer and exact-variant fallback behavior.
- A small bootstrap starts the worker and scene fetch before graph setup. Inside
  the worker, the WASM stream and compressed mathematical registry load in parallel.
  The registry is fetched/parsed once there, and query messages contain endpoints
  and limits instead of cloning the full graph on every call. The existing eight-query
  cache, retained idle worker and termination on cancellation remain.
- Routes still appear before comparison completes; users can search, pan and inspect
  nodes while the engine downloads. No default routes or comparisons are fabricated,
  skipped, precomputed for the benchmark or replaced with weaker mathematical checks.

The WASM binary is **byte-identical before and after**. Worker linear-memory high
water remains 3.375 / 3.375 MiB; polynomial kernels,
comparison verdicts and proof APIs are unchanged. A worker pool would duplicate
module/data memory and add scheduling/serialization for a default comparison of
about 70 ms; it does not
address the measured download/renderer bottleneck. Repeated Rust-side API parsing
remains in those small calls; adding a new stateful WASM registry API is not justified
by this profile. SVG uses more cumulative browser layout/style time than Canvas,
but spreads it across short frames; the table reports that cost instead of hiding it.

A three-run exploratory **preload ablation**, with the compressed slow profile,
measured 3.09 s full readiness with WASM preload versus 3.25 s without it. The scene
painted about 90 ms earlier without preload, while usable results arrived about
150 ms later. Preload stays because it improves the query critical path; it does
not eliminate the download or initialize WASM by itself.

Memory numbers are **sampled peaks**, not exact physical peaks: approximately every
50 ms plus instrumentation overhead. The Chromium RSS sum includes browser, GPU,
network and tracing processes and may double-count shared pages. Page JS heap uses
precise V8 reporting without forced GC and excludes native DOM, worker heap and
WASM. WASM capacity records its allocation high water. Blocking means the sum of
main-thread task time exceeding 50 ms from navigation to full readiness, rather
than Lighthouse's FCP-to-TTI TBT definition.

Verification: all 49 browser checks pass, including CLI route/profile comparisons,
copy-and-solve, comparison-time selection, cancellation, exact-variant proof drawers,
mobile layout, drag, wheel anchoring and real browser touch events. Build/security,
MathML and existing graph-JS tests pass. V8 coverage records 98.38% of the new SVG
renderer source positions exercised. All 257 node parameter schemas and 344 mathematical
edge records match the source registry; the full WASM regression preserves all 190
pair verdicts/certificates for the default query and agrees with the summary API.
No Rust source changed; native workspace tests and Rust coverage were not rerun.

Earlier exploratory runs exposed the default Python HTTP server's five-entry
accept queue rejecting scripts during the cold burst. Those entire batches were
invalidated. The final harness uses a 128-entry queue and fails on any request or
browser error; no serving failures are included in these statistics. This also
explains why a simple preview server is unsuitable as an unchecked performance
measurement server.

Reproduce after building the website:

```sh
uv run --no-project --with playwright==1.58.0 python scripts/benchmark_graph_startup.py book --output /tmp/graph-startup --repeat 20
# Same link and CPU, with normal HTTP compression for text assets:
uv run --no-project --with playwright==1.58.0 python scripts/benchmark_graph_startup.py book --output /tmp/graph-startup-gzip --repeat 20 --gzip
# Separate desktop measurement:
uv run --no-project --with playwright==1.58.0 python scripts/benchmark_graph_startup.py book --output /tmp/graph-startup-desktop --repeat 10 --profile desktop
```

Each output directory contains full `results.json`, incremental `samples.json` and
one compressed Chrome/Perfetto trace per run. Keep sample measurements, artifact hashes
and metric definitions outside Git as described in [engineering checks](ENGINEERING.md).
The isolated historical
renderer probe remains available in `scripts/benchmark_website_renderers.py`;
it is not used as evidence for the complete application numbers above.

Production CDN headers, Firefox/Safari and physical phones have not been measured.
The user's exact ten-second device/network case remains unverified; the report
claims improvements only for the explicitly measured cold-start environments.
Early worker construction/network failure recovery has not been fault-injected.

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
- No frontend framework is required. The graph uses native SVG; Cytoscape/fCoSE
  run only at build time to generate the preset layout.

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

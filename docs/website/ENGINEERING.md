# Frontend engineering checks

Follow [FRONTEND_PLAN.md](FRONTEND_PLAN.md). Preserve the current appearance;
Claude's design phase starts after the verified engineering handoff. The
[data boundary inventory](DATA_BOUNDARIES.md) tracks the remaining contracts.

Develop on the user-selected branch and keep changes independently reviewable.
Generated screenshots, traces, raw samples and run reports belong outside Git or
in CI artifacts. Commit source,
tests, contracts, configuration and concise maintained documentation. Feature
pushes require explicit authorization.

## Build and verify a fresh production artifact

Prerequisites: Node 24.21.0, Rust 1.99.0 with `wasm32-unknown-unknown`,
wasm-bindgen-cli 0.2.129, mdBook 0.5.2, Typst 0.15.1, `uv`, and the native
HiGHS build requirements. Keep Cargo on its normal PATH for local scheduling
and caching. The verifier appends `$CARGO_HOME/bin` (default `~/.cargo/bin`)
to its own PATH and reports missing tools before starting work. It does not
change shell configuration or Apple's Python installation.

Install the pinned browsers once:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python -m playwright install chromium firefox webkit
```

Build and verify:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/check_website.py
```

The command installs locked Node dependencies, checks types and lint, regenerates
registry/example data and CLI snippets, and assembles mdBook, paper, API and website
in a new temporary directory. It serves that exact artifact over HTTP and checks
mathematical agreement, copied-route reduction/solving, CSP, lazy delivery,
accessibility, failure recovery and benchmark instrumentation. Servers close on failure.

`verification.json` records source revision/dirty fingerprints, untracked source
hashes, tools, lockfiles, runner image and artifact hashes. `verification.log`
retains command output. Failed artifacts remain available for diagnosis and cannot
be used as passed benchmark inputs. Use `--output /tmp/new-website-check` for a
named destination; existing output paths are refused. The command leaves `book/`
alone. Existing ignored registry/snippet export locations are still regenerated.

Run only one build workflow per checkout. Finish edits, staging and commits before
verification: the verifier rejects source changes made during the run. Use a
separate checkout for concurrent work. For an existing complete artifact, use
`--site /path/to/artifact`; its provenance explicitly identifies verification-only
mode and does not claim the current checkout built an externally supplied artifact.
`Cargo.lock` is now tracked, matching the existing local dependency resolution.
Schema generation/checking uses `--locked`; CI's lockfile drift check can now detect
Rust resolution changes. There are no dependency upgrades in this adoption.

## Build integration and rollback

Python data generation and wasm-bindgen precede the two-page Vite emission step;
finalization then assigns immutable release-directory URLs. Stable filenames avoid
invalidating content hashes during finalization. ES imports and HTML preloads use
the same URL. A narrow worker resolver leaves the generated WASM binding beside
the worker. Static hosting, compressed payloads and the Rust string API stay intact.
CSS minification is disabled because its color/alpha transformations changed pixels.

`npm run typecheck` is a strict, separate `noEmit` check; Vite only emits runtime
JavaScript. `npm run lint:frontend` applies correctness rules without formatting
changes. All application entry points and modules are now TypeScript; the checker
no longer enables legacy JavaScript. The remaining `graph-layout.js` is a Node-only
input to the existing build-time layout generator, not a browser application module. Real browser/compiler
fixtures prove TS emission, strict rejection, CSP, native CSS rendering and shared
module delivery. Runtime contracts migrate incrementally.

The worker buffers decompressed WASM before binding initialization. Native Firefox
cold-query replays exposed a pre-existing streaming stall before Vite; buffered
initialization passed the diagnostic replays and subsequent production workflows.
The temporary input buffer currently holds 776,965 bytes. The exact browser cause remains
unconfirmed. Compression, early preload, worker termination and bounded caches are
preserved. This compatibility fix does not establish that the sporadic frame hitch
is fixed.

Rollback build integration, module entries and worker URL changes together to the
prior Python asset-copy path and a complete prior static release. Keep useful
compatibility checks. Never combine old HTML/data with newly compiled modules.

## Runtime contracts and strict migration

The first completed local slice uses `graph-scene.schema.json` (version 2) for
Python producer validation and browser validation. `graph-startup.ts` parses as
`unknown`; the generated schema guard and `graph-scene-data.mts` reference checks
run before scene construction. They reject unsafe indices, nonfinite coordinates,
unsupported versions, duplicate variants independent of key order, invalid defaults,
orphan aliases and incomplete/extra layout entries. Rust mathematics is unchanged.

`graph-details.schema.json` (version 1) validates producer output and the shared
`details.ts` loader before caching or article lookup. Cross-field checks reject
dangling anchors, external/stale-release URLs, non-article paths and malformed or
traversing encoded paths. Resolve the article directory from the index asset URL;
Vite treats a literal directory URL relative to `import.meta.url` as a source asset.
Lazy article loading, the 24-entry cache, eviction/retry, references, footnotes,
generated-HTML safety validation, CSP and existing markup/styles are preserved.

The existing SVG class is now `graph-scene.ts`, imported directly by `graph.ts`.
Its actual node/edge inputs, callbacks, pointer gestures, keyboard traversal,
camera operations and label measurements are strictly checked. Existing DOM/CSS
ownership stays intact; there is no renderer abstraction or additional renderer.

`graph-growth.mts` owns the existing certificate growth summaries. Exponents,
monomial degree sums and degree ordering use `bigint`; certificate/coefficient
strings stay unchanged. A real WASM-backed browser case first reproduced exponent
9007199254740993 rounding down to 9007199254740992. It now verifies the exact badge,
leading term, degree sum and copied route. Pure checks also cover rational
coefficient preservation, adjacent wide-degree ordering and unavailable values.
Node coverage for this module measured 100% of lines and 97.9% of branches.

`find_paths` now serializes a Rust `WasmPathResult` with `schema_version: 1`.
Schemars 1.2.2 derives `wasm-paths.schema.json` from the response/route structs in
serialization mode, including closed objects, required fields and exact safe-integer
bounds. Its native-only `schema-export` feature is absent from normal WASM builds.
The worker calls strict `wasm-paths-data.mts`: parse unknown, validate shape/version,
check step counts and chain continuity, then use the result. Variant maps compare
semantically without reordering routes. A real-worker test first demonstrated an
invalid endpoint reaching the main thread; it now receives an actionable error
before routes are posted. Requests, cached routes and worker envelopes
still need their own contracts.

`path-graph.schema.json` (version 1) now covers the Python-produced worker metadata.
The producer and strict `path-graph-data.mts` consumer share shape and cross-field
checks. The worker parses unknown, validates once, then retains the typed graph.
Checks reject unsafe/dangling indices, duplicate variants independent of key order,
ambiguous defaults or edges, repeated parameter coordinates, and duplicate/unknown
or missing target fields. Parameter tags require exact/upper-bound formulas with
null reasons, or unavailable fields with null formulas and nonempty reasons.
Explicit native contract errors may retain an empty parameter list so route discovery
and later native error reporting remain available. Exact formula strings are preserved;
Rust still parses expressions and validates the mathematics. Metadata rejection now
precedes the existing path-response guard. A real-browser negative test first showed
duplicate coordinates being accepted, then verified rejection and successful retry
after repairing the actual resource. The strict helper's coverage was 100% of lines
and 96.7% of branches. Worker typing and incoming/outgoing message contracts remain open.

`compare_paths` and `summarize_paths` now serialize typed Rust wire responses with
the existing comparison version 5. Schemars derives `wasm-comparison.schema.json`;
the strict `wasm-comparison-data.mts` consumer validates unknown JSON before the
worker posts it or the benchmark uses it. Checks cover safe counts/indices, profile
counts, coordinate references, certificate/accuracy consistency, pair completeness,
dominance endpoints and the full result's front. Exact certificate strings retain
wide integers and rational coefficients. Full results require `pairs`; summaries
omit it. Nondominated pairs omit `dominates`; unavailable certificates remain null.
This validates the wire contract, not mathematical truth; native proof checks remain
independent. Native adapter tests cover all four verdicts, wide rational bounds and
unavailable results. Five real before/after queries, a wide rational fixture and all
four verdict fixtures agreed semantically with the prior WASM build. Changed adapter
coverage was 81/82 executable lines; the schema CLI's 20 changed executable lines and
the strict comparison parser's lines/branches were fully covered.

Ajv 8.20.0 generates standalone validation code; Vite bundles its static helpers,
so production does not compile schemas or require dynamic evaluation.
`json-schema-to-typescript` 16.0.0 generates matching strict declarations. Typescript
6.0.3 matches the lint parser's supported range (`@typescript-eslint/parser` 8.71.2).
These tools are development dependencies. TS source/declarations are removed from
published assets after Vite emission. The public commands generate/check the Rust
schemas before the standalone validators and declarations:

```sh
npm run generate:frontend-contracts
npm run check:frontend-contracts
npm run test:frontend-contracts
npm run typecheck
npm run lint:frontend
```

Direct Python builds also check generation drift. Fixtures prove stale Rust schemas
and standalone validators fail their respective checks. Native path tests run in
the verifier; a separate 146-case native math/proof run passed locally. Scoped
coverage measured all 39 added executable Rust adapter lines, all 24 schema-export
CLI lines and all branches/lines of the strict path parser. Fixtures also confirm
typed source reaches lint's control-flow rules. The unknown-
category browser test failed on the preceding artifact, then passed before any SVG
was rendered on rejection. The invalid-index browser case also failed on the prior
artifact and now confirms rejection and successful retry after repairing the real
HTTP resource. Latest required-board verification passed 79 root cases, thirteen
failure and data-contract cases, and seven nested workflows per engine, with the
documented multi-touch skips.
All fourteen screenshots matched; no raw TS/declarations were published.

Earlier isolated pilots measured scene validation at 0.75/0.90 ms and first lazy-index
validation at 1.6/1.7 ms median/sampled p95. A ten-process-per-API Node batch on 100
genuine routes measured comparison parsing plus validation at 12.03/14.59 ms full
and 8.91/9.37 ms summary. These are informational costs, not stable-tail estimates.
Earlier evidence is retained under `scene-contract/`, `details-contract/`,
`typed-scene/`, `wide-growth/`, `wasm-path-contract/` and `wasm-comparison-contract/`.

After metadata validation, decoded startup JS is 131,276 bytes (+21,061 over the
comparison-contract slice). WASM remains byte-identical at 776,965 raw / 267,409 gzip
bytes. Before/after metadata nodes/edges, route ordering, variants, profiles, exact
certificates, verdicts and optional fields agreed semantically; metadata version 1
is the declared addition. All fourteen screenshots matched exactly.

The first ten-run cold batch measured metadata validation at 2.7/3.2 ms and showed
variable graph/route readiness at 63.95/133.1 and 87.4/164.6 ms. A separate control
alternated ten old/new fresh-browser pairs in AB/BA order without concurrent builds,
tests or captures. Median/sampled-p95 observations were:

| Metric | Before metadata guard | After metadata guard |
| --- | ---: | ---: |
| Graph readiness proxy | 61.05 / 138.2 ms | 60.95 / 65.5 ms |
| Route readiness proxy | 78.35 / 167.9 ms | 84.2 / 112.3 ms |
| Full readiness proxy | 656.9 / 746.8 ms | 667.2 / 694.1 ms |
| Fill-to-suggestion round trip | 41.6 / 57.5 ms | 41.0 / 46.5 ms |
| Metadata validation after JSON parsing | unavailable | 3.05 / 3.6 ms |

The control did not reproduce the graph-readiness median difference, but route/full
readiness medians rose 5.85/10.3 ms. These costs are recorded, not waived as statistical
non-regression. All twenty control inputs preceded comparison completion. Keep the
first batch and paired control separate; ten observations still make nearest-rank
p95 the maximum. Raw samples include variation and instrumentation support. The
startup report now distinguishes metadata JSON parsing from schema/reference
validation and reports missing validation marks as unavailable. Evidence is retained
externally in `path-metadata-contract/`; timing gates and the earlier hitch stay open.

The path benchmark's sample mode now validates full/summary results and reports
`parse_validate_ms` rather than calling the combined interval `parse_ms`. Its legacy
default HTML entry lookup still fails on the current Vite `graph.js` entry; that
discovery repair remains open. The adopted startup/interaction harnesses are separate.

A separate two-route wide-exponent stress fixture also traps in the pre-fix WASM
comparator (`unreachable`). Its suspected cause is resource growth while computing
exact witness coefficients; the trap cause has not been profiled. The reproducer
is retained in `wide-growth/`. This existing extreme-input limitation remains open;
the frontend precision fix preserves the mathematical engine unchanged.

`graph.ts` and `path-worker.ts` now migrate the complete controller and worker
under the existing strict configuration. Worker requests/replies use version-1
schemas; incoming structured clones are unknown until validated. Cached/returned
routes must match the query and registered witness edges, with paths preceding
comparison. Scene version 2 carries witness/Turing capabilities for that check.
Comparison coordinates/fields must match the selected native registry variants.
Internal typed requests rely on these established invariants instead of repeating
runtime validation. The existing eight-entry cache, termination/retry behavior,
DOM, CSS, renderer and Rust/WASM algorithms are preserved.

The binding declaration `assets/path-selection.d.ts` comes from pinned
wasm-bindgen; regenerate it when the exported Rust API changes. Vite emits only
JavaScript, and generated declarations are removed from production assets.
Existing contract/browser suites cover this migration; no new test files or
frameworks were added. Request/reply validation measures distinguish measured
cost from unavailable instrumentation. Run full acceptance at stage boundaries;
do not repeat cold pilots or coverage runs for type-only edits.

Local controller/worker acceptance passed the fresh production command above:
strict checking, lint, contract drift, mathematical workflows and root/nested
production plus recovery checks in Chromium, Firefox and WebKit. The two existing
CDP-only multitouch skips remain explicit. CSS and WASM bytes are unchanged;
manual accessibility and CI acceptance remain open. No new benchmark pilot was
run for the type migration.

`site.ts` now types the complete Home/Atlas module. The version-1
`atlas-data.schema.json` covers both the embedded summary and lazy family records.
Producer output is validated in one batch before writing records; browser inputs
are unknown until schema/reference validation, before catalog access or caching.
The contract preserves exact formula strings, optional metadata, nullable fields,
variant/edge order and safe indices, and restricts embedded MathML and link paths.
Reference-page functions receive their typed record explicitly; the summary stays
separate. Malformed input reports an alert and failed record loads can be retried.
Markup, CSS, native mathematical computation and lazy loading are preserved.
Local acceptance passed production, recovery and nested-path checks in all three
engines, with the existing two CDP-only skips. All 218 family records and the
browser-delivered summary agree semantically with the previous artifact apart
from the version/kind fields. Fourteen existing desktop/mobile screenshot
references match exactly; CSS and WASM bytes are unchanged. Validator timing
remains part of the final contract-stage performance check.

Revert the Atlas schema/generated files, producer markers/validation, typed site
entry and checker configuration together to roll back this migration. Deploy a
complete prior release; do not mix old data with the new consumer.

Native registry exports are validated against `native-exports.schema.json`
before the builder indexes or transforms them. Reference validation reuses the
path metadata checks rather than copying them; schema/family/category and alias
checks preserve the registry-owned model categories and exact expression text.

WASM request schemas now derive from the real Rust deserialization structs.
Generated request types check the worker's construction from already-validated
graph, message and route data. Rust still deserializes incoming JSON and checks
its native semantics. Optional fields remain optional/nullable, and the native
API still accepts its existing ignored metadata. Generated TypeScript describes
the known fields without adding index signatures that force internal copies.
There is no second runtime validation of typed internal requests.

`details.ts` exports the loader/render functions and keeps its index/cache private.
Graph startup is a shared module export, retaining the existing early-start
sequence and compressed-WASM preload. Both pages import their actual dependencies;
there are no window-level application methods or startup objects. The existing
`REDUCTIONS` data exposure remains for benchmark consumers and the embedded home
data boundary. Graph HTML also names its public scene URL in `data-scene`.
The browser cache test navigates actual pages, and scene/reference tests read
public metadata rather than accessing private module state. No renderer abstraction,
state framework, styling changes or new test infrastructure were introduced.

Rollback the native-export contract/entry validation together, and request
schema/type generation with its native structs and typed worker calls together.
Roll back module exports/imports and their HTML entries/callers as one change;
preserve early downloads, bounded caches, cancellation and failure handling.
Rollback the worker protocol, scene capability version, validators, producer and
both typed entry points together to the prior complete release; preserve the
independent path/comparison schemas and Vite integration.
Rollback this slice's schema, generated code, producer version/validation and loader
entry together, restoring a complete prior release while retaining Vite. Revert the
documentation-index schema, producer version/validation and shared consumer together
independently of the scene migration.
Roll back scene typing by restoring its prior class, global caller and HTML entry
together; that change is independent of both data-contract slices.
Revert the growth helper and its caller together to roll back precision formatting
independently, retaining the regression evidence and documenting the restored defect.
Roll back the Rust path-result type/version, schema exporter and generated files,
worker parser/caller and their build checks together. Deploy the prior complete
release, preserving Vite and the independent scene/details contracts.
Roll back comparison wire structs, schema/generated guards, parser and worker/tool
callers together to the prior complete release, retaining the independent path,
scene and documentation contracts. Revert the exporter/fixture changes with that
slice so generation and drift checks stay aligned.
Roll back metadata schema/version, generated code, worker/producer validation and
their tests together to the prior complete release. Restore the previous metadata
measurement scope at the same time, retaining the independent response contracts.

## Open Question Board and CI

The board is a separate build. Include actual built output with:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/check_website.py \
  --open-questions /path/to/autoresearch-gadgets/website/dist --require-open-questions
```

Absent board output permits three explicit local-only skips. CI and explicitly
supplied board output require all features; missing files fail before upload.
Verification-only deployment checks use `--site book --require-open-questions`.
The board's own schema coverage is a separate boundary; frontend TS does not
implicitly validate its scripts.

CI pins action revisions, board source, Rust/Node/Python/browser versions and
rejects lockfile drift. The hosted Ubuntu 24.04 image remains mutable; provenance
records its image/version. Available logs and provenance are uploaded as CI diagnostics,
including after failures. Production publication remains gated to `main`.

CI acceptance is incomplete. The first branch run selected a board tree without
the required prompt feature. The corrected source pin matches the tested local
board tree, but its fresh CI build failed fetching a commit-pinned `Challenge.lean`
with HTTP 404. That external source/access failure must be resolved and the full
workflow verified. A clean archive of that pinned board source was also rebuilt locally in this
stage: fetching Q045 `comparator.json` failed with a proxy tunnel HTTP 503. This
fresh dependency build remains unverified; the existing built board passes the
integrated production checks. Neither failed CI run certifies Linux browser behavior. Further
feature pushes and workflow runs require explicit authorization.

## Performance and appearance evidence

Keep evidence under `$HOME/.local/share/problem-reductions/frontend-evidence/` or
another directory outside the checkout. The preserved local batch is
`2026-10-16-e4d37373`; it includes raw samples/traces, screenshots, original source
provenance and failed CI logs. These are local records, not repository fixtures.
The trace self-check captures and accounts for a real browser trace at test time.
Independent unit examples verify overlapping intervals and unsupported/truncated traces.

Capture fourteen desktop/mobile states from a passed, unchanged artifact:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/capture_website_baselines.py \
  /path/to/verified-artifact --output /tmp/new-reference-capture
```

Capture waits for fonts, asserts states and verifies artifact hashes before/after.
Use the same pinned browser, viewport, DPR, locale, timezone and motion setting for
pixel comparisons. Local post-Vite screenshots matched pre-Vite controls exactly.
Linux screenshot stability and physical mobile appearance remain unverified.

Measure without concurrent builds, tests or captures:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/benchmark_graph_startup.py \
  /path/to/verified-artifact --profile desktop --repeat 10 --output /tmp/new-cold-pilot
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/benchmark_graph_interaction.py \
  http://127.0.0.1:PORT/ --repeat 10 --output /tmp/new-interactions.json --trace-dir /tmp/new-traces
```

Serve the verified artifact for the interaction command. Each scenario uses a fresh
browser process/context and asserts its actual effect. Keep idle controls, first
inputs, raw events/frames and run-level summaries. `--trace` enables a separate cold
trace diagnostic; traced/untraced runs must not be pooled.

The reference Mac pilot used Playwright 1.58.0, Chromium 145.0.7632.6, native CPU,
1440×1000/DPR 1 and loopback HTTP with uncompressed text beside compressed WASM/data.
Physical display refresh was not measured. Ten-run nearest-rank p95 is the maximum;
these pilots do not establish stable tails or calibrated regression margins.

| Post-Vite observation | Median | Sampled p95 |
| --- | ---: | ---: |
| Graph readiness proxy | 55.0 ms | 61.0 ms |
| Route readiness proxy | 82.3 ms | 92.2 ms |
| Full readiness including camera/fonts | 646.65 ms | 667.0 ms |
| Startup fill-to-suggestion round trip | 37.9 ms | 45.9 ms |
| Sampled Chromium RSS sum | 464.23 MiB | 475.22 MiB |

Startup JS fell from 68,890 to 46,832 decoded bytes. WASM remained 806,666 raw /
277,560 gzip bytes with unchanged hashes. Route readiness increased about 11 ms
median over the preceding pilot; this is reported, not waived as a statistical
non-regression. RSS polling misses short peaks and can double count shared pages;
main-page heap and WASM capacity do not isolate worker/compiler allocations.

Fifty fresh-process traced scenarios after scene typing measured p95-of-run-p95
named main-thread slice costs of 2.157 ms wheel, 1.501 ms pan, 2.421 ms drag and
2.988 ms activation. First-input rAF interval sampled p95s were 31.0/32.2/30.0/21.1 ms
respectively. This
union includes measurement overhead and excludes GPU/raster/unclassified work.
No LoAF appeared in that batch; the earlier sporadic hitch remains unresolved.
rAF measures scheduling cadence, LoAF only diagnoses frames above 50 ms, and neither
certifies the complete 8 ms render/presentation target. Timing gates remain
informational pending uncertainty, a practical margin and ten comparable CI runs.

## Browser and accessibility acceptance

Locally, the required-board root suite, failure recovery and seven nested-path
workflows passed in Chromium 145.0.7632.6, Firefox 146.0.1 and WebKit 26.0. Exact
scope, source state and durations are in the retained artifact provenance/logs.
CDP throttling and native multi-touch injection are unavailable outside Chromium;
unsupported instrumentation is explicit. Firefox uses direct loopback access in
its test process. WebKit clipboard checks use native paste when script reads are
denied. Firefox pre-execution worker-close reporting remains inconclusive; the
scene-failure closure check waits for an actual WASM request before serving its 404.

Engineering fixes cover graph keyboard traversal/focus, search semantics, generated
article roles/headings, native code-block scrolling, readable small-text contrast,
click/tap movement alternatives and modal isolation below 1168 CSS pixels. Desktop
details remain nonmodal. Tests preserve copied routes and exercise cancellation,
error/retry and async focus. CSS changes reuse the current styling and serve those
functional/accessibility needs.

Axe checks eight Atlas/Graph states. Serious/critical, role, landmark and heading
violations block; incomplete results require review. Automated scans do not certify
manual screen-reader behavior. [ACCESSIBILITY_CHECKS.md](ACCESSIBILITY_CHECKS.md)
specifies Safari 27.0.1/VoiceOver desktop and named physical-mobile acceptance;
those required manual checks remain unrun. Stages 0–2 and the final handoff retain
their unresolved timing, CI and manual gates. Claude's visual phase is unstarted.

## Latest contract/module acceptance

The complete local production command passed after native contract integration
and module cleanup: strict checking, lint, schema drift, independent mathematical
and CLI/browser workflows, root/nested hosting and recovery in all three engines.
The existing two CDP-only skips remain explicit. No single listed test exceeded
4.06 seconds. Request-schema additions affect native schema export only; five
real route queries, the wide-rational fixture and all four verdicts agree with
the previous WASM artifact. WASM remains 776,965 bytes raw / 267,406 gzip; its binary
hash changed, while its tested mathematical responses did not. Published JavaScript
is 334,676 decoded bytes, down 366 bytes; CSS and both compressed graph payloads
are byte-identical.

Thirteen screenshot files matched byte for byte in the batch. Mobile Atlas had
50 rounded-corner pixels differing; a focused repeat matched
the previous reference exactly. Keep both captures outside Git rather than replacing
the first sample. Appearance and existing markup/styles are preserved.

A desktop pilot with ten fresh browser processes measured median/p95 milliseconds:
usable routes 78.75/87.50, full readiness 645.30/667.90,
and startup input/suggestion round trip 35.90/44.60. All ten inputs
occurred before comparison finished. Worker metadata validation was
1.75/2.20; first worker request validation
0.20/0.20; main-thread path and comparison validation
0.90/1.00 and 1.20/1.40.

A separate ten-process Node pilot of first-call Atlas validation measured
1.78/2.50 ms for the summary and
2.20/3.38 ms for the largest record (ILP.json).
These exclude module loading, I/O and rendering; JSON parsing is recorded separately.
Browser timing, Node guard cost and renderer cost are distinct measurements. Each
nearest-rank p95 here is the maximum of ten observations, so these remain informative
pilots, not calibrated regression gates or proof of the absolute frame targets.
Manual screen-reader/device acceptance and fresh CI verification remain open.

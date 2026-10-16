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
changes. Legacy JavaScript has an explicit `checkJs: false` migration boundary;
that does not certify semantic typing of legacy files. Real browser/compiler
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

The first completed local slice uses `graph-scene.schema.json` (version 1) for
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

The existing SVG class is now `graph-scene.ts`, imported directly by `graph.js`.
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
before routes are posted. Metadata, requests, cached routes and worker envelopes
still need their own contracts.

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
HTTP resource. Latest required-board verification passed 79 root cases, twelve
failure and data-contract cases, and seven nested workflows per engine, with the
documented multi-touch skips.
All fourteen screenshots matched; no raw TS/declarations were published.

The scene-only pilot measured validation at 0.75 ms median / 0.90 ms sampled p95.
After the Rust path-result contract, ten isolated untraced cold runs measured 76,992
decoded startup JS bytes (+10,449 for the worker validator). WASM was 805,798 raw /
277,440 gzip bytes, down 868/120 bytes after typed response serialization. Five
before/after route queries and a wide rational-bound fixture agreed semantically
on ordered routes, variants, profiles, certificates, verdicts and optional fields;
the path response's added version is the declared difference. Graph/route readiness
proxies were 52.5/73.45 ms median and 57.3/79.3 ms sampled p95. Startup
fill-to-suggestion round trips were 29.9/38.5 ms; all ten inputs preceded comparison
completion. A separate ten-process first lazy-index batch measured validation at
1.6/1.7 ms median/sampled p95. These are informational observations, not speedup,
tail-stability or non-regression certificates. Evidence is stored externally under
the retained batch's `scene-contract/`, `details-contract/`, `typed-scene/` and
`wide-growth/`, with the latest response evidence in `wasm-path-contract/`.
The worker paths-start/end interval was 4.35/4.8 ms median/sampled p95; it includes
request serialization, native search/output serialization, parsing and validation.

After the comparison contract, ten fresh untraced cold runs measured 110,215 decoded
startup JS bytes (+33,223). WASM fell to 776,965 raw / 267,409 gzip bytes
(-28,833/-10,031). Graph/routes readiness was 52.35/74.55 ms median and 61.4/87.8 ms
sampled p95. Full readiness was 641.75/664.9 ms; fill-to-suggestion was 32.55/40.9 ms,
with all ten inputs preceding comparison completion. The combined worker comparison
interval rose from 69.3/72.7 to 75.1/87.5 ms. This includes request serialization,
native computation/output serialization, JSON parsing and validation; it does not
isolate validator cost. A separate ten-process-per-API Node batch on 100 genuine
routes measured parsing plus validation at 12.03/14.59 ms full and 8.91/9.37 ms summary.
These measurements remain informational; the observed increase is not waived as a
statistical non-regression. All fourteen screenshots matched exactly. Evidence is
retained externally in `wasm-comparison-contract/`.

The path benchmark's sample mode now validates full/summary results and reports
`parse_validate_ms` rather than calling the combined interval `parse_ms`. Its legacy
default HTML entry lookup still fails on the current Vite `graph.js` entry; that
discovery repair remains open. The adopted startup/interaction harnesses are separate.

A separate two-route wide-exponent stress fixture also traps in the pre-fix WASM
comparator (`unreachable`). Its suspected cause is resource growth while computing
exact witness coefficients; the trap cause has not been profiled. The reproducer
is retained in `wide-growth/`. This existing extreme-input limitation remains open;
the frontend precision fix preserves the mathematical engine unchanged.

Other data boundaries, request wire structs, worker envelopes and
remaining application files still need migration.
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
workflow verified. Neither failed run certifies Linux browser behavior. Further
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

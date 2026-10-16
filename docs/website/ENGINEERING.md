# Frontend engineering checks

Follow [FRONTEND_PLAN.md](FRONTEND_PLAN.md). Engineering preserves the current
appearance; Claude's visual-design phase starts after the engineering handoff.
The [data boundary inventory](DATA_BOUNDARIES.md) records the current validation
gaps and the minimum contract migration requirements.

## Fresh production artifact

Prerequisites match deployment: Node 24.21.0, the Rust toolchain and
`wasm32-unknown-unknown` target, wasm-bindgen-cli 0.2.129, mdBook 0.5.2,
Typst 0.15.1, `uv`, and the repository's native HiGHS build requirements.
Keep Cargo on its normal PATH so local build scheduling and caching apply.
The verifier appends `$CARGO_HOME/bin` (default `~/.cargo/bin`) to its own
process PATH so installed mdBook/wasm-bindgen are discoverable without replacing
the active Cargo shim. It checks required executables before creating output or
starting the build. Missing tools produce an actionable error, not a traceback.
No shell configuration or Apple's Python installation is changed.

Install the pinned browsers once:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python -m playwright install chromium firefox webkit
```

Build and verify with one command:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/check_website.py
```

The command installs Node dependencies from the lockfile, generates current
registry/example data and CLI snippets, builds
mdBook, the paper, Rust API and the website into a new temporary directory,
and tests that artifact over loopback HTTP. It runs the real CLI/browser
comparison and copy/reduce/solve workflows, build-security and MathML checks,
graph JavaScript tests, and benchmark/reporting regression tests. The server
uses a free port and closes even when verification fails.

The printed artifact path is retained for inspection. `verification.json`
records results, commands, source revision and dirty-state fingerprints,
untracked source hashes, lockfile hashes, tool versions, and artifact hashes.
`verification.log` retains complete build and check output. The command prints
the relevant tail if a subprocess fails. This evidence does not establish
cross-machine timing reproducibility or certify unrun
checks. Inspect failed commands before treating a failed artifact as usable.

Use `--output /tmp/new-website-check` for a named destination. Existing output
paths are refused, so neither a stale `book/` nor a previous scratch build can
silently become the test target. This command does not overwrite `book/`.
Registry/example exports and CLI snippets are regenerated at their existing
ignored source locations; run only one build workflow per checkout at a time.
The verifier records source state before starting and rejects a changed checkout
before reporting success. Finish edits, staging and commits before running it, or
use another checkout for concurrent work. The chosen output directory is excluded
from the source comparison. Failed runs retain their original source snapshot and
cannot be used as passed benchmark inputs.

## Minimal frontend build integration

The integration pins Vite 8.3.4, TypeScript 7.0.2 and ESLint 10.12.0.
`scripts/build_frontend.mjs` builds the two existing HTML entry points after Python
data generation and wasm-bindgen, before finalization. It uses relative URLs and
module workers. A narrow worker resolver preserves the generated `path-selection.js`
binding beside the worker; Rust computation and its string API are unchanged.
Vite emits stable filenames inside the existing immutable release directory.
Finalization may then rewrite URLs without invalidating a content-hashed filename.
Owned modules and their HTML preloads use the same immutable URL, without an extra
query string that would download a shared module twice.
CSS minification is disabled: the first candidate changed Graph pixels through
color/alpha transformations; the same fourteen screenshots match exactly with
that transformation disabled. CSS/markup ownership and current appearance stay intact.
The separate board build, mdBook, API and paper still follow their existing paths.

Rapid cold query changes exposed a pre-existing Firefox worker initialization
failure, also reproduced before Vite. In ten native replays, six pre-Vite queries
stalled; streamed compilation could stay pending even after decompression finished.
An initialization keepalive and separate streaming compilation did not solve it.
Buffered initialization passed all ten diagnostic replays. The worker now buffers
the decompressed 806,666-byte module before calling the unchanged generated binding.
Compressed network delivery, early preload, termination cancellation, bounded caching
and the Rust binary/string API remain intact. This adds a temporary worker buffer;
post-change startup/memory measurements are still required. Do not claim the exact
underlying browser cause, or describe this as a fix for the separate long-frame hitch.

Run `npm run typecheck` and `npm run lint:frontend` separately. The verifier requires
both, independently of Vite emission. The strict `noEmit` configuration is ready
for incremental migration; existing JavaScript remains explicitly outside semantic
type checking (`checkJs: false`) until its contract slice migrates. No application
TypeScript has been introduced. A two-page real-browser fixture proves TypeScript
emission, CSS delivery and finalized CSP; a separate positive/negative compiler
fixture verifies strict checking and absence of emitted JS. The adopted lint rules
cover concrete JavaScript correctness errors, with no mass formatting or style rules.

Production verification also reuses seven critical workflows under
`/research/reductions/`, against the same artifact via a temporary directory alias:
lazy data, compressed scene/WASM and workers, copy/reduce/solve, details/Atlas,
proof/documentation/PDF/API/CSP and the required board. This extends delivery
coverage without a second deployment pipeline. The verifier runs the full root
suite and those seven nested workflows in Chromium, Firefox and WebKit, recording
completed engines in provenance. WebKit clipboard checks use actual native paste
when script reads are denied. Firefox/WebKit explicitly report unsupported CDP
network throttling; ordering still runs, using an eager-worker state snapshot at
main-script entry rather than rounded timestamps. Native multipoint injection is unavailable outside Chromium and remains
a required physical-mobile check.

Eight failure/recovery workflows use temporary mutable copies of the real production
assets over HTTP: missing worker, invalid WASM, corrupt path metadata, cancellation
before paths, offline worker startup, missing documentation index/article, and scene
failure followed by reload. Restoring real bytes must recover a usable copied route;
the source artifact is untouched. HTTP response gates make cancellation precede paths
and let the eager worker start before the scene's 404 arrives. The latter checks actual
worker closure; Firefox's pre-execution worker-close reporting was inconclusive in the
immediate-error diagnostic, so that timing is not claimed as verified resource cleanup.
The existing root suite also checks cancellation during comparison. Test durations are
recorded; none exceeded five seconds in the full local production run. The complete
root, failure and nested suites passed in all three engines with required board output.
All fourteen screenshots match the pre-Vite controls. Isolated post-bundle cold and
interaction pilots, raw samples/traces, source provenance and their limitations are in
[the Vite build evidence](baselines/2026-10-16-chromium-macos/VITE_BUILD.md). Startup JS
fell to 46,832 decoded bytes; WASM hashes are unchanged. Cold route readiness increased
about 11 ms median, with timing gates still informational. CI and required manual
acceptance remain unverified. Claude's design phase remains unstarted.

Rollback the isolated Vite integration, HTML module entries, worker URL changes and
tooling together, restoring Python's asset-copy path and a prior complete static
release. Retain the diagnostic fixtures and compatible delivery checks. Do not mix
compiled scripts with old HTML/data or introduce application TS before this
emission/checking boundary is established.

## Open Question Board and deployment

The private board is a separate build. Include its real built output with:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/check_website.py \
  --open-questions ../autoresearch-gadgets/website/dist
```

Without it, only the three board-dependent browser tests skip locally, with an
explicit reason. The legacy graph redirect still runs. HTTP errors other than
an absent registry are failures, and existing board content must pass its tests.
An explicitly supplied board is required, not optional.

CI always requires the board. Deployment verifies the already assembled
artifact, including the PDF and API, with:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/check_website.py \
  --site book --require-open-questions
```

`--site` is an explicit verification-only mode: it never claims to rebuild
that directory. Missing required board, guide, paper or API output fails
verification before upload. The same required-board policy applies to direct
browser-suite runs when `CI` is set or `WEBSITE_REQUIRE_OPEN_QUESTIONS=1`.

## Interaction diagnostics

The repaired `benchmark_graph_interaction.py` verifies zoom, camera pan and
node displacement in separate fresh Chromium processes. It retains the first
frame, records observer support/errors, and keeps each run's raw samples
separate. Its rAF intervals describe scheduling cadence; LoAF diagnoses only
frames exceeding 50 ms. Neither establishes the sub-50 ms rendering budget.

Keep timing diagnostics informational until the plan's sampling, provenance,
uncertainty and reference-device requirements pass. Browser compatibility,
manual screen-reader checks, screenshots and performance gates are tracked
separately; a passing Chromium verification command does not certify them.

The [initial reference capture and pilot](baselines/2026-10-16-chromium-macos/README.md)
retain raw samples, screenshots, source hashes, limitations and pending acceptance.
Run the interaction pilot against the served verified artifact:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/benchmark_graph_interaction.py \
  http://127.0.0.1:PORT/ --repeat 10 --output /tmp/interactions.json
```

Capture the current appearance from a passed, unchanged artifact:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/capture_website_baselines.py \
  /path/to/verified-artifact --output /tmp/new-reference-capture
```

The capture command verifies all recorded artifact hashes before and after capture,
refuses existing output, asserts each page state and records image/source hashes.
The production verifier also exercises these desktop/mobile capture workflows;
it does not enable a pixel comparison gate or certify visual approval.

The [follow-up measurements](baselines/2026-10-16-chromium-macos/FOLLOWUP.md)
include pointerdown, keyboard activation, cold navigation, actual search/query
effects and sampled memory. Reproduce cold runs without concurrent builds/tests:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/benchmark_graph_startup.py \
  /path/to/verified-artifact --profile desktop --repeat 10 --output /tmp/new-cold-pilot
```

Use a separate diagnostic run with `--trace` for cold-start traces, or
`--trace-dir /tmp/new-interaction-traces` with the interaction tool. Traced timings
include instrumentation overhead and must not be pooled with untraced baselines.
Missing trace categories are unavailable. For the prepared SVG runtime, exercise
the real trace/activation and cold-query self-checks against its served artifact:

```sh
WEBSITE_BASE_URL=http://127.0.0.1:PORT/ uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt \
  python -m unittest discover -s scripts -p 'test_graph_*benchmark.py' -v
WEBSITE_BASE_URL=http://127.0.0.1:PORT/ uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt \
  python -m unittest discover -s scripts -p test_graph_interaction_trace.py -v
```

The prepared SVG/startup runtime is now committed separately from the accessibility
fixes. The production verifier runs these real trace/activation and cold-query
self-checks against its actual served artifact, alongside the pure accounting
checks. Unsupported instrumentation remains explicit. Full browser/failure
acceptance is still pending.
The [frame-cost pilot](baselines/2026-10-16-chromium-macos/FRAME_COSTS.md) accounts
for overlapping main-thread slices and retains complete first-input windows. Its
cost scope, excluded work, invalid concurrent batch and informational gate status
are recorded with the raw traces.
The [post-accessibility follow-up](baselines/2026-10-16-chromium-macos/ENGINEERING_FOLLOWUP.md)
retains ten new runs per interaction and ten cold runs on the 79-workflow artifact,
including the added control costs, exact unchanged WASM/scene hashes and unresolved
timing limits. It does not replace the initial baseline or establish a CI gate.

The fresh required-board build and a verification-only repeat passed locally with
all fifty browser workflows and both reference-capture workflows, plus build,
MathML, graph, benchmark-accounting and provenance checks. The revised CI workflow
uses the same fresh-build command; it has not run on GitHub. Required manual
accessibility acceptance is specified in [ACCESSIBILITY_CHECKS.md](ACCESSIBILITY_CHECKS.md)
and remains unrun. These limits remain open through final engineering acceptance.

## Browser and tool acceptance matrix

The engineering acceptance targets the pinned Playwright 1.58.0 browsers below,
Safari 27.0.1 on this Mac for manual VoiceOver testing, and a named physical mobile
browser/screen reader. Older browser versions have not been certified. Automated
WebKit testing does not certify Safari or iOS screen-reader behavior.

| Browser | Current evidence | Remaining acceptance |
| --- | --- | --- |
| Chromium 145.0.7632.6 | Complete required-board root, failure and nested suites; eight axe states; fourteen exact appearance matches; isolated pilots | Manual acceptance, GitHub CI and timing calibration |
| Firefox 146.0.1 | Complete required-board root, failure and nested suites, including axe, native query/copy and CLI checks; direct loopback setting | Physical multi-touch; pre-execution closure reporting; manual acceptance; GitHub CI |
| WebKit 26.0 | Complete required-board root, failure and nested suites, including axe and native clipboard paste | Physical multi-touch, Safari/manual acceptance and GitHub CI |
| Safari 27.0.1 | Installed version identified | Manual keyboard/VoiceOver script and results |
| Physical mobile | Device/browser/screen reader selection pending | Manual touch, keyboard where applicable and screen-reader results |

The smoke report is in `baselines/2026-10-16-chromium-macos/browser-smoke.json.gz`;
it records exact scope and feature detection. Firefox's default launch timed out
before receiving the loopback response here. An explicit
`firefox_user_prefs={"network.proxy.type": 0}` succeeded in a diagnostic navigation;
retain that local test setting when reproducing, without changing system settings.
The initial failure is retained separately. Firefox and WebKit lack LoAF instrumentation;
its absence must be reported as unsupported. It cannot be compared as zero LoAFs.

Current build tools are recorded per artifact rather than inferred from this
table: the captured local build used Cargo/rustc 1.99.0, Node 24.21.0, Python
3.12.14, mdBook 0.5.2, Typst 0.15.1 and wasm-bindgen 0.2.129. npm uses its lockfile;
Python browser tooling, including its transitive dependencies, is pinned in
`scripts/website-requirements.txt`; CI and the command above use Python 3.12.14.
CI now selects
Ubuntu 24.04, Rust 1.99.0 and Node 24.21.0, rejects build-time lockfile drift, and
records the GitHub runner image version in artifact provenance. GitHub's maintained
runner image and action tags can still change; this is not an immutable environment.
The revised workflow has not run on GitHub yet. Action-revision pinning and
the full browser/failure matrix remain required
before claiming CI reproducibility or adopting cross-runner performance gates.
Verification-only provenance labels the current checkout as the verifier source;
it does not claim to identify the build source of an externally supplied artifact.

## Accessibility implementation progress

The first drawer fix makes background content inert while the mobile modal is
open, moves focus inside when resizing from desktop to mobile, and restores prior
background inert states when closing or returning to desktop. Desktop details
remain nonmodal. Tests exercise Tab containment, blocked background focus, Escape
return from route details, and both responsive transitions. The same two behavior
tests passed with real Chromium 145.0.7632.6, Firefox 146.0.1 and WebKit 26.0 on the
prepared SVG artifact, and with the previous Cytoscape runtime in Chromium.

A fresh required-board artifact passed all 52 browser workflows and capture/build
checks after this fix. Its fourteen reference image hashes exactly match the
initial capture. No CSS or appearance change was needed. Rollback this isolated
focus fix to restore the previous behavior, with its known modal-isolation defect;
retain the failing regression tests as evidence until the fix is restored.

The next focus fix gives the popover its problem name, moves Enter/Space activation
into its controls, and returns Escape/Details focus to the originating graph node.
Comparison completion reorders route edges without detaching focused nodes. All
three new behavior checks passed in Chromium, Firefox and WebKit; the full fresh
artifact suite passed all 55 browser workflows and graph harness checks. All
fourteen reference image hashes remained unchanged.

Keyboard navigation now includes Home/End and cyclic Page Up/Down traversal through
all problem families in catalog order, including disconnected families. Arrows
choose a spatial neighbor inside a 90-degree directional cone, using distance
along the direction plus twice the lateral distance. Focus reveals the node and
label inside the available canvas, clear of panels; focused labels reuse the
existing important-label treatment. Variant repaint keeps focus in the same group,
and node names refresh route/start/end state without requiring a new focus event.

Direct click activation without pointer events is covered separately from pointer
and keyboard gestures. Space activates on release. This follows the interaction
and focus guidance in the [W3C button pattern](https://www.w3.org/WAI/ARIA/apg/patterns/button/);
the event checks do not establish real screen-reader compatibility. The six new
cases passed in Chromium, Firefox and WebKit; all 61 full production browser
workflows and the graph harness checks passed on an unchanged source snapshot.

Pinned axe-core 4.14.0 now checks eight rendered Atlas/Graph states on the actual
production artifact, including empty suggestions, popover and both drawer modes.
It is a test-only dependency. Serious/critical violations and invalid explicit
roles fail the suite; every other violation and every incomplete result is retained
for review. An incomplete contrast result is not a passing contrast check.

Those checks exposed empty-listbox/active-option defects and invalid document-link
roles. Search now clears stale active descendants, exposes an empty-result status,
and survives refocus before a queued blur. Variant repaint preserves endpoint focus.
Generated footnote roles belong to their links; following notes/backlinks transfers
keyboard focus to the destination. Overflowing examples have an explicit Tab stop
and reuse the existing focus indicator, which fixes WebKit's missing native Tab stop.
The scroll check uses a 100 ms keypress: an immediate down/up pair did not initiate
WebKit's first native scroll. No application scroll handler was added.

The fresh required-board artifact passed all 73 browser workflows and the existing
mathematical/build/harness checks. Its fourteen reference image hashes still match
the initial capture exactly. The focused cross-browser evidence and outstanding
scan findings are recorded in
[ACCESSIBILITY.md](baselines/2026-10-16-chromium-macos/ACCESSIBILITY.md).

Stage 1 remains incomplete: responsive edge cases, scan follow-ups
and the required manual/device checks still need implementation or verification.
The sporadic long-frame defect remains under investigation; the trace pilot is not
a claim that it was fixed.

A direct WCAG luminance check found 3.57925:1 contrast for small popup metadata on
its opaque background. The labels now reuse the existing secondary text color
(7.49836:1 there), as do small placeholder, suggestion, parameter and rule-link text
that used the same insufficient color. Palette values, typography and layout are
unchanged. The browser regression reads the computed colors and checks the unrounded
ratio against [WCAG's 4.5:1 minimum](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html).
This deliberately changes those text pixels for accessibility; the original
screenshots remain the reference, and other incomplete contrast checks remain open.

Graph navigation now offers native click/tap pan controls alongside zoom/fit. The
mobile dock sits above the results sheet. Each pan moves graph content 80 screen
pixels in its named direction; popup movement buttons reposition one problem by
40 screen pixels and update its connected edges through the same operation as
pointer dragging. Keyboard spatial navigation is unchanged. The popup scrolls when
its controls exceed the free canvas height, including the 320x640 check.

The new behavior checks compare the copied route before/after camera/node changes,
check connected-edge placement and preserved focus, and exercise actual
Playwright touch-emulated taps on the narrow mobile dock. Existing pointer pan,
node dragging and pinch checks still apply. Physical mobile/assistive-technology
acceptance remains separate. Screenshot changes are limited to the required
navigation/movement controls; their colors and styling reuse existing controls.
The two new mobile cases passed in the pinned Chromium, Firefox and WebKit engines;
this focused result does not certify the remaining cross-browser suite.
Rollback the isolated controls and shared node-movement extraction together to
restore the original drag-only behavior, keeping its known accessibility gap open.

Keyboard graph focus now stops camera animation and keeps the focused node revealed
when asynchronous comparison updates select a route. A real startup test focuses
isolated Bin Packing after paths arrive and checks visibility throughout the next
600 ms, including the original 450 ms camera transition. It failed before this fix;
no engine timing or renderer change is involved.

Details use their modal layout below 1168 CSS pixels, independently of the 760px
phone results-sheet breakpoint. The threshold reserves the supported 320px graph
viewport beside existing 392px/456px side-panel areas. Focus containment, background
inertness and shortcut handling follow the actual modal layout. Tests cross both
sides of this threshold, tablet and phone widths. The drawer's local heading uses
an ordinary container, so it does not add a duplicate page-banner landmark. Desktop
1440px and mobile 390px appearance are preserved. Rollback these focus/responsive
changes together with their mode-dependent handlers; retain the observed startup
and narrow-width defects in the acceptance record.

Atlas documentation now embeds its generated sections as native h2 headings below
the page h1, retaining attributes, content and the previous computed heading styles.
Graph articles keep h3 headings below the drawer h2. The problem-tools controls have
a named group role. These fixes address the heading/group findings in the first
scan; heading-order and duplicate/nonunique landmarks now also block automated
acceptance. They do not resolve incomplete contrast or real screen-reader checks.
The four focused camera/drawer/semantic cases passed in Chromium, Firefox and
WebKit. Both scanned states now report no violations in those engines, while
incomplete contrast and mathematical-label checks remain explicit. The Atlas
heading change preserves every computed heading style and the full-page screenshot
hash at 1440px and 390px in Chromium.
The shared-article end-to-end check compares text, exact formula markup and link
targets/roles across Graph and Atlas, and verifies their respective h3/h2 section
levels. It no longer requires identical HTML where the embedding hierarchy differs.

Rollback each accessibility fix independently, retaining its regression test as
evidence of the known defect. Revert the scan integration and its test-only pin
together if necessary; keep the existing fresh-artifact build/preview command.
No mathematical implementation changed.

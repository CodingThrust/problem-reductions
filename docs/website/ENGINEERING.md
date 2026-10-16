# Frontend engineering checks

Follow [FRONTEND_PLAN.md](FRONTEND_PLAN.md). Engineering preserves the current
appearance; Claude's visual-design phase starts after the engineering handoff.
The [data boundary inventory](DATA_BOUNDARIES.md) records the current validation
gaps and the minimum contract migration requirements.

## Fresh production artifact

Prerequisites match deployment: Node 22 or later, the Rust toolchain and
`wasm32-unknown-unknown` target, wasm-bindgen-cli 0.2.129, mdBook 0.5.2,
Typst 0.15.1, `uv`, and the repository's native HiGHS build requirements.
Keep Cargo on its normal PATH so local build scheduling and caching apply.
The verifier appends `$CARGO_HOME/bin` (default `~/.cargo/bin`) to its own
process PATH so installed mdBook/wasm-bindgen are discoverable without replacing
the active Cargo shim. It checks required executables before creating output or
starting the build. Missing tools produce an actionable error, not a traceback.
No shell configuration or Apple's Python installation is changed.

Install the pinned browser once:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python -m playwright install chromium
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
| Chromium 145.0.7632.6 | Complete required-board artifact suite; current-appearance capture; diagnostic pilots | Failure matrix, nested deployment, automated/manual accessibility, CI variance |
| Firefox 146.0.1 | Same initial smoke as WebKit, with explicit direct loopback access | Complete critical workflows, copy, failure recovery and accessibility |
| WebKit 26.0 | Initial smoke: home, Atlas alias/detail, compressed scene/WASM worker, comparison, warm query and zoom | Complete critical workflows, copy, failure recovery and accessibility |
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

Stage 1 remains incomplete: variant-control focus, visible/disconnected-node access,
drag alternatives, automated accessibility scans and the required manual/device
checks still need implementation or verification. The sporadic long-frame defect
remains under investigation; the trace pilot is not a claim that it was fixed.

Rollback: revert the isolated verification changes and use the existing
build/preview commands. No application assets or mathematical code change
as part of this increment.

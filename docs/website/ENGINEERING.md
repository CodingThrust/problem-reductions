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
uv run --no-project --with playwright==1.58.0 python -m playwright install chromium
```

Build and verify with one command:

```sh
uv run --no-project --with playwright==1.58.0 python scripts/check_website.py
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
uv run --no-project --with playwright==1.58.0 python scripts/check_website.py \
  --open-questions ../autoresearch-gadgets/website/dist
```

Without it, only the three board-dependent browser tests skip locally, with an
explicit reason. The legacy graph redirect still runs. HTTP errors other than
an absent registry are failures, and existing board content must pass its tests.
An explicitly supplied board is required, not optional.

CI always requires the board. Deployment verifies the already assembled
artifact, including the PDF and API, with:

```sh
uv run --no-project --with playwright==1.58.0 python scripts/check_website.py \
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
uv run --no-project --with playwright==1.58.0 python scripts/benchmark_graph_interaction.py \
  http://127.0.0.1:PORT/ --repeat 10 --output /tmp/interactions.json
```

Capture the current appearance from a passed, unchanged artifact:

```sh
uv run --no-project --with playwright==1.58.0 python scripts/capture_website_baselines.py \
  /path/to/verified-artifact --output /tmp/new-reference-capture
```

The capture command verifies all recorded artifact hashes before and after capture,
refuses existing output, asserts each page state and records image/source hashes.
The production verifier also exercises these desktop/mobile capture workflows;
it does not enable a pixel comparison gate or certify visual approval.

Rollback: revert the isolated verification changes and use the existing
build/preview commands. No application assets or mathematical code change
as part of this increment.

# Frontend engineering checks

See the [data boundary inventory](DATA_BOUNDARIES.md) for runtime contracts and
[ACCESSIBILITY_CHECKS.md](ACCESSIBILITY_CHECKS.md) for manual acceptance. Generated
screenshots, traces and raw samples belong outside Git or in CI artifacts.

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

## Size budgets

Blocking deterministic budgets, with about 5% raw / 6% gzip production headroom:

- `build_frontend.mjs`: all emitted `.js` chunks/assets, worker included and shared
  chunks counted once, at most **320,000 raw / 60,000 gzip bytes**. External binding
  JS, mdBook, Open Question Board and non-JS resources are outside this budget.
- `test:frontend-contracts`: committed guards and runtime helpers together, at most
  **230,000 raw / 34,000 gzip bytes** (about 10% / 13% headroom).

The build budget runs in Python builds and CI. The contract suite exercises budget

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

## Cold-start delivery (2026-10-16)

Slow-4G profile (1.6 Mbps, 100 ms RTT, 4x CPU, Chromium, text compressed): the
transfer of the path engine dominates. The browser WASM build alone uses
the `wasm-release` Cargo profile (`opt-level = "z"`, `panic = "abort"`); the workspace release
profile used by the CLI and tests is unchanged. Gzipped size: 267 KB to 167 KB
(raw 777 KB to 549 KB). First routes: about 2.25 s to 1.8 s (3-5 runs each; treat as
directional). Unthrottled, path search time is unchanged (66 vs 69 ms) and the
comparison summary takes 204 ms instead of 129 ms; it already follows the routes.
`graph.html` also preloads the scene and path-graph JSON, so the scene appears at
about 0.97 s in every run instead of 1.0 to 1.45 s. A low-priority WASM preload made
no difference. Not yet measured: real hosting, physical devices, Firefox and WebKit timing.

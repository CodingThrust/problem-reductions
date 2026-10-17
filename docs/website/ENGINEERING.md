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
accessibility, and failure recovery. Servers close on failure.

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

## Build integration and rollback

Python data generation and wasm-bindgen precede the two-page Vite emission step;
finalization then assigns immutable release-directory URLs. Stable filenames avoid
invalidating content hashes during finalization. ES imports and HTML preloads use
the same URL. A narrow worker resolver leaves the generated WASM binding beside
the worker. Static hosting, compressed payloads and the Rust string API stay intact.
CSS minification is disabled because its color/alpha transformations changed pixels.

`npm run typecheck` is a strict, separate `noEmit` check; Vite only emits runtime
JavaScript. `npm run lint:frontend` applies correctness rules without formatting
changes. Application entry points and modules are TypeScript; `graph-layout.js`
is a Node-only input to the build-time layout generator. Browser/compiler fixtures
check TS emission, strict rejection, CSP, CSS rendering and shared module delivery.

The worker buffers decompressed WASM before binding initialization to avoid a
Firefox streaming stall. Compression, early preload, worker termination and
bounded caches are preserved.

Rollback build integration, module entries and worker URL changes together to the
prior Python asset-copy path and a complete prior static release. Keep useful
compatibility checks. Never combine old HTML/data with newly compiled modules.

## Size budgets

Blocking deterministic budgets:

- `build_frontend.mjs`: all emitted `.js` chunks/assets, worker included and shared
  chunks counted once, at most **320,000 raw / 60,000 gzip bytes**. External binding
  JS, mdBook, Open Question Board and non-JS resources are outside this budget.
- `test:frontend-contracts`: committed guards and runtime helpers together, at most
  **230,000 raw / 34,000 gzip bytes**.

The build budget runs in Python builds and CI. The contract suite checks its
committed guard budget and verifies that oversized production JavaScript is rejected.

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

## Browser delivery

The browser WASM build uses the `wasm-release` Cargo profile (`opt-level = "z"`,
`panic = "abort"`); the CLI and tests use the workspace profiles. `graph.html`
preloads the scene and path-graph JSON. Measure current artifacts on representative
hosting and devices before making timing or size claims.

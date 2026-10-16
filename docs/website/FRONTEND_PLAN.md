# Frontend engineering modernization plan

Status: revised scope accepted on 2026-10-16; build integration, runtime contracts,
strict application TypeScript and selective module cleanup are implemented locally.
Required CI, manual accessibility and physical-device performance acceptance remain open.
See [engineering checks](ENGINEERING.md) for the fresh-artifact command and evidence
limits. Stage 0 remains in progress until all of its acceptance criteria pass.

The goal is a small research platform that is mathematically rigorous, fast,
accessible, and maintainable. Keep the shared Rust/WASM mathematics and the static,
release-directory deployment architecture intact. Modernize incrementally, without
rewriting the frontend or introducing a UI framework.

## Sequential ownership and scope

**Phase one is engineering only.** Complete, commit, and verify this phase before
preparing a clean handoff to Claude. Preserve the current visual appearance. CSS and
UI changes are allowed only where necessary for accessibility, functional correctness,
responsiveness, or performance; document their purpose and screenshot impact.

Do not perform aesthetic exploration, visual redesign, typography selection, palette
selection, design-token creation, or a visual-design audit during phase one. Visual
design is not a dependency of TypeScript migration, engineering work, or mathematical
development. Keep styling boundaries understandable so later design work can use the
existing DOM and application interfaces without a structural rewrite.

**Phase two belongs to Claude and starts only after the engineering handoff.** Claude
will review the website and define UI/UX direction, tokens, component styling,
responsive designs, and the graph's visual language. Implement those supplied designs
as a separate, reviewable stage after phase one; do not mix them with engineering
refactors. This is a sequential workflow, not parallel workstreams.

## Non-negotiables

1. Preserve mathematical semantics, exact values, independent proof verification,
   existing capabilities, route ordering, and end-to-end workflows.
2. Preserve the shared Rust computation and WASM string API. Typed wire responses may
   replace dynamic JSON assembly; do not change the comparator, proofs, or algorithms
   as part of frontend modernization.
3. Preserve static hosting, CSP, compressed delivery, lazy resources, and immutable
   release URLs. No server runtime or deployment-platform migration.
4. Ship small, complete changes. No speculative abstractions, renderer replacement,
   dependency, directory tree, or state framework. Explain each dependency's concrete
   benefit and measure its runtime cost where applicable.
5. Establish expected behavior independently and write a meaningful failing test
   before changing behavior. Reuse behavior tests for refactors; use real dependencies.
6. Every stage has its own acceptance evidence and rollback path. Revert only that
   stage's isolated changes; never reset, stash away, or discard unrelated working-tree
   work. Do not claim unavailable browser, hardware, or manual checks have passed.
7. No aesthetic redesign or visual exploration until phase one is complete, committed,
   verified, and handed off to Claude.
8. Keep screenshots, traces, raw benchmark samples and generated run reports outside
   Git, in local evidence storage or CI artifacts. Commit source, tests, contracts,
   configuration and concise maintained documentation. Keep frontend changes
   independently reviewable on the user-selected branch; do not push feature work
   without explicit authorization.

## Implementation and verification cadence

Complete coherent modules within a stage before running its relevant existing
suite. Reuse current behavior tests for migrations and refactors. Add a focused
regression test only for meaningful new behavior, a confirmed defect, or an
important invariant without existing coverage; do not expand test infrastructure
or chase coverage percentages. Use targeted checks during implementation only to
diagnose actual issues. Reserve the complete production suite for major milestones
and final acceptance. Repeat benchmarks only for performance changes or a concrete
performance concern, and keep evidence summaries concise.

Validate untrusted inputs at entry, then trust documented internal invariants.
Do not add redundant guards, hidden fallbacks, speculative abstractions, or
intermediate compatibility layers. Migrate complete modules directly where that
is simpler. Keep all existing mathematical, accessibility, and delivery gates.

## Starting observations and evidence limits (2026-10-16)

- The current source has no bundler, TypeScript checking, or linting. Globals connect
  the scripts. `graph.js` is 581 lines; `graph-scene.js` is 225 lines. These sizes alone
  do not justify a broad architecture rewrite.
- An earlier fresh build reported 46/49 browser checks passing, with three requiring
  the separate open-questions output. Re-establish this baseline; local absence may
  produce explicit skips, but production CI must build and test that output.
- `book/` can be stale. Build and serve a new scratch output; record tool versions,
  source state and artifact hashes, including uncommitted preparation.
- The earlier headless rAF p95 of about 26 ms is also present while idle and is not a
  renderer-cost result. A reported 68-82 ms first pan/drag hitch needs confirmation
  with the repaired harness before attribution or optimization.
- The earlier interaction harness captured a node rectangle before camera changes,
  did not assert the intended drag occurred, and suppressed instrumentation errors.
  The Stage 0 repair now asserts gesture effects, isolates first interactions and
  reports observer support. Sub-50 ms render cost and stable timing gates remain
  unverified; do not infer them from the repaired rAF/LoAF diagnostics.
- WASM inputs are deserialized Rust structs; outputs use dynamic `json!` assembly.
  `find_paths`, `compare_paths`, and `summarize_paths` exchange JSON strings. Exported
  wasm-bindgen declarations describe those strings, not their payloads. Comparison
  responses already carry `schema_version: 5`; preserve compatibility deliberately.
- Python generates scene, path-graph and problem-record data without a shared schema.
  Worker messages are untyped. Inventory every actual JSON consumption boundary,
  including lazy records, article/index data and embedded JSON, before migration.

## Correctness and runtime contract policy

For every untrusted JSON boundary: **parse as `unknown`, validate against the
appropriate schema, then use the typed value**. This covers fetched or decompressed
JSON, WASM results, incoming worker messages in both directions, and persisted or
embedded JSON when consumed. Structured-cloned messages still require validation.
A TypeScript declaration, assertion, successful JSON parse, or version check alone
is not validation. Validate once on entry rather than repeatedly inside pure logic.

- Maintain one authoritative schema per wire payload. Prefer typed Rust responses
  with Schemars-derived JSON Schema and checked Python-owned schemas; generate TS
  types and runtime validators from those schemas. Confirm tool compatibility in a
  small boundary migration before adopting it across the site. Keep generation out
  of the production WASM path and measure bundle/WASM cost.
- Validate producer output at build time and consumer input at runtime. CI must detect
  drift in Rust/Python schemas, generated TS types, and validators. Reject missing or
  mistyped fields, invalid union tags, invalid references/ranges, and unsupported
  schema versions with actionable user-visible errors; retain all construction and
  mathematical validation inside Rust.
- Define version compatibility for each payload, reusing the existing comparison
  version. A valid version cannot excuse an invalid shape. Keep previous static
  releases self-contained so open pages cannot mix new data with old code.
- Preserve exact integers and rationals through the entire wire path. Keep existing
  exact certificate/expression strings exact. Integers outside JS's safe range must
  use validated decimal strings; rational components use exact strings with validated
  sign and nonzero denominator conventions. Never route exact values through `Number`,
  floating-point parsing, rounded schema limits, or `BigInt`-to-`Number` conversions.
  Safe numeric indices/counts require explicit integer/range checks. Validate strings
  before conversion and test values beyond 2^53, signs, fractions and zero denominators.
- Compare mathematical JSON semantically: ignore object-key order and whitespace,
  preserve array/route order, exact values and certificates, verdicts, source/target
  variants, and absent-versus-null optional fields. Do not normalize away differences
  in mathematical expressions without an independent equivalence check. Preserve
  independent proof/certificate checks and representative CLI-to-browser end-to-end
  tests. Byte equality is required only where bytes are the actual contract.

## Measurement and gating policy

Repair and validate instrumentation before collecting a baseline or optimizing.
Store raw samples, sample units/counts, scenario duration, environment/tool versions,
artifact hashes, cache state, instrumentation support, and summaries with the results.

- Assert observable effects for wheel zoom, background pan, node drag and keyboard
  actions. Resolve visible hit targets immediately before each action; confirm a drag
  moves the node rather than the camera and confirm a pan moves the camera.
- Measure the first zoom, pan, drag and relevant activation separately in fresh pages,
  without preceding gestures warming the path. Use fresh browser processes for
  independent cold runs. Also test representative interactions during startup.
- Distinguish event-level input delay/handler/presentation latency, active-frame
  script/render cost, and run-level cold-start/query/first-interaction observations.
  Report per-run summaries and distributions across runs separately; do not pool
  correlated events into apparently independent runs.
- Keep an idle control. Use traces or suitable instrumentation for costs below 50 ms;
  LoAF only diagnoses frames above that threshold. Record support explicitly;
  unsupported/error is not zero long frames. rAF intervals alone prove no renderer
  budget. INP supplements discrete interactions, not continuous pan/zoom measurements.
- Start with at least ten independent pilot runs. Record median, p95, sample count,
  quantile estimator and run-to-run variation, including CV. With nearest-rank p95,
  ten observations yield the maximum; do not describe this as a stable tail estimate.
  Collect further runs until the chosen uncertainty and practical regression margin
  are documented, or keep that metric informational. CV alone is not a stability test.
- Pin browser/tool versions and CI image, control concurrency and cache/network/CPU
  settings, and record display refresh rate/pixel ratio. Mobile emulation is not a
  substitute for the documented physical-device/manual acceptance checks.
- **Baseline non-regression gates** compare like-for-like measurements against the
  Stage 0 baseline with a documented practical margin and measured uncertainty.
  CI timing remains informational until at least ten CI runs and the metric's
  uncertainty demonstrate the gate can detect that margin reliably. Do not silently
  loosen a gate or regenerate baselines to accept a regression.
- **Absolute product targets** are separate: initially p95 active-update script plus
  render cost below 8 ms on the declared reference device, no >50 ms sustained-input
  frames in the fixed test scenario, and first-interaction frame below 50 ms. Record
  coalesced-event counts and input-to-presentation latency separately. Confirm these
  targets against real measurements; a pre-existing miss is a tracked defect, not a
  newly introduced regression. Never hide the first hitch by dropping initial frames.
- Type checks, adopted lint rules, schema/drift checks, behavior tests, accessibility
  checks, and measured asset-size budgets block from adoption. Screenshots block
  after stability is demonstrated in their pinned environment. Initial reported WASM
  sizes (812 KB raw / 280 KB gzip) must be remeasured before setting exact budgets.

## Phase one stages

Each stage consists of small reviewable commits with its own evidence. Do not combine
build migration, mathematical serialization, module extraction, or styling changes in
one commit. A stage is incomplete while required acceptance checks remain unverified.

### Stage 0 - Reproducible builds and trustworthy benchmarks

Deliverables:
- One documented command builds into a fresh scratch directory, serves exactly that
  artifact, runs the applicable suite, and cleans up its server. Pin dependencies and
  record build provenance. Preserve existing preparation and unfinished a11y work.
- Explicit local-only skips for absent open-questions output; a required-output CI
  mode fails if it is absent and runs the full production suite.
- Repair the interaction harness as specified above, with a real-browser self-check
  that fails if gestures have no intended effect. Report missing instrumentation.
- Capture current-appearance screenshots for home, Atlas, graph, popover, drawer and
  empty/error states at desktop 1440x1000 and mobile 390x844. Pin rendering environment,
  await fonts, and use reduced motion. These are engineering regression references,
  not a visual-design audit or a redesign proposal.
- Establish cold-start, route/search latency, WASM initialization/computation, memory,
  asset-size and interaction baselines. Record the boundary inventory and supported
  browser/tool matrix. No mandatory 20x/100x graph or renderer prototypes.

Acceptance: a fresh checkout can follow the documented command; applicable existing
checks pass, with explicit skip reasons locally and no missing production feature
checks in CI. Gesture assertions and independent first-interaction scenarios pass;
support metadata and raw samples exist. Document each metric's uncertainty and gate
status. Preserve runtime behavior and current appearance.

Rollback: revert the isolated harness/build-test changes; retain prior samples with
provenance and mark invalid measurements unusable. No runtime asset rollback needed.

### Stage 1 - Early interaction defect fixes and accessibility

Deliverables:
- Confirm and profile the reported first-interaction hitch with Stage 0, attribute its
  script/style/layout/paint cost, and implement the smallest evidenced fix. Do not
  change renderers or mathematical algorithms to address it.
- Complete keyboard graph access: roving tabindex, documented neighbor direction and
  disconnected-node access, Enter/Space activation, semantic names and route state,
  visible focus, reveal-on-focus, and popover focus entry/Escape return.
- Keep desktop details nonmodal unless functionality requires otherwise: background
  remains operable and focus can leave. Mobile modal drawer moves focus inside,
  contains it, makes background inert, closes on Escape, and returns focus correctly.
- Provide equivalent click/tap controls for needed drag/pan functionality, alongside
  keyboard access. Preserve existing touch, wheel, selection, copy, and cancel flows.
- Cover search combobox, `/`, Escape, deep links/back-forward where supported, route
  selection, details, status announcements, and error/retry with behavior tests.
- Run axe on graph and Atlas states, plus a focused manual keyboard/screen-reader
  script: macOS Safari/VoiceOver desktop and a named mobile screen-reader/device;
  record versions, steps, announcements, outcomes and any unavailable check.

Acceptance: keyboard-only query, route selection, copy and details open/close succeed;
manual checks confirm direction, focus/selection, isolated-node access and async
status are understandable; no serious/critical axe violations or known blocking
manual defects. Each first interaction is measured independently. Meet the <50 ms
first-interaction target on the reference environment, or retain the stage as
incomplete with an explicit unresolved defect. Other changes meet baseline
non-regression gates; existing absolute-target misses are tracked separately.
Screenshot changes are limited to the necessary engineering fixes and explained.

Rollback: revert individual fixes independently, restore prior focus/interaction behavior
with its known defects documented, and retain the harness and regression evidence.

### Stage 2 - Minimal Vite build integration

Deliverables:
- Add a pinned minimal Vite multi-page build for the existing pages/scripts before
  introducing executable TypeScript. Establish JS emission from TypeScript and
  separate `tsc --noEmit` strict checking with isolated-module-compatible settings.
  A small build smoke fixture may prove emission; no application rewrite.
- Integrate with Python generation, wasm-bindgen, gzip streams, CSP and the existing
  release/finalize process. Define asset-rewriting order; do not invalidate content
  hashes by mutating fingerprinted assets afterward. Use standard analyzable module
  worker construction and explicit asset URLs as needed; preserve preload ordering.
- Add minimal linting for correctness to touched frontend code, with an explicit
  migration boundary for legacy files. No mass formatting or stylistic cleanup.
- Verify Chromium, Firefox and WebKit critical workflows using pinned Playwright
  browsers; record supported real-browser versions and manual Safari/mobile results.
  Keep engine-specific tracing informational where unavailable, never fabricate parity.
- Test the finalized production artifact at root and nested base paths: scene and
  compressed JSON, compressed WASM delivery/decompression and generated-glue
  initialization, workers,
  article/index lazy loading, navigation, CSP, preloads and immutable release URLs.
  Keep large resources off pages that do not need them.
- Buffer decompressed WASM bytes inside the worker before binding initialization.
  Real Firefox replay exposed a pre-existing streamed-compilation stall after rapid
  cold query changes; buffered initialization passed the diagnostic replays. Keep
  compressed network delivery, early preload, the unchanged Rust binary/string API,
  worker termination and bounded caching. Measure the temporary buffer and startup
  cost; this is a compatibility fix, not a renderer or mathematical-engine change.

Acceptance: the existing JS app passes production-artifact behavior and CSP tests with
unchanged appearance; TS emission and separate strict checking both work; lint passes.
All three browser engines pass critical flows. Test offline/404/corrupt resources,
worker-startup and WASM-init failures, cancellation during both phases, and successful
retry without stuck busy state or stale results. Confirm pinned fresh CI installs,
required open-questions checks, size budgets and cold-start non-regression.

Rollback: revert the isolated build integration and return to the existing Python
asset-copy path; deploy a prior complete static release, never a mixed artifact.
Keep useful compatibility/failure tests. No executable application TS precedes this gate.

### Stage 3 - Runtime contracts and incremental strict TypeScript migration

Deliverables:
- Implement the boundary inventory using the correctness/contract policy. Start with
  one complete producer-to-consumer slice, then migrate remaining boundaries.
  Introduce Rust `Serialize` wire-response structs without changing math. Enable
  wasm-bindgen declarations but do not mistake string declarations for JSON schemas.
- Generate and drift-check schemas, strict TS types and runtime validators. Parse
  as unknown, validate, then use the value; validate incoming worker requests and
  replies as discriminated unions with version compatibility. Preserve termination
  cancellation and the existing bounded cache rather than adding scheduling machinery.
- Adopt strict TS for each migrated file; remove the legacy JavaScript exception
  once the application migration is complete. Run every relevant config in CI. Avoid `any`, suppression
  directives and blanket assertions; allow a justified assertion after validation.
- Type the existing `GraphScene` public interface and real operations. Preserve its
  SVG implementation and DOM-independent application inputs; no generic
  `GraphRenderer`, Canvas/WebGL adapter, or renderer contract framework.
- Retain exact serialization, optional-field semantics, semantic response comparison,
  independent certificate/proof verification and CLI/browser end-to-end workflows.

Acceptance: every inventoried boundary has a validated path and negative-input tests;
intentional producer/schema/type drift fails CI. Strict checks, lint, unit and browser
checks pass. Beyond-safe-range integer and rational fixtures round-trip without
precision loss; malformed encodings fail clearly. Parsed mathematical outputs match
independent fixtures and existing pair verdicts/certificates, including ordering and
absent/null distinctions. Measure validator/serialization startup, transfer and WASM
size costs against Stage 0 and adopted budgets; no mathematical regression.
The current byte measurements, blocking budgets and browser-measurement limits are
recorded in [ENGINEERING.md](ENGINEERING.md#validator-delivery-and-size-budgets-2026-10-16).

Rollback: revert one complete schema/producer/consumer slice together. Restore its
previous self-contained release; never deploy a new producer with an old consumer.
Keep validators and generated files aligned with the reverted schema version.

### Stage 4 - Selective module refactoring and maintainability

Deliverables:
- Extract only independently testable or separately owned responsibilities that
  currently hinder maintenance: registry/search helpers, the worker client, route
  presentation or page wiring as justified by actual use. Keep the existing state
  object and GraphScene unless a concrete defect requires otherwise.
- Use native ES modules and strict TS for migrated runtime code. Remove corresponding
  globals when their callers migrate; do not invent a prescribed directory tree,
  store framework, generic UI components or alternate rendering API.
- Add a directory only when multiple real modules need it. Keep existing CSS/markup
  ownership and selectors legible for the later design handoff without choosing tokens,
  typography, palettes or component aesthetics.
- Use the existing real unit-test tooling where sufficient; add a shared Vite-compatible
  runner only if a concrete test need justifies it. Retain end-to-end behavior tests.

Acceptance: each extraction names its responsibility/test boundary; all callers are
updated, strict checks and existing behavior checks pass, production delivery and
failure recovery remain intact, screenshots match and measured performance stays
within non-regression budgets. Refactors preserve independent mathematical evidence.

Rollback: revert each behavior-preserving extraction separately, restoring its callers
at the same time. Preserve the earlier Vite pipeline and runtime contracts.

### Stage 5 - Conditional scalability and performance work

Deliverables: only open this stage when actual registry growth or a measured bottleneck
at the documented plausible future size warrants it. Otherwise record the trigger as
unmet and defer it; this is not an engineering-completion dependency. Test realistic
node/edge density, degree distribution, labels, viewport and device pixel ratio with
the repaired harness. Optimize the current SVG scene's measured hot path first.

Acceptance: a written comparison identifies the workload, bottleneck, costs and
before/after evidence, with no correctness or accessibility loss. A second renderer
requires demonstrated need and a separate scoped decision; no speculative prototype
is part of the mandatory modernization. Larger stress tiers are explicitly exploratory.

Rollback: revert each optimization independently to the validated SVG implementation;
retain workload/sample provenance. Defer unproven changes.

### Stage 6 - Engineering completion and clean handoff to Claude

Deliverables: commit a handoff describing the actual frontend entry points and module
responsibilities, contracts/generation commands, relevant files, DOM/CSS ownership and
styling boundaries, build/test commands, supported browser matrix, manual accessibility
results, performance baselines/budgets, known limits, deployment/rollback instructions,
and current-appearance screenshot references. Explain visual constraints imposed by
focus, hit targets, contrast, labels, loading states, responsive drawers and performance;
do not propose aesthetic direction or create a visual design system.

Acceptance: mandatory Stages 0-4 are complete and committed, applicable Stage 5 work is
verified or explicitly deferred by its trigger, and the committed engineering revision
passes the complete production-artifact suite, strict checking, lint, schema drift,
independent mathematical/proof checks, manual accessibility and supported-browser gates.
Record commands, versions, artifact hashes and results against that revision. No required
check is silently waived. Only then hand off to Claude and start phase two.

Rollback: correct an inaccurate handoff or revert a failing engineering change and
repeat the affected gates; phase two remains unstarted until the engineering gate passes.

## Phase two - Claude-owned UI/UX and visual refinement

This phase begins only after Stage 6. Claude reviews the actual validated site and
produces visual direction, tokens, component styling specifications, responsive designs
and graph visual language. None of that work is performed during engineering.

Integrate supplied designs in separate small reviewable changes, with before/after
screenshots and design approval. Preserve all capabilities, accessibility/manual
screen-reader outcomes, runtime contracts, exact mathematical semantics, CSP/static
release compatibility and cold-start/interaction budgets. Revert a styling change
independently if it fails; do not use a design change to reopen architecture without a
concrete engineering need. This later phase does not block mathematical development.

## Stop rule

After the engineering handoff, presume against further architecture work unless an
actual requirement, correctness/accessibility defect, or measured performance issue
justifies it. Do not impose a permanent freeze that prevents necessary maintenance.

## Technical references informing this revision

- [Type assertions do not validate at runtime](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#type-assertions)
- [TypeScript noEmit](https://www.typescriptlang.org/tsconfig/noEmit.html) and
  [Vite transpilation, workers and CSP](https://vite.dev/guide/features)
- [Schemars and Serde](https://graham.cool/schemars/)
- [JSON semantics](https://www.rfc-editor.org/rfc/rfc8259) and
  [exact large integers in JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt)
- [Long Animation Frames and support detection](https://developer.chrome.com/docs/web-platform/long-animation-frames)
- [INP scope](https://web.dev/articles/inp) and
  [percentile estimation](https://itl.nist.gov/div898/handbook/prc/section2/prc262.htm)
- [Automated and manual accessibility checks](https://playwright.dev/docs/accessibility-testing),
  [keyboard interfaces](https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/),
  [modal dialogs](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/) and
  [drag alternatives](https://www.w3.org/WAI/WCAG22/Understanding/dragging-movements.html)
- [Screenshot environment reproducibility](https://playwright.dev/docs/test-snapshots) and
  [Vite public base paths](https://vite.dev/guide/build#public-base-path)

# Vite delivery and worker compatibility evidence

The fresh production artifact in `vite-source-verification.json.gz` passed its
complete production verification with unchanged source throughout the run. Its
original revision/dirty-source fingerprints are authoritative; the evidence was
captured before the integration commit. The artifact includes the real Open Question
Board, mdBook, API, paper, lazy articles and unchanged Rust/WASM engine. This local
result does not certify GitHub CI, manual Safari/VoiceOver or physical mobile.

`vite-production-verification.log.gz` retains the complete build/check output.
Each of Chromium 145.0.7632.6, Firefox 146.0.1 and WebKit 26.0 ran the 79-case root
suite, eight failure/recovery cases and seven critical nested-path workflows under
`/research/reductions/`. Firefox/WebKit each explicitly skip the one native multi-touch
injection test because it requires CDP. Network-throttling instrumentation is also
reported unavailable there; eager-worker ordering is still checked. Native WebKit
paste verifies the real clipboard when script reads are denied. Independent CLI
comparison, certificate and copy/reduce/solve checks remain in the suite. The longest
reported individual case was 3.074 seconds, below the repository's five-second limit.

Failure checks mutate copies of actual delivered bytes, never the verified original.
They cover missing worker, invalid WASM, corrupt metadata, cancellation before paths,
offline startup, missing documentation index/article and scene failure/reload. Existing
root coverage also cancels during comparison. Recovery must produce usable route
JSON; expected endpoints remain SAT → QUBO. The scene-error check waits for the eager
worker's actual WASM request before serving the 404, then observes worker closure.
Firefox's closure reporting before worker execution was inconclusive in an immediate
404 diagnostic; resource cleanup at that timing remains unverified.

## Appearance and delivery

All fourteen captured screenshots match the pre-Vite current-appearance control
exactly, at 1440×1000 and 390×844. `vite-screenshots.json.gz` records settings/hashes;
`vite-appearance/` retains these current images. The earlier references are preserved.
Vite's CSS minification is disabled because its alpha/color transformations changed
pixels. This is an engineering regression check, with no new visual direction.

The two-page build fixture runs emitted TypeScript in a real browser with finalized
CSP, verifies rendered CSS against the native source, and rejects duplicate shared
module downloads. A separate compiler fixture verifies strict errors and no JS
emission. Production uses immutable release-directory URLs; preloads and ES imports
share one URL. The generated binding, worker cancellation, bounded caches, static
hosting and compressed WASM/data delivery remain intact.

## Isolated performance pilots

Both batches used the recorded reference Mac, native CPU speed, 1440×1000/DPR 1,
Playwright 1.58.0 and pinned Chromium. No builds, test suites or capture jobs ran
concurrently. Ten untraced cold runs precede the fifty traced interaction scenarios;
each scenario starts in a fresh browser process/context. Physical display refresh
was not instrumented. Ten-run nearest-rank p95 is the maximum, not a stable tail.

| Untraced cold observation | Median | p95 |
| --- | ---: | ---: |
| Graph readiness proxy | 55.0 ms | 61.0 ms |
| Route readiness proxy | 82.3 ms | 92.2 ms |
| Full readiness, including camera | 646.65 ms | 667.0 ms |
| Startup fill-to-suggestion round trip | 37.9 ms | 45.9 ms |
| Worker engine fetch through initialization | 4.35 ms | 7.5 ms |
| Warm search DOM latency | 0.25 ms | 0.4 ms |
| Warm route query DOM latency | 19.8 ms | 29.2 ms |
| Sampled Chromium RSS sum | 464.23 MiB | 475.22 MiB |
| Sampled main-page JS heap | 4.68 MiB | 5.01 MiB |

All ten startup inputs began before comparison finished. WASM linear capacity stays
3.375 MiB. RSS sums can double count shared pages, 50 ms polling misses short peaks,
and page heap/WASM capacity do not isolate the temporary worker input buffer.
Buffered initialization adds an 806,666-byte JS input buffer before the binding call.
Its lifetime and compilation's other allocations are not separately measured here.

Startup JavaScript fell from 68,890 to 46,832 decoded bytes. WASM stays exactly
806,666 raw / 277,560 gzip bytes with unchanged hashes; scene/path metadata hashes
also match the preceding artifact. Route readiness rose from 71.3 to 82.3 ms median
and 84.4 to 92.2 ms sampled p95; full readiness rose about 10 ms. Engine initialization
rose 0.8 ms median. These are observed differences, not isolated causal effects or a
statistical non-regression certificate. The corresponding worker-ready offset and
comparison durations also varied. Timing gates remain informational pending calibrated
uncertainty, a practical margin and ten comparable CI runs; no baseline is replaced.

| Interaction | Median of run p95 slice costs | p95 of run p95 slice costs | CV | First-input rAF p95 |
| --- | ---: | ---: | ---: | ---: |
| Wheel zoom | 1.776 ms | 2.259 ms | 0.094 | 17.7 ms |
| Background pan | 1.571 ms | 1.857 ms | 0.086 | 38.6 ms |
| Node drag | 2.656 ms | 2.980 ms | 0.110 | 30.5 ms |
| Keyboard activation | 3.328 ms | 4.249 ms | 0.120 | 24.9 ms |

Slice accounting unions named script/render work on the renderer main thread and
includes tracing overhead; GPU, raster threads and unclassified work are excluded.
All active scenarios had supported trace coverage. Idle active cost is unavailable,
not zero. No LoAF occurred in these fifty runs. The earlier sporadic hitch remains
unresolved; these pilots do not certify the complete 8 ms render/presentation target.
First-input rAF intervals describe scheduling, separately from slice costs. Raw event,
frame and run samples, observer status and verified effects remain in
`vite-interactions.json.gz`; all fifty traces are in `vite-interaction-traces.tar.gz`.
`vite-cold-start.json.gz` retains all ten cold samples and summary CVs.

## Firefox initialization diagnostic

`vite-firefox-initialization-diagnostic.tar.gz` retains the native cold-query replay
script and logs: six of ten pre-Vite rapid MVC/One → LCS queries stalled; ten buffered
replays passed. Keepalive and separate streaming-compilation attempts still failed.
This diagnostic used scratch overlays, not passed benchmark artifacts; its runtimes
are not performance evidence. The production suite subsequently passed the same
native query flow with buffered initialization. Do not infer an exact underlying
Firefox cause or confuse this stall with the separate sporadic interaction hitch.

Rollback the isolated build integration to Python's prior asset-copy path and a
complete prior static release. Retain the worker compatibility fix and useful checks.
Runtime contracts/application TypeScript, CI verification, required manual acceptance
and final engineering handoff remain separate work. Claude's visual phase is unstarted.

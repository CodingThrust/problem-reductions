# Performance after the early accessibility fixes

Ten independent runs per interaction (including idle controls) and ten cold
navigation runs use the unchanged, freshly verified production artifact recorded
in `accessibility-followup-source-verification.json.gz`. That artifact passed all
79 browser workflows and the mathematical/build/harness checks. Its source was
captured before commit `d77d3365`; the original revision and dirty-source fingerprints
remain in provenance. No builds, browser suites or captures ran concurrently with
either measurement batch. Both batches use the same reference Mac, pinned Chromium
145.0.7632.6/Playwright 1.58.0, native CPU speed, 1440x1000/DPR 1 and fresh browser
processes/contexts. No physical-mobile performance claim follows from these runs.
Physical display refresh was not instrumented in headless mode; rAF cadence cannot
be treated as certification of a display-specific presentation budget.

`accessibility-interactions.json.gz` retains raw events/frames, observer status,
verified gesture effects and per-run results. The fifty diagnostic traces are in
`accessibility-interaction-traces.tar.gz`. Interaction timings include tracing
overhead; cold navigation is separately untraced. Do not pool them.

| Interaction | Median of run p95 slice costs (ms) | p95 of run p95 slice costs (ms) | CV | First-input rAF p95 (ms) |
| --- | ---: | ---: | ---: | ---: |
| Wheel zoom | 1.982 | 2.480 | 0.107 | 20.3 |
| Background pan | 1.584 | 2.150 | 0.133 | 32.8 |
| Node drag | 2.524 | 2.823 | 0.077 | 34.9 |
| Keyboard activation | 3.199 | 4.202 | 0.110 | 19.8 |

Slice accounting has the same limited scope as [FRAME_COSTS.md](FRAME_COSTS.md):
union of named main-thread script/render slices in complete frame windows, including
instrumentation and excluding GPU/raster threads and unclassified work. All active
scenarios had supported trace coverage. Idle active cost is unavailable, not zero.
No LoAF was observed in these fifty runs; the earlier sporadic long-frame defect
remains unresolved. First-input intervals are scheduling observations, not costs.
Their CVs are 0.129, 0.795, 0.635 and 0.587 respectively; timing gates remain
informational. Ten-run nearest-rank p95 is the maximum, not a stable tail estimate.

Activation's median slice cost is about 0.91 ms above the initial traced pilot;
its sampled p95 is lower. This is a recorded cost change, not a claim of statistical
non-regression or of meeting the complete 8 ms render target. Other differences
also do not establish speedups. These pilots do not isolate causality or establish
a practical regression margin and CI uncertainty.

`accessibility-cold-start.json.gz` contains all ten cold samples and summaries.
Loopback HTTP sends uncompressed HTML/JS/CSS alongside explicit compressed data/WASM.
Readiness uses DOM/two-rAF proxies, plus stable camera/font checks for full readiness.
All ten real search interactions began before comparison completed.

| Cold or warm observation | Median | p95 |
| --- | ---: | ---: |
| Graph readiness proxy (ms) | 54.75 | 63.4 |
| Route readiness proxy (ms) | 71.3 | 84.4 |
| Full readiness incl. camera (ms) | 636.65 | 657.4 |
| Startup fill-to-suggestion round trip (ms) | 36.35 | 47.2 |
| Warm suggestion DOM latency (ms) | 0.25 | 0.4 |
| Warm MIS to QUBO query DOM latency (ms) | 16.7 | 23.7 |
| WASM fetch to initialized (ms) | 3.55 | 5.0 |
| Initial path search (ms) | 5.85 | 10.1 |
| Initial comparison (ms) | 76.75 | 80.2 |
| Sampled Chromium RSS sum (MiB) | 466.12 | 479.86 |
| Sampled page JS heap (MiB) | 4.78 | 4.90 |

The memory limitations from the initial pilot remain: 50 ms polling misses peaks,
RSS can double-count shared pages, page heap omits workers, and 3.375 MiB WASM memory
is capacity rather than live usage. Startup input includes driver overhead; warm
DOM measurements exclude presentation and OS delay. Observer support and raw data
remain in the archive. Missing instrumentation is never a passing zero.

The raw and compressed WASM binaries and compressed scene are byte-identical to
the initial capture. Downloaded/decoded startup JS is now 68,890 bytes, 6,060 bytes
above the initial 62,830-byte sample after the functional/accessibility additions.
No palette, font, renderer or mathematical engine change is part of this increment.
`engineering_accessibility_followup` in `../../performance-results.json` records
archive/source hashes, environment, complete summaries and limitations. Initial
references remain intact; roll back this evidence independently without changing
runtime behavior. Real-browser/manual checks, CI variance and the sporadic hitch
remain open before final engineering acceptance.

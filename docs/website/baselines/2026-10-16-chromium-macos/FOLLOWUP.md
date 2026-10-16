# Pointerdown, keyboard and cold-navigation measurements

This follow-up uses the same unchanged production artifact and source manifest as
the initial capture. The `engineering_stage0_followup` section in
`../../performance-results.json` records tool hashes, archive hashes, environment,
all summaries, sample counts, unavailable counts and CV. Each untraced metric
below has ten independent fresh-process observations; nearest-rank p95 is still
the maximum. All timing remains informational, with no inferred regression margin.

## First interaction coverage

The corrected harness starts pan/drag measurement at pointerdown. It also verifies
that Enter on a focused graph node opens that node's popover. Event Timing support,
errors and raw entries are separate from LoAF support; its discrete, thresholded,
quantized observations do not measure continuous wheel/move latency or establish
INP from this small workload. A missing entry is not zero latency.

| Scenario | Median first observed input rAF interval (ms) | p95 (ms) | CV |
| --- | ---: | ---: | ---: |
| Wheel | 13.35 | 26.40 | 0.359 |
| Pan pointerdown | 13.40 | 34.00 | 0.999 |
| Drag pointerdown | 27.90 | 35.50 | 0.138 |
| Keyboard activation | 9.85 | 13.80 | 0.502 |

`pointerdown-interactions.json.gz` retains fifty scenario trials, including ten
idle controls. All input effects passed. LoAF/Event Timing were supported; no LoAF
was observed in this batch. This does not invalidate the two earlier long frames
or prove their cause was fixed. The first-input timestamps are observations by the
capture listener; native event timestamps are retained separately. Neither is a
direct measurement of OS input-to-pixel latency.

`interaction-traces/` and `traced-interactions.json.gz` retain one additional traced
trial per scenario. Tracing adds overhead, so these are diagnostic rather than
comparable baseline samples. On the main renderer thread, maximum EventDispatch
slices were 0.607 ms for wheel, 0.466 ms for pan, 0.999 ms for drag and 1.549 ms
for keyboard activation. The largest single Paint slice in the drag trace was
3.649 ms. Slices can nest; adding such maxima does not yield a frame cost. The raw
traces include style, layout, paint, animation-frame and GC data for subsequent
attribution. They do not yet establish the p95 8 ms active-update target or explain
the earlier 63/71 ms long frames. No renderer or mathematical change follows from
these observations alone.

## Cold navigation, search and memory

`cold-start.json.gz` uses fresh Chromium processes, cleared browser caches,
1440×1000/DPR 1, native CPU speed and uncompressed loopback HTTP. Startup uses the
normal animations. It verifies loaded routes and final comparison, loaded fonts,
stable camera samples, the actual MIS suggestion, and a subsequent uncached
MIS→QUBO query through the warm mathematical engine.

| Metric | Median | p95 | CV |
| --- | ---: | ---: | ---: |
| Graph readiness proxy (ms) | 64.00 | 114.00 | 0.241 |
| Route readiness proxy (ms) | 87.55 | 157.10 | 0.266 |
| Full readiness proxy, including camera settling (ms) | 660.90 | 712.70 | 0.030 |
| Startup fill-to-suggestion automation round trip (ms) | 48.00 | 61.40 | 0.133 |
| Warm input-to-suggestion DOM update (ms) | 0.30 | 0.50 | 0.323 |
| Warm query keydown-to-comparison DOM update (ms) | 20.15 | 31.70 | 0.223 |
| Engine fetch through initialization (ms) | 3.90 | 5.90 | 0.216 |
| Path computation (ms) | 5.20 | 9.90 | 0.272 |
| Comparison computation (ms) | 86.70 | 95.50 | 0.077 |
| Sampled Chromium RSS sum (MiB) | 471.67 | 478.84 | 0.014 |
| Sampled main-page JS heap peak (MiB) | 4.59 | 4.84 | 0.029 |
| Final WASM linear memory capacity (MiB) | 3.375 | 3.375 | 0.000 |

Readiness uses two rAF callbacks after a DOM mark, plus three stable camera samples
and loaded fonts for full readiness; it does not observe physical pixels. Search
occurred before comparison completed in all ten startup trials. Its automation
round trip includes driver overhead; the later warm metrics start in input/keydown
capture listeners and stop at DOM readiness. None measures presentation latency.
Engine timing can benefit from the eager WASM preload; it is not total cold download
time. Resource observations recorded 523,268 transferred bytes and 62,830 script
body bytes per navigation under these server settings, not production-CDN sizes.

Memory polling waits 50 ms between samples plus instrumentation/driver overhead.
It can miss peaks, RSS can double count shared pages, and main-page heap excludes
worker heaps. Precise-memory instrumentation was enabled for these cold runs;
do not compare those values directly with the rounded heap snapshots in the
initial interaction pilot. WASM capacity is allocated linear memory, not live usage.
Long Tasks was supported and reported zero blocking time above its 50 ms threshold;
this says nothing about smaller costs. `cold-start-trace.json.gz` and
`cold-start-traced.json.gz` retain one separate instrumented run. Missing trace
categories now report unavailable, never zero. CPU slice unions are computed per
thread and clipped at readiness, retaining all-thread WASM work separately.

Remaining engineering acceptance includes trace-based frame-cost accounting,
reproducibility in CI, production network conditions, cross-engine critical flows,
and the Stage 1 manual accessibility/device checks. The original unresolved-defect
record remains in force. Roll back the instrumentation independently; preserve the
raw data and never regenerate a baseline to silently accept a regression.

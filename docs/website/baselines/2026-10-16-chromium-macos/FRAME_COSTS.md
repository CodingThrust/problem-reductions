# Main-thread frame-cost pilot

`frame-cost-pilot.json.gz` retains ten independent traced runs per scenario,
including ten idle controls. `frame-cost-traces.tar.gz` contains all fifty Chromium
traces, grouped by run. Source/artifact identity matches the earlier captures.
`engineering_stage0_frame_costs` in `../../performance-results.json` records
source/tool/archive hashes, per-run summaries, sample counts and CV.

The parser partitions work at Chromium's recorded animation-frame starts, using
complete windows only. An active window contains the requested input dispatch or
style/layout/paint work. It takes the union of named script/render slices on the
single identified renderer main thread, clipping overlaps at window boundaries.
Nested layout inside a JS callback is counted once; idle scheduling time is not
work. The first requested input must be inside a complete window. Missing/unknown
frame markers, missing inputs, or truncated update windows report unsupported and
unavailable costs, never a passing zero. Tests cover a known 8 ms overlapping
interval example, first-input truncation and an actual recorded Chromium trace.

| Scenario | Median of run p95 costs (ms) | p95 of run p95 costs (ms) | CV |
| --- | ---: | ---: | ---: |
| Wheel zoom | 2.758 | 2.934 | 0.053 |
| Background pan | 1.606 | 1.991 | 0.105 |
| Node drag | 2.910 | 3.453 | 0.109 |
| Keyboard activation | 2.290 | 4.723 | 0.323 |

These are ten run-level observations per row, not pooled frame observations.
Nearest-rank p95 is the maximum with ten runs. Frame samples stay within their
respective runs. First-input slice-cost medians/p95s were wheel 2.861/3.005 ms,
pan 1.910/2.251 ms, drag 2.488/3.544 ms and keyboard 2.290/4.723 ms. The idle
control contained no active update windows, so its active cost is not applicable.

This pilot used Chromium 145.0.7632.6/Playwright 1.58.0 on the previously recorded
Mac, native CPU speed, fresh browser processes/contexts, normal motion and
1440×1000/DPR 1. No builds or browser tests ran concurrently. All effects and trace
coverage checks passed; all observers supported LoAF and no LoAF was recorded.
First-input rAF interval p95s were wheel 36.6 ms, pan 35.8 ms, drag 36.2 ms and
keyboard 29.6 ms. They remain scheduling measurements, separate from slice costs.

The elapsed slice union includes measurement/automation overhead and excludes GPU,
raster-thread and unclassified work. It is not total CPU usage, presentation latency
or proof of the complete 8 ms rendering target. It provides a baseline for named
main-thread costs below the LoAF threshold. The earlier long frames remain an
unresolved sporadic defect; this batch did not reproduce or explain them.

Timing gates remain informational pending uncertainty, practical-margin calibration
and at least ten comparable CI runs. Absolute targets are evaluated separately;
these samples do not waive unmeasured portions of a target or justify a renderer
change. A prior pilot accidentally overlapped verification during its final seconds.
It is retained as `frame-cost-pilot-invalid-concurrency.json.gz` and excluded from
all timing conclusions above. Do not pool it with the isolated pilot.

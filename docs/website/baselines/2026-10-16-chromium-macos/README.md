# Engineering reference capture and interaction pilot

These references preserve the current appearance; they do not propose a design.
They describe the prepared SVG working tree at `b6c019fd`, including uncommitted
runtime preparation. `source-verification.json.gz` records that exact source
state, lockfiles, tools and all production artifact hashes. This is an initial
reference, not evidence of a clean-checkout build or completed modernization.

Chromium 145.0.7632.6, Playwright 1.58.0, macOS 27.0.1 arm64, Apple M6,
32 GiB RAM. No CPU/network throttling; loopback HTTP without automatic text
compression. Each interaction scenario used a new browser process and context.
No build or other benchmark ran concurrently. The headless browser's physical
display refresh rate was not measured. Device pixel ratio was 1.

## Appearance references

`desktop/` (1440×1000) and `mobile/` (390×844) each contain home, Atlas, default
graph route, node popover, loaded details drawer, empty Atlas and unknown-record
states. Capture awaits fonts, uses reduced motion and checks each state before
saving. `screenshots.json.gz` records image hashes, browser settings and capture
tool/source hashes. Mobile screenshots are viewport emulation, not device testing.
Pixel comparisons remain informational until repeatability is established.
Two independent local captures produced identical hashes for all fourteen images;
repeatability in the eventual pinned CI environment is still unverified.

## Interaction pilot

`interactions.json.gz` retains all raw frame/input observations, observer status,
camera/node effect assertions and page/worker marks. There are ten independent
runs per scenario, rather than one pooled set of correlated frames. Each pan/drag
is 120×60 px in 30 moves; wheel zoom uses twenty wheel inputs. The idle control is
one second. Scenario durations are recoverable from the raw timestamps.

| Scenario | Median of run p95 rAF intervals (ms) | Run p95 (ms) | CV |
| --- | ---: | ---: | ---: |
| Idle | 20.70 | 23.10 | 0.098 |
| Wheel zoom | 22.80 | 25.60 | 0.113 |
| Background pan | 23.75 | 35.80 | 0.163 |
| Node drag | 23.65 | 24.50 | 0.052 |

CV is sample standard deviation divided by the absolute sample mean. Quantiles
use nearest rank: with ten runs, p95 is the maximum. These values neither establish
a stable tail nor justify a regression margin. All observers supported LoAF.
Run 5 wheel zoom recorded a 70.7 ms LoAF (0 ms attributed script, 44.6 ms render);
run 10 pan recorded 63.1 ms (0 ms script, 0.6 ms render). Neither contained the
first recorded wheel/move input. Attribution outside those reported categories
is unresolved. Do not infer that a first-interaction defect is fixed.

First-input rAF interval medians/p95s were zoom 16.15/26.40 ms, pan 32.90/48.70 ms,
and drag 22.15/40.00 ms. Pan/drag currently start this measurement at pointermove,
so pointerdown work needs separate measurement. rAF measures scheduling cadence,
not active script/render cost or event-to-presentation latency. LoAF only describes
frames above 50 ms. The 8 ms rendering target remains unmeasured.

The ten idle-control navigations also recorded these cold DOM/worker timings:

| Metric | Median (ms) | p95 (ms) | CV |
| --- | ---: | ---: | ---: |
| Controls DOM ready, from navigation | 46.90 | 88.70 | 0.271 |
| Routes DOM ready, from navigation | 65.80 | 107.30 | 0.224 |
| Comparison DOM ready, from navigation | 157.90 | 208.10 | 0.126 |
| Engine fetch through initialization | 4.00 | 5.80 | 0.166 |
| Path computation | 5.85 | 8.80 | 0.189 |
| Comparison computation | 89.60 | 107.50 | 0.093 |

These marks are not painted readiness, explicit user-query latency, or proof
that input stays responsive during startup. Page heap snapshots were rounded to
10,000,000 bytes by this browser; they are not reliable memory measurements and
exclude workers/process memory. The graph had 1,358 DOM elements at idle.

The exact WASM artifact is 806,666 bytes raw and 277,560 bytes in its delivered
gzip stream; scene gzip is 11,076 bytes. Hashes and remaining selected sizes are
in the `engineering_stage0_pilot` section of `../../performance-results.json`.
No absolute size budget is adopted from a single capture.

## Reproduction and outstanding acceptance

Use the commands in [ENGINEERING.md](../../ENGINEERING.md). Validate the supplied
artifact against its passed manifest before capturing. The archive retains the
original manifest; a later verification run must not be described as the original
measurement. Baseline updates require explicit review and an explanation.

Stage 0 remains incomplete pending painted cold-start/startup-input, explicit
query/search and useful memory measurements, trace attribution below 50 ms,
supported-browser documentation and screenshot stability evidence. CI timing
gates also require at least ten CI runs and a measured uncertainty/regression
margin. Manual keyboard and real-device screen-reader acceptance belong to
Stage 1; none is claimed here. Rollback the capture/reporting increment separately
from runtime code and retain this evidence as historical, informational data.

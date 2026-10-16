#!/usr/bin/env python3
"""Assert real graph gestures and record frame-cadence/LoAF diagnostics.

Each scenario starts a fresh browser process without preceding gestures. rAF
intervals include browser scheduling and are not handler/render cost or proof of
an 8 ms budget. LoAF reports only frames exceeding 50 ms. Unsupported observers
are reported as unavailable, never as zero long frames. Repeated runs remain
separate; frame observations are not independent run-level samples.

    uv run --with playwright==1.58.0 python scripts/benchmark_graph_interaction.py \
        http://127.0.0.1:3001/ --repeat 10 --output /tmp/graph-interactions.json
"""

import argparse
import hashlib
import json
import math
import platform
import statistics

from playwright.sync_api import sync_playwright

PROBE = """() => {
  const probe = window.__interactionProbe = {frames: [], loaf: [], events: [],
    loaf_status: 'unsupported', loaf_error: null};
  let last = performance.now();
  const tick = () => {
    const now = performance.now();
    probe.frames.push({start: last, duration: now - last});
    last = now;
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
  for (const type of ['wheel', 'pointerdown', 'pointermove', 'pointerup'])
    document.addEventListener(type, e => {
      if (type === 'pointermove' && !e.buttons) return;
      probe.events.push({type, start: performance.now(), buttons: e.buttons});
    }, {capture: true, passive: true});
  if (PerformanceObserver.supportedEntryTypes.includes('long-animation-frame')) {
    try {
      new PerformanceObserver(list => {
        for (const e of list.getEntries()) probe.loaf.push({start: e.startTime,
          duration: e.duration, script: e.scripts.reduce((s, x) => s + x.duration, 0),
          render: e.renderStart ? e.startTime + e.duration - e.renderStart : 0});
      }).observe({type: 'long-animation-frame', buffered: false});
      probe.loaf_status = 'supported';
    } catch (error) {
      probe.loaf_status = 'error'; probe.loaf_error = String(error);
    }
  }
}"""


def quantile(values, q):
    ordered = sorted(values)
    return ordered[min(len(ordered) - 1, math.ceil(q * len(ordered)) - 1)] if ordered else None


def summarize_runs(values):
    available = [value for value in values if value is not None]
    mean = statistics.mean(available) if available else None
    return {"samples": len(available), "unavailable": len(values) - len(available),
            "median": statistics.median(available) if available else None,
            "p95": quantile(available, .95), "quantile_estimator": "nearest_rank",
            "cv": statistics.stdev(available) / abs(mean) if len(available) > 1 and mean else None}


def summarize(frames, loaf, loaf_status="supported"):
    return dict(
        frames=len(frames), sample_unit="raf_interval", quantile_estimator="nearest_rank",
        p50_ms=quantile(frames, .5), p95_ms=quantile(frames, .95),
        p99_ms=quantile(frames, .99), max_ms=max(frames, default=None),
        over_16_7=sum(f > 1000 / 60 for f in frames), over_50=sum(f > 50 for f in frames),
        loaf_status=loaf_status, loaf_count=len(loaf) if loaf_status == "supported" else None,
        loaf_p95_script_ms=quantile([e["script"] for e in loaf], .95),
        loaf_p95_render_ms=quantile([e["render"] for e in loaf], .95),
    )


def camera(page):
    return page.locator("svg.graph-scene > g").evaluate("""e => {
      const m = e.getCTM(); return {zoom: m.a, x: m.e, y: m.f};
    }""")


def target(page, drag):
    # Resolve a visible hit target now, rather than retaining a pre-pan rectangle.
    return page.locator("svg.graph-scene").evaluate("""(svg, drag) => {
      const rect = svg.getBoundingClientRect();
      const candidates = drag ? [...svg.querySelectorAll('.node-disc')].map(node => {
        const r = node.getBoundingClientRect();
        return {x: r.x + r.width/2, y: r.y + r.height/2,
          id: node.closest('[data-node]').dataset.node};
      }) : Array.from({length: 20}, (_, i) => ({
        x: 600 + (i % 5) * 130, y: 250 + Math.floor(i/5) * 140}));
      return candidates.find(p => {
        if (p.x < 420 || p.x > rect.right - 150 || p.y < rect.top + 80 || p.y > rect.bottom - 120) return false;
        const hit = document.elementFromPoint(p.x, p.y);
        return drag ? hit?.closest('[data-node]')?.dataset.node === p.id : hit === svg;
      });
    }""", drag)


def center(node):
    return node.evaluate("""e => {
      const r = e.getBoundingClientRect(); return {x: r.x+r.width/2, y: r.y+r.height/2};
    }""")


def phase(page, name):
    drag = name == "node_drag"
    hit = None if name == "idle_1s" else target(page, drag)
    assert name == "idle_1s" or hit, f"No visible hit target for {name}"
    node = page.locator('.scene-node[data-node="' + hit["id"] + '"] .node-disc') if drag else None
    before = camera(page)
    node_before = center(node) if drag else None
    page.evaluate(PROBE)
    if name == "idle_1s":
        page.wait_for_timeout(1000)
    else:
        page.mouse.move(hit["x"], hit["y"])
        if name == "wheel_zoom":
            for _ in range(20):
                page.mouse.wheel(0, -20)
            page.wait_for_function("""zoom => document.querySelector('svg.graph-scene > g').getCTM().a > zoom""", arg=before["zoom"])
        else:
            page.mouse.down()
            page.mouse.move(hit["x"] + 120, hit["y"] + 60, steps=30)
            page.mouse.up()
    page.wait_for_timeout(100)
    after = camera(page)
    if drag:
        node_after = center(node)
        assert after == before, "Node drag moved the camera"
        assert abs(node_after["x"] - node_before["x"] - 120) < 5, "Node drag did not move the intended node horizontally"
        assert abs(node_after["y"] - node_before["y"] - 60) < 5, "Node drag did not move the intended node vertically"
    elif name == "background_pan":
        assert after["zoom"] == before["zoom"], "Pan changed zoom"
        assert abs(after["x"] - before["x"] - 120) < 5, "Pan did not move the camera horizontally"
        assert abs(after["y"] - before["y"] - 60) < 5, "Pan did not move the camera vertically"
    elif name == "wheel_zoom":
        assert after["zoom"] > before["zoom"], "Wheel input did not zoom the graph"
    probe = page.evaluate("() => window.__interactionProbe")
    assert probe["frames"], "No animation frames were observed"
    if name != "idle_1s":
        event_type = "wheel" if name == "wheel_zoom" else "pointermove"
        inputs = [e for e in probe["events"] if e["type"] == event_type]
        assert inputs, f"No {event_type} input observed for {name}"
        first = inputs[0]["start"]
        probe["first_input_raf_interval_ms"] = next((f["duration"] for f in probe["frames"]
            if f["start"] <= first < f["start"] + f["duration"]), None)
        probe["first_input_loaf_ms"] = next((f["duration"] for f in probe["loaf"]
            if f["start"] <= first < f["start"] + f["duration"]), None)
    return {**summarize([f["duration"] for f in probe["frames"]], probe["loaf"], probe["loaf_status"]),
            "fresh_process": True, "verified_effect": {"before": before, "after": after,
                "node_before": node_before, "node_after": center(node) if drag else None},
            "raw_samples": probe}


def run(base, cpu):
    rows = {}
    with sync_playwright() as p:
        for name in ["idle_1s", "wheel_zoom", "background_pan", "node_drag"]:
            browser = p.chromium.launch()
            try:
                page = browser.new_page(viewport={"width": 1440, "height": 1000}, reduced_motion="no-preference")
                errors = []
                page.on("pageerror", lambda e: errors.append(str(e)))
                if cpu > 1:
                    page.context.new_cdp_session(page).send("Emulation.setCPUThrottlingRate", {"rate": cpu})
                response = page.goto(base.rstrip("/") + "/graph.html")
                page.wait_for_function("""() => {
                  const done = performance.getEntriesByName('graph:comparison-rendered').at(-1);
                  const settled = performance.getEntriesByName('graph:camera-settled').at(-1);
                  return done && settled && settled.startTime >= done.startTime;
                }""")
                page.evaluate("() => document.fonts.ready")
                rows[name] = phase(page, name)
                rows[name]["browser_version"] = browser.version
                rows[name]["html_sha256"] = hashlib.sha256(response.body()).hexdigest()
                rows[name]["page_snapshot"] = page.evaluate("""() => ({
                  device_pixel_ratio: devicePixelRatio, viewport: {width: innerWidth, height: innerHeight},
                  dom_nodes: document.getElementsByTagName('*').length,
                  page_js_heap_bytes_approx: performance.memory?.usedJSHeapSize ?? null,
                  time_origin: performance.timeOrigin,
                  marks: performance.getEntriesByType('mark').map(e => ({name: e.name, start: e.startTime}))
                })""")
                rows[name]["worker_snapshots"] = [worker.evaluate("""() => ({time_origin: performance.timeOrigin,
                  marks: performance.getEntriesByType('mark').map(e => ({name: e.name, start: e.startTime}))})""")
                  for worker in page.workers]
                assert not errors, errors
            finally:
                browser.close()
    return rows


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("base")
    parser.add_argument("--cpu", type=int, default=1)
    parser.add_argument("--repeat", type=int, default=1, help="Independent runs; keep event/frame samples separate")
    parser.add_argument("--output")
    args = parser.parse_args()
    if args.cpu < 1 or args.repeat < 1:
        parser.error("--cpu and --repeat must be positive integers")
    runs = []
    for index in range(args.repeat):
        rows = run(args.base, args.cpu)
        runs.append(rows)
        for name, row in rows.items():
            print(index + 1, name, json.dumps({k: v for k, v in row.items()
                if k not in {"raw_samples", "verified_effect", "page_snapshot", "worker_snapshots"}}), flush=True)
    if args.output:
        summary = {name: {key: summarize_runs([row[name][key] for row in runs])
                         for key in ["p95_ms", "max_ms", "loaf_count"]} for name in runs[0]}
        with open(args.output, "w") as f:
            json.dump({"cpu_throttle": args.cpu, "platform": platform.platform(),
                       "measurement_scope": "rAF cadence and >50 ms LoAF diagnostics; sub-50 ms rendering cost is unmeasured",
                       "cache_state": "fresh_browser_process_and_context_per_scenario",
                       "gating": "informational; no stable CI variance or tail estimate established",
                       "run_summary": summary, "runs": runs}, f, indent=1)

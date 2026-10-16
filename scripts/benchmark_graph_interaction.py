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
from contextlib import contextmanager, nullcontext
import gzip
import hashlib
import json
import math
import platform
from pathlib import Path
import statistics
import time

from playwright.sync_api import expect, sync_playwright

from check_website import prepare_output

PROBE = """() => {
  const probe = window.__interactionProbe = {frames: [], loaf: [], events: [],
    loaf_status: 'unsupported', loaf_error: null, event_status: 'unsupported',
    event_error: null, event_timing: [],
    coalesced_status: typeof PointerEvent.prototype.getCoalescedEvents === 'function' ? 'supported' : 'unsupported',
    coalesced_count: typeof PointerEvent.prototype.getCoalescedEvents === 'function' ? 0 : null};
  let last = performance.now();
  const tick = () => {
    const now = performance.now();
    probe.frames.push({start: last, duration: now - last});
    last = now;
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
  for (const type of ['wheel', 'pointerdown', 'pointermove', 'pointerup', 'keydown', 'keyup'])
    document.addEventListener(type, e => {
      if (type === 'pointermove' && !e.buttons) return;
      if (type === 'pointermove' && probe.coalesced_status === 'supported')
        probe.coalesced_count += e.getCoalescedEvents().length;
      probe.events.push({type, start: performance.now(), timestamp: e.timeStamp, buttons: e.buttons});
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
  if (PerformanceObserver.supportedEntryTypes.includes('event')) {
    try {
      new PerformanceObserver(list => {
        for (const e of list.getEntries()) probe.event_timing.push({name: e.name,
          start: e.startTime, duration: e.duration, processing_start: e.processingStart,
          processing_end: e.processingEnd, interaction_id: e.interactionId});
      }).observe({type: 'event', durationThreshold: 16, buffered: false});
      probe.event_status = 'supported';
    } catch (error) {
      probe.event_status = 'error'; probe.event_error = String(error);
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


def first_input(probe, name):
    event_type = "wheel" if name == "wheel_zoom" else "keydown" if name == "keyboard_activation" else "pointerdown"
    first = next((e for e in probe["events"] if e["type"] == event_type), None)
    assert first, f"No {event_type} input observed for {name}"
    def containing(entries):
        return next((f["duration"] for f in entries if f["start"] <= first["start"] < f["start"] + f["duration"]), None)
    return {"type": event_type, "start": first["start"],
            "raf_interval_ms": containing(probe["frames"]), "loaf_ms": containing(probe["loaf"])}


def trace_frame_costs(events, input_type):
    presentation = {"presentation_status": "unsupported", "first_input_frame_to_presentation_ms": None,
                    "first_input_handler_to_presentation_ms": None}
    unavailable = {"status": "unsupported", "p95_active_script_render_ms": None,
                   "first_input_script_render_ms": None, **presentation}
    main = {(e["pid"], e["tid"]) for e in events if e["name"] == "thread_name"
            and e.get("args", {}).get("name") == "CrRendererMain"}
    if len(main) != 1:
        return {**unavailable, "reason": "Expected one identified renderer main thread"}
    events = [e for e in events if (e["pid"], e["tid"]) in main]
    # Chromium's frame markers partition work; waiting between slices is not work.
    boundaries = sorted({e["ts"] for e in events if e["name"] == "AnimationFrame" and e.get("ph") == "b"
                         and "animation_frame_timing_info" in e.get("args", {})})
    if len(boundaries) < 2:
        return {**unavailable, "reason": "Missing Chromium animation-frame boundaries"}
    input_events = sorted((e for e in events if e["name"] == "EventDispatch" and e.get("ph") == "X"
                           and e.get("args", {}).get("data", {}).get("type") == input_type),
                          key=lambda e: e["ts"]) if input_type else []
    inputs = [event["ts"] for event in input_events]
    if input_type and (not inputs or inputs[0] < boundaries[0] or inputs[-1] >= boundaries[-1]):
        return {**unavailable, "reason": "Trace boundaries do not cover every requested input"}
    updates = {"UpdateLayoutTree", "Layout", "Paint"}
    names = updates | {"EventDispatch", "FireAnimationFrame", "FunctionCall", "TimerFire", "RunMicrotasks",
                       "EvaluateScript", "EvaluateModule", "PrePaint", "Layerize", "Commit"}
    slices = [e for e in events if e.get("ph") == "X" and e["name"] in names and e.get("dur", 0) > 0]
    if any(e["name"] in updates and e["ts"] + e["dur"] > boundaries[-1] for e in slices):
        return {**unavailable, "reason": "Trace ends before the final update has a complete frame window"}
    frames = []
    first_cost = None
    for start, end in zip(boundaries, boundaries[1:]):
        current = [e for e in slices if e["ts"] < end and e["ts"] + e["dur"] > start]
        active = any(start <= value < end for value in inputs) or any(e["name"] in updates for e in current)
        if not active:
            continue
        total, previous_end = 0, start
        for left, right in sorted((max(start, e["ts"]), min(end, e["ts"] + e["dur"])) for e in current):
            total += max(0, right - max(left, previous_end))
            previous_end = max(previous_end, right)
        cost = total / 1000
        frames.append(cost)
        if inputs and start <= inputs[0] < end:
            first_cost = cost
    if input_events:
        timings = sorted((e for e in events if e["name"] == "AnimationFrame" and e.get("ph") == "b"
                          and "animation_frame_timing_info" in e.get("args", {})), key=lambda e: e["ts"])
        index = max(i for i, frame in enumerate(timings) if frame["ts"] <= inputs[0])
        first = input_events[0]
        # An input between completed frames belongs to the next rendered frame.
        for frame in timings[index:index + 2]:
            if not frame.get("id2") or not frame.get("args", {}).get("id"):
                break
            end = min((e["ts"] for e in events if e["name"] == "AnimationFrame" and e.get("ph") == "e"
                       and e.get("id2") == frame["id2"] and e["ts"] >= frame["ts"]), default=None)
            if end is None:
                break
            if end < first["ts"] + first["dur"]:
                continue
            # Matching trace IDs matter: a nearby presentation can belong to a different frame.
            shown = next((e for e in events if e["name"] == "AnimationFrame::Presentation"
                          and e.get("args", {}).get("id") == frame["args"]["id"]), None)
            if shown is not None and shown["ts"] >= end:
                presentation = {"presentation_status": "supported",
                    "first_input_frame_to_presentation_ms": (shown["ts"] - frame["ts"]) / 1000,
                    "first_input_handler_to_presentation_ms": (shown["ts"] - first["ts"]) / 1000}
            break
    else:
        presentation["presentation_status"] = "not_applicable"
    return {"status": "supported", "sample_unit": "active_main_thread_frame_window",
            "quantile_estimator": "nearest_rank", "complete_windows": len(boundaries) - 1,
            "active_script_render_ms": frames, "first_input_script_render_ms": first_cost, **presentation,
            "p95_active_script_render_ms": quantile(frames, .95),
            "scope": "union of named main-thread script/style/layout/prepaint/paint/compositing slices; includes instrumentation, excludes GPU/raster threads and unclassified work"}


@contextmanager
def trace_interaction(page, path):
    session = page.context.new_cdp_session(page)
    events, completed = [], []
    session.on("Tracing.dataCollected", lambda data: events.extend(data["value"]))
    session.on("Tracing.tracingComplete", lambda data: completed.append(True))
    session.send("Tracing.start", {"categories": "devtools.timeline,blink.user_timing,disabled-by-default-devtools.timeline",
                                   "transferMode": "ReportEvents"})
    try:
        yield
    finally:
        try:
            session.send("Tracing.end")
            deadline = time.monotonic() + 5
            while not completed and time.monotonic() < deadline:
                page.wait_for_timeout(20)
            assert completed, "Trace did not complete; do not use partial measurements"
            path.parent.mkdir(parents=True, exist_ok=True)
            with path.open("xb") as stream:
                stream.write(gzip.compress(json.dumps({"traceEvents": events}).encode(), mtime=0))
        finally:
            session.detach()


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
    keyboard = name == "keyboard_activation"
    hit = None if name == "idle_1s" else target(page, drag or keyboard)
    assert name == "idle_1s" or hit, f"No visible hit target for {name}"
    node = page.locator('.scene-node[data-node="' + hit["id"] + '"] .node-disc') if drag else None
    if keyboard:
        page.locator('.scene-node[data-node="' + hit["id"] + '"]').focus()
    before = camera(page)
    node_before = center(node) if drag else None
    page.evaluate(PROBE)
    if name == "idle_1s":
        page.wait_for_timeout(1000)
    elif keyboard:
        page.keyboard.press("Enter")
        expect(page.locator("#pop")).to_be_visible()
        label = page.locator('.scene-node[data-node="' + hit["id"] + '"]').get_attribute("aria-label").split(",")[0]
        expect(page.locator("#pop h2")).to_have_text(label)
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
        probe["first_input"] = first_input(probe, name)
    return {**summarize([f["duration"] for f in probe["frames"]], probe["loaf"], probe["loaf_status"]),
            "fresh_process": True, "verified_effect": {"before": before, "after": after,
                "node_before": node_before, "node_after": center(node) if drag else None},
            "raw_samples": probe}


def run(base, cpu, trace_dir=None):
    rows = {}
    with sync_playwright() as p:
        for name in ["idle_1s", "wheel_zoom", "background_pan", "node_drag", "keyboard_activation"]:
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
                trace = trace_dir / f"{name}.json.gz" if trace_dir is not None else None
                with trace_interaction(page, trace) if trace else nullcontext():
                    rows[name] = phase(page, name)
                rows[name]["trace"] = str(trace) if trace else None
                if trace:
                    events = json.loads(gzip.decompress(trace.read_bytes()))["traceEvents"]
                    rows[name]["trace_frame_costs"] = trace_frame_costs(events,
                        rows[name]["raw_samples"].get("first_input", {}).get("type"))
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
    parser.add_argument("--trace-dir", type=Path, help="New directory for Chromium timeline traces; tracing adds overhead")
    args = parser.parse_args()
    if args.cpu < 1 or args.repeat < 1:
        parser.error("--cpu and --repeat must be positive integers")
    if args.trace_dir is not None:
        try:
            args.trace_dir = prepare_output(args.trace_dir)
        except OSError as error:
            parser.error(str(error))
    runs = []
    for index in range(args.repeat):
        rows = run(args.base, args.cpu, args.trace_dir / str(index + 1) if args.trace_dir else None)
        runs.append(rows)
        for name, row in rows.items():
            print(index + 1, name, json.dumps({k: v for k, v in row.items()
                if k not in {"raw_samples", "verified_effect", "page_snapshot", "worker_snapshots", "trace_frame_costs"}}), flush=True)
    if args.output:
        summary = {name: {key: summarize_runs([row[name][key] for row in runs])
                         for key in ["p95_ms", "max_ms", "loaf_count"]} for name in runs[0]}
        trace_summary = {name: {key: summarize_runs([row[name].get("trace_frame_costs", {}).get(key) for row in runs])
                               for key in ["p95_active_script_render_ms", "first_input_script_render_ms",
                                           "first_input_frame_to_presentation_ms", "first_input_handler_to_presentation_ms"]}
                         for name in runs[0]}
        with open(args.output, "w") as f:
            json.dump({"cpu_throttle": args.cpu, "platform": platform.platform(),
                       "measurement_scope": "rAF cadence and >50 ms LoAF diagnostics; optional Chromium traces account for named main-thread script/render slices separately",
                       "cache_state": "fresh_browser_process_and_context_per_scenario",
                       "trace_instrumentation": "enabled; adds overhead" if args.trace_dir else "disabled",
                       "benchmark_script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                       "presentation_scope": "first requested event's completed frame correlated by trace ID with Chromium presentation feedback; handler-to-feedback excludes prior input queueing; headless feedback is not physical display latency",
                       "event_timing_scope": "discrete events at durationThreshold=16 ms, quantized durations; continuous wheel/move not covered; absent entries are not zero latency",
                       "gating": "informational; no stable CI variance or tail estimate established",
                       "run_summary": summary, "trace_run_summary": trace_summary, "runs": runs}, f, indent=1)

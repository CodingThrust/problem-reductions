#!/usr/bin/env python3
"""Cold Graph navigation: real HTTP, a new Chromium process/context per sample.

uv run --no-project --with playwright==1.58.0 python scripts/benchmark_graph_startup.py book --output /tmp/startup --repeat 20
"""

import argparse
from functools import partial
import gzip
import hashlib
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import platform
import subprocess
from threading import Lock, Thread
import time

from playwright.sync_api import expect, sync_playwright

from benchmark_graph_interaction import summarize_runs
from check_website import prepare_output, verified_artifact


class PreviewServer(ThreadingHTTPServer):
    request_queue_size = 128


class Handler(SimpleHTTPRequestHandler):
    def log_message(self, *args):
        pass

    def do_GET(self):
        path = Path(self.translate_path(self.path))
        if not path.is_file():
            return super().do_GET()
        body = path.read_bytes()
        compressed = (self.server.compress and path.suffix in {'.js', '.css', '.json', '.svg', '.html'}
                      and 'gzip' in self.headers.get('Accept-Encoding', ''))
        if compressed:
            body = gzip.compress(body, mtime=0)
        if self.server.slow:
            time.sleep(.150)
        self.send_response(200)
        self.send_header('Content-Type', self.guess_type(str(path)))
        self.send_header('Content-Length', str(len(body)))
        self.send_header('Cache-Control', 'public, max-age=31536000, immutable')
        if compressed:
            self.send_header('Content-Encoding', 'gzip')
        self.end_headers()
        try:
            for offset in range(0, len(body), 4096):
                chunk = body[offset:offset + 4096]
                if self.server.slow:
                    # One aggregate link, including worker fetches and preloads.
                    with self.server.link:
                        deadline = max(time.monotonic(), self.server.next_byte) + len(chunk) / 200000
                        self.server.next_byte = deadline
                    time.sleep(max(0, deadline - time.monotonic()))
                self.wfile.write(chunk)
        except (BrokenPipeError, ConnectionResetError):
            pass


OBSERVER = """(() => {
 window.startup = {longTasks: [], paints: {}, instrumentation: {longtask: 'unsupported'}};
 if (PerformanceObserver.supportedEntryTypes.includes('longtask')) {
   try {
     new PerformanceObserver(list => startup.longTasks.push(...list.getEntries().map(e =>
       ({start:e.startTime, duration:e.duration})))).observe({type:'longtask', buffered:true});
     startup.instrumentation.longtask = 'supported';
   } catch (error) {
     startup.instrumentation.longtask = 'error'; startup.instrumentation.longtask_error = String(error);
   }
 }
 new PerformanceObserver(list => {
   for (const e of list.getEntries()) if (['graph:interactive','graph:routes-rendered','graph:comparison-rendered'].includes(e.name)) {
     requestAnimationFrame(() => requestAnimationFrame(() => { startup.paints[e.name] ||= performance.now(); }));
     if (e.name === 'graph:comparison-rendered') (async () => {
       await document.fonts.ready;
       // Wait for the real viewport to stop changing, through its final painted frame.
       let previous, stable = 0;
       while (stable < 3) {
         await new Promise(requestAnimationFrame);
         const svg = document.querySelector('svg.graph-scene > g');
         const cy = document.querySelector('#cy')._cyreg?.cy;
         const position = svg?.getAttribute('transform') || JSON.stringify([cy?.zoom(), cy?.pan()]);
         stable = position === previous ? stable + 1 : 0; previous = position;
       }
       startup.fullReady ||= performance.now();
     })();
   }
 }).observe({type:'mark', buffered:true});
})();"""


def sample(playwright, url, cpu, trace_path):
    browser = playwright.chromium.launch(args=['--no-proxy-server', '--proxy-bypass-list=*', '--enable-precise-memory-info'])
    try:
        return sample_browser(browser, url, cpu, trace_path)
    finally:
        browser.close()


def sample_browser(browser, url, cpu, trace_path):
    context = browser.new_context(viewport={'width': 1440, 'height': 1000},
                                  service_workers='block')
    page = context.new_page()
    page.set_default_timeout(3000)
    page.add_init_script(OBSERVER)
    errors = []
    request_failures = []
    page.on('pageerror', lambda error: errors.append(str(error)))
    page.on('requestfailed', lambda request: request_failures.append({'url': request.url, 'error': request.failure}))
    session = context.new_cdp_session(page)
    browser_session = browser.new_browser_cdp_session()
    session.send('Network.enable')
    session.send('Network.clearBrowserCache')
    session.send('Performance.enable')
    session.send('Emulation.setCPUThrottlingRate', {'rate': cpu})
    trace_events = []
    if trace_path:
        browser_session.on('Tracing.dataCollected', lambda event: trace_events.extend(event['value']))
        browser_session.send('Tracing.start', {'categories':
            'devtools.timeline,blink.user_timing,v8,v8.wasm,disabled-by-default-v8.compile',
            'transferMode': 'ReportEvents'})
    # Sample total Chromium process RSS. Shared pages can be counted more than once.
    peak_rss = 0
    peak_heap = 0
    response = page.goto(url, wait_until='commit')
    interaction = None
    deadline = time.monotonic() + 60
    while time.monotonic() < deadline:
        state = page.evaluate('''() => ({marks:performance.getEntriesByType('mark').map(e=>e.name),
            ready:startup.fullReady, heap:performance.memory?.usedJSHeapSize})''')
        peak_heap = max(peak_heap, state.get('heap') or 0)
        pids = [str(p['id']) for p in browser_session.send('SystemInfo.getProcessInfo')['processInfo']]
        rss = subprocess.run(['ps', '-o', 'rss=', '-p', ','.join(pids)], capture_output=True, text=True, check=True)
        peak_rss = max(peak_rss, sum(int(n) for n in rss.stdout.split()) * 1024)
        if interaction is None and 'graph:interactive' in state['marks']:
            began = page.evaluate('performance.now()')
            comparison_done_at_input = 'graph:comparison-rendered' in state['marks']
            page.locator('#source').fill('MIS')
            suggestion = page.locator('#suggest button').filter(has_text='Maximum Independent Set')
            suggestion.wait_for()
            suggestion_label = suggestion.locator('span').first.inner_text()
            assert suggestion_label == 'Maximum Independent Set', suggestion_label
            interaction = page.evaluate('performance.now()')
            page.locator('#source').press('Escape')
            input_latency = interaction - began
        if state['ready'] or errors or request_failures:
            break
        page.wait_for_timeout(50)
    if not state.get('ready'):
        failure = page.evaluate('''() => ({...startup, fonts:document.fonts.status,
            text:document.querySelector('#results')?.innerText,
            marks:performance.getEntriesByType('mark').map(e=>e.toJSON()),
            resources:performance.getEntriesByType('resource').map(e=>e.toJSON())})''')
        failure['errors'] = errors
        failure['request_failures'] = request_failures
        failure['workers'] = [worker.evaluate('''() => ({marks:performance.getEntriesByType('mark').map(e=>e.toJSON()),
            resources:performance.getEntriesByType('resource').map(e=>e.toJSON())})''') for worker in page.workers]
        if trace_path:
            trace_path.with_suffix('.failed.json').write_text(json.dumps(failure, indent=2))
            page.screenshot(path=str(trace_path.with_suffix('.failed.png')))
        raise AssertionError(f'Graph readiness timed out: {json.dumps(failure)}')
    metrics = {m['name']: m['value'] for m in session.send('Performance.getMetrics')['metrics']}
    assert not errors, errors
    assert not request_failures, request_failures
    assert page.locator('.path').count() > 0
    assert page.locator('.size.pending').count() == 0
    assert 'best route' in page.locator('#results').inner_text()
    row = page.evaluate('''() => ({...startup, timeOrigin:performance.timeOrigin,
       marks:performance.getEntriesByType('mark').map(e=>({name:e.name,start:e.startTime})),
       resources:performance.getEntriesByType('resource').map(e=>e.toJSON()),
       navigation:performance.getEntriesByType('navigation')[0].toJSON(),
       paint:performance.getEntriesByType('paint').map(e=>e.toJSON())})''')
    row['workers'] = [worker.evaluate('''() => ({timeOrigin:performance.timeOrigin,
       marks:performance.getEntriesByType('mark').map(e=>({name:e.name,start:e.startTime})),
       resources:performance.getEntriesByType('resource').map(e=>e.toJSON())})''') for worker in page.workers]
    row['wasm_memory_bytes'] = page.workers[0].evaluate('''async () => {
        const api = await import(new URL('./path-selection.js', location.href));
        return (await api.default()).memory.buffer.byteLength;
    }''')
    row.update(interaction_ms=interaction, input_latency_ms=input_latency,
               interaction_started_ms=began, comparison_done_at_input=comparison_done_at_input,
               html_sha256=hashlib.sha256(response.body()).hexdigest(),
               verification={'routes': page.locator('.path').count(), 'search_suggestion': suggestion_label},
               peak_chromium_rss_bytes=peak_rss, peak_page_js_heap_bytes=peak_heap)
    row['main_thread_ms'] = {name: metrics[name] * 1000 for name in
                            ['ScriptDuration', 'TaskDuration', 'LayoutDuration', 'RecalcStyleDuration']}
    if trace_path:
        completed = []
        browser_session.on('Tracing.tracingComplete', lambda event: completed.append(True))
        browser_session.send('Tracing.end')
        while not completed:
            page.wait_for_timeout(20)
        with gzip.open(trace_path, 'wt') as stream:
            json.dump({'traceEvents': trace_events}, stream)
        comparison = next(e for e in trace_events if e['name'] == 'graph:comparison-rendered')
        mark = next(m for m in row['marks'] if m['name'] == 'graph:comparison-rendered')
        cutoff = comparison['ts'] + (row['fullReady'] - mark['start']) * 1000
        row['trace_cpu_ms'] = trace_cpu(trace_events, cutoff)
    # A second, uncached route query uses the warm engine; keep it outside cold-start accounting.
    page.evaluate('''() => {
      window.queryTiming = {};
      const input = document.querySelector('#source');
      input.addEventListener('input', () => { queryTiming.searchStart = performance.now(); }, {capture: true});
      input.addEventListener('keydown', e => { if (e.key === 'Enter') queryTiming.queryStart = performance.now(); }, {capture: true});
      new MutationObserver(() => {
        const box = document.querySelector('#suggest');
        if (queryTiming.searchStart && !queryTiming.searchEnd && !box.hidden && box.textContent.includes('Maximum Independent Set'))
          queryTiming.searchEnd = performance.now();
      }).observe(document.querySelector('#suggest'), {childList: true, subtree: true});
    }''')
    page.locator('#source').fill('MIS')
    page.wait_for_function('() => queryTiming.searchEnd > 0')
    comparisons = page.evaluate("performance.getEntriesByName('graph:comparison-rendered').length")
    page.locator('#source').press('Enter')
    page.wait_for_function("n => performance.getEntriesByName('graph:comparison-rendered').length > n", arg=comparisons)
    expect(page.locator('#source')).to_have_value('Maximum Independent Set')
    route = page.locator('.path .chain').first.get_attribute('title')
    assert route.startswith('Maximum Independent Set → ') and route.endswith('QUBO'), route
    row['warm_query'] = page.evaluate('''() => ({
      search_dom_ms: queryTiming.searchEnd - queryTiming.searchStart,
      route_dom_ms: performance.getEntriesByName('graph:comparison-rendered').at(-1).startTime - queryTiming.queryStart
    })''')
    row['verification']['warm_query_route'] = route
    assert not errors, errors
    assert not request_failures, request_failures
    return row


def trace_cpu(events, cutoff=float('inf')):
    main = {(e['pid'], e['tid']) for e in events if e['name'] == 'thread_name'
            and e.get('args', {}).get('name') == 'CrRendererMain'}
    groups = {
        'js_parse_main': {'V8.ParseProgram', 'V8.ParseFunction', 'V8.PreParse'},
        'js_compile_main': {'V8.CompileCode'},
        'js_evaluate_scripts_main': {'EvaluateScript'},
        'js_execute_main': {'EvaluateScript', 'EvaluateModule', 'FunctionCall', 'RunMicrotasks'},
        'wasm_compile_all_threads': {'wasm.ExecuteCompilationUnits', 'wasm.CompileLazy'},
        'wasm_instantiate': {'wasm.SyncInstantiate'},
    }
    result = {}
    for name, names in groups.items():
        threads = {}
        for event in events:
            thread = (event['pid'], event['tid'])
            if (event['name'] in names and event['ph'] == 'X' and event['ts'] < cutoff
                    and (not name.endswith('_main') or thread in main)):
                threads.setdefault(thread, []).append((event['ts'], min(cutoff, event['ts'] + event.get('dur', 0))))
        total = 0
        for ranges in threads.values():
            end = 0
            for start, finish in sorted(ranges):
                total += max(0, finish - max(start, end))
                end = max(end, finish)
        # No matching slice cannot establish zero work in a browser-dependent trace category.
        result[name] = total / 1000 if threads else None
    return result


def summarize(rows):
    values = []
    for row in rows:
        marks = {m['name']: m['start'] for m in row['marks']}
        worker = row['workers'][0]
        w = {m['name']: m['start'] for m in worker['marks']}
        resources = row['resources'] + worker['resources']
        ready = row['fullReady']
        wasm = next(r for r in resources if 'path-selection_bg.wasm' in r['name'] and r['transferSize'])
        scripts = [r for r in resources if r['name'].split('?')[0].endswith('.js')]
        offset = worker['timeOrigin'] - row['timeOrigin']
        values.append({
            'meaningful_graph_ms': row['paints']['graph:interactive'],
            'usable_routes_ms': row['paints']['graph:routes-rendered'],
            'verified_interaction_ms': row['interaction_ms'],
            'full_ready_ms': ready,
            'input_latency_ms': row['input_latency_ms'],
            'graph_construction_ms': marks['graph:constructed'] - marks.get('graph:scene-parsed', marks['graph:script']),
            'graph_controls_ms': marks['graph:interactive'] - marks['graph:constructed'],
            'js_downloaded_bytes': sum(r['encodedBodySize'] for r in scripts if r['transferSize']),
            'js_decoded_bytes': sum(r['decodedBodySize'] for r in scripts if r['transferSize']),
            'wasm_download_ms': wasm['duration'],
            'engine_fetch_to_initialized_ms': w['engine:initialized'] - w['engine:fetch'],
            'path_search_ms': w['engine:paths-end'] - w['engine:paths-start'],
            'comparison_ms': w['engine:comparison-end'] - w['engine:comparison-start'],
            'comparison_delivery_and_render_ms': marks['graph:comparison-rendered'] - w['engine:comparison-end'] - offset,
            'scene_json_parse_ms': (marks['graph:scene-json-parsed'] - marks['graph:scene-decoded']
                                  if 'graph:scene-json-parsed' in marks else None),
            'worker_metadata_json_parse_ms': (w['engine:metadata-parsed'] - w['engine:metadata-decoded']
                                              if 'engine:metadata-decoded' in w else None),
            'worker_metadata_validation_ms': (w['engine:metadata-validated'] - w['engine:metadata-parsed']
                                               if 'engine:metadata-validated' in w and 'engine:metadata-parsed' in w else None),
            'engine_ready_ms': offset + w['engine:initialized'],
            'total_blocking_ms': sum(max(0, min(t['duration'], ready-t['start'])-50)
                                     for t in row['longTasks'] if t['start'] < ready)
                                 if row['instrumentation']['longtask'] == 'supported' else None,
            'transferred_bytes': sum(r['transferSize'] for r in resources) + row['navigation']['transferSize'],
            'peak_chromium_rss_mib': row['peak_chromium_rss_bytes'] / 1048576,
            'peak_page_js_heap_mib': row['peak_page_js_heap_bytes'] / 1048576,
            'wasm_memory_mib': row['wasm_memory_bytes'] / 1048576,
            'warm_search_dom_ms': row['warm_query']['search_dom_ms'],
            'warm_route_query_dom_ms': row['warm_query']['route_dom_ms'],
            **row.get('trace_cpu_ms', {}),
            **row['main_thread_ms'],
        })
    return {key: summarize_runs([r[key] for r in values]) for key in values[0]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('book', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--repeat', type=int, default=20)
    parser.add_argument('--profile', choices=['desktop', 'slow'], default='slow')
    parser.add_argument('--gzip', action='store_true', help='Use normal HTTP gzip for text assets')
    parser.add_argument('--trace', action='store_true', help='Record Chromium timeline traces; adds overhead')
    args = parser.parse_args()
    if args.repeat < 1:
        parser.error('--repeat must be positive')
    try:
        verification = verified_artifact(args.book)
        args.output = prepare_output(args.output)
    except (OSError, ValueError) as error:
        parser.error(str(error))
    server = PreviewServer(('127.0.0.1', 0), partial(Handler, directory=str(args.book.resolve())))
    server.slow = args.profile == 'slow'
    server.compress = args.gzip
    server.link = Lock()
    server.next_byte = 0
    thread = Thread(target=server.serve_forever, daemon=True)
    thread.start()
    rows = []
    try:
        with sync_playwright() as playwright:
            for index in range(args.repeat):
                row = sample(playwright, f'http://127.0.0.1:{server.server_port}/graph.html',
                             4 if server.slow else 1, args.output / f'trace-{index+1:02}.json.gz' if args.trace else None)
                rows.append(row)
                # Preserve partial evidence if a later run fails.
                (args.output / 'samples.json').write_text(json.dumps(rows, indent=2) + '\n')
                print(f"{index + 1}/{args.repeat}: ready {row['fullReady']:.0f} ms", flush=True)
            environment = {'platform': platform.platform(), 'playwright_chromium_path': playwright.chromium.executable_path,
                           'launch_mode': 'Playwright default headless shell',
                           'profile': args.profile, 'http_gzip': args.gzip, 'repeat': args.repeat,
                           'cpu_throttle': 4 if server.slow else 1,
                           'viewport': {'width': 1440, 'height': 1000}, 'device_pixel_ratio': 1,
                           'trace_instrumentation': 'enabled; adds overhead' if args.trace else 'disabled',
                           'benchmark_script_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                           'browser_version': browser_version(playwright),
                           'source_html_sha256': hashlib.sha256((args.book / 'graph.html').read_bytes()).hexdigest()}
        verified_artifact(args.book)
        assert all(row['html_sha256'] == verification['artifacts']['graph.html'] for row in rows)
        environment['source_verification_sha256'] = hashlib.sha256((args.book / 'verification.json').read_bytes()).hexdigest()
        results = {'environment': environment, 'summary': summarize(rows), 'samples': rows,
                   'gating': 'informational pilot; uncertainty and CI reproducibility not established',
                   'measurement_scope': {
                       'paint_proxy': 'two rAF callbacks after DOM readiness; fullReady also requires three stable camera samples and loaded fonts, not physical pixels',
                       'startup_input': 'Playwright fill-to-suggestion round trip; includes automation overhead; comparison_done_at_input records whether it overlapped startup',
                       'warm_query': 'input/keydown handler observation to suggestion/comparison DOM readiness; excludes presentation and OS input delay',
                       'memory': '50 ms polling: sampled peak Chromium RSS sum (may double count shared pages), sampled main-page JS heap, final WASM linear capacity; excludes unsampled peaks',
                       'metadata_validation': 'worker schema and cross-field validation interval after JSON parsing; missing marks are unavailable, not zero',
                       'trace': 'union of matching slices per thread up to readiness; missing categories unavailable; not per-frame render cost'}}
        (args.output / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
        print(json.dumps(results['summary'], indent=2))
    finally:
        server.shutdown()
        thread.join()
        server.server_close()


def browser_version(playwright):
    browser = playwright.chromium.launch()
    version = browser.version
    browser.close()
    return version


if __name__ == '__main__':
    main()

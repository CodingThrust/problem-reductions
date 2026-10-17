import type { Query } from '../generated/wasm-path-request.validator.mjs';
import type { ComparisonQuery } from '../generated/wasm-comparison-request.validator.mjs';
import init, { find_paths, summarize_paths } from "./path-selection.js";
import type { WasmPathResult } from '../generated/wasm-paths.validator.mjs';
import type { WasmComparisonResult } from '../generated/wasm-comparison.validator.mjs';
import type { PathGraphData } from '../generated/path-graph.validator.mjs';
import { validatePathRequest } from '../lib/path-worker-data.mts';

performance.mark("engine:script");
const graph = fetch(new URL('./path-graph.json.gz', import.meta.url)).then(async (response) => {
  if (!response.ok) throw new Error(`Cannot load path metadata (${response.status})`);
  const text = await new Response(response.body!.pipeThrough(new DecompressionStream('gzip'))).text();
  performance.mark("engine:metadata-decoded");
  const result = JSON.parse(text) as PathGraphData;
  performance.mark("engine:metadata-parsed");
  return result;
});
let engine: Promise<void> | undefined;
function loadEngine() {
  return engine ||= (async () => {
    performance.mark("engine:fetch");
    const response = await fetch(new URL('./path-selection_bg.wasm.gz', import.meta.url));
    performance.mark("engine:headers");
    if (!response.ok) throw new Error(`Cannot load path engine (${response.status})`);
    const wasm = response.body!.pipeThrough(new DecompressionStream('gzip'));
    // shortcut: Buffering avoids a Firefox streaming stall; revisit if module growth makes this buffer material.
    const bytes = await new Response(wasm).arrayBuffer();
    performance.mark("engine:wasm-decoded");
    await init({module_or_path: bytes});
    performance.mark("engine:initialized");
  })();
}
const ready = Promise.all([loadEngine(), graph]);
ready.catch(() => {});

self.onmessage = async ({data}: MessageEvent<unknown>) => {
  performance.mark("engine:request-received");
  let phase = "paths";
  try {
    const [, graph] = await ready;
    const started = performance.now();
    const request = validatePathRequest(data);
    performance.measure('engine:request-validation', {start: started});
    performance.mark("engine:paths-start");
    const result: WasmPathResult = request.kind === 'compare' ? request.cached : JSON.parse(find_paths(JSON.stringify({
      graph, source: request.source, target: request.target, limit: request.limit,
    } satisfies Query)));
    performance.mark("engine:paths-end");
    if (request.kind === 'search') self.postMessage({phase, result});
    phase = "comparison";
    if (result.paths.length) {
      performance.mark("engine:comparison-start");
      const comparison: WasmComparisonResult = JSON.parse(summarize_paths(JSON.stringify({graph, paths: result.paths} satisfies ComparisonQuery)));
      performance.mark("engine:comparison-end");
      self.postMessage({phase, result: comparison});
    }
  } catch (error) {
    self.postMessage({phase, error: error instanceof Error ? error.message : String(error)});
  }
};

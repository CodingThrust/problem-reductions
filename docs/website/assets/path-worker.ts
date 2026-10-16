import init, { find_paths, summarize_paths } from "./path-selection.js";
import { parseWasmPathResult } from './wasm-paths-data.mts';
import { parseWasmComparison, validateComparisonParameters } from './wasm-comparison-data.mts';
import { validatePathGraphData } from './path-graph-data.mts';
import { validatePathRequest, validatePathRoutes } from './path-worker-data.mts';

performance.mark("engine:script");
const graph = fetch(new URL('./path-graph.json.gz', import.meta.url)).then(async (response) => {
  if (!response.ok) throw new Error(`Cannot load path metadata (${response.status})`);
  const text = await new Response(response.body!.pipeThrough(new DecompressionStream('gzip'))).text();
  performance.mark("engine:metadata-decoded");
  const value: unknown = JSON.parse(text);
  performance.mark("engine:metadata-parsed");
  const result = validatePathGraphData(value);
  performance.mark("engine:metadata-validated");
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
    const request = validatePathRequest(data, graph);
    performance.measure('engine:request-validation', {start: started});
    performance.mark("engine:paths-start");
    const result = request.kind === 'compare' ? request.cached : parseWasmPathResult(find_paths(JSON.stringify({
      graph, source: request.source, target: request.target, limit: request.limit,
    })));
    if (request.kind === 'search') validatePathRoutes(result, graph, request);
    performance.mark("engine:paths-end");
    if (request.kind === 'search') self.postMessage({schema_version: 1, phase, result});
    phase = "comparison";
    if (result.paths.length) {
      performance.mark("engine:comparison-start");
      const comparison = parseWasmComparison(summarize_paths(JSON.stringify({graph, paths: result.paths})), result.paths.length, 'summary');
      validateComparisonParameters(comparison, graph.nodes[request.source]!.parameter_names, graph.nodes[request.target]!.parameter_names);
      performance.mark("engine:comparison-end");
      self.postMessage({schema_version: 1, phase, result: comparison});
    }
  } catch (error) {
    self.postMessage({schema_version: 1, phase, error: error instanceof Error ? error.message : String(error)});
  }
};

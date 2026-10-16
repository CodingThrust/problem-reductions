import init, { find_paths, summarize_paths } from "./path-selection.js";

performance.mark("engine:script");
const graph = fetch(new URL('./path-graph.json.gz', import.meta.url)).then(async (response) => {
  if (!response.ok) throw new Error(`Cannot load path metadata (${response.status})`);
  const text = await new Response(response.body.pipeThrough(new DecompressionStream('gzip'))).text();
  performance.mark("engine:metadata-decoded");
  const result = JSON.parse(text);
  performance.mark("engine:metadata-parsed");
  return result;
});
let engine;
function loadEngine() {
  return engine ||= (async () => {
    performance.mark("engine:fetch");
    const response = await fetch(new URL('./path-selection_bg.wasm.gz', import.meta.url));
    performance.mark("engine:headers");
    if (!response.ok) throw new Error(`Cannot load path engine (${response.status})`);
    const wasm = response.body.pipeThrough(new DecompressionStream('gzip'));
    // shortcut: Buffering avoids a Firefox streaming stall; revisit if module growth makes this buffer material.
    const bytes = await new Response(wasm).arrayBuffer();
    performance.mark("engine:wasm-decoded");
    await init({module_or_path: bytes});
    performance.mark("engine:initialized");
  })();
}
const ready = Promise.all([loadEngine(), graph]);
ready.catch(() => {});

self.onmessage = async ({data}) => {
  performance.mark("engine:request-received");
  let phase = "paths";
  try {
    const [, graph] = await ready;
    performance.mark("engine:paths-start");
    const result = data.paths ? {paths: data.paths} : JSON.parse(find_paths(JSON.stringify({...data, graph})));
    performance.mark("engine:paths-end");
    if (!data.paths) self.postMessage({phase, result});
    phase = "comparison";
    if (result.paths.length) {
      performance.mark("engine:comparison-start");
      const comparison = JSON.parse(summarize_paths(JSON.stringify({graph, paths: result.paths})));
      performance.mark("engine:comparison-end");
      self.postMessage({phase, result: comparison});
    }
  } catch (error) {
    self.postMessage({phase, error: error.message});
  }
};

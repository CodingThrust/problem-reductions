import init, { find_paths, summarize_paths } from "./path-selection.js";

let engine;
function loadEngine() {
  return engine ||= (async () => {
    const response = await fetch(new URL('./path-selection_bg.wasm.gz', import.meta.url));
    if (!response.ok) throw new Error(`Cannot load path engine (${response.status})`);
    const wasm = response.body.pipeThrough(new DecompressionStream('gzip'));
    await init({module_or_path: new Response(wasm, {headers: {'Content-Type': 'application/wasm'}})});
  })();
}

self.onmessage = async ({data}) => {
  let phase = "paths";
  try {
    await loadEngine();
    const result = data.paths ? {paths: data.paths} : JSON.parse(find_paths(JSON.stringify(data)));
    if (!data.paths) self.postMessage({phase, result});
    phase = "comparison";
    if (result.paths.length) {
      const comparison = JSON.parse(summarize_paths(JSON.stringify({graph: data.graph, paths: result.paths})));
      self.postMessage({phase, result: comparison});
    }
  } catch (error) {
    self.postMessage({phase, error: error.message});
  }
};

import type { GraphSceneData } from '../generated/graph-scene.mjs';

// Start independent downloads before the renderer finishes loading.
export const graphStartup: {scene: Promise<GraphSceneData>; worker: Worker | null} = {
  scene: fetch(document.body.dataset.scene!).then(async (response) => {
    if (!response.ok || !response.body) throw new Error(`Cannot load graph (${response.status})`);
    const text = await new Response(response.body.pipeThrough(new DecompressionStream('gzip'))).text();
    performance.mark('graph:scene-decoded');
    const scene = JSON.parse(text) as GraphSceneData;
    performance.mark('graph:scene-json-parsed');
    return scene;
  }),
  worker: null,
};
graphStartup.scene.catch(() => {});
try {
  const worker = graphStartup.worker = new Worker(new URL('../workers/path-worker.ts', import.meta.url), {type: 'module'});
  performance.mark('graph:worker-started');
  worker.addEventListener('error', (event) => {
    event.preventDefault();
    if (graphStartup.worker === worker) { worker.terminate(); graphStartup.worker = null; }
  });
}
catch { /* Path search reports worker creation errors when requested. */ }

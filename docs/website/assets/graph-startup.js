// Start independent downloads before the renderer finishes loading.
window.graphStartup = {
  scene: fetch('./assets/graph-scene.json.gz').then(async (response) => {
    if (!response.ok) throw new Error(`Cannot load graph (${response.status})`);
    const text = await new Response(response.body.pipeThrough(new DecompressionStream('gzip'))).text();
    performance.mark('graph:scene-decoded');
    const scene = JSON.parse(text);
    performance.mark('graph:scene-json-parsed');
    return scene;
  }),
  worker: null,
};
window.graphStartup.scene.catch(() => {});
try {
  const worker = window.graphStartup.worker = new Worker('./assets/path-worker.js', {type: 'module'});
  performance.mark('graph:worker-started');
  worker.addEventListener('error', (event) => {
    event.preventDefault();
    if (window.graphStartup.worker === worker) { worker.terminate(); window.graphStartup.worker = null; }
  });
}
catch { /* Path search reports worker creation errors when requested. */ }

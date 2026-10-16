import { resolve } from 'node:path';
import { build } from 'vite';

const [source, destination] = process.argv.slice(2);
if (!source || !destination) throw new Error('Usage: node scripts/build_frontend.mjs SOURCE OUTPUT');
const output = {
  entryFileNames: 'assets/[name].js',
  chunkFileNames: 'assets/[name].js',
  assetFileNames: 'assets/[name][extname]',
};

await build({
  configFile: false,
  root: resolve(source),
  publicDir: false,
  base: './',
  build: {
    outDir: resolve(destination),
    emptyOutDir: false,
    assetsInlineLimit: 0,
    // Keep the existing CSS rendering exact during the build migration.
    cssMinify: false,
    modulePreload: { polyfill: false },
    // The immutable release directory supplies versioning after URL rewriting.
    rolldownOptions: { input: ['index.html', 'graph.html'].map(name => resolve(source, name)), output },
  },
  worker: {
    format: 'es',
    // Keep the generated WASM binding/API intact beside the worker.
    plugins: () => [{
      name: 'preserve-wasm-binding',
      enforce: 'pre',
      resolveId(id) {
        if (id === './path-selection.js') return { id, external: true };
      },
    }],
    rolldownOptions: { output },
  },
});

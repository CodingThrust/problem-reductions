import { resolve } from 'node:path';
import { gzipSync } from 'node:zlib';
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
  plugins: [{
    name: 'javascript-size-budget',
    generateBundle(_options, bundle) {
      const scripts = Object.values(bundle).filter(item => item.fileName.endsWith('.js'))
        .map(item => Buffer.from(item.type === 'chunk' ? item.code : item.source));
      const raw = scripts.reduce((sum, bytes) => sum + bytes.length, 0);
      const gzip = scripts.reduce((sum, bytes) => sum + gzipSync(bytes).length, 0);
      if (raw > 320_000 || gzip > 60_000) this.error(`JavaScript size budget exceeded: ${raw} raw / ${gzip} gzip bytes`);
    },
  }],
  build: {
    outDir: resolve(destination),
    emptyOutDir: false,
    assetsInlineLimit: 0,
    // Keep the existing CSS rendering exact during the build migration.
    cssMinify: false,
    modulePreload: { polyfill: false },
    // The immutable release directory supplies versioning after URL rewriting.
    rolldownOptions: { input: ['index.html', 'graph.html'].map(name => resolve(source, name)), output,
      // Generated guards only define functions; discard guards unused by this realm.
      treeshake: { moduleSideEffects: [{ test: /\.validator\.mjs$/, sideEffects: false }] },
    },
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
    rolldownOptions: { output,
      treeshake: { moduleSideEffects: [{ test: /\.validator\.mjs$/, sideEffects: false }] },
    },
  },
});

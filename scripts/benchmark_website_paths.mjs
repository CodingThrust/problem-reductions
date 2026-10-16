// Real Rust/WASM benchmarks; each sample runs in a fresh process to isolate memory and tiering.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import vm from 'node:vm';
import {spawnSync} from 'node:child_process';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {performance} from 'node:perf_hooks';
import {parseWasmComparison} from '../docs/website/assets/wasm-comparison-data.mts';

const script = fileURLToPath(import.meta.url);
if (process.argv[2] === '--sample') {
  const [assets, inputFile, engine, outputFile] = process.argv.slice(3);
  const api = await import(pathToFileURL(path.join(assets, 'path-selection.js')));
  const input = JSON.parse(fs.readFileSync(inputFile));
  const started = performance.now();
  const wasm = api.initSync({module: fs.readFileSync(path.join(assets, 'path-selection_bg.wasm'))});
  const initialized = performance.now();
  const routes = JSON.parse(api.find_paths(JSON.stringify(input)));
  const enumerated = performance.now();
  const query = JSON.stringify({graph: input.graph, paths: routes.paths});
  const encoded = performance.now();
  const raw = api[engine](query);
  const compared = performance.now();
  const result = parseWasmComparison(raw, routes.paths.length, engine === 'summarize_paths' ? 'summary' : 'full');
  const parsed = performance.now();
  fs.writeFileSync(outputFile, raw);
  console.log(JSON.stringify({routes: routes.paths.length, truncated: routes.truncated,
    init_ms: initialized-started, search_ms: enumerated-initialized,
    encode_ms: encoded-enumerated, compare_ms: compared-encoded, parse_validate_ms: parsed-compared,
    bytes: Buffer.byteLength(raw), wasm_mib: wasm.memory.buffer.byteLength/1048576,
    peak_rss_mib: process.resourceUsage().maxRSS/1024, counts: result.counts, front: result.front.length}));
} else {
  const book = path.resolve(process.argv[2] || 'book');
  const option = (name, fallback) => {
    const i = process.argv.indexOf(name);
    return i < 0 ? fallback : process.argv[i+1];
  };
  const baseline = option('--baseline', null);
  const repeats = Number(option('--repeat', '3'));
  assert(Number.isInteger(repeats) && repeats > 0);
  const html = fs.readFileSync(path.join(book, 'graph.html'), 'utf8');
  const graphScript = html.match(/src="([^"]*graph-data\.js)[^"]*"/)[1];
  const assets = path.dirname(path.resolve(book, graphScript));
  const context = {window: {}};
  vm.runInNewContext(fs.readFileSync(path.join(assets, 'graph-data.js'), 'utf8'), context);
  const graph = context.window.REDUCTIONS;
  const endpoint = name => graph.nodes.findIndex(n => n.name === name && n.default);
  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), 'website-path-benchmark-'));
  const rows = [];
  const cases = [20,50,100,999].map(limit => ({name: `SAT-QUBO-${limit}`,
    query: {graph, source: endpoint('Satisfiability'), target: endpoint('QUBO'), limit}}));
  const widestTarget = graph.nodes.findIndex(n => n.name === 'ILP' && n.variant.coefficient === 'f64'
    && n.variant.variable === 'bool' && n.variant.bounds === 'general');
  if (widestTarget >= 0) cases.push({name:'Factoring-ILP-f64-999',
    query:{graph,source:endpoint('Factoring'),target:widestTarget,limit:999}});
  // A labeled scaling workload: the current registry has fewer than 999 simple paths per endpoint.
  // Independent oracle: n^a*m^b <= n^c*m^d iff a<=c and b<=d, for nonnegative exponents.
  for (const size of [100,999]) {
    const nodes = [{name:'Source',variant:{},parameter_names:['n','m']},
      {name:'Target',variant:{},parameter_names:['size']}];
    const edges = [], exponents = [];
    for (let i=0;i<size;i++) {
      const a=1+Math.floor(i/33), b=1+i%33;
      exponents.push([a,b]);
      nodes.push({name:`Branch${String(i).padStart(4,'0')}`,variant:{},parameter_names:['size']});
      edges.push({source:0,target:i+2,witness:true,turing:false,
        parameters:[{field:'size',contract:'exact',formula:`n^${a}*m^${b}`}]},
      {source:i+2,target:1,witness:true,turing:false,
        parameters:[{field:'size',contract:'exact',formula:'size'}]});
    }
    const counts = {Dominated:0,Equivalent:0,Incomparable:0,Unknown:0};
    for (let i=0;i<size;i++) for(let j=i+1;j<size;j++) {
      const [a,b]=exponents[i], [c,d]=exponents[j];
      counts[(a<=c&&b<=d)||(c<=a&&d<=b)?'Dominated':'Incomparable']++;
    }
    cases.push({name:`synthetic-monomial-grid-${size}`,query:{graph:{nodes,edges},source:0,target:1,limit:size},
      oracle: {counts,front:[1]}});
  }
  const selected = option('--case', null);
  const selectedCases = cases.filter(c => !selected || c.name === selected);
  assert(selectedCases.length, 'Unknown benchmark case');
  try {
    for (const fixture of selectedCases) {
      const inputFile = path.join(temporary, 'input.json');
      fs.writeFileSync(inputFile, JSON.stringify(fixture.query));
      const engines = baseline ? [['baseline',path.resolve(baseline),'compare_paths'],
        ['full',assets,'compare_paths'], ['summary',assets,'summarize_paths']] :
        [['full',assets,'compare_paths'],['summary',assets,'summarize_paths']];
      let reference, full;
      for (const [label, directory, engine] of engines) {
        const outputFile=path.join(temporary, `${label}.json`);
        for (let repeat=0;repeat<repeats;repeat++) {
          const child = spawnSync(process.execPath,[script,'--sample',directory,inputFile,engine,outputFile],
            {encoding:'utf8',timeout:180000,maxBuffer:1048576});
          if (child.status !== 0) {
            const row={case:fixture.name,engine:label,repeat,error:child.error?.message || child.stderr};
            rows.push(row); console.log(JSON.stringify(row)); break;
          }
          const result = JSON.parse(fs.readFileSync(outputFile));
          if (fixture.oracle) {
            assert.deepEqual(result.counts,fixture.oracle.counts);
            assert.deepEqual(result.front,fixture.oracle.front);
          }
          if (label==='baseline') reference=result;
          if (label==='full') {
            full=result;
            if(reference) {
              assert.deepEqual(full.profiles,reference.profiles);
              assert.equal(full.pairs.length,reference.pairs.length);
              full.pairs.forEach((pair,i)=> {
                assert.deepEqual(pair.paths,reference.pairs[i].paths);
                if(reference.pairs[i].verdict!=='Unknown') assert.deepEqual(pair.verdict,reference.pairs[i].verdict);
              });
            }
          }
          if (label==='summary') {
            assert.deepEqual(result.profiles,full.profiles);
            assert.deepEqual(result.counts,full.counts);
            assert.deepEqual(result.front,full.front);
          }
          const digest = value => createHash('sha256').update(JSON.stringify(value)).digest('hex');
          const row={case:fixture.name,engine:label,repeat,...JSON.parse(child.stdout),
            profiles_sha256:digest(result.profiles),front_sha256:digest(result.front),
            pairs_sha256:result.pairs?digest(result.pairs):null};
          rows.push(row); console.log(JSON.stringify(row));
        }
      }
    }
    if (rows.some(row => row.error)) process.exitCode = 1;
    const output=option('--output',null);
    if(output) fs.writeFileSync(output,JSON.stringify({node:process.version,platform:process.platform,
      architecture:process.arch,cpu:os.cpus()[0]?.model,memory_gib:os.totalmem()/1073741824,
      wasm_sha256:createHash('sha256').update(fs.readFileSync(path.join(assets,'path-selection_bg.wasm'))).digest('hex'),assets,rows},null,2)+'\n');
  } finally { fs.rmSync(temporary,{recursive:true,force:true}); }
}

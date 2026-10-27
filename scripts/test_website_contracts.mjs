import { validateNativeExportData, validateWasmPathRequest, validateWasmComparisonRequest, validateGraphSceneData, validateDetailsIndexData, parseWasmPathResult, parseWasmComparison, validateComparisonParameters, validatePathGraphData, validateAtlasSummary, validateAtlasRecord } from './website_contracts.mjs';
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { validatePathRequest, validatePathReply } from '../docs/website/src/lib/path-worker-data.mts';

test('production build rejects JavaScript over the delivery budget', () => {
  const source = mkdtempSync(join(tmpdir(), 'frontend-budget-'));
  try {
    for (const name of ['index', 'graph']) writeFileSync(join(source, `${name}.html`),
      '<script type="module" src="./large.js"></script>');
    writeFileSync(join(source, 'large.js'), `document.body.textContent = "${'x'.repeat(350_000)}";`);
    const result = spawnSync(process.execPath, ['scripts/build_frontend.mjs', source, join(source, 'out')], { encoding: 'utf8' });
    assert.notEqual(result.status, 0, 'Oversized production JavaScript was accepted');
    assert.match(result.stderr, /JavaScript size budget/);
  } finally {
    rmSync(source, { recursive: true, force: true });
  }
});

function scene() {
  return { nodes: [
      { name: 'Alpha', category: 'graph', variant: { weight: 'One' }, default: true },
      { name: 'Alpha', category: 'graph', variant: { weight: 'i64' }, default: false },
      { name: 'Beta', category: 'misc', variant: {}, default: true },
    ],
    edges: [{ source: 1, target: 2, witness: true, aggregate: false, turing: false, fields: { n: { certificate: 'num_vertices^2', accuracy: { kind: 'exact' } } } },
      { source: 0, target: 2, witness: false, aggregate: false, turing: true, fields: {} }],
    aliases: { Alpha: ['ALPHA'], Beta: [] },
    layout: { Alpha: { x: -0.5, y: 2 }, Beta: { x: 3.25, y: -1 } },
  };
}

test('scene validation preserves variants, edge ordering and fractional layout', () => {
  const input = scene();
  assert.deepEqual(validateGraphSceneData(input), input);
});

test('scene validation rejects malformed shapes and unsafe indices', () => {
  for (const mutate of [
    value => { delete value.edges[0].witness; },
    value => { value.edges[0].turing = null; },
    value => { value.nodes[0].category = 'unknown'; },
    value => { value.nodes[0].default = 'true'; },
    value => { value.layout.Alpha.x = Infinity; },
    value => { value.edges[0].source = -1; },
    value => { value.edges[0].source = 0.5; },
    value => { value.edges[0].source = 9007199254740992; },
  ]) {
    const value = scene();
    mutate(value);
    assert.throws(() => validateGraphSceneData(value));
  }
  for (const value of [null, [], 'scene', 1]) assert.throws(() => validateGraphSceneData(value));
});

test('scene validation rejects dangling references and ambiguous variants', () => {
  for (const mutate of [
    value => { value.edges[0].target = 3; },
    value => { value.aliases.Missing = ['X']; },
    value => { delete value.layout.Beta; },
    value => { value.layout.Missing = { x: 0, y: 0 }; },
    value => { value.nodes[1].default = true; },
    value => { value.nodes[2].default = false; },
    value => { value.nodes[1].variant = { weight: 'One' }; },
    value => {
      value.nodes[0].variant = { weight: 'One', graph: 'SimpleGraph' };
      value.nodes[1].variant = { graph: 'SimpleGraph', weight: 'One' };
    },
  ]) {
    const value = scene();
    mutate(value);
    assert.throws(() => validateGraphSceneData(value));
  }
});

test('producer validation accepts valid data and fails on an invalid scene', () => {
  const value = scene();
  const run = data => spawnSync(process.execPath, ['scripts/validate_website_data.mjs', 'graph-scene'],
    { input: JSON.stringify(data), encoding: 'utf8' });
  const valid = run(value);
  assert.equal(valid.status, 0, valid.stderr);
  value.edges[0].target = 3;
  assert.notEqual(run(value).status, 0);
});

const page = new URL('https://example.test/research/reductions/index.html');
const articles = new URL('./releases/current/assets/details/', page);
function details() {
  return { entries: { 'problem:Alpha': './releases/current/assets/details/problem/Alpha.html',
      'rule:Alpha->Beta': './releases/current/assets/details/rule/Alpha-%3EBeta.html' },
    anchors: { 'def:Alpha': 'problem:Alpha' },
  };
}

test('documentation index preserves encoded filenames and anchor targets', () => {
  const value = details();
  assert.deepEqual(validateDetailsIndexData(value, page, articles), value);
  const result = spawnSync(process.execPath,
    ['scripts/validate_website_data.mjs', 'graph-details', page.href, articles.href],
    { input: JSON.stringify(value), encoding: 'utf8' });
  assert.equal(result.status, 0, result.stderr);
});

test('documentation index rejects invalid shapes and dangling anchors', () => {
  for (const mutate of [
    value => { value.entries = []; },
    value => { value.entries = {}; },
    value => { value.entries.unexpected = './releases/current/assets/details/Alpha.html'; },
    value => { value.anchors['def:Beta'] = 'problem:Missing'; },
    value => { value.anchors['def:Alpha'] = null; },
  ]) {
    const value = details();
    mutate(value);
    assert.throws(() => validateDetailsIndexData(value, page, articles));
  }
});

test('documentation index rejects URLs outside the current release article subtree', () => {
  for (const url of [
    './index.html', './releases/old/assets/details/Alpha.html',
    'https://other.test/research/reductions/releases/current/assets/details/Alpha.html',
    './releases/current/assets/details/../graph.html',
    './releases/current/assets/details/%2e%2e/graph.html',
    './releases/current/assets/details/%2e%2e%2fgraph.html',
    './releases/current/assets/details/%5cAlpha.html',
    './releases/current/assets/details/%XX.html',
    './releases/current/assets/details/Alpha.html?release=old',
    './releases/current/assets/details/Alpha.html#section',
    './releases/current/assets/details/Alpha.json',
    'javascript:alert(1)',
  ]) {
    const value = details();
    value.entries['problem:Alpha'] = url;
    assert.throws(() => validateDetailsIndexData(value, page, articles), url);
  }
});

function pathResult() {
  const source = { name: 'Source', variant: {} };
  const middle = { name: 'Middle', variant: { graph: 'SimpleGraph', weight: 'One' } };
  const target = { name: 'Target', variant: { weight: 'i64' } };
  return { truncated: false, paths: [
    { steps: 2, path: [{ from: source, to: middle },
      { from: { name: 'Middle', variant: { weight: 'One', graph: 'SimpleGraph' } }, to: target }] },
    { steps: 1, path: [{ from: source, to: target }] },
  ] };
}

test('WASM path validation preserves route ordering and variant key semantics', () => {
  const result = pathResult();
  assert.deepEqual(parseWasmPathResult(JSON.stringify(result)), result);
  assert.deepEqual(parseWasmPathResult('{"paths":[],"truncated":false}'),
    { paths: [], truncated: false });
});

test('WASM path validation rejects invalid shapes and integer domains', () => {
  for (const mutate of [
    value => { value.truncated = 'false'; },
    value => { value.paths[0].steps = 0.5; },
    value => { value.paths[0].steps = -1; },
    value => { value.paths[0].steps = 9007199254740992; },
    value => { value.paths[0].path[0].from.name = ''; },
    value => { value.paths[0].path[0].from.variant = null; },
    value => { value.paths[0].path[0].to.variant.weight = 1; },
    value => { value.paths = Array(1000).fill(value.paths[0]); },
    value => { value.paths[0].path = []; },
  ]) {
    const value = pathResult();
    mutate(value);
    assert.throws(() => parseWasmPathResult(JSON.stringify(value)));
  }
  for (const text of ['{invalid', 'null', '[]']) assert.throws(() => parseWasmPathResult(text));
});

test('WASM path validation rejects incorrect step counts and discontinuous variants', () => {
  for (const mutate of [
    value => { value.paths[0].steps = 1; },
    value => { value.paths[0].path[1].from.name = 'Missing'; },
    value => { value.paths[0].path[1].from.variant.weight = 'i64'; },
    value => { delete value.paths[0].path[1].from.variant.graph; },
    value => { value.paths = []; value.truncated = true; },
  ]) {
    const value = pathResult();
    mutate(value);
    assert.throws(() => parseWasmPathResult(JSON.stringify(value)));
  }
});

function pathGraph() {
  return { nodes: [
    { name: 'Source', variant: { graph: 'SimpleGraph', weight: 'One' }, default: true, parameter_names: ['m', 'n'] },
    { name: 'Target', variant: {}, default: true, parameter_names: ['size'] },
  ], edges: [{ source: 0, target: 1, witness: true, turing: false, parameter_contract_error: null,
    parameters: [{ field: 'size', contract: 'upper_bound',
      formula: '9007199254740993 * n^9007199254740993 * 7^-1', reason: null }] }] };
}

test('path metadata preserves exact formulas, ordering and explicit nulls', () => {
  const value = pathGraph();
  assert.deepEqual(validatePathGraphData(value), value);
  const variants = structuredClone(value);
  variants.nodes.push({ ...structuredClone(variants.nodes[0]), variant: { weight: 'i64', graph: 'SimpleGraph' }, default: false });
  variants.edges[0].parameters[0].contract = 'exact';
  variants.edges[0].turing = true;
  variants.edges[0].witness = false;
  assert.deepEqual(validatePathGraphData(variants), variants);
  const unavailable = structuredClone(value);
  unavailable.edges[0].parameters[0] = { field: 'size', contract: 'unavailable', formula: null, reason: 'No certified bound' };
  assert.deepEqual(validatePathGraphData(unavailable), unavailable);
  const invalidContract = structuredClone(value);
  invalidContract.edges[0].parameter_contract_error = 'Missing native parameter contract';
  invalidContract.edges[0].parameters = [];
  assert.deepEqual(validatePathGraphData(invalidContract), invalidContract);
  assert.deepEqual(validatePathGraphData({ nodes: value.nodes, edges: [] }),
    { nodes: value.nodes, edges: [] });
  const unparameterized = structuredClone(value);
  unparameterized.nodes[1].parameter_names = [];
  unparameterized.edges[0].parameters = [];
  assert.deepEqual(validatePathGraphData(unparameterized), unparameterized);
});

test('path metadata rejects malformed shapes, flags and unsafe indices', () => {
  for (const mutate of [
    value => { value.nodes = []; },
    value => { value.nodes[0].name = ''; },
    value => { value.nodes[0].default = 1; },
    value => { value.nodes[0].variant.weight = 1; },
    value => { value.nodes[0].parameter_names = ['n', 'n']; },
    value => { value.edges[0].source = -1; },
    value => { value.edges[0].source = 0.5; },
    value => { value.edges[0].target = 9007199254740992; },
    value => { value.edges[0].witness = 'true'; },
    value => { value.edges[0].turing = null; },
    value => { delete value.edges[0].parameter_contract_error; },
    value => { value.edges[0].parameter_contract_error = ''; },
  ]) {
    const value = pathGraph(); mutate(value);
    assert.throws(() => validatePathGraphData(value));
  }
  for (const value of [null, [], 'metadata']) assert.throws(() => validatePathGraphData(value));
});

test('path metadata rejects ambiguous variants, defaults, edges and target-field references', () => {
  for (const mutate of [
    value => { value.nodes.push({ ...value.nodes[0], variant: { weight: 'One', graph: 'SimpleGraph' } }); },
    value => { value.nodes.push({ ...value.nodes[0], variant: { weight: 'i64' } }); },
    value => { value.nodes[0].default = false; },
    value => { value.edges[0].source = 2; },
    value => { value.edges[0].target = 2; },
    value => { value.edges.push(structuredClone(value.edges[0])); },
    value => { value.edges[0].parameters[0].field = 'unknown'; },
    value => { value.edges[0].parameters.push(structuredClone(value.edges[0].parameters[0])); },
    value => { value.edges[0].parameters = []; },
  ]) {
    const value = pathGraph(); mutate(value);
    assert.throws(() => validatePathGraphData(value));
  }
});

test('path metadata validates parameter tags and nullable formula/reason semantics', () => {
  for (const mutate of [
    field => { field.contract = 'guessed'; },
    field => { field.formula = 9007199254740992; },
    field => { field.formula = ''; },
    field => { field.formula = null; },
    field => { delete field.formula; },
    field => { field.reason = 'Unexpected reason on a certified formula'; },
    field => { delete field.reason; },
    field => { field.contract = 'unavailable'; },
    field => { field.contract = 'unavailable'; field.formula = null; field.reason = ''; },
  ]) {
    const value = pathGraph(); mutate(value.edges[0].parameters[0]);
    assert.throws(() => validatePathGraphData(value));
  }
});

test('the metadata producer accepts a valid graph and rejects a missing target contract', () => {
  const value = pathGraph();
  const run = data => spawnSync(process.execPath, ['scripts/validate_website_data.mjs', 'path-graph'],
    { input: JSON.stringify(data), encoding: 'utf8' });
  const valid = run(value);
  assert.equal(valid.status, 0, valid.stderr);
  value.edges[0].parameters = [];
  assert.notEqual(run(value).status, 0);
});

function protocolGraph() {
  const graph = pathGraph();
  graph.nodes.push({ name: 'Middle', variant: {}, default: true, parameter_names: ['size'] });
  graph.edges.push({ ...structuredClone(graph.edges[0]), target: 2 },
    { ...structuredClone(graph.edges[0]), source: 2, parameters: [{ field: 'size', contract: 'exact', formula: 'size', reason: null }] });
  return graph;
}
function protocolPaths() {
  const graph = protocolGraph();
  const endpoint = i => ({ name: graph.nodes[i].name, variant: graph.nodes[i].variant });
  return { truncated: false, paths: [[0, 1], [0, 2, 1]].map(indices => ({
    steps: indices.length - 1, path: indices.slice(1).map((to, i) => ({ from: endpoint(indices[i]), to: endpoint(to) })),
  })) };
}
function protocolRequest(cached = false) {
  return { kind: cached ? 'compare' : 'search', source: 0, target: 1, limit: 2,
    ...(cached ? { cached: protocolPaths() } : {}) };
}

test('worker replies consume engine results without a second graph registry', () => {
  const request = protocolRequest(), paths = protocolPaths();
  const first = { phase: 'paths', result: paths };
  assert.deepEqual(validatePathReply(first, request), first);
  const last = { phase: 'comparison', result: comparison() };
  assert.deepEqual(validatePathReply(last, request, paths), last);
  for (const phase of ['paths', 'comparison']) {
    const error = { phase, error: 'Cannot complete the query' };
    assert.deepEqual(validatePathReply(error, request, paths), error);
  }
  assert.throws(() => validatePathReply(last, request));
  assert.throws(() => validatePathReply(first, request, paths));
  assert.throws(() => validatePathReply(first, protocolRequest(true), paths));
  const empty = { truncated: false, paths: [] };
  assert.throws(() => validatePathReply(last, request, empty));
  assert.deepEqual(validatePathReply({ ...first, result: empty }, request), { ...first, result: empty });
});

test('worker requests preserve search and cached comparison inputs', () => {
  for (const cached of [false, true]) {
    const request = protocolRequest(cached);
    assert.deepEqual(validatePathRequest(request), request);
  }
});

test('worker requests reject invalid indices and limits', () => {
  for (const field of ['source', 'target']) for (const value of [-1, 0.5, NaN, Infinity, 9007199254740992]) {
    assert.throws(() => validatePathRequest({...protocolRequest(), [field]: value}));
  }
  for (const limit of [0, 1000, 1.5, NaN, Infinity]) {
    assert.throws(() => validatePathRequest({...protocolRequest(), limit}));
  }
});

function comparison() {
  return { counts: { Dominated: 1, Equivalent: 0, Incomparable: 0, Unknown: 0 }, front: [2],
    profiles: ['n^2', 'n'].map(certificate => ({ source_coordinates: ['m', 'n'],
      fields: { n: { certificate, accuracy: { kind: 'exact' } } } })),
  };
}

test('comparison validation preserves exact strings, profile ordering and summary fields', () => {
  const value = comparison();
  value.profiles[0].fields.n.certificate = '9007199254740993 * 7^-1 * n^9007199254740993';
  assert.deepEqual(parseWasmComparison(JSON.stringify(value), 2), value);
});

test('comparison results retain source coordinates and target fields from the selected registry variants', () => {
  const value = comparison();
  value.profiles[0].source_coordinates = ['n', 'm'];
  const original = structuredClone(value);
  validateComparisonParameters(value, ['m', 'n'], ['n']);
  assert.deepEqual(value, original);
  for (const mutate of [
    value => { value.profiles[0].source_coordinates.pop(); },
    value => { value.profiles[0].source_coordinates.push('unknown'); },
    value => { delete value.profiles[0].fields.n; },
    value => { value.profiles[0].fields.other = value.profiles[0].fields.n; },
  ]) {
    const invalid = comparison(); mutate(invalid);
    assert.throws(() => validateComparisonParameters(invalid, ['m', 'n'], ['n']));
  }
});

test('comparison validation rejects malformed shapes and unsafe numeric counts', () => {
  for (const mutate of [
    value => { value.mode = 'other'; },
    value => { value.counts.Unknown = 0.5; },
    value => { value.counts.Unknown = -1; },
    value => { value.counts.Unknown = 9007199254740992; },
    value => { value.profiles[0].fields.n.certificate = 1; },
    value => { delete value.profiles[0].fields.n.certificate; },
    value => { value.profiles[0].fields.n.accuracy.kind = 'guessed'; },
    value => { value.profiles[0].source_coordinates = 'n'; },
    value => { value.profiles[0].fields[''] = value.profiles[0].fields.n; delete value.profiles[0].fields.n; },
    value => { value.pairs = null; },
  ]) {
    const value = comparison();
    mutate(value);
    assert.throws(() => parseWasmComparison(JSON.stringify(value), 2));
  }
});

test('comparison validation checks profile, coordinate, front and aggregate references', () => {
  for (const mutate of [
    value => { value.profiles.pop(); },
    value => { value.counts.Dominated = 0; },
    value => { value.front = [0]; },
    value => { value.front = [3]; },
    value => { value.front = [2, 2]; },
    value => { value.front = []; },
    value => { value.profiles[0].source_coordinates = ['n', 'n']; },
    value => { value.profiles[0].fields.n.certificate = 'missing'; },
    value => { value.profiles[0].fields.n.certificate = null; },
    value => { value.profiles[0].fields.n.accuracy = { kind: 'unavailable', reason: { kind: 'missing_field' } }; },
    value => { value.pairs = []; },
  ]) {
    const value = comparison();
    mutate(value);
    assert.throws(() => parseWasmComparison(JSON.stringify(value), 2));
  }
});

test('comparison validation accepts nondominated verdicts and all declared unavailable-reason shapes', () => {
  for (const verdict of ['Equivalent', 'Incomparable', 'Unknown']) {
    const value = comparison();
    value.counts = { Dominated: 0, Equivalent: 0, Incomparable: 0, Unknown: 0 };
    value.counts[verdict] = 1;
    value.front = [1, 2];
    value.profiles[0].fields.n.certificate = verdict === 'Incomparable' ? 'm' : 'n';
    if (verdict === 'Unknown') value.profiles[0].fields.n = {
      certificate: null, accuracy: { kind: 'unavailable', reason: { kind: 'missing_field' } },
    };
    assert.deepEqual(parseWasmComparison(JSON.stringify(value), 2), value);
  }
  for (const reason of [{ kind: 'not_polynomial', detail: 'n^-1' }, { kind: 'unavailable', detail: 'No bound' },
    { kind: 'missing_field' }, { kind: 'validity' }, { kind: 'resource_limit', detail: 'No certificate' }]) {
    const value = comparison();
    value.counts = { Dominated: 0, Equivalent: 0, Incomparable: 0, Unknown: 1 };
    value.front = [1, 2];
    value.profiles[0].fields.n = { certificate: null, accuracy: { kind: 'unavailable', reason } };
    assert.deepEqual(parseWasmComparison(JSON.stringify(value), 2), value);
  }
});

test('comparison validation covers empty batches and rejects unsafe path counts', () => {
  const value = { profiles: [], front: [],
    counts: { Dominated: 0, Equivalent: 0, Incomparable: 0, Unknown: 0 } };
  assert.deepEqual(parseWasmComparison(JSON.stringify(value), 0), value);
  for (const count of [-1, NaN, 9007199254740992]) {
    assert.throws(() => parseWasmComparison(JSON.stringify(comparison()), count));
  }
});

function atlasSummary() {
  return { kind: 'summary',
    nodes: [{name: 'Alpha', variant: {weight: 'One'}, category: 'graph'}, {name: 'Beta', variant: {}, category: 'misc'}],
    edges: [{source: 0, target: 1}], schemas: [{name: 'Alpha', description: 'First problem'}, {name: 'Beta', description: 'Second problem'}]};
}
function atlasRecord() {
  const summary = atlasSummary(), mathml = '<math xmlns="http://www.w3.org/1998/Math/MathML" aria-label="n"><mi mathvariant="normal">n</mi></math>';
  return { kind: 'record',
    nodes: summary.nodes.map(node => ({...node, default: true, parameter_names: ['n'], complexity: 'n', complexity_mathml: mathml,
      doc_path: `models/graph/struct.${node.name}.html`, api_path: `models/graph/struct.${node.name}.html`})),
    edges: [{source: 0, target: 1, witness: true, aggregate: false, turing: false, parameter_contract_error: null,
      parameters: [{field: 'n', contract: 'exact', formula: '9007199254740993 / 7 * n^9007199254740995', reason: null,
        mathml: '<math xmlns="http://www.w3.org/1998/Math/MathML" aria-label="9007199254740993 / 7 * n^9007199254740995"><mrow><mfrac><mn>9007199254740993</mn><mn>7</mn></mfrac><mo>·</mo><msup><mi mathvariant="normal">n</mi><mn>9007199254740995</mn></msup></mrow></math>'}],
      doc_path: 'rules/alpha_beta/index.html', source_path: 'src/rules/alpha_beta.rs', api_path: 'rules/trait.ReduceTo.html', test_path: null}],
    schemas: [{...summary.schemas[0], category: 'graph', fields: [{name: 'n', type_name: 'u64', description: ''}],
      module_path: 'problemreductions::models::graph::alpha', source_path: 'src/models/graph/alpha.rs'}]};
}
test('Atlas contracts preserve summary metadata, exact formulas and nullable fields', () => {
  const summary = atlasSummary(), record = atlasRecord();
  summary.schemas[0].aliases = ['A']; summary.schemas[0].display_name = 'Alpha problem';
  assert.deepEqual(validateAtlasSummary(structuredClone(summary)), summary);
  assert.deepEqual(validateAtlasRecord(structuredClone(record), 'Alpha'), record);
  assert.equal(Object.hasOwn(validateAtlasRecord(record).schemas[0], 'aliases'), false);
  const producer = spawnSync(process.execPath, ['scripts/validate_website_data.mjs', 'atlas-records'],
    {input: JSON.stringify([record]), encoding: 'utf8'});
  assert.equal(producer.status, 0, producer.stderr);
});
test('Atlas contracts reject malformed data and unsafe embedded markup or links', () => {
  for (const mutate of [
    value => { value.nodes[0].category = 'other'; }, value => { value.edges[0].parameters[0].formula = 9007199254740992; },
    value => { value.nodes[0].complexity_mathml = '<img src=x onerror=alert(1)>'; },
    value => { value.edges[0].parameters[0].mathml = '<math><script>alert(1)</script></math>'; },
    value => { value.nodes[0].api_path = '../index.html'; }, value => { value.schemas[0].aliases = null; },
  ]) { const record = atlasRecord(); mutate(record); assert.throws(() => validateAtlasRecord(record)); }
  assert.throws(() => validateAtlasSummary(atlasRecord()));
  assert.throws(() => validateAtlasRecord(atlasSummary()));
});
test('Atlas contracts reject mismatched families and invalid graph references', () => {
  assert.throws(() => validateAtlasRecord(atlasRecord(), 'Beta'));
  const summary = atlasSummary(); summary.schemas.pop();
  assert.throws(() => validateAtlasSummary(summary));
  const record = atlasRecord(); record.edges[0].target = 2;
  assert.throws(() => validateAtlasRecord(record));
  const duplicate = atlasSummary(); duplicate.nodes.push({...duplicate.nodes[0]});
  assert.throws(() => validateAtlasSummary(duplicate));
});

function nativeExports() {
  const record = atlasRecord();
  return {graph: {
    nodes: record.nodes.map(({api_path, complexity_mathml, ...node}) => node),
    edges: record.edges.map(({api_path, source_path, test_path, parameters, ...edge}) =>
      ({...edge, parameters: parameters.map(({mathml, ...parameter}) => parameter)})),
    aliases: {Alpha: ['A'], Beta: []}},
    schemas: record.nodes.map(node => ({name: node.name, category: node.category, description: '',
      fields: [{name: 'n', type_name: 'u64', description: ''}], module_path: 'problemreductions::models::graph::alpha'}))};
}
test('native export boundary preserves exact metadata and rejects malformed registry references', () => {
  const input = nativeExports();
  assert.deepEqual(validateNativeExportData(structuredClone(input)), input);
  for (const mutate of [
    value => { value.graph.nodes[0].category = 123; }, value => { value.schemas[0].fields = null; },
    value => { value.graph.edges[0].target = 2; }, value => { value.graph.aliases.Missing = ['X']; },
    value => { value.schemas[1].name = 'Missing'; },
  ]) {const value = nativeExports(); mutate(value); assert.throws(() => validateNativeExportData(value));}
});
test('Rust-derived request schemas preserve optional fields and reject malformed native input', () => {
  const query = {graph: protocolGraph(), source: 0, target: 1, limit: 2};
  assert.equal(validateWasmPathRequest(query), true);
  const comparison = {graph: query.graph, paths: protocolPaths().paths};
  assert.equal(validateWasmComparisonRequest(comparison), true);
  delete query.graph.edges[0].parameter_contract_error;
  delete query.graph.edges[0].parameters[0].reason;
  assert.equal(validateWasmPathRequest(query), true);
  query.graph.nodes[0].parameter_names = [1];
  assert.equal(validateWasmPathRequest(query), false);
  assert.equal(validateWasmComparisonRequest({...comparison, paths: [{steps: 1, path: []}]}), false);
});

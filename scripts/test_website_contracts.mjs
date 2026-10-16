import assert from 'node:assert/strict';
import { test } from 'node:test';
import { spawnSync } from 'node:child_process';
import { validateGraphSceneData } from '../docs/website/assets/graph-scene-data.mts';
import { validateDetailsIndexData } from '../docs/website/assets/graph-details-data.mts';
import { parseWasmPathResult } from '../docs/website/assets/wasm-paths-data.mts';
import { parseWasmComparison } from '../docs/website/assets/wasm-comparison-data.mts';

function scene() {
  return {
    schema_version: 1,
    nodes: [
      { name: 'Alpha', category: 'graph', variant: { weight: 'One' }, default: true },
      { name: 'Alpha', category: 'graph', variant: { weight: 'i64' }, default: false },
      { name: 'Beta', category: 'misc', variant: {}, default: true },
    ],
    edges: [{ source: 1, target: 2 }, { source: 0, target: 2 }],
    aliases: { Alpha: ['ALPHA'], Beta: [] },
    layout: { Alpha: { x: -0.5, y: 2 }, Beta: { x: 3.25, y: -1 } },
  };
}

test('scene validation preserves variants, edge ordering and fractional layout', () => {
  const input = scene();
  assert.deepEqual(validateGraphSceneData(input), input);
});

test('scene validation rejects malformed shapes, versions and unsafe indices', () => {
  for (const mutate of [
    value => { delete value.schema_version; },
    value => { value.schema_version = 2; },
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
  return {
    schema_version: 1,
    entries: { 'problem:Alpha': './releases/current/assets/details/problem/Alpha.html',
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
    value => { delete value.schema_version; },
    value => { value.schema_version = 2; },
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
  return { schema_version: 1, truncated: false, paths: [
    { steps: 2, path: [{ from: source, to: middle },
      { from: { name: 'Middle', variant: { weight: 'One', graph: 'SimpleGraph' } }, to: target }] },
    { steps: 1, path: [{ from: source, to: target }] },
  ] };
}

test('WASM path validation preserves route ordering and variant key semantics', () => {
  const result = pathResult();
  assert.deepEqual(parseWasmPathResult(JSON.stringify(result)), result);
  assert.deepEqual(parseWasmPathResult('{"paths":[],"truncated":false,"schema_version":1}'),
    { schema_version: 1, paths: [], truncated: false });
});

test('WASM path validation rejects invalid versions, shapes and integer domains', () => {
  for (const mutate of [
    value => { delete value.schema_version; },
    value => { value.schema_version = 2; },
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

function comparison(full = false) {
  return { schema_version: 5, mode: 'generic',
    counts: { Dominated: 1, Equivalent: 0, Incomparable: 0, Unknown: 0 }, front: [2],
    profiles: ['n^2', 'n'].map(certificate => ({ source_coordinates: ['m', 'n'],
      fields: { n: { certificate, accuracy: { kind: 'exact' } } } })),
    ...(full ? { pairs: [{ paths: [1, 2], verdict: 'Dominated', dominates: { winner: 2, loser: 1 } }] } : {}),
  };
}

test('comparison validation preserves exact strings, profile ordering and optional pair omission', () => {
  for (const full of [false, true]) {
    const value = comparison(full);
    value.profiles[0].fields.n.certificate = '9007199254740993 * 7^-1 * n^9007199254740993';
    const result = parseWasmComparison(JSON.stringify(value), 2, full ? 'full' : 'summary');
    assert.deepEqual(result, value);
    assert.equal(Object.hasOwn(result, 'pairs'), full);
  }
});

test('comparison validation rejects malformed shapes, versions and unsafe numeric counts', () => {
  for (const mutate of [
    value => { value.schema_version = 4; },
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
    assert.throws(() => parseWasmComparison(JSON.stringify(value), 2, 'summary'));
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
    assert.throws(() => parseWasmComparison(JSON.stringify(value), 2, 'summary'));
  }
});

test('full comparisons require every pair, matching counts, dominance endpoints and the resulting front', () => {
  for (const mutate of [
    value => { delete value.pairs; },
    value => { value.pairs = []; },
    value => { value.pairs[0].paths = [1, 3]; },
    value => { value.pairs[0].paths = [2, 2]; },
    value => { value.pairs[0].paths = [2, 1]; },
    value => { value.pairs[0].dominates = { winner: 3, loser: 1 }; },
    value => { value.pairs[0].dominates = { winner: 1, loser: 1 }; },
    value => { delete value.pairs[0].dominates; },
    value => { value.pairs[0].verdict = 'Equivalent'; },
    value => { value.counts = { Dominated: 0, Equivalent: 1, Incomparable: 0, Unknown: 0 }; },
    value => { value.front = [1]; },
  ]) {
    const value = comparison(true);
    mutate(value);
    assert.throws(() => parseWasmComparison(JSON.stringify(value), 2, 'full'));
  }
});

test('comparison validation accepts nondominated verdicts and all declared unavailable-reason shapes', () => {
  for (const verdict of ['Equivalent', 'Incomparable', 'Unknown']) {
    const value = comparison(true);
    value.counts = { Dominated: 0, Equivalent: 0, Incomparable: 0, Unknown: 0 };
    value.counts[verdict] = 1;
    value.front = [1, 2];
    value.pairs = [{ paths: [1, 2], verdict }];
    value.profiles[0].fields.n.certificate = verdict === 'Incomparable' ? 'm' : 'n';
    if (verdict === 'Unknown') value.profiles[0].fields.n = {
      certificate: null, accuracy: { kind: 'unavailable', reason: { kind: 'missing_field' } },
    };
    assert.deepEqual(parseWasmComparison(JSON.stringify(value), 2, 'full'), value);
  }
  for (const reason of [{ kind: 'not_polynomial', detail: 'n^-1' }, { kind: 'unavailable', detail: 'No bound' },
    { kind: 'missing_field' }, { kind: 'validity' }, { kind: 'resource_limit', detail: 'No certificate' }]) {
    const value = comparison();
    value.counts = { Dominated: 0, Equivalent: 0, Incomparable: 0, Unknown: 1 };
    value.front = [1, 2];
    value.profiles[0].fields.n = { certificate: null, accuracy: { kind: 'unavailable', reason } };
    assert.deepEqual(parseWasmComparison(JSON.stringify(value), 2, 'summary'), value);
  }
});

test('comparison validation covers empty batches and rejects repeated pairs in a larger batch', () => {
  for (const mode of ['full', 'summary']) {
    const value = { schema_version: 5, mode: 'generic', profiles: [], front: [],
      counts: { Dominated: 0, Equivalent: 0, Incomparable: 0, Unknown: 0 }, ...(mode === 'full' ? { pairs: [] } : {}) };
    assert.deepEqual(parseWasmComparison(JSON.stringify(value), 0, mode), value);
  }
  const value = comparison(true);
  value.profiles = ['n', 'n^2', 'n^3'].map(certificate => ({ source_coordinates: ['n'],
    fields: { n: { certificate, accuracy: { kind: 'upper_bound' } } } }));
  value.counts.Dominated = 3;
  value.front = [1];
  value.pairs = [[1, 2], [1, 3], [2, 3]].map(([winner, loser]) => ({ paths: [winner, loser],
    verdict: 'Dominated', dominates: { winner, loser } }));
  assert.deepEqual(parseWasmComparison(JSON.stringify(value), 3, 'full'), value);
  value.pairs[1] = value.pairs[0];
  assert.throws(() => parseWasmComparison(JSON.stringify(value), 3, 'full'));
  for (const count of [-1, NaN, 9007199254740992]) {
    assert.throws(() => parseWasmComparison(JSON.stringify(comparison()), count, 'summary'));
  }
});

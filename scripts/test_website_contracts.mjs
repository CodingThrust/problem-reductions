import assert from 'node:assert/strict';
import { test } from 'node:test';
import { spawnSync } from 'node:child_process';
import { validateGraphSceneData } from '../docs/website/assets/graph-scene-data.mts';

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

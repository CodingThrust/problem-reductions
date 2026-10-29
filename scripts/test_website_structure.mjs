import assert from 'node:assert/strict';
import { test } from 'node:test';
import { edgeGrowth, growthClass, activeRules, components, reach, familyRules, CLASSES } from '../docs/website/src/lib/graph-structure.mts';

const field = (certificate, kind = 'exact') => ({ certificate, accuracy: { kind } });
const rule = (source, target, fields, caps = {}) => ({ source, target, witness: true, aggregate: false, turing: false, fields, ...caps });

test('an edge is classified by its highest-degree field and keeps exact vs bound', () => {
  const g = edgeGrowth(rule(0, 1, { a: field('num_vertices'), b: field('2 * num_edges^2 + 1', 'upper_bound') }));
  assert.equal(g.degree, 2n);
  assert.equal(g.cls, 'quadratic');
  assert.equal(g.accuracy, 'upper_bound');
  assert.equal(edgeGrowth(rule(0, 1, { a: field('num_vertices * num_edges') })).accuracy, 'exact');
});

test('constants, higher degrees and unavailable fields get their own classes', () => {
  assert.equal(edgeGrowth(rule(0, 1, { a: field('3') })).cls, 'constant');
  assert.equal(edgeGrowth(rule(0, 1, { a: field('num_vertices^4') })).cls, 'higher');
  const partial = edgeGrowth(rule(0, 1, { a: field('num_vertices'), b: field(null, 'unavailable') }));
  assert.equal(partial.cls, 'unknown');
  assert.equal(partial.degree, null);
  assert.equal(growthClass(null), 'unknown');
  assert.deepEqual(CLASSES.map(c => c.id), ['constant', 'linear', 'quadratic', 'cubic', 'higher', 'unknown']);
});

test('scope selects edges by the capability alone and the limit keeps provable classes only', () => {
  const rules = [
    rule(0, 1, { a: field('num_vertices') }, { aggregate: true }),
    rule(1, 2, { a: field('num_vertices^2') }),
    rule(2, 3, { a: field(null, 'unavailable') }),
    rule(3, 0, { a: field('1') }, { witness: false, turing: true }),
  ];
  assert.deepEqual(activeRules(rules, { scope: 'witness', limit: 'unknown' }), [0, 1, 2]);
  assert.deepEqual(activeRules(rules, { scope: 'witness', limit: 'quadratic' }), [0, 1]);
  assert.deepEqual(activeRules(rules, { scope: 'witness', limit: 'linear' }), [0]);
  assert.deepEqual(activeRules(rules, { scope: 'aggregate', limit: 'unknown' }), [0]);
  assert.deepEqual(activeRules(rules, { scope: 'turing', limit: 'unknown' }), [3]);
  assert.deepEqual(activeRules(rules, { scope: ['witness', 'turing'], limit: 'unknown' }), [0, 1, 2, 3]);
  assert.deepEqual(activeRules(rules, { scope: 'witness', limit: 'unknown', skip: new Set([1]) }), [0, 2]);
});

test('components are strongly connected sets and follow the active edge set', () => {
  const edges = [[0, 1], [1, 0], [1, 2], [2, 3], [3, 2]].map(([s, t]) => ({ source: s, target: t }));
  const { groups } = components(5, edges);
  assert.deepEqual(groups.filter(g => g.length > 1).map(g => [...g].sort()).sort(), [[0, 1], [2, 3]]);
  assert.equal(groups.length, 3);
  const cut = components(5, edges.filter(e => !(e.source === 1 && e.target === 0)));
  assert.equal(cut.groups.filter(g => g.length > 1).length, 1);
});

test('reachability is directed over concrete variants', () => {
  const edges = [[0, 1], [1, 2], [3, 1]].map(([s, t]) => ({ source: s, target: t }));
  assert.deepEqual([...reach(4, edges, 0, 'down')].sort(), [1, 2]);
  assert.deepEqual([...reach(4, edges, 2, 'up')].sort(), [0, 1, 3]);
  assert.deepEqual([...reach(4, edges, 2, 'down')], []);
});

test('family rules keep every variant rule traceable and never join distinct variants', () => {
  const nodes = [{ name: 'A' }, { name: 'A' }, { name: 'B' }, { name: 'B' }];
  const rules = [rule(0, 2, { a: field('num_vertices') }), rule(1, 3, { a: field('num_vertices^3') })];
  const fam = familyRules(nodes, rules, [0, 1]);
  assert.equal(fam.size, 1);
  const edge = fam.get('A→B');
  assert.deepEqual(edge.rules, [0, 1]);
  assert.equal(edge.cls, 'linear');
  assert.equal(edge.worst, 'cubic');
  assert.equal(familyRules(nodes, rules, [1]).get('A→B').cls, 'cubic');
});

import { depthsFrom } from '../docs/website/src/lib/graph-structure.mts';
import * as force from '../docs/website/src/lib/force-layout.mts';

test('browser layout yields to input and preserves the settled layout', async () => {
  const input = { ids: ['A', 'B', 'C'], links: [['A', 'B'], ['B', 'C']], hubs: new Set(), radius: () => 5 };
  const simulation = force.createSimulation(input);
  let inputHandled = false;
  setTimeout(() => { inputHandled = true; }, 0);
  assert.equal(await force.settleSimulation(simulation, new AbortController().signal), true);
  assert.equal(inputHandled, true, 'Layout must let pending input run before it finishes');
  assert.deepEqual(simulation.positions(), force.forceLayout(input));
});

test('an interrupted layout stops before moving its nodes', async () => {
  const simulation = force.createSimulation({ ids: ['A', 'B'], links: [['A', 'B']], hubs: new Set(), radius: () => 5 });
  simulation.pin('A', { x: 100, y: 200 });
  const before = simulation.positions(), controller = new AbortController();
  const pending = force.settleSimulation(simulation, controller.signal);
  controller.abort();
  assert.equal(await pending, false);
  assert.deepEqual(simulation.positions(), before);
});

const link = (source, target) => ({ source, target });

test('depth is the fewest reductions from any root, and unreachable problems stay null', () => {
  const depths = depthsFrom(['SAT', 'A', 'B', 'C', 'Z'], [link('SAT', 'A'), link('A', 'B'), link('SAT', 'B'), link('Z', 'C')], ['SAT']);
  assert.deepEqual([...depths], [['SAT', 0], ['A', 1], ['B', 1], ['C', null], ['Z', null]]);
  assert.equal(depthsFrom(['X'], [], ['Missing']).get('X'), null);
});


test('concrete route depths do not join sibling variants or seed the entire SAT family', () => {
  // 0=SAT, 1=3SAT, 2=2SAT, 3=A(default), 4=A(other), 5=ILP.
  // A family-level graph would incorrectly invent SAT -> A(other) -> ILP.
  const ids = [0, 1, 2, 3, 4, 5], links = [link(0, 1), link(1, 3), link(2, 0), link(4, 5)];
  assert.deepEqual([...depthsFrom(ids, links, [0])], [[0, 0], [1, 1], [2, null], [3, 2], [4, null], [5, null]]);
  const toIlp = depthsFrom(ids, links.map(e => link(e.target, e.source)), [5]);
  assert.equal(toIlp.get(3), null);
  assert.equal(toIlp.get(4), 1);
});

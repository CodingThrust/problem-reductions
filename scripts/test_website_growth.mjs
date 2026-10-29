import assert from 'node:assert/strict';
import { test } from 'node:test';
import { growth, pathDegree, sizeBadge, compareDegrees } from '../docs/website/src/lib/graph-growth.mts';

const field = (certificate, kind = 'exact') => ({ certificate, accuracy: { kind } });

test('growth distinguishes adjacent wide powers without dropping independent terms', () => {
  const value = field('num_edges^9007199254740992 + num_vertices^9007199254740993');
  assert.deepEqual(growth(value), { degree: 9007199254740993n, lead: 'O(edges⁹⁰⁰⁷¹⁹⁹²⁵⁴⁷⁴⁰⁹⁹² + vertices⁹⁰⁰⁷¹⁹⁹²⁵⁴⁷⁴⁰⁹⁹³)' });
  assert.equal(value.certificate, 'num_edges^9007199254740992 + num_vertices^9007199254740993');
  assert.equal(sizeBadge(pathDegree({ fields: { size: value } })), 'n⁹⁰⁰⁷¹⁹⁹²⁵⁴⁷⁴⁰⁹⁹³');
});

test('growth sums large monomial exponents without rounding or changing rational coefficients', () => {
  const value = field('9007199254740993 * 7^-1 * num_edges^9007199254740993 * num_vertices');
  const original = structuredClone(value);
  assert.deepEqual(growth(value), { degree: 9007199254740994n, lead: 'O(edges⁹⁰⁰⁷¹⁹⁹²⁵⁴⁷⁴⁰⁹⁹³·vertices)' });
  assert.deepEqual(value, original);
  assert.equal(growth(field('num_vertices^123456789012345678901234567890')).degree, 123456789012345678901234567890n);
});

test('growth retains constants, bounds, empty profiles and unavailable fields', () => {
  assert.deepEqual(growth(field('2 * num_edges^2 + 3^-1 * num_vertices^2 + 1', 'upper_bound')),
    { degree: 2n, lead: 'O(edges² + vertices² + 1)' });
  assert.equal(sizeBadge(pathDegree({ fields: { size: field('9007199254740993 * 3^-1') } })), '1');
  assert.equal(sizeBadge(pathDegree({ fields: {} })), '1');
  assert.equal(sizeBadge(pathDegree({ fields: { size: field('num_vertices') } })), 'n');
  assert.equal(pathDegree({ fields: { known: field('num_vertices'), missing: field(null, 'unavailable') } }), null);
  assert.deepEqual(growth(field(null, 'unavailable')), { degree: null, lead: 'unavailable' });
  assert.equal(sizeBadge(null), '?');
});

test('degree ordering retains exact comparisons, ties and unavailable-last behavior', () => {
  assert.equal(compareDegrees(9007199254740993n, 9007199254740992n), 1);
  assert.equal(compareDegrees(9007199254740992n, 9007199254740993n), -1);
  assert.equal(compareDegrees(9007199254740993n, 9007199254740993n), 0);
  assert.equal(compareDegrees(null, 0n), 1);
  assert.equal(compareDegrees(0n, undefined), -1);
  assert.equal(compareDegrees(null, undefined), 0);
});


test('signed exact formulas have conservative bounds and exact zero stays zero', () => {
  // Complete graphs make this complement edge count zero, not quadratic.
  assert.deepEqual(growth(field('2^-1 * num_vertices^2 + -1 * 2^-1 * num_vertices + -1 * num_edges')),
    { degree: 2n, lead: 'O(vertices² + vertices + edges)' });
  assert.deepEqual(growth(field('0')), { degree: 0n, lead: '0' });
  assert.deepEqual(growth(field('7')), { degree: 0n, lead: 'O(1)' });
});

test('independent lower-degree parameters remain in size bounds', () => {
  // Edges may grow faster than vertices²; total degree alone cannot drop them.
  assert.deepEqual(growth(field('num_vertices^2 + num_edges')),
    { degree: 2n, lead: 'O(vertices² + edges)' });
});

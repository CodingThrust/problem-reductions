import { growth, compareDegrees } from './graph-growth.mts';

export interface GrowthField { certificate: string | null; accuracy: { kind: 'exact' | 'upper_bound' | 'unavailable' } }
export interface RuleEdge {
  source: number; target: number; witness: boolean; aggregate: boolean; turing: boolean;
  fields: Record<string, GrowthField>;
}
export type Scope = 'witness' | 'aggregate' | 'turing';
export type ClassId = 'constant' | 'linear' | 'quadratic' | 'cubic' | 'higher' | 'unknown';
export interface Link { source: number; target: number }

/* Polynomial total degree when every source parameter scales together: a visualization convention, not a runtime claim. */
export const CLASSES: readonly { id: ClassId; label: string; name: string }[] = [
  { id: 'constant', label: '1', name: 'Constant' },
  { id: 'linear', label: 'n', name: 'Linear' },
  { id: 'quadratic', label: 'n²', name: 'Quadratic' },
  { id: 'cubic', label: 'n³', name: 'Cubic' },
  { id: 'higher', label: 'n⁴⁺', name: 'Higher degree' },
  { id: 'unknown', label: '?', name: 'Undetermined' },
];
const order = new Map(CLASSES.map((c, i) => [c.id, i]));

export function growthClass(degree: bigint | null): ClassId {
  if (degree === null) return 'unknown';
  return degree >= 4n ? 'higher' : (['constant', 'linear', 'quadratic', 'cubic'] as const)[Number(degree)]!;
}

export interface EdgeGrowth {
  degree: bigint | null; cls: ClassId; accuracy: 'exact' | 'upper_bound';
  fields: { name: string; degree: bigint | null; lead: string; kind: GrowthField['accuracy']['kind'] }[];
}
const memo = new WeakMap<RuleEdge, EdgeGrowth>();
/* Overview policy: the highest field degree. One unavailable field leaves the whole edge undetermined. */
export function edgeGrowth(edge: RuleEdge): EdgeGrowth {
  const cached = memo.get(edge);
  if (cached) return cached;
  const fields = Object.entries(edge.fields).map(([name, f]) => ({ name, kind: f.accuracy.kind, ...growth(f) }));
  let degree: bigint | null = 0n;
  for (const f of fields) {
    if (f.degree === null) { degree = null; break; }
    if (compareDegrees(f.degree, degree) > 0) degree = f.degree;
  }
  const out = { degree, cls: growthClass(degree), fields,
    accuracy: fields.every(f => f.kind === 'exact') ? 'exact' as const : 'upper_bound' as const };
  memo.set(edge, out);
  return out;
}

/* Indices of the rules in scope whose class is at most `limit`. `skip` lists rules excluded by other filters. */
export function activeRules(rules: readonly RuleEdge[], { scope, limit, skip }: { scope: Scope; limit: ClassId; skip?: Set<number> }) {
  const max = order.get(limit)!;
  return rules.flatMap((rule, i) => rule[scope] && !skip?.has(i) && order.get(edgeGrowth(rule).cls)! <= max ? [i] : []);
}

/* Strongly connected components (iterative Tarjan); `comp[v]` indexes `groups`. */
export function components(n: number, links: readonly Link[]) {
  const out: number[][] = Array.from({ length: n }, () => []);
  for (const l of links) out[l.source]!.push(l.target);
  const index = new Array<number>(n).fill(-1), low = new Array<number>(n).fill(0), on = new Array<boolean>(n).fill(false);
  const comp = new Array<number>(n).fill(-1), groups: number[][] = [], stack: number[] = [];
  let counter = 0;
  for (let root = 0; root < n; root++) {
    if (index[root] !== -1) continue;
    const work: [number, number][] = [[root, 0]];
    while (work.length) {
      const frame = work[work.length - 1]!, v = frame[0];
      if (frame[1] === 0) { index[v] = low[v] = counter++; stack.push(v); on[v] = true; }
      if (frame[1] < out[v]!.length) {
        const w = out[v]![frame[1]++]!;
        if (index[w] === -1) work.push([w, 0]);
        else if (on[w]) low[v] = Math.min(low[v]!, index[w]!);
        continue;
      }
      work.pop();
      if (work.length) { const p = work[work.length - 1]![0]; low[p] = Math.min(low[p]!, low[v]!); }
      if (low[v] === index[v]) {
        const group: number[] = [];
        for (let w = -1; w !== v;) { w = stack.pop()!; on[w] = false; comp[w] = groups.length; group.push(w); }
        groups.push(group);
      }
    }
  }
  return { comp, groups };
}

/* Concrete variants reachable from `start` along (down) or towards (up) the active links, excluding `start` itself. */
export function reach(n: number, links: readonly Link[], start: number, direction: 'down' | 'up') {
  const next: number[][] = Array.from({ length: n }, () => []);
  for (const l of links) (direction === 'down' ? next[l.source]! : next[l.target]!).push(direction === 'down' ? l.target : l.source);
  const seen = new Set<number>([start]), queue = [start];
  for (let v = queue.pop(); v !== undefined; v = queue.pop()) for (const w of next[v]!) if (!seen.has(w)) { seen.add(w); queue.push(w); }
  seen.delete(start);
  return seen;
}

export interface FamilyRule { source: string; target: string; rules: number[]; cls: ClassId; worst: ClassId }
/* Family-level navigation edges. Each keeps the exact variant rules behind it; `cls` is its cheapest rule, `worst` its dearest. */
export function familyRules(nodes: readonly { name: string }[], rules: readonly RuleEdge[], active: readonly number[]) {
  const out = new Map<string, FamilyRule>();
  for (const i of active) {
    const rule = rules[i]!, s = nodes[rule.source]!.name, t = nodes[rule.target]!.name;
    if (s === t) continue;
    const cls = edgeGrowth(rule).cls, key = s + '→' + t, entry = out.get(key);
    if (!entry) out.set(key, { source: s, target: t, rules: [i], cls, worst: cls });
    else {
      entry.rules.push(i);
      if (order.get(cls)! < order.get(entry.cls)!) entry.cls = cls;
      if (order.get(cls)! > order.get(entry.worst)!) entry.worst = cls;
    }
  }
  return out;
}

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

/* Fewest reductions from any root family; null when no active rule chain leads there. */
export function depthsFrom(names: readonly string[], links: readonly { source: string; target: string }[], roots: readonly string[]) {
  const next = new Map<string, string[]>();
  for (const l of links) (next.get(l.source) ?? next.set(l.source, []).get(l.source)!).push(l.target);
  const depth = new Map<string, number>(roots.filter(r => names.includes(r)).map(r => [r, 0]));
  const queue = [...depth.keys()];
  for (let i = 0; i < queue.length; i++) for (const w of next.get(queue[i]!) ?? []) if (!depth.has(w)) { depth.set(w, depth.get(queue[i]!)! + 1); queue.push(w); }
  return new Map(names.map(n => [n, depth.get(n) ?? null] as const));
}

export interface Guide { x: number; y: number; text: string; count: number }
/* Columns by depth (wrapped into sub-columns when crowded, ordered by the mean row of their sources), then a grid for the unreachable. */
export function layeredLayout(names: readonly string[], depths: ReadonlyMap<string, number | null>,
  links: readonly { source: string; target: string }[], { column = 260, row = 26, perColumn = 22, rank = (n: string) => n, root = 'SAT' } = {}) {
  const positions = new Map<string, { x: number; y: number }>(), guides: Guide[] = [];
  const preds = new Map<string, string[]>();
  for (const l of links) (preds.get(l.target) ?? preds.set(l.target, []).get(l.target)!).push(l.source);
  const layers = new Map<number, string[]>(), lost: string[] = [];
  for (const n of names) {
    const d = depths.get(n) ?? null;
    if (d === null) lost.push(n); else (layers.get(d) ?? layers.set(d, []).get(d)!).push(n);
  }
  const rows = new Map<string, number>();
  const mean = (n: string) => {
    const ys = (preds.get(n) ?? []).filter(p => rows.has(p)).map(p => rows.get(p)!);
    return ys.length ? ys.reduce((a, b) => a + b, 0) / ys.length : Infinity;
  };
  let height = 0;
  [...layers.keys()].sort((a, b) => a - b).forEach(d => {
    const members = layers.get(d)!.sort((a, b) => mean(a) - mean(b) || rank(a).localeCompare(rank(b)));
    const subs = Math.ceil(members.length / perColumn), per = Math.ceil(members.length / subs), gap = Math.min(110, (column - 70) / subs);
    members.forEach((n, i) => {
      const p = { x: d * column + Math.floor(i / per) * gap, y: (i % per) * row };
      positions.set(n, p); rows.set(n, p.y); height = Math.max(height, p.y);
    });
    guides.push({ x: d * column, y: -56, text: d === 0 ? root : d === 1 ? '1 step' : `${d} steps`, count: members.length });
  });
  if (lost.length) {
    const width = Math.max(1, layers.size) * column, cell = 96, across = Math.max(6, Math.floor(width / cell)), top = height + 150;
    lost.sort((a, b) => rank(a).localeCompare(rank(b))).forEach((n, i) => positions.set(n, { x: (i % across) * cell, y: top + Math.floor(i / across) * row }));
    guides.push({ x: 0, y: top - 56, text: `Not reachable from ${root}`, count: lost.length });
  }
  return { positions, guides };
}

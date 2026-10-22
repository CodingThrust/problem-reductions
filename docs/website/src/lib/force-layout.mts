/* Deterministic force model for the problem graph, shared by the build (published map) and the page (live
   dragging and re-layout). It follows d3-force: link springs biased by degree, pairwise charge, collision, and a
   weak pull to the origin. Rules into a solver hub pull weakly and the hubs repel strongly, so the hundred-odd ILP
   encodings fan out instead of collapsing the map into a star. */

export interface Point { x: number; y: number }
export interface LayoutInput {
  ids: readonly string[];
  links: readonly (readonly [string, string])[];
  hubs: ReadonlySet<string>;
  radius: (id: string) => number;
  // Where to begin; missing ids start on d3's phyllotaxis spiral, so the result never depends on randomness.
  start?: ReadonlyMap<string, Point>;
  // Total ticks for a full cool-down from alpha 1.
  ticks?: number;
}
export interface Simulation {
  alpha: number;
  // Live interaction holds alpha near this target instead of letting it cool to zero.
  alphaTarget: number;
  tick(): void;
  // A pinned node stays where it is put and the rest of the graph responds to it.
  pin(id: string, p: Point | null): void;
  positions(): Map<string, Point>;
}
const ALPHA_MIN = 0.001;

export function createSimulation({ ids, links, hubs, radius, start, ticks = 300 }: LayoutInput): Simulation {
  const index = new Map(ids.map((id, i) => [id, i]));
  const nodes = ids.map((id, i) => {
    const p = start?.get(id), r = 10 * Math.sqrt(0.5 + i), a = i * Math.PI * (3 - Math.sqrt(5));
    return { id, x: p?.x ?? r * Math.cos(a), y: p?.y ?? r * Math.sin(a), vx: 0, vy: 0, r: radius(id), hub: hubs.has(id),
      degree: 0, fixed: null as Point | null };
  });
  const edges = links.flatMap(([a, b]) => {
    const s = index.get(a), t = index.get(b);
    return s === undefined || t === undefined || s === t ? [] : [{ s: nodes[s]!, t: nodes[t]!, hub: hubs.has(a) || hubs.has(b) }];
  });
  for (const { s, t } of edges) { s.degree++; t.degree++; }
  const decay = 1 - Math.pow(ALPHA_MIN, 1 / ticks), velocityDecay = 0.4;

  const sim: Simulation = {
    alpha: 1, alphaTarget: 0,
    tick() {
      const alpha = sim.alpha += (sim.alphaTarget - sim.alpha) * decay;
      for (const { s, t, hub } of edges) {
        let dx = t.x + t.vx - s.x - s.vx || 1e-6, dy = t.y + t.vy - s.y - s.vy || 1e-6;
        const len = Math.hypot(dx, dy), k = (len - (hub ? 120 : 32)) / len * alpha * (hub ? 0.04 : 0.5);
        dx *= k; dy *= k;
        const bias = s.degree / (s.degree + t.degree);
        t.vx -= dx * bias; t.vy -= dy * bias; s.vx += dx * (1 - bias); s.vy += dy * (1 - bias);
      }
      for (let i = 0; i < nodes.length; i++) {
        const a = nodes[i]!;
        for (let j = i + 1; j < nodes.length; j++) {
          const b = nodes[j]!, dx = b.x - a.x || 1e-6, dy = b.y - a.y || 1e-6, d2 = dx * dx + dy * dy;
          // Each node pushes the other with its own charge.
          const pa = (b.hub ? -600 : -34) * alpha / d2, pb = (a.hub ? -600 : -34) * alpha / d2;
          a.vx += dx * pa; a.vy += dy * pa; b.vx -= dx * pb; b.vy -= dy * pb;
          const min = a.r + b.r + 2;
          if (d2 < min * min) {
            const d = Math.sqrt(d2), push = (min - d) / d * 0.5;
            a.vx -= dx * push; a.vy -= dy * push; b.vx += dx * push; b.vy += dy * push;
          }
        }
      }
      for (const n of nodes) {
        if (n.fixed) { n.x = n.fixed.x; n.y = n.fixed.y; n.vx = n.vy = 0; continue; }
        n.vx -= n.x * (n.degree ? 0.03 : 0.08) * alpha;
        n.vy -= n.y * (n.degree ? 0.05 : 0.09) * alpha;
        n.x += n.vx *= 1 - velocityDecay; n.y += n.vy *= 1 - velocityDecay;
      }
    },
    pin(id, p) { const n = nodes[index.get(id) ?? -1]; if (n) n.fixed = p; },
    positions: () => new Map(nodes.map(n => [n.id, { x: n.x, y: n.y }])),
  };
  return sim;
}

/* Runs the model to rest: the build's published map, or a view laid out without waiting for frames. */
export function forceLayout(input: LayoutInput): Map<string, Point> {
  const sim = createSimulation(input);
  while (sim.alpha > ALPHA_MIN) sim.tick();
  return sim.positions();
}
export const settled = (sim: Simulation) => sim.alpha <= ALPHA_MIN && sim.alphaTarget === 0;

// Let input and painting run between short batches; canceled views must not apply stale positions.
export async function settleSimulation(sim: Simulation, signal: AbortSignal): Promise<boolean> {
  while (!settled(sim)) {
    await new Promise<void>(resolve => setTimeout(resolve, 0));
    if (signal.aborted) return false;
    const until = performance.now() + 4;
    do { sim.tick(); } while (!settled(sim) && performance.now() < until);
  }
  return !signal.aborted;
}

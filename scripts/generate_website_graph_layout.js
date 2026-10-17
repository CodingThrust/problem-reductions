const fs = require("node:fs");
const cytoscape = require("cytoscape");
cytoscape.use(require("cytoscape-fcose"));
const options = require("../docs/website/src/lib/graph-layout.js");
const { createReductionGraphModel } = require("../docs/src/static/reduction-graph.js");
const graph = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const model = createReductionGraphModel(graph);
const cy = cytoscape({
  headless: true, styleEnabled: true,
  elements: model.initialElements,
  style: [{ selector: "node", style: { width: 18, height: 18, label: "" } }],
  layout: { name: "preset" },
});
// fCoSE has no seed option; a fixed mulberry32 stream keeps published positions reproducible.
let seed = 0x9e3779b9;
Math.random = () => {
  seed = (seed + 0x6d2b79f5) | 0;
  let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
  t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
};
cy.layout({ ...options, randomize: true }).run();
const core = cy.nodes().filter((node) => node.neighborhood("node").length >= 2);
const bounds = core.boundingBox();
const positions = {};
for (const [name, id] of Object.entries(model.problemNodeIds)) {
  const { x, y } = cy.getElementById(id).position();
  if (!Number.isFinite(x) || !Number.isFinite(y)) throw new Error(`Invalid position: ${name}`);
  positions[name] = bounds.h > bounds.w ? { x: y, y: -x } : { x, y };
}
// Publish the same overlap-adjusted positions once, instead of recomputing on every visit.
const names = [...new Set(graph.nodes.map(node => node.name))];
const neighbors = new Map(names.map(name => [name, new Set()]));
for (const edge of graph.edges) {
  const a = graph.nodes[edge.source].name, b = graph.nodes[edge.target].name;
  if (a !== b) { neighbors.get(a).add(b); neighbors.get(b).add(a); }
}
const radii = names.map(name => Math.max(6, Math.min(22, 6 + 2.4 * Math.sqrt(neighbors.get(name).size))) / 2 + 9);
for (const p of Object.values(positions)) { p.x *= 1.6; p.y *= 1.6; }
for (let iteration = 0; iteration < 80; iteration++) {
  let moved = false;
  for (let i = 0; i < names.length; i++) for (let j = i + 1; j < names.length; j++) {
    const a = positions[names[i]], b = positions[names[j]];
    const dx = b.x - a.x, dy = b.y - a.y, distance = Math.hypot(dx, dy) || 0.01;
    const minimum = radii[i] + radii[j];
    if (distance < minimum) {
      const push = (minimum - distance) / 2 / distance;
      a.x -= dx * push; a.y -= dy * push; b.x += dx * push; b.y += dy * push;
      moved = true;
    }
  }
  if (!moved) break;
}
for (const p of Object.values(positions)) { p.x /= 1.6; p.y /= 1.6; }
process.stdout.write(JSON.stringify(positions));
cy.destroy();

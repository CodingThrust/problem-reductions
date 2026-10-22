const fs = require("node:fs");
const { forceLayout } = require("../docs/website/src/lib/force-layout.mts");

// The page draws families, so the layout works on the family graph: one node per problem name.
const graph = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const names = [...new Set(graph.nodes.map(node => node.name))];
const neighbors = new Map(names.map(name => [name, new Set()]));
const links = new Map();
for (const edge of graph.edges) {
  const a = graph.nodes[edge.source].name, b = graph.nodes[edge.target].name;
  if (a === b) continue;
  neighbors.get(a).add(b); neighbors.get(b).add(a);
  links.set(a + "→" + b, [a, b]);
}
// Same disc size as the page draws (graph.ts nodeSize); positions are published in page units / 1.6.
const radius = name => Math.max(6, Math.min(22, 6 + 2.4 * Math.sqrt(neighbors.get(name).size))) / 2;
const layout = forceLayout({ ids: names, links: [...links.values()], hubs: new Set(["ILP", "QUBO"]), radius });
// Landscape orientation suits the page; the force model itself has no preferred direction.
const core = names.filter(name => neighbors.get(name).size >= 2).map(name => layout.get(name));
const span = axis => Math.max(...core.map(p => p[axis])) - Math.min(...core.map(p => p[axis]));
const turn = span("y") > span("x");
const positions = {};
for (const name of names) {
  const { x, y } = layout.get(name);
  if (!Number.isFinite(x) || !Number.isFinite(y)) throw new Error(`Invalid position: ${name}`);
  positions[name] = turn ? { x: y / 1.6, y: -x / 1.6 } : { x: x / 1.6, y: y / 1.6 };
}
process.stdout.write(JSON.stringify(positions));

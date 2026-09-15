const fs = require("node:fs");
const cytoscape = require("cytoscape");
cytoscape.use(require("cytoscape-fcose"));
const options = require("../docs/website/assets/graph-layout.js");
const { createReductionGraphModel } = require("../docs/src/static/reduction-graph.js");
const graph = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const model = createReductionGraphModel(graph);
const cy = cytoscape({
  headless: true, styleEnabled: true,
  elements: model.initialElements,
  style: [{ selector: "node", style: { width: 18, height: 18, label: "" } }],
  layout: { name: "preset" },
});
cy.layout({ ...options, randomize: true }).run();
const core = cy.nodes().filter((node) => node.neighborhood("node").length >= 2);
const bounds = core.boundingBox();
const positions = {};
for (const [name, id] of Object.entries(model.problemNodeIds)) {
  const { x, y } = cy.getElementById(id).position();
  if (!Number.isFinite(x) || !Number.isFinite(y)) throw new Error(`Invalid position: ${name}`);
  positions[name] = bounds.h > bounds.w ? { x: y, y: -x } : { x, y };
}
process.stdout.write(JSON.stringify(positions));
cy.destroy();

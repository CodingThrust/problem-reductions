const assert = require("assert");
const fs = require("fs");

const {
  applyPrecomputedLayout,
  createReductionGraphModel,
  reductionGraphFingerprint,
} = require("../docs/src/static/reduction-graph.js");

const data = JSON.parse(
  fs.readFileSync("docs/src/reductions/reduction_graph.json", "utf8")
);

const model = createReductionGraphModel(data);
const fingerprint = reductionGraphFingerprint(model);
const layoutNodes = Object.fromEntries(
  model.initialElements
    .filter((e) => !e.data.source)
    .map((e, index) => [e.data.id, { x: index * 10, y: index * 20 }])
);

const initialNodes = model.initialElements.filter((e) => !e.data.source).length;
const initialEdges = model.initialElements.filter((e) => e.data.source).length;
const variantNodes = Object.values(model.variantNodesByParent).reduce(
  (sum, nodes) => sum + nodes.length,
  0
);

assert.strictEqual(initialNodes, model.problemCount);
assert(initialNodes < data.nodes.length, "initial graph should collapse variants");
assert.strictEqual(initialEdges, model.collapsedEdgeCount);
assert.strictEqual(variantNodes + initialNodes - model.parentCount, data.nodes.length);
assert.strictEqual(model.variantEdges.length, data.edges.length);
assert(
  model.initialElements.length < data.nodes.length + data.edges.length,
  "initial layout should not include hidden variant elements"
);

const positioned = applyPrecomputedLayout(model.initialElements, {
  fingerprint,
  nodes: layoutNodes,
});

assert.deepStrictEqual(positioned[0].position, layoutNodes[model.initialElements[0].data.id]);
assert.strictEqual(model.initialElements[0].position, undefined);

const fallback = applyPrecomputedLayout(model.initialElements, {
  fingerprint: "stale",
  nodes: {
    [model.initialElements[0].data.id]: { x: 56, y: 78 },
  },
});
assert.strictEqual(fallback[0].position, undefined);

const incomplete = applyPrecomputedLayout(model.initialElements, {
  fingerprint,
  nodes: {
    [model.initialElements[0].data.id]: { x: 56, y: 78 },
  },
});
assert.strictEqual(incomplete[0].position, undefined);

console.log(
  `initial=${model.initialElements.length}, variants=${variantNodes + model.variantEdges.length}`
);

#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

const ELK = require("elkjs/lib/elk.bundled.js");

const repoRoot = path.resolve(__dirname, "..");
const {
  createReductionGraphModel,
  reductionGraphFingerprint,
} = require(path.join(repoRoot, "docs/src/static/reduction-graph.js"));

const inputPath = path.resolve(
  process.cwd(),
  process.argv[2] || "docs/src/reductions/reduction_graph.json"
);
const outputPath = path.resolve(
  process.cwd(),
  process.argv[3] || "docs/src/reductions/reduction_graph_layout.json"
);

const STRESS_PARAMS = {
  desiredEdgeLength: 200,
  nodeNodeSpacing: 40,
};

function roundPosition(value) {
  return Math.round(value * 1000) / 1000;
}

function nodeSize(element) {
  const label = element.data.label || "";
  if (element.data.isParent) {
    return {
      width: Math.max(label.length * 6.5 + 16, 60),
      height: 28,
    };
  }
  return {
    width: Math.max(label.length * 6.5 + 10, 50),
    height: 24,
  };
}

function buildElkGraph(model) {
  const nodeElements = model.initialElements.filter((element) => !element.data.source);
  const edgeElements = model.initialElements.filter((element) => element.data.source);

  return {
    id: "reduction-graph",
    layoutOptions: {
      "elk.algorithm": "stress",
      "elk.stress.desiredEdgeLength": String(STRESS_PARAMS.desiredEdgeLength),
      "elk.spacing.nodeNode": String(STRESS_PARAMS.nodeNodeSpacing),
      // cytoscape-elk accepts this shorter key; keep it for parity with the old browser config.
      "nodeNode.spacing": String(STRESS_PARAMS.nodeNodeSpacing),
    },
    children: nodeElements.map((element) => {
      const size = nodeSize(element);
      return {
        id: element.data.id,
        width: size.width,
        height: size.height,
      };
    }),
    edges: edgeElements.map((element) => ({
      id: element.data.id,
      sources: [element.data.source],
      targets: [element.data.target],
    })),
  };
}

async function main() {
  const data = JSON.parse(fs.readFileSync(inputPath, "utf8"));
  const model = createReductionGraphModel(data);
  const elk = new ELK();
  const elkGraph = await elk.layout(buildElkGraph(model));
  const nodes = {};

  for (const node of elkGraph.children || []) {
    if (!Number.isFinite(node.x) || !Number.isFinite(node.y)) {
      throw new Error(`ELK produced an invalid position for ${node.id}`);
    }
    nodes[node.id] = {
      x: roundPosition(node.x),
      y: roundPosition(node.y),
    };
  }

  const layout = {
    version: 1,
    generator: "scripts/generate_reduction_graph_layout.js",
    algorithm: "elk-stress",
    elkVersion: require("elkjs/package.json").version,
    params: STRESS_PARAMS,
    fingerprint: reductionGraphFingerprint(model),
    nodeCount: Object.keys(nodes).length,
    edgeCount: model.collapsedEdgeCount,
    nodes,
  };

  fs.mkdirSync(path.dirname(outputPath), { recursive: true });
  fs.writeFileSync(outputPath, `${JSON.stringify(layout, null, 2)}\n`);
  console.log(`Wrote ${path.relative(repoRoot, outputPath)} (${layout.nodeCount} nodes)`);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});

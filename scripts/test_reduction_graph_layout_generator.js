const assert = require("assert");
const childProcess = require("child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");

const {
  createReductionGraphModel,
  reductionGraphFingerprint,
} = require("../docs/src/static/reduction-graph.js");

const dataPath = "docs/src/reductions/reduction_graph.json";
const data = JSON.parse(fs.readFileSync(dataPath, "utf8"));
const model = createReductionGraphModel(data);
const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "reduction-graph-layout-"));
const outputPath = path.join(tmpDir, "layout.json");

childProcess.execFileSync(
  process.execPath,
  ["scripts/generate_reduction_graph_layout.js", dataPath, outputPath],
  { stdio: "pipe" }
);

const layout = JSON.parse(fs.readFileSync(outputPath, "utf8"));
const nodeIds = model.initialElements
  .filter((element) => !element.data.source)
  .map((element) => element.data.id);

assert.strictEqual(layout.algorithm, "elk-stress");
assert.strictEqual(layout.fingerprint, reductionGraphFingerprint(model));
assert.strictEqual(layout.nodeCount, nodeIds.length);

for (const id of nodeIds) {
  assert(layout.nodes[id], `missing position for ${id}`);
  assert(Number.isFinite(layout.nodes[id].x), `invalid x for ${id}`);
  assert(Number.isFinite(layout.nodes[id].y), `invalid y for ${id}`);
}

console.log(`generated ELK stress layout for ${layout.nodeCount} nodes`);

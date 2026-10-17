const reductionLayoutOptions = {
  name: "fcose", quality: "proof", randomize: false,
  animate: false, fit: false, packComponents: false, tile: true,
  nodeDimensionsIncludeLabels: true,
  nodeRepulsion: () => 6500,
  idealEdgeLength: (edge) => edge.source().isChild() || edge.target().isChild() ? 200 : 95,
  edgeElasticity: (edge) => 0.45 * Math.sqrt(Math.max(
    edge.source().degree(), edge.target().degree(),
  )),
  gravity: 0.12, numIter: 1200,
};

if (typeof module !== "undefined") module.exports = reductionLayoutOptions;

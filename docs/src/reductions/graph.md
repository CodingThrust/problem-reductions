# Reduction Graph

This interactive diagram shows the reduction relationships between NP-hard problems
implemented in this library. Click two nodes to find the shortest reduction path between them.

<div id="controls" style="margin-bottom: 10px; display: flex; align-items: center; gap: 12px; flex-wrap: wrap;">
  <span style="font-size: 14px; color: #555;">Click two nodes to highlight a reduction path.</span>
  <button id="clear-btn" style="padding: 4px 14px; font-size: 13px; cursor: pointer; border: 1px solid #aaa; border-radius: 4px; background: #f5f5f5;">Clear Path</button>
  <span id="legend" style="font-size: 13px; display: flex; gap: 10px; flex-wrap: wrap;">
    <span><span style="display:inline-block;width:12px;height:12px;background:#c8f0c8;border:1px solid #888;border-radius:2px;vertical-align:middle;"></span> Graph</span>
    <span><span style="display:inline-block;width:12px;height:12px;background:#f0c8c8;border:1px solid #888;border-radius:2px;vertical-align:middle;"></span> Set</span>
    <span><span style="display:inline-block;width:12px;height:12px;background:#f0f0a0;border:1px solid #888;border-radius:2px;vertical-align:middle;"></span> Optimization</span>
    <span><span style="display:inline-block;width:12px;height:12px;background:#c8c8f0;border:1px solid #888;border-radius:2px;vertical-align:middle;"></span> Satisfiability</span>
    <span><span style="display:inline-block;width:12px;height:12px;background:#f0c8e0;border:1px solid #888;border-radius:2px;vertical-align:middle;"></span> Specialized</span>
  </span>
</div>

<div id="cy" style="width: 100%; height: 600px; border: 1px solid #ccc; background: #fafafa;"></div>

<div id="tooltip" style="display:none; position:fixed; padding:6px 12px; background:#333; color:#fff; border-radius:4px; font-size:13px; pointer-events:none; z-index:1000; white-space:nowrap;"></div>

<script src="https://unpkg.com/cytoscape@3/dist/cytoscape.min.js"></script>

<script>
(function() {
  var CATEGORY_COLORS = {
    graph: '#c8f0c8',
    set: '#f0c8c8',
    optimization: '#f0f0a0',
    satisfiability: '#c8c8f0',
    specialized: '#f0c8e0'
  };

  var CATEGORY_BORDER = {
    graph: '#5a9e5a',
    set: '#b06060',
    optimization: '#a0a040',
    satisfiability: '#6060b0',
    specialized: '#b060a0'
  };

  // Insert readable spaces before capitals: "MaximumIndependentSet" -> "Maximum Independent Set"
  function displayName(name) {
    return name.replace(/([a-z])([A-Z])/g, '$1 $2').replace(/([A-Z]+)([A-Z][a-z])/g, '$1 $2');
  }

  fetch('reduction_graph.json')
    .then(function(r) { return r.json(); })
    .then(function(data) { buildGraph(data); })
    .catch(function(err) {
      document.getElementById('cy').innerHTML =
        '<p style="padding:20px;color:#c00;">Failed to load reduction_graph.json: ' + err + '</p>';
    });

  function buildGraph(data) {
    // 1. Filter base nodes (variant is empty object)
    var baseNodes = data.nodes.filter(function(n) {
      return Object.keys(n.variant).length === 0;
    });

    // 2. Build node elements (deduplicate by name)
    var seenNodes = {};
    var nodeElements = baseNodes.filter(function(n) {
      if (seenNodes[n.name]) return false;
      seenNodes[n.name] = true;
      return true;
    }).map(function(n) {
      return {
        data: {
          id: n.name,
          label: displayName(n.name),
          category: n.category
        }
      };
    });

    // 3. Deduplicate edges by base name pair, detect bidirectionality
    //    We collect unique directed edges between base problem names.
    //    If any variant edge is bidirectional, the base edge is bidirectional.
    var edgeMap = {}; // key: "source->target"
    data.edges.forEach(function(e) {
      var src = e.source.name;
      var tgt = e.target.name;
      if (src === tgt) return; // skip self-loops

      var fwdKey = src + '->' + tgt;
      var revKey = tgt + '->' + src;

      if (!edgeMap[fwdKey]) {
        edgeMap[fwdKey] = { source: src, target: tgt, bidirectional: false };
      }
      if (e.bidirectional) {
        edgeMap[fwdKey].bidirectional = true;
        // Also ensure the reverse direction exists for bidirectional edges
        if (!edgeMap[revKey]) {
          edgeMap[revKey] = { source: tgt, target: src, bidirectional: true };
        } else {
          edgeMap[revKey].bidirectional = true;
        }
      }
    });

    // 4. Build edge elements
    //    For visual display: bidirectional pairs become one edge with arrows on both ends.
    //    For path-finding: we keep separate directed edges in edgeMap (used by Dijkstra).
    //    Visual edges use a separate list to avoid duplicate display.
    var edgeElements = [];
    var visualProcessed = {};
    Object.keys(edgeMap).forEach(function(key) {
      var e = edgeMap[key];
      var pairKey = [e.source, e.target].sort().join('--');

      if (e.bidirectional) {
        // Only add one visual edge per bidirectional pair
        if (visualProcessed[pairKey]) return;
        visualProcessed[pairKey] = true;
        edgeElements.push({
          data: {
            id: 'e-' + e.source + '-' + e.target,
            source: e.source,
            target: e.target,
            bidirectional: true
          }
        });
      } else {
        edgeElements.push({
          data: {
            id: 'e-' + e.source + '-' + e.target,
            source: e.source,
            target: e.target,
            bidirectional: false
          }
        });
      }
    });

    // 4b. Add hidden reverse edges for bidirectional pairs so directed
    //     Dijkstra can traverse them in both directions.
    var biPairs = {};
    Object.keys(edgeMap).forEach(function(key) {
      var e = edgeMap[key];
      if (!e.bidirectional) return;
      var pairKey = [e.source, e.target].sort().join('--');
      if (biPairs[pairKey]) return;
      biPairs[pairKey] = true;
      // The visual edge goes source->target. Add a hidden reverse edge.
      edgeElements.push({
        data: {
          id: 'e-rev-' + e.target + '-' + e.source,
          source: e.target,
          target: e.source,
          bidirectional: true,
          hidden: true
        },
        classes: 'hidden-edge'
      });
    });

    // 5. Create Cytoscape instance
    var cy = cytoscape({
      container: document.getElementById('cy'),
      elements: nodeElements.concat(edgeElements),
      style: [
        {
          selector: 'node',
          style: {
            'label': 'data(label)',
            'text-valign': 'center',
            'text-halign': 'center',
            'font-size': '11px',
            'font-family': 'sans-serif',
            'width': 'label',
            'height': 'label',
            'padding': '10px',
            'shape': 'round-rectangle',
            'background-color': function(ele) {
              return CATEGORY_COLORS[ele.data('category')] || '#ddd';
            },
            'border-width': 2,
            'border-color': function(ele) {
              return CATEGORY_BORDER[ele.data('category')] || '#999';
            },
            'text-wrap': 'wrap',
            'text-max-width': '120px',
            'color': '#222'
          }
        },
        {
          selector: 'edge',
          style: {
            'width': 2,
            'line-color': '#999',
            'curve-style': 'bezier',
            'target-arrow-shape': 'triangle',
            'target-arrow-color': '#999',
            'arrow-scale': 1.2
          }
        },
        {
          selector: 'edge[?bidirectional]',
          style: {
            'source-arrow-shape': 'triangle',
            'source-arrow-color': '#999'
          }
        },
        {
          selector: '.highlighted',
          style: {
            'background-color': '#ff9900',
            'border-color': '#cc6600',
            'border-width': 3,
            'color': '#000',
            'z-index': 10
          }
        },
        {
          selector: 'edge.highlighted',
          style: {
            'line-color': '#ff6600',
            'target-arrow-color': '#ff6600',
            'source-arrow-color': '#ff6600',
            'width': 4,
            'z-index': 10
          }
        },
        {
          selector: '.dimmed',
          style: {
            'opacity': 0.25
          }
        },
        {
          selector: '.hidden-edge',
          style: {
            'visibility': 'hidden',
            'width': 0
          }
        }
      ],
      layout: {
        name: 'cose',
        animate: false,
        nodeDimensionsIncludeLabels: true,
        nodeRepulsion: function() { return 8000; },
        idealEdgeLength: function() { return 120; },
        edgeElasticity: function() { return 200; },
        gravity: 0.3,
        numIter: 1000,
        padding: 30
      },
      minZoom: 0.3,
      maxZoom: 3,
      wheelSensitivity: 0.3
    });

    // 6. Tooltip on hover
    var tooltip = document.getElementById('tooltip');

    cy.on('mouseover', 'node', function(evt) {
      var node = evt.target;
      tooltip.textContent = node.data('label');
      tooltip.style.display = 'block';
    });

    cy.on('mousemove', 'node', function(evt) {
      var pos = evt.originalEvent;
      tooltip.style.left = (pos.clientX + 12) + 'px';
      tooltip.style.top = (pos.clientY + 12) + 'px';
    });

    cy.on('mouseout', 'node', function() {
      tooltip.style.display = 'none';
    });

    // 7. Two-click path highlighting
    var selected = [];

    cy.on('tap', 'node', function(evt) {
      var node = evt.target;

      if (selected.length === 0) {
        selected.push(node);
        node.addClass('highlighted');
      } else if (selected.length === 1) {
        if (selected[0].id() === node.id()) {
          // Clicked same node twice - deselect
          clearPath();
          return;
        }
        selected.push(node);
        node.addClass('highlighted');
        findAndHighlightPath(selected[0], selected[1]);
      } else {
        // Reset and start over
        clearPath();
        selected.push(node);
        node.addClass('highlighted');
      }
    });

    // Tap on background clears selection
    cy.on('tap', function(evt) {
      if (evt.target === cy) {
        clearPath();
      }
    });

    function findAndHighlightPath(src, tgt) {
      // Use Dijkstra to find shortest directed path.
      // Hidden reverse edges ensure bidirectional reductions are traversable
      // in both directions while keeping the graph properly directed.
      var dijkstra = cy.elements().dijkstra({
        root: src,
        weight: function() { return 1; },
        directed: true
      });

      var pathToTarget = dijkstra.pathTo(tgt);

      if (pathToTarget.length > 0) {
        // Dim everything, then highlight the path.
        // For hidden reverse edges in the path, highlight the visible
        // bidirectional edge instead.
        cy.elements().addClass('dimmed');
        pathToTarget.forEach(function(ele) {
          if (ele.isEdge() && ele.hasClass('hidden-edge')) {
            // Find the visible forward edge for this bidirectional pair
            var visId = 'e-' + ele.data('target') + '-' + ele.data('source');
            var visEdge = cy.getElementById(visId);
            if (visEdge.length > 0) {
              visEdge.removeClass('dimmed').addClass('highlighted');
            }
          } else {
            ele.removeClass('dimmed').addClass('highlighted');
          }
        });
      } else {
        // No path exists — show non-blocking message in controls bar
        selected.forEach(function(n) { n.removeClass('highlighted'); });
        selected = [];
        var msg = document.createElement('span');
        msg.textContent = 'No reduction path found.';
        msg.style.cssText = 'color:#c00;font-size:13px;margin-left:8px;';
        document.getElementById('controls').appendChild(msg);
        setTimeout(function() { msg.remove(); }, 3000);
      }
    }

    function clearPath() {
      cy.elements().removeClass('highlighted').removeClass('dimmed');
      selected = [];
    }

    document.getElementById('clear-btn').addEventListener('click', function() {
      clearPath();
    });
  }
})();
</script>

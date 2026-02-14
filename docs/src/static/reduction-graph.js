document.addEventListener('DOMContentLoaded', function() {
  // Check if the cy container exists on this page
  var cyContainer = document.getElementById('cy');
  if (!cyContainer) return;

  var categoryColors = {
    graph: '#c8f0c8', set: '#f0c8c8', optimization: '#f0f0a0',
    satisfiability: '#c8c8f0', specialized: '#f0c8e0'
  };
  var categoryBorders = {
    graph: '#4a8c4a', set: '#8c4a4a', optimization: '#8c8c4a',
    satisfiability: '#4a4a8c', specialized: '#8c4a6a'
  };

  function variantId(name, variant) {
    var keys = Object.keys(variant).sort();
    return name + '/' + keys.map(function(k) { return k + '=' + variant[k]; }).join(',');
  }

  // Default values per variant key — omitted in concise labels
  var variantDefaults = { graph: 'SimpleGraph', weight: 'One' };

  function variantLabel(variant) {
    var keys = Object.keys(variant);
    var parts = [];
    keys.forEach(function(k) {
      var v = variant[k];
      if (variantDefaults[k] && v === variantDefaults[k]) return; // skip defaults
      parts.push(k === 'graph' || k === 'weight' ? v : k + '=' + v);
    });
    return parts.length > 0 ? parts.join(', ') : 'base';
  }

  function fullVariantLabel(variant) {
    var keys = Object.keys(variant);
    if (keys.length === 0) return 'no parameters';
    var parts = [];
    keys.forEach(function(k) {
      parts.push(k === 'graph' || k === 'weight' ? variant[k] : k + '=' + variant[k]);
    });
    return parts.join(', ');
  }

  function isBaseVariant(variant) {
    var keys = Object.keys(variant);
    return keys.every(function(k) {
      return variantDefaults[k] && variant[k] === variantDefaults[k];
    });
  }

  fetch('reductions/reduction_graph.json')
    .then(function(r) { if (!r.ok) throw new Error('HTTP ' + r.status); return r.json(); })
    .then(function(data) {
      // Group all nodes by problem name
      var problems = {};
      data.nodes.forEach(function(n) {
        if (!problems[n.name]) {
          problems[n.name] = { category: n.category, doc_path: n.doc_path, children: [] };
        }
        // Only track nodes with non-empty variants as children;
        // empty-variant nodes are base placeholders
        if (n.variant && Object.keys(n.variant).length > 0) {
          problems[n.name].children.push(n);
        }
      });

      // Build edges at variant level, detecting bidirectional pairs
      var edgeMap = {};
      data.edges.forEach(function(e) {
        var src = data.nodes[e.source];
        var dst = data.nodes[e.target];
        var srcId = variantId(src.name, src.variant);
        var dstId = variantId(dst.name, dst.variant);
        var fwd = srcId + '->' + dstId;
        var rev = dstId + '->' + srcId;
        if (edgeMap[rev]) { edgeMap[rev].bidirectional = true; }
        else if (!edgeMap[fwd]) {
          edgeMap[fwd] = { source: srcId, target: dstId, bidirectional: false, overhead: e.overhead || [], doc_path: e.doc_path || '' };
        }
      });

      // Precompute per-problem base/non-base split
      var problemNames = Object.keys(problems);
      var problemInfo = {};
      problemNames.forEach(function(name) {
        var info = problems[name];
        var baseChild = null, nonBase = [];
        info.children.forEach(function(child) {
          if (isBaseVariant(child.variant)) baseChild = child;
          else nonBase.push(child);
        });
        problemInfo[name] = { baseChild: baseChild, nonBase: nonBase };
      });

      // ── Step 1: Layout one node per problem using cose ──
      var tempElements = [];
      problemNames.forEach(function(name) {
        var info = problems[name];
        tempElements.push({
          data: { id: name, label: name, category: info.category }
        });
      });
      var tempEdgeSet = {};
      data.edges.forEach(function(e) {
        var srcName = data.nodes[e.source].name;
        var dstName = data.nodes[e.target].name;
        var key = srcName + '->' + dstName;
        var rev = dstName + '->' + srcName;
        if (!tempEdgeSet[key] && !tempEdgeSet[rev]) {
          tempEdgeSet[key] = true;
          tempElements.push({ data: { id: 'te_' + key, source: srcName, target: dstName } });
        }
      });

      var tempCy = cytoscape({
        container: document.getElementById('cy'),
        elements: tempElements,
        style: [
          { selector: 'node', style: {
            'label': 'data(label)', 'text-valign': 'center', 'text-halign': 'center',
            'font-size': '11px', 'font-family': 'monospace',
            'width': function(ele) { return Math.max(ele.data('label').length * 7 + 14, 60); },
            'height': 28, 'shape': 'round-rectangle'
          }},
          { selector: 'edge', style: { 'width': 1, 'line-color': '#ddd', 'target-arrow-shape': 'none' } }
        ],
        layout: {
          name: 'cose', animate: false,
          nodeRepulsion: function() { return 16000; },
          idealEdgeLength: function() { return 200; },
          gravity: 0.15, numIter: 1000, padding: 40
        }
      });

      var positions = {};
      tempCy.nodes().forEach(function(n) {
        positions[n.id()] = { x: n.position('x'), y: n.position('y') };
      });
      tempCy.destroy();

      // ── Step 2: Place flat variant nodes near parent positions ──
      var elements = [];
      var variantOffsetY = 30;

      problemNames.forEach(function(name) {
        var info = problems[name];
        var pi = problemInfo[name];
        var pos = positions[name];

        if (info.children.length === 0) {
          // No parameterized variants — single node with empty variant
          var vid = variantId(name, {});
          elements.push({
            data: { id: vid, label: name, fullLabel: name + ' (no parameters)', category: info.category, doc_path: info.doc_path },
            position: { x: pos.x, y: pos.y }
          });
        } else if (pi.baseChild) {
          // Base variant at parent position, labeled with problem name
          var baseId = variantId(name, pi.baseChild.variant);
          elements.push({
            data: { id: baseId, label: name, fullLabel: name + ' (' + fullVariantLabel(pi.baseChild.variant) + ')', category: info.category, doc_path: info.doc_path },
            position: { x: pos.x, y: pos.y }
          });
          // Non-base variants placed below
          pi.nonBase.forEach(function(child, i) {
            var vid = variantId(name, child.variant);
            var vl = variantLabel(child.variant);
            elements.push({
              data: { id: vid, label: name + ' (' + vl + ')', fullLabel: name + ' (' + fullVariantLabel(child.variant) + ')', category: child.category, doc_path: child.doc_path },
              position: { x: pos.x, y: pos.y + (i + 1) * variantOffsetY }
            });
          });
        } else if (pi.nonBase.length === 1) {
          // Single non-base variant — place at parent position with just problem name
          var child = pi.nonBase[0];
          var vid = variantId(name, child.variant);
          elements.push({
            data: { id: vid, label: name, fullLabel: name + ' (' + fullVariantLabel(child.variant) + ')', category: child.category, doc_path: child.doc_path },
            position: { x: pos.x, y: pos.y }
          });
        } else {
          // Multiple non-base variants, no base — first at parent, rest below
          pi.nonBase.forEach(function(child, i) {
            var vid = variantId(name, child.variant);
            var vl = variantLabel(child.variant);
            elements.push({
              data: { id: vid, label: name + ' (' + vl + ')', fullLabel: name + ' (' + fullVariantLabel(child.variant) + ')', category: child.category, doc_path: child.doc_path },
              position: { x: pos.x, y: pos.y + i * variantOffsetY }
            });
          });
        }
      });

      // ── Step 3: Connect edges ──
      Object.keys(edgeMap).forEach(function(k) {
        var e = edgeMap[k];
        elements.push({
          data: {
            id: k, source: e.source, target: e.target,
            bidirectional: e.bidirectional, overhead: e.overhead, doc_path: e.doc_path
          }
        });
      });

      var cy = cytoscape({
        container: document.getElementById('cy'),
        elements: elements,
        style: [
          { selector: 'node', style: {
            'label': 'data(label)', 'text-valign': 'center', 'text-halign': 'center',
            'font-size': '10px', 'font-family': 'monospace',
            'width': function(ele) { return Math.max(ele.data('label').length * 6.5 + 10, 50); },
            'height': 24, 'shape': 'round-rectangle',
            'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
            'border-width': 1,
            'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
            'text-wrap': 'none', 'cursor': 'pointer'
          }},
          { selector: 'edge', style: {
            'width': 1.5, 'line-color': '#999', 'target-arrow-color': '#999', 'target-arrow-shape': 'triangle',
            'source-arrow-color': '#999',
            'source-arrow-shape': function(ele) { return ele.data('bidirectional') ? 'triangle' : 'none'; },
            'curve-style': 'bezier', 'arrow-scale': 0.7, 'cursor': 'pointer'
          }},
          { selector: '.highlighted', style: {
            'background-color': '#ff6b6b', 'border-color': '#cc0000', 'border-width': 2, 'z-index': 10
          }},
          { selector: 'edge.highlighted', style: {
            'line-color': '#ff4444', 'target-arrow-color': '#ff4444', 'source-arrow-color': '#ff4444', 'width': 3, 'z-index': 10
          }},
          { selector: '.selected-node', style: {
            'border-color': '#0066cc', 'border-width': 2, 'background-color': '#cce0ff'
          }}
        ],
        layout: { name: 'preset' },
        userZoomingEnabled: true, userPanningEnabled: true, boxSelectionEnabled: false
      });

      // Tooltip for nodes
      var tooltip = document.getElementById('cy-tooltip');
      cy.on('mouseover', 'node', function(evt) {
        var d = evt.target.data();
        tooltip.innerHTML = '<strong>' + d.fullLabel + '</strong><br><em>Double-click to view API docs</em>';
        tooltip.style.display = 'block';
      });
      cy.on('mousemove', 'node', function(evt) {
        var pos = evt.renderedPosition || evt.position;
        var rect = document.getElementById('cy').getBoundingClientRect();
        tooltip.style.left = (rect.left + window.scrollX + pos.x + 15) + 'px';
        tooltip.style.top = (rect.top + window.scrollY + pos.y - 10) + 'px';
      });
      cy.on('mouseout', 'node', function() { tooltip.style.display = 'none'; });

      // Edge tooltip
      cy.on('mouseover', 'edge', function(evt) {
        var d = evt.target.data();
        var arrow = d.bidirectional ? ' \u2194 ' : ' \u2192 ';
        var html = '<strong>' + evt.target.source().data('label') + arrow + evt.target.target().data('label') + '</strong>';
        if (d.overhead && d.overhead.length > 0) {
          html += '<br>' + d.overhead.map(function(o) { return '<code>' + o.field + '</code> = <code>' + o.formula + '</code>'; }).join('<br>');
        }
        html += '<br><em>Click to highlight, double-click for source code</em>';
        tooltip.innerHTML = html;
        tooltip.style.display = 'block';
      });
      cy.on('mousemove', 'edge', function(evt) {
        var pos = evt.renderedPosition || evt.position;
        var rect = document.getElementById('cy').getBoundingClientRect();
        tooltip.style.left = (rect.left + window.scrollX + pos.x + 15) + 'px';
        tooltip.style.top = (rect.top + window.scrollY + pos.y - 10) + 'px';
      });
      cy.on('mouseout', 'edge', function() { tooltip.style.display = 'none'; });

      // Double-click node → rustdoc API page
      cy.on('dbltap', 'node', function(evt) {
        var d = evt.target.data();
        if (d.doc_path) {
          window.location.href = 'api/problemreductions/' + d.doc_path;
        }
      });
      // Double-click edge → GitHub source code
      cy.on('dbltap', 'edge', function(evt) {
        var d = evt.target.data();
        if (d.doc_path) {
          var module = d.doc_path.replace('/index.html', '');
          window.open('https://github.com/CodingThrust/problem-reductions/blob/main/src/' + module + '.rs', '_blank');
        }
      });

      // Single-click path selection
      var selectedNode = null;
      var instructions = document.getElementById('instructions');
      var clearBtn = document.getElementById('clear-btn');

      function clearPath() {
        cy.elements().removeClass('highlighted selected-node');
        selectedNode = null;
        instructions.textContent = 'Click a node to start path selection';
        clearBtn.style.display = 'none';
      }

      clearBtn.addEventListener('click', clearPath);

      cy.on('tap', 'node', function(evt) {
        var node = evt.target;
        if (!selectedNode) {
          selectedNode = node;
          node.addClass('selected-node');
          instructions.textContent = 'Now click a target node to find path from ' + node.data('label');
        } else if (node === selectedNode) {
          clearPath();
        } else {
          var dijkstra = cy.elements().dijkstra({ root: selectedNode, directed: true });
          var path = dijkstra.pathTo(node);
          cy.elements().removeClass('highlighted selected-node');
          if (path && path.length > 0) {
            path.addClass('highlighted');
            instructions.textContent = 'Path: ' + path.nodes().map(function(n) { return n.data('label'); }).join(' \u2192 ');
          } else {
            instructions.textContent = 'No path from ' + selectedNode.data('label') + ' to ' + node.data('label');
          }
          clearBtn.style.display = 'inline';
          selectedNode = null;
        }
      });

      cy.on('tap', 'edge', function(evt) {
        var edge = evt.target;
        var d = edge.data();
        cy.elements().removeClass('highlighted selected-node');
        edge.addClass('highlighted');
        edge.source().addClass('highlighted');
        edge.target().addClass('highlighted');
        var arrow = d.bidirectional ? ' \u2194 ' : ' \u2192 ';
        var text = edge.source().data('label') + arrow + edge.target().data('label');
        if (d.overhead && d.overhead.length > 0) {
          text += '  |  ' + d.overhead.map(function(o) { return o.field + ' = ' + o.formula; }).join(', ');
        }
        instructions.textContent = text;
        clearBtn.style.display = 'inline';
        selectedNode = null;
      });

      cy.on('tap', function(evt) { if (evt.target === cy) { clearPath(); } });
    })
    .catch(function(err) {
      document.getElementById('cy').innerHTML = '<p style="padding:1em;color:#c00;">Failed to load reduction graph: ' + err.message + '</p>';
    });
});

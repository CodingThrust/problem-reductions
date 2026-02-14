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

  fetch('reductions/reduction_graph.json')
    .then(function(r) { if (!r.ok) throw new Error('HTTP ' + r.status); return r.json(); })
    .then(function(data) {
      // Group nodes by problem name
      var problems = {};
      data.nodes.forEach(function(n, idx) {
        if (!problems[n.name]) {
          problems[n.name] = { category: n.category, doc_path: n.doc_path, variants: [] };
        }
        problems[n.name].variants.push({ index: idx, variant: n.variant, category: n.category, doc_path: n.doc_path });
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

      // ── Build compound nodes ──
      var elements = [];
      var parentIds = {};  // name → parent node id

      Object.keys(problems).forEach(function(name) {
        var info = problems[name];
        var hasMultipleVariants = info.variants.length > 1;

        if (hasMultipleVariants) {
          // Create compound parent node
          var parentId = 'parent_' + name;
          parentIds[name] = parentId;
          elements.push({
            data: {
              id: parentId,
              label: name,
              category: info.category,
              doc_path: info.doc_path,
              isParent: true,
              variantCount: info.variants.length
            }
          });

          // Create child nodes (hidden initially — collapsed)
          info.variants.forEach(function(v) {
            var vid = variantId(name, v.variant);
            elements.push({
              data: {
                id: vid,
                parent: parentId,
                label: variantLabel(v.variant),
                fullLabel: name + ' (' + fullVariantLabel(v.variant) + ')',
                category: v.category,
                doc_path: v.doc_path,
                isVariant: true,
                problemName: name
              }
            });
          });
        } else {
          // Single variant — simple node (no parent)
          var v = info.variants[0];
          var vid = variantId(name, v.variant);
          elements.push({
            data: {
              id: vid,
              label: name,
              fullLabel: name + ' (' + fullVariantLabel(v.variant) + ')',
              category: v.category,
              doc_path: v.doc_path,
              isVariant: false,
              problemName: name
            }
          });
        }
      });

      // ── Build collapsed-mode edges (name-level) ──
      var nameLevelEdges = {};
      data.edges.forEach(function(e) {
        var srcName = data.nodes[e.source].name;
        var dstName = data.nodes[e.target].name;
        if (srcName === dstName) return; // skip intra-problem natural casts
        var key = srcName + '->' + dstName;
        if (!nameLevelEdges[key]) {
          nameLevelEdges[key] = { count: 0, overhead: e.overhead, doc_path: e.doc_path };
        }
        nameLevelEdges[key].count++;
      });

      // Add collapsed edges to elements
      Object.keys(nameLevelEdges).forEach(function(key) {
        var parts = key.split('->');
        var srcId = parentIds[parts[0]] || variantId(parts[0], problems[parts[0]].variants[0].variant);
        var dstId = parentIds[parts[1]] || variantId(parts[1], problems[parts[1]].variants[0].variant);
        var info = nameLevelEdges[key];
        elements.push({
          data: {
            id: 'collapsed_' + key,
            source: srcId,
            target: dstId,
            label: info.count > 1 ? '\u00d7' + info.count : '',
            edgeLevel: 'collapsed',
            overhead: info.overhead,
            doc_path: info.doc_path
          }
        });
      });

      // ── Build variant-level edges (hidden, shown when expanded) ──
      Object.keys(edgeMap).forEach(function(k) {
        var e = edgeMap[k];
        elements.push({
          data: {
            id: 'variant_' + k,
            source: e.source,
            target: e.target,
            bidirectional: e.bidirectional,
            edgeLevel: 'variant',
            overhead: e.overhead,
            doc_path: e.doc_path
          }
        });
      });

      var cy = cytoscape({
        container: document.getElementById('cy'),
        elements: elements,
        style: [
          // Base node style (simple nodes — single variant, no parent)
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
          // Parent (compound) node — collapsed appearance
          { selector: 'node[?isParent]', style: {
            'label': 'data(label)',
            'text-valign': 'top',
            'text-halign': 'center',
            'font-size': '11px',
            'font-family': 'monospace',
            'padding': '10px',
            'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
            'border-width': 1.5,
            'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
            'shape': 'round-rectangle',
            'cursor': 'pointer'
          }},
          // Child variant nodes
          { selector: 'node[?isVariant]', style: {
            'label': 'data(label)',
            'text-valign': 'center',
            'text-halign': 'center',
            'font-size': '9px',
            'font-family': 'monospace',
            'width': function(ele) { return Math.max(ele.data('label').length * 5.5 + 8, 40); },
            'height': 18,
            'shape': 'round-rectangle',
            'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
            'border-width': 1,
            'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
            'cursor': 'pointer'
          }},
          // Edge styles
          { selector: 'edge', style: {
            'width': 1.5, 'line-color': '#999', 'target-arrow-color': '#999', 'target-arrow-shape': 'triangle',
            'source-arrow-color': '#999',
            'source-arrow-shape': function(ele) { return ele.data('bidirectional') ? 'triangle' : 'none'; },
            'curve-style': 'bezier', 'arrow-scale': 0.7, 'cursor': 'pointer',
            'label': 'data(label)', 'font-size': '9px', 'text-rotation': 'autorotate',
            'color': '#666', 'text-margin-y': -8
          }},
          // Hidden variant-level edges
          { selector: 'edge[edgeLevel="variant"]', style: { 'display': 'none' } },
          // Highlighted styles
          { selector: '.highlighted', style: {
            'background-color': '#ff6b6b', 'border-color': '#cc0000', 'border-width': 2, 'z-index': 10
          }},
          { selector: 'edge.highlighted', style: {
            'line-color': '#ff4444', 'target-arrow-color': '#ff4444', 'source-arrow-color': '#ff4444', 'width': 3, 'z-index': 10
          }},
          { selector: '.selected-node', style: {
            'border-color': '#0066cc', 'border-width': 2, 'background-color': '#cce0ff'
          }},
          { selector: '.faded', style: { 'opacity': 0.1 } },
          { selector: '.variant-selected', style: {
            'border-color': '#0066cc',
            'border-width': 2.5,
            'background-color': '#cce0ff'
          }}
        ],
        layout: {
          name: 'elk',
          elk: {
            algorithm: 'stress',
            'stress.desiredEdgeLength': 200,
            'nodeNode.spacing': 40,
          },
          animate: true,
          animationDuration: 500,
          padding: 40
        },
        userZoomingEnabled: true, userPanningEnabled: true, boxSelectionEnabled: false
      });

      // Start collapsed — hide all child variant nodes
      cy.nodes('[?isVariant]').style('display', 'none');

      var expandedParents = {};  // parentId → true/false
      var activeVariantFilter = null;

      function toggleExpand(parentNode) {
        var parentId = parentNode.id();
        var isExpanded = expandedParents[parentId];
        var children = parentNode.children();
        var name = parentNode.data('label');

        if (isExpanded) {
          // Collapse: hide children, show collapsed edges, hide variant edges
          children.style('display', 'none');
          cy.edges('[edgeLevel="collapsed"]').forEach(function(e) {
            var srcName = e.source().data('label') || e.source().data('problemName');
            var dstName = e.target().data('label') || e.target().data('problemName');
            if (srcName === name || dstName === name) {
              e.style('display', 'element');
            }
          });
          cy.edges('[edgeLevel="variant"]').forEach(function(e) {
            var srcParent = e.source().data('parent');
            var dstParent = e.target().data('parent');
            if (srcParent === parentId || dstParent === parentId) {
              e.style('display', 'none');
            }
          });
          expandedParents[parentId] = false;
        } else {
          // Expand: show children, hide collapsed edges, show variant edges
          children.style('display', 'element');
          cy.edges('[edgeLevel="collapsed"]').forEach(function(e) {
            // Hide collapsed edges connected to this parent
            if (e.source().id() === parentId || e.target().id() === parentId) {
              e.style('display', 'none');
            }
          });
          cy.edges('[edgeLevel="variant"]').forEach(function(e) {
            var srcParent = e.source().data('parent');
            var dstParent = e.target().data('parent');
            if (srcParent === parentId || dstParent === parentId) {
              e.style('display', 'element');
            }
          });
          expandedParents[parentId] = true;
        }

        // Re-layout with animation
        cy.layout({
          name: 'elk',
          elk: { algorithm: 'stress', 'stress.desiredEdgeLength': 200 },
          animate: true,
          animationDuration: 300,
          padding: 40
        }).run();
      }

      // Tooltip for nodes
      var tooltip = document.getElementById('cy-tooltip');
      cy.on('mouseover', 'node', function(evt) {
        var d = evt.target.data();
        var title = d.fullLabel || d.label;
        if (d.isParent) {
          title += ' (' + d.variantCount + ' variants)';
        }
        tooltip.innerHTML = '<strong>' + title + '</strong><br><em>Double-click to view API docs</em>';
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
        if (node.data('isParent')) {
          toggleExpand(node);
          return;
        }
        if (node.data('isVariant')) {
          if (activeVariantFilter === node.id()) {
            // Toggle off — clear filter
            cy.elements().removeClass('faded variant-selected');
            activeVariantFilter = null;
            instructions.textContent = 'Click a node to start path selection';
            return;
          }
          // Apply filter
          activeVariantFilter = node.id();
          cy.elements().addClass('faded');
          node.removeClass('faded').addClass('variant-selected');
          var connectedEdges = node.connectedEdges('[edgeLevel="variant"]');
          connectedEdges.removeClass('faded');
          connectedEdges.connectedNodes().removeClass('faded');
          if (node.data('parent')) {
            cy.getElementById(node.data('parent')).removeClass('faded');
          }
          instructions.textContent = 'Showing edges for ' + node.data('fullLabel') + ' — click again to clear';
          return;
        }
        if (!selectedNode) {
          selectedNode = node;
          node.addClass('selected-node');
          instructions.textContent = 'Now click a target node to find path from ' + node.data('label');
        } else if (node === selectedNode) {
          clearPath();
        } else {
          var visibleElements = cy.elements().filter(function(ele) {
            return ele.style('display') !== 'none';
          });
          var dijkstra = visibleElements.dijkstra({ root: selectedNode, directed: true });
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

      cy.on('tap', function(evt) {
        if (evt.target === cy) {
          clearPath();
          cy.elements().removeClass('faded variant-selected');
          activeVariantFilter = null;
        }
      });

      // Search bar handler
      var searchInput = document.getElementById('search-input');
      if (searchInput) {
        searchInput.addEventListener('input', function() {
          var query = this.value.trim().toLowerCase();
          if (query === '') {
            cy.elements().removeClass('faded');
            return;
          }
          cy.nodes().forEach(function(node) {
            var label = (node.data('label') || '').toLowerCase();
            var fullLabel = (node.data('fullLabel') || '').toLowerCase();
            if (label.includes(query) || fullLabel.includes(query)) {
              node.removeClass('faded');
            } else {
              node.addClass('faded');
            }
          });
          cy.edges().addClass('faded');
          cy.nodes().not('.faded').connectedEdges().forEach(function(edge) {
            if (!edge.source().hasClass('faded') && !edge.target().hasClass('faded')) {
              edge.removeClass('faded');
            }
          });
        });
      }
    })
    .catch(function(err) {
      document.getElementById('cy').innerHTML = '<p style="padding:1em;color:#c00;">Failed to load reduction graph: ' + err.message + '</p>';
    });
});

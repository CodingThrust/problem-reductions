document.addEventListener('DOMContentLoaded', function() {
  // Check if the cy container exists on this page
  var cyContainer = document.getElementById('cy');
  if (!cyContainer) return;

  // Register ELK layout extension if available (CDN scripts may load before cytoscape)
  var elkAvailable = false;
  if (typeof cytoscapeElk !== 'undefined') {
    cytoscape.use(cytoscapeElk);
    elkAvailable = true;
  } else if (typeof cytoscape !== 'undefined' && cytoscape.use) {
    // cytoscape-elk may have auto-registered if loaded after cytoscape
    try { cytoscape({ headless: true, elements: [] }).layout({ name: 'elk' }); elkAvailable = true; } catch(e) {}
  }

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

  function variantLabel(variant) {
    var keys = Object.keys(variant);
    if (keys.length === 0) return 'default';
    var parts = [];
    keys.forEach(function(k) {
      parts.push(k === 'graph' || k === 'weight' ? variant[k] : k + '=' + variant[k]);
    });
    return parts.join(', ');
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

      // Build edges at variant level — each directed edge is separate
      var edgeMap = {};
      data.edges.forEach(function(e) {
        var src = data.nodes[e.source];
        var dst = data.nodes[e.target];
        var srcId = variantId(src.name, src.variant);
        var dstId = variantId(dst.name, dst.variant);
        var fwd = srcId + '->' + dstId;
        if (!edgeMap[fwd]) {
          edgeMap[fwd] = { source: srcId, target: dstId, overhead: e.overhead || [], doc_path: e.doc_path || '' };
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

      // ── Build collapsed-mode edges (name-level) — each direction is separate ──
      var nameLevelEdges = {};
      data.edges.forEach(function(e) {
        var srcName = data.nodes[e.source].name;
        var dstName = data.nodes[e.target].name;
        if (srcName === dstName) return; // skip intra-problem natural casts
        var fwd = srcName + '->' + dstName;
        if (!nameLevelEdges[fwd]) {
          nameLevelEdges[fwd] = { count: 0, overhead: e.overhead, doc_path: e.doc_path };
        }
        nameLevelEdges[fwd].count++;
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
        var srcName = e.source.split('/')[0];
        var dstName = e.target.split('/')[0];
        var isNaturalCast = srcName === dstName;
        elements.push({
          data: {
            id: 'variant_' + k,
            source: e.source,
            target: e.target,
            edgeLevel: 'variant',
            overhead: e.overhead,
            doc_path: e.doc_path,
            isNaturalCast: isNaturalCast
          }
        });
      });

      var cy = cytoscape({
        container: document.getElementById('cy'),
        elements: elements,
        style: [
          // Use manual z-index on all elements so we have full control
          // over rendering order (bypasses compound-depth conventions)
          { selector: '*', style: {
            'z-index-compare': 'manual'
          }},
          // Base node style (simple nodes — single variant, no parent)
          { selector: 'node', style: {
            'label': 'data(label)', 'text-valign': 'center', 'text-halign': 'center',
            'font-size': '10px', 'font-family': 'monospace',
            'width': function(ele) { return Math.max(ele.data('label').length * 6.5 + 10, 50); },
            'height': 24, 'shape': 'round-rectangle',
            'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
            'border-width': 1,
            'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
            'text-wrap': 'none', 'cursor': 'pointer',
            'z-index': 2
          }},
          // Parent (compound) node — collapsed by default
          { selector: 'node[?isParent]', style: {
            'label': 'data(label)',
            'text-valign': 'center',
            'text-halign': 'center',
            'font-size': '10px',
            'font-family': 'monospace',
            'min-width': function(ele) { return Math.max(ele.data('label').length * 6.5 + 16, 60); },
            'min-height': 28,
            'padding': '4px',
            'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
            'border-width': 1.5,
            'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
            'shape': 'round-rectangle',
            'compound-sizing-wrt-labels': 'include',
            'cursor': 'pointer',
            'z-index': 2
          }},
          // Parent (compound) node — expanded appearance
          { selector: 'node[?isParent].expanded', style: {
            'text-valign': 'top',
            'font-size': '11px',
            'padding': '10px',
            'min-width': 0,
            'min-height': 0,
            'z-index': 5
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
            'cursor': 'pointer',
            'z-index': 6
          }},
          // Edge styles (z-index 1 = below nodes)
          { selector: 'edge', style: {
            'width': 1.5, 'line-color': '#999', 'target-arrow-color': '#999', 'target-arrow-shape': 'triangle',
            'curve-style': 'bezier', 'arrow-scale': 0.7, 'cursor': 'pointer',
            'source-distance-from-node': 5,
            'target-distance-from-node': 5,
            'overlay-padding': 2,
            'label': 'data(label)', 'font-size': '9px', 'text-rotation': 'autorotate',
            'color': '#666', 'text-margin-y': -8,
            'z-index': 1
          }},
          // Variant-level edges (hidden programmatically after init; above expanded parent)
          { selector: 'edge[edgeLevel="variant"]', style: {
            'z-index': 7
          } },
          // Natural cast edges (intra-problem)
          { selector: 'edge[?isNaturalCast]', style: {
            'line-style': 'dashed',
            'line-color': '#bbb',
            'target-arrow-color': '#bbb',
            'width': 1
          }},
          // Highlighted styles
          { selector: '.highlighted', style: {
            'background-color': '#ff6b6b', 'border-color': '#cc0000', 'border-width': 2, 'z-index': 20
          }},
          { selector: 'edge.highlighted', style: {
            'line-color': '#ff4444', 'target-arrow-color': '#ff4444', 'width': 3, 'z-index': 20
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
        layout: { name: 'preset' },  // delay layout until children are hidden
        userZoomingEnabled: true, userPanningEnabled: true, boxSelectionEnabled: false
      });

      // Shared layout helper
      function getLayoutOpts(animate) {
        return elkAvailable ? {
          name: 'elk',
          elk: {
            algorithm: 'stress',
            'stress.desiredEdgeLength': 200,
            'nodeNode.spacing': 40
          },
          nodeDimensionsIncludeLabels: true,
          fit: true,
          animate: animate,
          animationDuration: animate ? 400 : 0,
          padding: 40
        } : {
          name: 'cose',
          nodeDimensionsIncludeLabels: true,
          fit: true,
          animate: animate,
          animationDuration: animate ? 300 : 0,
          nodeRepulsion: function() { return 16000; },
          idealEdgeLength: function() { return 200; },
          gravity: 0.15,
          numIter: 1000,
          padding: 40
        };
      }

      // Run initial layout with children visible (ELK needs compound structure
      // for accurate sizing/positioning), then collapse after layout completes.
      cyContainer.style.opacity = '0';
      var initOpts = getLayoutOpts(false);
      initOpts.stop = function() {
        cy.nodes('[?isVariant]').style('display', 'none');
        cy.edges('[edgeLevel="variant"]').style('display', 'none');
        cy.fit(40);
        cyContainer.style.opacity = '1';
      };
      cy.layout(initOpts).run();

      var expandedParents = {};  // parentId → true/false
      var activeVariantFilter = null;

      function toggleExpand(parentNode) {
        var parentId = parentNode.id();
        var isExpanded = expandedParents[parentId];
        var children = parentNode.children();

        if (isExpanded) {
          // ── Collapse ──
          children.style('display', 'none');
          parentNode.removeClass('expanded');
          expandedParents[parentId] = false;

          // Show collapsed edges connected to this parent
          cy.edges('[edgeLevel="collapsed"]').forEach(function(e) {
            if (e.source().id() === parentId || e.target().id() === parentId) {
              e.style('display', 'element');
            }
          });

          // Hide all variant edges touching this parent's children
          cy.edges('[edgeLevel="variant"]').forEach(function(e) {
            var srcParent = e.source().data('parent');
            var dstParent = e.target().data('parent');
            if (srcParent === parentId || dstParent === parentId) {
              e.style('display', 'none');
            }
          });
        } else {
          // ── Expand ──
          children.style('display', 'element');
          parentNode.addClass('expanded');
          expandedParents[parentId] = true;

          // Hide collapsed edges from this parent ONLY when the other endpoint
          // can be reached via variant edges. If the other endpoint is a
          // collapsed compound parent, keep the collapsed edge (its children
          // are hidden, so variant edges can't replace it).
          cy.edges('[edgeLevel="collapsed"]').forEach(function(e) {
            if (e.source().id() === parentId || e.target().id() === parentId) {
              var otherId = e.source().id() === parentId ? e.target().id() : e.source().id();
              var otherNode = cy.getElementById(otherId);
              var otherIsCollapsedParent = otherNode.data('isParent') && !expandedParents[otherId];
              if (!otherIsCollapsedParent) {
                e.style('display', 'none');
              }
            }
          });

          // Show variant edges where both endpoints are visible
          cy.edges('[edgeLevel="variant"]').forEach(function(e) {
            var srcParent = e.source().data('parent');
            var dstParent = e.target().data('parent');
            if (srcParent === parentId || dstParent === parentId) {
              // A node is visible if it's not a variant child,
              // or if its parent is expanded
              var srcOk = !e.source().data('isVariant') || expandedParents[srcParent];
              var dstOk = !e.target().data('isVariant') || expandedParents[dstParent];
              if (srcOk && dstOk) {
                e.style('display', 'element');
              }
            }
          });
        }
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
        var html = '<strong>' + evt.target.source().data('label') + ' \u2192 ' + evt.target.target().data('label') + '</strong>';
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

        // Path selection in progress → any node completes the path
        if (selectedNode) {
          if (node === selectedNode) {
            clearPath();
            return;
          }
          // For parent nodes, find path to the parent itself
          var target = node;
          var visibleElements = cy.elements().filter(function(ele) {
            return ele.style('display') !== 'none';
          });
          var dijkstra = visibleElements.dijkstra({ root: selectedNode, directed: true });
          var path = dijkstra.pathTo(target);
          cy.elements().removeClass('highlighted selected-node');
          if (path && path.length > 0) {
            path.addClass('highlighted');
            instructions.textContent = 'Path: ' + path.nodes().map(function(n) {
              return n.data('fullLabel') || n.data('label');
            }).join(' \u2192 ');
          } else {
            instructions.textContent = 'No path from ' +
              (selectedNode.data('fullLabel') || selectedNode.data('label')) +
              ' to ' + (target.data('fullLabel') || target.data('label'));
          }
          clearBtn.style.display = 'inline';
          selectedNode = null;
          return;
        }

        // No path selection active — Parent → expand/collapse
        if (node.data('isParent')) {
          toggleExpand(node);
          return;
        }

        // No path selection active — Variant node → variant filter
        if (node.data('isVariant')) {
          if (activeVariantFilter === node.id()) {
            cy.elements().removeClass('faded variant-selected');
            activeVariantFilter = null;
            instructions.textContent = 'Click a node to start path selection';
            return;
          }
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

        // No path selection active — Simple/any node → start path selection
        selectedNode = node;
        node.addClass('selected-node');
        instructions.textContent = 'Now click a target node to find path from ' +
          (node.data('fullLabel') || node.data('label'));
      });

      cy.on('tap', 'edge', function(evt) {
        var edge = evt.target;
        var d = edge.data();
        cy.elements().removeClass('highlighted selected-node');
        edge.addClass('highlighted');
        edge.source().addClass('highlighted');
        edge.target().addClass('highlighted');
        var text = edge.source().data('label') + ' \u2192 ' + edge.target().data('label');
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

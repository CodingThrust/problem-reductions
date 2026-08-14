(function() {
  var categoryColors = {
    graph: '#c8f0c8', set: '#f0c8c8', algebraic: '#f0f0a0',
    formula: '#c8c8f0', misc: '#f0c8e0'
  };
  var categoryBorders = {
    graph: '#4a8c4a', set: '#8c4a4a', algebraic: '#8c8c4a',
    formula: '#4a4a8c', misc: '#8c4a6a'
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

  function cloneElement(ele) {
    var out = { data: {} };
    Object.keys(ele.data).forEach(function(k) {
      out.data[k] = ele.data[k];
    });
    if (ele.position) out.position = { x: ele.position.x, y: ele.position.y };
    return out;
  }

  function hashString(value) {
    var hash = 0x811c9dc5;
    for (var i = 0; i < value.length; i++) {
      hash ^= value.charCodeAt(i);
      hash = Math.imul(hash, 0x01000193) >>> 0;
    }
    return ('00000000' + hash.toString(16)).slice(-8);
  }

  function reductionGraphFingerprint(modelOrElements) {
    var elements = Array.isArray(modelOrElements) ?
      modelOrElements :
      modelOrElements.initialElements;
    var nodes = [];
    var edges = [];

    elements.forEach(function(ele) {
      if (ele.data.source) {
        edges.push(ele.data.source + '->' + ele.data.target + '|' + (ele.data.label || ''));
      } else {
        nodes.push(ele.data.id + '|' + (ele.data.label || ''));
      }
    });

    nodes.sort();
    edges.sort();
    return hashString(nodes.join('\n') + '\n--\n' + edges.join('\n'));
  }

  function applyPrecomputedLayout(elements, layout) {
    var positioned = elements.map(cloneElement);
    if (!layout || layout.fingerprint !== reductionGraphFingerprint(elements) || !layout.nodes) {
      return positioned;
    }

    for (var i = 0; i < positioned.length; i++) {
      var ele = positioned[i];
      if (ele.data.source) continue;
      var pos = layout.nodes[ele.data.id];
      if (!pos || !Number.isFinite(pos.x) || !Number.isFinite(pos.y)) {
        return elements.map(cloneElement);
      }
      ele.position = { x: pos.x, y: pos.y };
    }

    return positioned;
  }

  function hasNodePositions(elements) {
    return elements.some(function(ele) {
      return !ele.data.source && ele.position;
    });
  }

  function createReductionGraphModel(data) {
    var problems = {};
    data.nodes.forEach(function(n, idx) {
      if (!problems[n.name]) {
        problems[n.name] = { category: n.category, doc_path: n.doc_path, variants: [] };
      }
      problems[n.name].variants.push({
        index: idx,
        variant: n.variant,
        category: n.category,
        doc_path: n.doc_path
      });
    });

    var initialElements = [];
    var parentIds = {};
    var problemNodeIds = {};
    var variantNodesByParent = {};
    var variantParentByNodeId = {};

    Object.keys(problems).forEach(function(name) {
      var info = problems[name];
      var hasMultipleVariants = info.variants.length > 1;

      if (hasMultipleVariants) {
        var parentId = 'parent_' + name;
        parentIds[name] = parentId;
        problemNodeIds[name] = parentId;
        initialElements.push({
          data: {
            id: parentId,
            label: name,
            category: info.category,
            doc_path: info.doc_path,
            isParent: true,
            variantCount: info.variants.length
          }
        });

        variantNodesByParent[parentId] = info.variants.map(function(v) {
          var vid = variantId(name, v.variant);
          variantParentByNodeId[vid] = parentId;
          return {
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
          };
        });
      } else {
        var v = info.variants[0];
        var vid = variantId(name, v.variant);
        problemNodeIds[name] = vid;
        initialElements.push({
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

    var nameLevelEdges = {};
    data.edges.forEach(function(e) {
      var srcName = data.nodes[e.source].name;
      var dstName = data.nodes[e.target].name;
      if (srcName === dstName) return;
      var key = srcName + '->' + dstName;
      if (!nameLevelEdges[key]) {
        nameLevelEdges[key] = { count: 0, sizeFields: e.size_fields, doc_path: e.doc_path };
      }
      nameLevelEdges[key].count++;
    });

    Object.keys(nameLevelEdges).forEach(function(key) {
      var parts = key.split('->');
      var info = nameLevelEdges[key];
      initialElements.push({
        data: {
          id: 'collapsed_' + key,
          source: problemNodeIds[parts[0]],
          target: problemNodeIds[parts[1]],
          label: info.count > 1 ? '\u00d7' + info.count : '',
          edgeLevel: 'collapsed',
          sizeFields: info.sizeFields,
          doc_path: info.doc_path
        }
      });
    });

    var edgeMap = {};
    data.edges.forEach(function(e) {
      var src = data.nodes[e.source];
      var dst = data.nodes[e.target];
      var srcId = variantId(src.name, src.variant);
      var dstId = variantId(dst.name, dst.variant);
      var key = srcId + '->' + dstId;
      if (!edgeMap[key]) {
        edgeMap[key] = {
          source: srcId,
          target: dstId,
          sizeFields: e.size_fields || [],
          doc_path: e.doc_path || ''
        };
      }
    });

    var variantEdges = Object.keys(edgeMap).map(function(key) {
      var e = edgeMap[key];
      var srcName = e.source.split('/')[0];
      var dstName = e.target.split('/')[0];
      var isVariantCast = srcName === dstName &&
        e.sizeFields &&
        e.sizeFields.length > 0 &&
        e.sizeFields.every(function(o) {
          return o.contract === 'exact' && o.field === o.formula;
        });
      return {
        data: {
          id: 'variant_' + key,
          source: e.source,
          target: e.target,
          edgeLevel: 'variant',
          sizeFields: e.sizeFields,
          doc_path: e.doc_path,
          isVariantCast: isVariantCast
        }
      };
    });

    return {
      initialElements: initialElements,
      problems: problems,
      parentIds: parentIds,
      problemNodeIds: problemNodeIds,
      parentCount: Object.keys(parentIds).length,
      problemCount: Object.keys(problems).length,
      collapsedEdgeCount: Object.keys(nameLevelEdges).length,
      variantNodesByParent: variantNodesByParent,
      variantParentByNodeId: variantParentByNodeId,
      variantEdges: variantEdges
    };
  }

  function installBrowserGraph() {
    var cyContainer = document.getElementById('cy');
    if (!cyContainer) return;

    var elkAvailable = false;
    if (typeof cytoscapeElk !== 'undefined') {
      cytoscape.use(cytoscapeElk);
      elkAvailable = true;
    } else if (typeof cytoscape !== 'undefined' && cytoscape.use) {
      try {
        cytoscape({ headless: true, elements: [] }).layout({ name: 'elk' });
        elkAvailable = true;
      } catch(e) {}
    }

    if (typeof cytoscapeSvg !== 'undefined') {
      cytoscape.use(cytoscapeSvg);
    }

    function fetchOptionalJson(path) {
      return fetch(path)
        .then(function(r) { return r.ok ? r.json() : null; })
        .catch(function() { return null; });
    }

    fetch('reductions/reduction_graph.json')
      .then(function(r) { if (!r.ok) throw new Error('HTTP ' + r.status); return r.json(); })
      .then(function(data) {
        return fetchOptionalJson('reductions/reduction_graph_layout.json')
          .then(function(layout) { return { data: data, layout: layout }; });
      })
      .then(function(payload) {
        var data = payload.data;
        var model = createReductionGraphModel(data);
        var expandedParents = {};
        var activeVariantFilter = null;
        var initialElements = applyPrecomputedLayout(model.initialElements, payload.layout);
        var usesPrecomputedLayout = hasNodePositions(initialElements);

        cyContainer.style.opacity = '0';
        var cy = cytoscape({
          container: cyContainer,
          elements: initialElements,
          style: [
            { selector: '*', style: { 'z-index-compare': 'manual' }},
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
            { selector: 'node[?isParent].expanded', style: {
              'text-valign': 'top',
              'font-size': '11px',
              'padding': '10px',
              'min-width': 0,
              'min-height': 0,
              'z-index': 5
            }},
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
            { selector: 'edge', style: {
              'width': 1, 'line-color': '#999', 'target-arrow-color': '#999', 'target-arrow-shape': 'triangle',
              'curve-style': 'bezier', 'arrow-scale': 0.7, 'cursor': 'pointer',
              'source-distance-from-node': 5,
              'target-distance-from-node': 5,
              'overlay-padding': 0,
              'label': 'data(label)', 'font-size': '9px', 'text-rotation': 'autorotate',
              'color': '#666', 'text-margin-y': -8,
              'z-index': 1
            }},
            { selector: 'edge[edgeLevel="variant"]', style: { 'z-index': 7 } },
            { selector: 'edge[?isVariantCast]', style: {
              'line-style': 'dashed',
              'line-color': '#bbb',
              'target-arrow-color': '#bbb',
              'width': 1
            }},
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
          layout: { name: 'preset' },
          userZoomingEnabled: true,
          userPanningEnabled: true,
          boxSelectionEnabled: false
        });

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
            numIter: 500,
            padding: 40
          };
        }

        if (usesPrecomputedLayout) {
          cy.fit(40);
          cyContainer.style.opacity = '1';
        } else {
          var initOpts = getLayoutOpts(false);
          initOpts.stop = function() {
            cy.fit(40);
            cyContainer.style.opacity = '1';
          };
          cy.layout(initOpts).run();
        }

        function positionVariantNodes(parentNode, elements) {
          var parentPos = parentNode.position();
          var count = elements.length;
          var cols = Math.ceil(Math.sqrt(count));
          var rows = Math.ceil(count / cols);
          var xGap = 78;
          var yGap = 34;
          elements.forEach(function(ele, i) {
            var col = i % cols;
            var row = Math.floor(i / cols);
            ele.position = {
              x: parentPos.x + (col - (cols - 1) / 2) * xGap,
              y: parentPos.y + (row - (rows - 1) / 2) * yGap + 10
            };
          });
        }

        function addVariantNodes(parentNode) {
          var parentId = parentNode.id();
          var templates = model.variantNodesByParent[parentId] || [];
          var toAdd = [];
          templates.forEach(function(template) {
            if (cy.getElementById(template.data.id).empty()) {
              toAdd.push(cloneElement(template));
            }
          });
          if (toAdd.length === 0) return;
          positionVariantNodes(parentNode, toAdd);
          cy.add(toAdd);
        }

        function syncExpandedState() {
          cy.batch(function() {
            cy.edges('[edgeLevel="variant"]').remove();

            var edgesToAdd = [];
            model.variantEdges.forEach(function(edge) {
              var srcId = edge.data.source;
              var dstId = edge.data.target;
              var srcParent = model.variantParentByNodeId[srcId];
              var dstParent = model.variantParentByNodeId[dstId];
              var touchesExpandedParent = (srcParent && expandedParents[srcParent]) ||
                (dstParent && expandedParents[dstParent]);

              if (!touchesExpandedParent) return;
              if (cy.getElementById(srcId).empty() || cy.getElementById(dstId).empty()) return;
              if (!cy.getElementById(edge.data.id).empty()) return;
              edgesToAdd.push(cloneElement(edge));
            });
            if (edgesToAdd.length > 0) cy.add(edgesToAdd);

            cy.edges('[edgeLevel="collapsed"]').style('display', 'element');
            cy.edges('[edgeLevel="collapsed"]').forEach(function(e) {
              var src = e.source();
              var dst = e.target();
              var srcExpanded = src.data('isParent') && expandedParents[src.id()];
              var dstExpanded = dst.data('isParent') && expandedParents[dst.id()];
              if (!srcExpanded && !dstExpanded) return;

              var other = srcExpanded ? dst : src;
              var otherCollapsedParent = other.data('isParent') && !expandedParents[other.id()];
              if (!otherCollapsedParent) e.style('display', 'none');
            });
          });
        }

        function toggleExpand(parentNode) {
          var parentId = parentNode.id();
          var isExpanded = expandedParents[parentId];

          if (isExpanded) {
            var pos = parentNode.position();
            cy.remove(parentNode.children());
            parentNode.position(pos);
            parentNode.removeClass('expanded');
            expandedParents[parentId] = false;
          } else {
            parentNode.addClass('expanded');
            expandedParents[parentId] = true;
            addVariantNodes(parentNode);
          }

          cy.elements().removeClass('faded variant-selected highlighted selected-node');
          activeVariantFilter = null;
          syncExpandedState();
        }

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
          var rect = cyContainer.getBoundingClientRect();
          tooltip.style.left = (rect.left + window.scrollX + pos.x + 15) + 'px';
          tooltip.style.top = (rect.top + window.scrollY + pos.y - 10) + 'px';
        });
        cy.on('mouseout', 'node', function() { tooltip.style.display = 'none'; });

        cy.on('mouseover', 'edge', function(evt) {
          var d = evt.target.data();
          var html = '<strong>' + evt.target.source().data('label') + ' \u2192 ' + evt.target.target().data('label') + '</strong>';
          if (d.sizeFields && d.sizeFields.length > 0) {
            html += '<br>' + d.sizeFields.map(function(o) {
              if (o.contract === 'exact') return '<code>' + o.field + '</code> = <code>' + o.formula + '</code> (exact)';
              if (o.contract === 'upper_bound') return '<code>' + o.field + '</code> &le; <code>' + o.formula + '</code> (upper bound)';
              return '<code>' + o.field + '</code> unavailable: ' + o.reason;
            }).join('<br>');
          }
          html += '<br><em>Click to highlight, double-click for source code</em>';
          tooltip.innerHTML = html;
          tooltip.style.display = 'block';
        });
        cy.on('mousemove', 'edge', function(evt) {
          var pos = evt.renderedPosition || evt.position;
          var rect = cyContainer.getBoundingClientRect();
          tooltip.style.left = (rect.left + window.scrollX + pos.x + 15) + 'px';
          tooltip.style.top = (rect.top + window.scrollY + pos.y - 10) + 'px';
        });
        cy.on('mouseout', 'edge', function() { tooltip.style.display = 'none'; });

        cy.on('dbltap', 'node', function(evt) {
          var d = evt.target.data();
          if (d.doc_path) {
            window.location.href = 'api/problemreductions/' + d.doc_path;
          }
        });
        cy.on('dbltap', 'edge', function(evt) {
          var d = evt.target.data();
          if (d.doc_path) {
            var module = d.doc_path.replace('/index.html', '');
            window.open('https://github.com/CodingThrust/problem-reductions/blob/main/src/' + module + '.rs', '_blank');
          }
        });

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

          if (selectedNode) {
            if (node === selectedNode) {
              clearPath();
              return;
            }
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

          if (node.data('isParent')) {
            toggleExpand(node);
            return;
          }

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
            instructions.textContent = 'Showing edges for ' + node.data('fullLabel') + ' \u2014 click again to clear';
            return;
          }

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
          if (d.sizeFields && d.sizeFields.length > 0) {
            text += '  |  ' + d.sizeFields.map(function(o) {
              if (o.contract === 'exact') return o.field + ' = ' + o.formula + ' (exact)';
              if (o.contract === 'upper_bound') return o.field + ' <= ' + o.formula + ' (upper bound)';
              return o.field + ' unavailable: ' + o.reason;
            }).join(', ');
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

        var downloadBtn = document.getElementById('download-svg-btn');
        if (downloadBtn) {
          downloadBtn.addEventListener('click', function() {
            var svgContent = cy.svg({ scale: 1, full: true, bg: getComputedStyle(document.documentElement).getPropertyValue('--bg').trim() || '#ffffff' });
            var blob = new Blob([svgContent], { type: 'image/svg+xml;charset=utf-8' });
            var url = URL.createObjectURL(blob);
            var a = document.createElement('a');
            a.href = url;
            a.download = 'reduction-graph.svg';
            a.click();
            URL.revokeObjectURL(url);
          });
        }

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
        cyContainer.innerHTML = '<p style="padding:1em;color:#c00;">Failed to load reduction graph: ' + err.message + '</p>';
      });
  }

  if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
      applyPrecomputedLayout: applyPrecomputedLayout,
      createReductionGraphModel: createReductionGraphModel,
      reductionGraphFingerprint: reductionGraphFingerprint,
      variantId: variantId
    };
  }

  if (typeof document !== 'undefined') {
    document.addEventListener('DOMContentLoaded', installBrowserGraph);
  }
})();

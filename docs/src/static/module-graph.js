document.addEventListener('DOMContentLoaded', function() {
  var container = document.getElementById('module-graph');
  if (!container) return;

  var categoryColors = {
    core: '#c8f0c8',
    model_graph: '#c8c8f0',
    model_formula: '#c8c8f0',
    model_set: '#c8c8f0',
    model_algebraic: '#c8c8f0',
    model_misc: '#c8c8f0',
    rule: '#f0d8b0',
    registry: '#b0e0f0',
    solver: '#d0f0d0',
    utility: '#e0e0e0'
  };
  var categoryBorders = {
    core: '#4a8c4a',
    model_graph: '#4a4a8c',
    model_formula: '#4a4a8c',
    model_set: '#4a4a8c',
    model_algebraic: '#4a4a8c',
    model_misc: '#4a4a8c',
    rule: '#8c6a4a',
    registry: '#4a6a8c',
    solver: '#4a8c6a',
    utility: '#888888'
  };
  var kindIcons = {
    'struct': 'S', 'enum': 'E', 'function': 'fn', 'trait': 'T',
    'type_alias': 'type', 'constant': 'const'
  };

  // Fixed positions — arranged in logical groups
  var fixedPositions = {
    // Core (left column)
    'traits':          { x: 80, y: 80 },
    'types':           { x: 80, y: 230 },
    'variant':         { x: 80, y: 380 },
    'topology':        { x: 80, y: 530 },
    // Models (center column)
    'models/graph':    { x: 340, y: 80 },
    'models/formula':  { x: 340, y: 230 },
    'models/set':      { x: 340, y: 350 },
    'models/algebraic':{ x: 340, y: 450 },
    'models/misc':     { x: 340, y: 550 },
    // Rules + Registry (right-center)
    'rules':           { x: 600, y: 80 },
    'registry':        { x: 600, y: 230 },
    // Solvers + Utilities (far right)
    'solvers':         { x: 800, y: 80 },
    'expr':            { x: 800, y: 230 },
    'io':              { x: 800, y: 380 }
  };

  fetch('static/module-graph.json')
    .then(function(r) { if (!r.ok) throw new Error('HTTP ' + r.status); return r.json(); })
    .then(function(data) {
      var elements = [];

      data.modules.forEach(function(mod) {
        var parentId = 'mod_' + mod.name.replace(/\//g, '_');
        var pos = fixedPositions[mod.name] || { x: 500, y: 280 };

        // Compound parent node
        elements.push({
          data: {
            id: parentId,
            label: mod.name,
            category: mod.category,
            doc_path: mod.doc_path,
            itemCount: mod.items.length,
            isParent: true
          }
        });

        // Child item nodes
        mod.items.forEach(function(item, idx) {
          var childId = mod.name.replace(/\//g, '_') + '::' + item.name;
          var icon = kindIcons[item.kind] || item.kind;
          elements.push({
            data: {
              id: childId,
              parent: parentId,
              label: icon + ' ' + item.name,
              fullLabel: mod.name + '::' + item.name,
              category: mod.category,
              kind: item.kind,
              doc: item.doc || '',
              isChild: true,
              moduleName: mod.name,
              itemName: item.name
            },
            position: {
              x: pos.x,
              y: pos.y + 18 + idx * 22
            }
          });
        });
      });

      // Module-level edges
      data.edges.forEach(function(e) {
        elements.push({
          data: {
            id: 'edge_' + e.source.replace(/\//g, '_') + '_' + e.target.replace(/\//g, '_'),
            source: 'mod_' + e.source.replace(/\//g, '_'),
            target: 'mod_' + e.target.replace(/\//g, '_')
          }
        });
      });

      var cy = cytoscape({
        container: container,
        elements: elements,
        style: [
          // Module nodes (compound parents)
          { selector: 'node[?isParent]', style: {
            'label': 'data(label)',
            'text-valign': 'center', 'text-halign': 'center',
            'font-size': '11px', 'font-family': 'monospace', 'font-weight': 'bold',
            'min-width': function(ele) { return Math.max(ele.data('label').length * 7.5 + 20, 80); },
            'min-height': 36,
            'padding': '4px',
            'shape': 'round-rectangle',
            'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
            'border-width': 2,
            'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
            'compound-sizing-wrt-labels': 'include',
            'cursor': 'pointer'
          }},
          // Expanded parent
          { selector: 'node[?isParent].expanded', style: {
            'text-valign': 'top',
            'padding': '10px'
          }},
          // Child item nodes
          { selector: 'node[?isChild]', style: {
            'label': 'data(label)',
            'text-valign': 'center', 'text-halign': 'center',
            'font-size': '9px', 'font-family': 'monospace',
            'width': function(ele) { return Math.max(ele.data('label').length * 5.5 + 8, 40); },
            'height': 18,
            'shape': 'round-rectangle',
            'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
            'border-width': 1,
            'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; }
          }},
          // Edges
          { selector: 'edge', style: {
            'width': 1.5, 'line-color': '#999', 'target-arrow-color': '#999',
            'target-arrow-shape': 'triangle', 'curve-style': 'bezier',
            'arrow-scale': 0.8,
            'source-distance-from-node': 5,
            'target-distance-from-node': 5
          }}
        ],
        layout: { name: 'preset' },
        userZoomingEnabled: true,
        userPanningEnabled: true,
        boxSelectionEnabled: false
      });

      // Initial state: hide all children, position parents at fixed positions
      cy.nodes('[?isChild]').style('display', 'none');
      Object.keys(fixedPositions).forEach(function(name) {
        var node = cy.getElementById('mod_' + name.replace(/\//g, '_'));
        if (node.length) node.position(fixedPositions[name]);
      });
      cy.fit(40);

      var expandedParents = {};

      // Click: toggle expand/collapse
      cy.on('tap', 'node[?isParent]', function(evt) {
        var parentNode = evt.target;
        var parentId = parentNode.id();
        var children = parentNode.children();

        if (expandedParents[parentId]) {
          // Collapse
          children.style('display', 'none');
          parentNode.removeClass('expanded');
          expandedParents[parentId] = false;
          var name = parentNode.data('label');
          if (fixedPositions[name]) {
            parentNode.position(fixedPositions[name]);
          }
        } else {
          // Expand
          children.style('display', 'element');
          parentNode.addClass('expanded');
          expandedParents[parentId] = true;
        }
      });

      // Rustdoc URL prefixes by kind
      var kindPrefix = {
        'function': 'fn', 'struct': 'struct', 'enum': 'enum',
        'trait': 'trait', 'type_alias': 'type', 'constant': 'constant'
      };

      // Double-click: open rustdoc
      cy.on('dbltap', 'node[?isParent]', function(evt) {
        var d = evt.target.data();
        if (d.doc_path) {
          window.open('api/problemreductions/' + d.doc_path, '_blank');
        }
      });
      cy.on('dbltap', 'node[?isChild]', function(evt) {
        var d = evt.target.data();
        var prefix = kindPrefix[d.kind] || d.kind;
        var modPath = d.moduleName.replace(/\//g, '/');
        window.open('api/problemreductions/' + modPath + '/' + prefix + '.' + d.itemName + '.html', '_blank');
      });

      // Tooltip
      var tooltip = document.getElementById('mg-tooltip');
      cy.on('mouseover', 'node[?isParent]', function(evt) {
        var d = evt.target.data();
        tooltip.innerHTML = '<strong>' + d.label + '</strong> (' + d.itemCount + ' items)<br><em>Click to expand, double-click for docs</em>';
        tooltip.style.display = 'block';
      });
      cy.on('mouseover', 'node[?isChild]', function(evt) {
        var d = evt.target.data();
        var html = '<strong>' + d.fullLabel + '</strong><br><code>' + d.kind + '</code>';
        if (d.doc) html += '<br><em>' + d.doc + '</em>';
        tooltip.innerHTML = html;
        tooltip.style.display = 'block';
      });
      cy.on('mousemove', 'node', function(evt) {
        var pos = evt.renderedPosition || evt.position;
        var rect = container.getBoundingClientRect();
        tooltip.style.left = (rect.left + window.scrollX + pos.x + 15) + 'px';
        tooltip.style.top = (rect.top + window.scrollY + pos.y - 10) + 'px';
      });
      cy.on('mouseout', 'node', function() { tooltip.style.display = 'none'; });

      // Edge tooltip
      cy.on('mouseover', 'edge', function(evt) {
        var src = evt.target.source().data('label');
        var dst = evt.target.target().data('label');
        tooltip.innerHTML = '<strong>' + src + ' \u2192 ' + dst + '</strong>';
        tooltip.style.display = 'block';
      });
      cy.on('mousemove', 'edge', function(evt) {
        var pos = evt.renderedPosition || evt.position;
        var rect = container.getBoundingClientRect();
        tooltip.style.left = (rect.left + window.scrollX + pos.x + 15) + 'px';
        tooltip.style.top = (rect.top + window.scrollY + pos.y - 10) + 'px';
      });
      cy.on('mouseout', 'edge', function() { tooltip.style.display = 'none'; });
    })
    .catch(function(err) {
      container.innerHTML = '<p style="padding:1em;color:#c00;">Failed to load module graph: ' + err.message + '</p>';
    });
});

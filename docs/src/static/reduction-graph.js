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
        nameLevelEdges[key] = { count: 0, parameters: e.parameters, doc_path: e.doc_path };
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
          parameters: info.parameters,
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
          parameters: e.parameters || [],
          doc_path: e.doc_path || ''
        };
      }
    });

    var variantEdges = Object.keys(edgeMap).map(function(key) {
      var e = edgeMap[key];
      var srcName = e.source.split('/')[0];
      var dstName = e.target.split('/')[0];
      var isVariantCast = srcName === dstName &&
        e.parameters &&
        e.parameters.length > 0 &&
        e.parameters.every(function(o) {
          return o.contract === 'exact' && o.field === o.formula;
        });
      return {
        data: {
          id: 'variant_' + key,
          source: e.source,
          target: e.target,
          edgeLevel: 'variant',
          parameters: e.parameters,
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


  if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
      applyPrecomputedLayout: applyPrecomputedLayout,
      createReductionGraphModel: createReductionGraphModel,
      reductionGraphFingerprint: reductionGraphFingerprint,
      variantId: variantId
    };
  }

})();

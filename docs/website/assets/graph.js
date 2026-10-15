(async () => {
  "use strict";

  try {
      const data = window.REDUCTIONS;
      if (!data) throw new Error("Cannot load graph data. Reload the page to get the current release.");
      const schemas = new Map(data.schemas.map((schema) => [schema.name, schema]));
      const aliases = new Map([
        ["MIS", "MaximumIndependentSet"],
        ["MVC", "MinimumVertexCover"],
        ["SAT", "Satisfiability"],
        ["3SAT", "KSatisfiability"],
        ["3-SAT", "KSatisfiability"],
      ]);
      const colors = {
        graph: "#b9d99b",
        formula: "#9bbfc0",
        set: "#d1a78d",
        algebraic: "#d8c87e",
        misc: "#bd9dbc",
      };
      const families = new Map();

      data.nodes.forEach((node) => {
        const family = families.get(node.name) || {
          name: node.name,
          category: node.category,
          variants: [],
          incoming: new Set(),
          outgoing: new Set(),
          rules: 0,
        };
        family.variants.push(node);
        families.set(node.name, family);
      });

      const connections = new Map();
      data.edges.forEach((edge) => {
        const source = data.nodes[edge.source].name;
        const target = data.nodes[edge.target].name;
        if (source === target) return;
        const key = `${source}\u0000${target}`;
        const connection = connections.get(key) || { source, target, count: 0 };
        connection.count += 1;
        connections.set(key, connection);
        families.get(source).outgoing.add(target);
        families.get(target).incoming.add(source);
        families.get(source).rules += 1;
        families.get(target).rules += 1;
      });

      const displayName = (name) =>
        schemas.get(name)?.display_name ||
        name
          .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
          .replace(/([A-Z])([A-Z][a-z])/g, "$1 $2");

      const variantKey = (variant) =>
        Object.entries(variant)
          .sort(([a], [b]) => a.localeCompare(b))
          .map(([key, value]) => `${key}=${value}`)
          .join(",");

      const search = document.querySelector("#graph-search");
      const inspector = document.querySelector(".graph-inspector");
      const selectionType = document.querySelector("#selection-type");
      const detail = document.querySelector("#graph-detail");
      const tooltip = document.querySelector("#graph-tooltip");
      let scope = "core";

      const elements = [];
      families.forEach((family) => {
        const neighbors = new Set([...family.incoming, ...family.outgoing]);
        const position = data.layout[family.name];
        if (!position) throw new Error(`Missing graph layout position: ${family.name}`);
        elements.push({
          data: {
            id: family.name,
            label: displayName(family.name),
            category: family.category,
            rules: family.rules,
            peripheral: neighbors.size < 2,
          },
          position: { x: position.x * 1.15, y: position.y * 1.15 },
        });
      });
      connections.forEach((connection) =>
        elements.push({
          data: {
            id: `edge:${connection.source}:${connection.target}`,
            source: connection.source,
            target: connection.target,
            count: connection.count,
          },
        }),
      );

      const cy = cytoscape({
        container: document.querySelector("#cy"),
        elements: [],
        layout: { name: "preset" },
        minZoom: 0.08,
        maxZoom: 3,
        wheelSensitivity: 0.22,
        boxSelectionEnabled: false,
        style: [
          {
            selector: "node",
            style: {
              width: (node) => Math.min(22, 10 + Math.log2(1 + node.data("rules")) * 1.5),
              height: (node) => Math.min(22, 10 + Math.log2(1 + node.data("rules")) * 1.5),
              shape: "ellipse",
              "background-color": (node) =>
                colors[node.data("category")] || colors.misc,
              "border-color": "#0d1410",
              "border-width": 2,
              label: "",
              color: "#b9c4bb",
              "font-family": "DM Sans, sans-serif",
              "font-size": 12,
              "font-weight": 400,
              "min-zoomed-font-size": 7,
              "text-valign": "bottom",
              "text-margin-y": 8,
              "text-background-color": "#0d1410",
              "text-background-opacity": 0.72,
              "text-background-padding": 2,
              cursor: "pointer",
            },
          },
          {
            selector: "node.named",
            style: { label: "data(label)" },
          },
          {
            selector: "edge",
            style: {
              width: 0.9,
              "line-color": "#8fa992",
              "target-arrow-shape": "none",
              "mid-target-arrow-shape": "none",
              "mid-target-arrow-color": "#d2e4bd",
              "arrow-scale": 0.7,
              "curve-style": "bezier",
              opacity: 0.45,
              cursor: "pointer",
            },
          },
          { selector: ".core-hidden", style: { display: "none" } },
          {
            selector: ".faded",
            style: { opacity: 0.045, "text-opacity": 0 },
          },
          {
            selector: "node.selected",
            style: {
              label: "data(label)",
              "background-color": "#e1f1c9",
              "border-color": "#5c7957",
              "border-width": 2,
              color: "#eef4ea",
              "font-size": 12,
              "text-opacity": 1,
              "z-index": 20,
            },
          },
          {
            selector: "edge.selected",
            style: {
              "line-color": "#c4dda6",
              "mid-target-arrow-shape": "triangle",
              width: 1.2,
              opacity: 0.95,
              "z-index": 18,
            },
          },
          {
            selector: "node.neighbor",
            style: { label: "", opacity: 0.8, width: 13, height: 13 },
          },
          {
            selector: "node.expanded",
            style: {
              shape: "round-rectangle",
              label: "data(label)",
              "background-opacity": 0.8,
              "background-color": "#0d1410",
              "border-color": "#344739",
              "border-width": 1,
              "text-valign": "top",
              "text-margin-y": -10,
              padding: 24,
            },
          },
          {
            selector: "node[?isVariant]",
            style: {
              label: "data(label)",
              shape: "ellipse",
              width: 20,
              height: 20,
              "text-valign": "bottom",
              "text-margin-y": 10,
              "text-wrap": "wrap",
              "text-max-width": 145,
              "font-size": 14,
              color: "#d6e2d1",
              "border-width": 2,
              "text-background-opacity": 1,
            },
          },
          { selector: ".focus-hidden", style: { display: "none" } },
          { selector: "node.route-node", style: {
            "border-width": 3, "border-color": "#c4dda6", opacity: 1,
            label: "data(routeLabel)", "text-opacity": 1, "text-wrap": "wrap", "text-max-width": 220,
          } },
          { selector: "node.route-family", style: { label: "" } },
          { selector: "node.path-source", style: { "border-color": "#9bd9db", "border-width": 4, opacity: 1 } },
          { selector: "node.path-target", style: { "border-color": "#e4b17e", "border-width": 4, opacity: 1 } },
          { selector: "edge.route-edge", style: {
            "target-arrow-shape": "triangle", "target-arrow-color": "#c4dda6",
            "mid-target-arrow-shape": "none", "line-color": "#c4dda6", width: 2, opacity: 1,
          } },
          {
            selector: "node.hovered, node.named",
            style: { label: "data(label)", "text-opacity": 1, "z-index": 30 },
          },
          {
            selector: "edge.hovered, edge.direction",
            style: {
              "mid-target-arrow-shape": "triangle",
              "line-color": "#b6c9a7",
              opacity: 0.9,
              width: 1.3,
            },
          },
          {
            selector: "node.search-match",
            style: {
              label: "data(label)",
              "background-color": "#e1f1c9",
              "border-color": "#8eaa7f",
              "border-width": 4,
              opacity: 1,
              "text-opacity": 1,
              "z-index": 20,
            },
          },
        ],
      });

      search.disabled = true;
      for (let index = 0; index < elements.length; index += 40) {
        cy.batch(() => cy.add(elements.slice(index, index + 40)));
        await new Promise(resolve => setTimeout(resolve, 0));
      }

      function relax(elements) {
        const variants = elements.nodes("[?isVariant]");
        // Reserve the full two-line label footprint while solving, then draw compact dots.
        variants.style({ width: 180, height: 120, label: "" });
        elements.layout({
          ...reductionLayoutOptions,
          idealEdgeLength: 95,
          edgeElasticity: 0.45,
        }).run();
        variants.removeStyle("width height label");
      }
      const expanded = new Set();
      const browserSearch = document.querySelector("#browser-search");
      const browserList = document.querySelector("#browser-list");
      let browseMode = "problems";
      let selectedProblem = "";
      let selectedRules = new Set();
      const pathControls = document.querySelector("#path-controls");
      const pathSource = document.querySelector("#path-source");
      const pathTarget = document.querySelector("#path-target");
      const pathFront = document.querySelector("#path-front");
      const canvasNote = document.querySelector(".graph-canvas-note");
      const overviewNote = canvasNote.textContent;
      let pathOptionsReady = false;
      let pathPicking = "source";
      let pathWorker;
      const pathLimit = document.querySelector("#path-limit");
      const pathCancel = document.querySelector("#path-cancel");
      let pathBatch;
      let selectedPath = 1;
      let routeDownload;
      const overviewPositions = new Map(cy.nodes().map((node) => [node.id(), { ...node.position() }]));
      const variantId = (node) => `${node.name}/${variantKey(node.variant)}`;
      const describeVariant = (node) => Object.entries(node.variant)
        .map(([key, value]) => `${key}: ${value}`).join(", ") || "Default variant";

      function showDetails(title, variant, key, parentKey) {
        inspector.scrollTop = 0;
        window.renderDetails(detail, title, variant, key, parentKey, (problem) => {
          transition(() => selectNode(cy.getElementById(problem.split("/")[0])));
        });
        const atlas = document.createElement("a");
        atlas.className = "detail-atlas";
        atlas.textContent = "Open in Atlas ↗";
        const endpoint = (value) => {
          const [name, variant = ""] = value.split("/");
          return [encodeURIComponent(name), encodeURIComponent(variant)];
        };
        if (key.startsWith("problem:")) {
          const [name, variant] = endpoint(key.slice(8));
          atlas.href = `./index.html#problem/${name}${variant ? `?variant=${variant}` : ""}`;
        } else {
          const [source, target] = key.slice(5).split("->").map(endpoint);
          atlas.href = `./index.html#reduction/${source[0]}/${target[0]}${key.includes("/") ? `?from=${source[1]}&to=${target[1]}` : ""}`;
        }
        detail.querySelector("h2").after(atlas);
        const pdf = document.createElement("a");
        pdf.href = "./reductions.pdf";
        pdf.target = "_blank";
        pdf.rel = "noopener";
        pdf.className = "detail-pdf";
        pdf.textContent = "Open PDF reference ↗";
        detail.append(pdf);
      }

      const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)");
      let finishMotion;
      let layoutLoaded;
      let transitionRequest = 0;

      async function transition(change) {
        const request = ++transitionRequest;
        try {
          layoutLoaded ||= new Promise((resolve, reject) => {
            const script = document.createElement("script");
            script.src = "./assets/layout-bundle.js";
            script.onload = resolve;
            script.onerror = () => reject(new Error("Cannot load graph layout. Reload the page to get the current release."));
            document.head.append(script);
          });
          await layoutLoaded;
        } catch (error) {
          layoutLoaded = null;
          detail.textContent = error.message;
          detail.setAttribute("role", "alert");
          return;
        }
        if (request !== transitionRequest) return;
        if (finishMotion) finishMotion();
        if (reducedMotion.matches) {
          change();
          return;
        }
        const before = new Map(cy.nodes().map((node) => [node.id(), {
          position: { ...node.position() }, data: { ...node.data() },
        }]));
        const fromView = { zoom: cy.zoom(), pan: { ...cy.pan() } };
        change();
        const toView = { zoom: cy.zoom(), pan: { ...cy.pan() } };
        const moving = cy.nodes().not(":parent").map((node) => ({
          node,
          from: before.get(node.id())?.position || before.get(node.data("parent")).position,
          to: { ...node.position() },
        }));
        const exiting = [];
        before.forEach(({ data, position }, id) => {
          if (!data.isVariant || cy.getElementById(id).nonempty()) return;
          const target = cy.getElementById(data.parent).position();
          const ghostData = { ...data };
          delete ghostData.parent;
          const ghost = cy.add({ data: ghostData, position, classes: "exiting" });
          ghost.style("events", "no");
          exiting.push(ghost);
          moving.push({ node: ghost, from: position, to: { ...target } });
        });
        const mix = (a, b, t) => a + (b - a) * t;
        let frame;
        function draw(t) {
          cy.batch(() => {
            moving.forEach(({ node, from, to }) => node.position({
              x: mix(from.x, to.x, t), y: mix(from.y, to.y, t),
            }));
            exiting.forEach((node) => node.style("opacity", 1 - t));
          });
          cy.viewport({ zoom: mix(fromView.zoom, toView.zoom, t), pan: {
            x: mix(fromView.pan.x, toView.pan.x, t),
            y: mix(fromView.pan.y, toView.pan.y, t),
          } });
        }
        finishMotion = () => {
          cancelAnimationFrame(frame);
          draw(1);
          exiting.forEach((node) => node.remove());
          finishMotion = null;
        };
        const start = performance.now();
        draw(0);
        function tick(now) {
          const progress = Math.min(1, (now - start) / 350);
          draw(progress * progress * (3 - 2 * progress));
          if (progress < 1) frame = requestAnimationFrame(tick);
          else finishMotion();
        }
        frame = requestAnimationFrame(tick);
      }

      function restoreOverview() {
        cy.elements().removeClass("focus-hidden named");
        cy.nodes().not(":parent").not("[?isVariant]").forEach((node) => {
          node.position(overviewPositions.get(node.id()));
        });
      }

      function arrangeNeighborhood(nodes, edges) {
        const neighbors = edges.connectedNodes().difference(nodes).not(":parent");
        const focus = nodes.union(neighbors).union(edges).union(nodes.ancestors());
        if (nodes.first().isParent()) {
          relax(focus);
        }
        cy.elements().difference(focus).addClass("focus-hidden");
        return focus;
      }

      function rebuildEdges() {
        const edges = new Map();
        data.edges.forEach((rule) => {
          const from = data.nodes[rule.source];
          const to = data.nodes[rule.target];
          const source = expanded.has(from.name) ? variantId(from) : from.name;
          const target = expanded.has(to.name) ? variantId(to) : to.name;
          if (source === target) return;
          const id = `edge:${source}:${target}`;
          const edge = edges.get(id) || { id, source, target, count: 0 };
          edge.count += 1;
          edges.set(id, edge);
        });
        cy.edges().remove();
        cy.add([...edges.values()].map((edge) => ({ data: edge })));
      }

      function toggleVariants(node) {
        const family = families.get(node.id());
        if (family.variants.length < 2) return;
        const center = { ...node.position() };
        if (expanded.has(node.id())) {
          node.children().remove();
          node.removeClass("expanded").removeStyle("font-size").position(center);
          expanded.delete(node.id());
        } else {
          expanded.add(node.id());
          node.addClass("expanded");
          const columns = Math.ceil(Math.sqrt(family.variants.length));
          const rows = Math.ceil(family.variants.length / columns);
          cy.add(family.variants.map((variant, index) => ({
            data: {
              id: variantId(variant),
              parent: node.id(),
              isVariant: true,
              label: Object.entries(variant.variant)
                .map(([key, value]) => `${key}: ${value}`).join("\n"),
              category: family.category,
              rules: 0,
            },
            position: {
              x: center.x + (index % columns - (columns - 1) / 2) * 175,
              y: center.y + (Math.floor(index / columns) - (rows - 1) / 2) * 105,
            },
          })));
        }
        rebuildEdges();
      }

      const visibleElements = () =>
        cy.elements().filter((element) => element.style("display") !== "none");

      function sizeFocusLabels() {
        const zoom = cy.zoom();
        cy.startBatch();
        cy.nodes().not(":parent").not("[?isVariant]")
          .style("font-size", (node) => (node.hasClass("selected") ? 14 : 13) / zoom);
        cy.nodes(".expanded").style("font-size", 18);
        cy.nodes(".route-node").style({ "font-size": 13 / zoom, "text-max-width": 220 / zoom });
        cy.endBatch();
      }
      let labelFrame;
      function updateLabels() {
        sizeFocusLabels();
          cy.nodes().removeClass("named");
          const reserved = cy.nodes("[?isVariant], :parent, .hovered, .selected")
            .filter((node) => node.visible())
            .map((node) => node.renderedBoundingBox({ includeNodes: false, includeLabels: true }));
          const candidates = cy.nodes().not(":parent").not("[?isVariant]").not(".selected, .hovered")
            .filter((node) => node.visible() && !node.hasClass("faded"))
            .sort((a, b) => b.data("rules") - a.data("rules") || a.id().localeCompare(b.id()));
          candidates.addClass("named");
          const hidden = [];
          candidates.forEach((node) => {
              const box = node.renderedBoundingBox({ includeNodes: false, includeLabels: true });
              const overlaps = reserved.some((other) => box.x1 < other.x2 + 8 &&
                box.x2 > other.x1 - 8 && box.y1 < other.y2 + 5 && box.y2 > other.y1 - 5);
              if (overlaps || box.x1 < 12 || box.x2 > cy.width() - 12 ||
                box.y1 < 12 || box.y2 > cy.height() - 55) hidden.push(node);
              else reserved.push(box);
            });
          cy.collection(hidden).removeClass("named");
      }
      cy.on("zoom pan", () => {
        cancelAnimationFrame(labelFrame);
        labelFrame = requestAnimationFrame(updateLabels);
      });

      function fit(focus = visibleElements(), maxZoom = 1) {
        cy.resize();
        cy.fit(focus, Math.max(32, Math.min(cy.width(), cy.height()) * 0.1));
        cy.zoom(Math.min(cy.zoom(), maxZoom));
        cy.center(focus);
        cancelAnimationFrame(labelFrame);
        updateLabels();
      }

      function applyScope() {
        cy.nodes().removeClass("core-hidden");
        if (scope === "core") cy.nodes("[?peripheral]").addClass("core-hidden");
      }

      function clearSelection() {
        clearPathView();
        if (browseMode === "paths") setBrowseMode("problems");
        [...expanded].forEach((name) => toggleVariants(cy.getElementById(name)));
        restoreOverview();
        cy.elements().removeClass("selected neighbor faded search-match");
        selectionType.textContent = "Details";
        detail.replaceChildren();
        applyScope();
        fit();
        selectedProblem = "";
        selectedRules.clear();
        renderBrowser();
      }

      function openInspector(type, focus) {
        selectionType.textContent = type;
        fit(focus, focus.nodes(":parent").empty() && focus.nodes().length <= 12 ? 1.8 : 1);
      }

      function selectNode(node) {
        if (browseMode === "paths") setBrowseMode("problems");
        clearPathView();
        cy.elements().removeClass("focus-hidden");
        if (!node.data("isVariant")) {
          const wasExpanded = expanded.has(node.id());
          [...expanded].forEach((name) => toggleVariants(cy.getElementById(name)));
          restoreOverview();
          if (wasExpanded) {
            clearSelection();
            return;
          }
          toggleVariants(node);
        }
        cy.elements().removeClass("selected neighbor faded search-match");
        cy.elements().addClass("faded");
        const nodes = node.union(node.children());
        nodes.removeClass("faded core-hidden").addClass("selected");
        node.ancestors().removeClass("faded core-hidden");
        const edges = nodes.connectedEdges();
        edges.removeClass("faded");
        if (node.data("isVariant")) edges.addClass("selected");
        edges.connectedNodes().difference(nodes).removeClass("faded core-hidden").addClass("neighbor");
        edges.connectedNodes().ancestors().removeClass("faded core-hidden");
        const focus = arrangeNeighborhood(nodes, edges);
        openInspector("Problem", focus);
        selectedProblem = node.id();
        const familyName = node.data("isVariant") ? node.data("parent") : node.id();
        const variant = node.data("isVariant")
          ? families.get(familyName).variants.find((item) => variantId(item) === node.id()) : null;
        showDetails(displayName(familyName), variant ? describeVariant(variant) : "",
          `problem:${node.id()}`, `problem:${familyName}`);
        setBrowseMode("problems");
        browserList.querySelector('[aria-current="true"]')?.scrollIntoView({ block: "nearest" });
      }

      function selectEdge(edge) {
        if (browseMode === "paths") setBrowseMode("rules");
        clearPathView();
        cy.elements().removeClass("selected neighbor faded search-match");
        cy.elements().addClass("faded");
        edge.removeClass("faded").addClass("selected");
        edge.connectedNodes().removeClass("faded core-hidden").addClass("selected");
        edge.connectedNodes().ancestors().removeClass("faded core-hidden");
        openInspector("Reduction", edge.connectedNodes());
        selectedRules = new Set(data.edges.flatMap((rule, index) => {
          const matches = (endpoint, graphNode) => graphNode.data("isVariant")
            ? variantId(data.nodes[endpoint]) === graphNode.id()
            : data.nodes[endpoint].name === graphNode.id();
          return matches(rule.source, edge.source()) && matches(rule.target, edge.target()) ? [index] : [];
        }));
        const rules = [...selectedRules].map((index) => data.edges[index]);
        const source = data.nodes[rules[0].source], target = data.nodes[rules[0].target];
        const parentKey = `rule:${source.name}->${target.name}`;
        showDetails(`${displayName(source.name)} → ${displayName(target.name)}`,
          rules.length === 1 ? `${describeVariant(source)} → ${describeVariant(target)}`
            : `${rules.length} concrete rules. Choose a variant in the list.`,
          rules.length === 1 ? `rule:${variantId(source)}->${variantId(target)}` : parentKey, parentKey);
        setBrowseMode("rules");
        browserList.querySelector('[aria-current="true"]')?.scrollIntoView({ block: "nearest" });
      }

      function setBrowseMode(mode) {
        if (browseMode === "paths" && mode !== "paths") {
          stopPathSearch();
          clearPathView();
          detail.replaceChildren();
          selectionType.textContent = "Details";
          cy.nodes().removeClass("path-source path-target");
          [...expanded].forEach(name => toggleVariants(cy.getElementById(name)));
          restoreOverview();
          canvasNote.textContent = overviewNote;
          applyScope();
          fit();
        }
        if (browseMode !== mode) browserSearch.value = "";
        browseMode = mode;
        pathControls.hidden = mode !== "paths";
        browserSearch.hidden = mode === "paths";
        document.querySelectorAll("[data-browse]").forEach((button) =>
          button.setAttribute("aria-pressed", button.dataset.browse === mode));
        browserSearch.placeholder = `Search ${mode}…`;
        if (mode === "paths") {
          restoreOverview();
          cy.elements().removeClass("selected neighbor faded search-match core-hidden");
          showPathControls();
          return;
        }
        renderBrowser();
      }

      function clearPathView() {
        cy.nodes(".route-node").removeStyle("font-size text-max-width").removeData("routeLabel");
        cy.elements().removeClass("route-node route-edge route-family selected neighbor faded search-match");
        if (routeDownload) URL.revokeObjectURL(routeDownload);
        routeDownload = null;
      }

      const graphEndpointId = endpoint => expanded.has(endpoint.name) ? variantId(endpoint) : endpoint.name;

      function markPathEndpoints() {
        cy.nodes().removeClass("path-source path-target");
        for (const [select, name] of [[pathSource, "source"], [pathTarget, "target"]]) {
          if (select.value !== "") cy.getElementById(graphEndpointId(data.nodes[Number(select.value)]))
            .removeClass("core-hidden faded").addClass(`path-${name}`);
        }
      }

      function setPathPicking(endpoint) {
        pathPicking = endpoint;
        document.querySelectorAll("[data-pick]").forEach(button =>
          button.setAttribute("aria-pressed", String(button.dataset.pick === endpoint)));
        const prompt = `Click a ${endpoint === "source" ? "start" : "end"} node. For a family with several variants, click its exact variant.`;
        document.querySelector("#path-prompt").textContent = prompt;
        canvasNote.textContent = prompt;
      }

      document.querySelectorAll("[data-pick]").forEach(button => button.addEventListener("click", () => {
        setPathPicking(button.dataset.pick);
        cy.elements().removeClass("faded search-match");
        search.value = "";
        fit();
      }));

      function pickPathNode(node) {
        if (!node.data("isVariant") && families.get(node.id()).variants.length > 1) {
          cy.elements().removeClass("search-match hovered direction");
          if (!expanded.has(node.id())) toggleVariants(node);
          relax(visibleElements());
          markPathEndpoints();
          fit(node.union(node.children()), 1.8);
          return;
        }
        const endpoint = node.data("isVariant")
          ? data.nodes.find(item => variantId(item) === node.id()) : families.get(node.id()).variants[0];
        const select = pathPicking === "source" ? pathSource : pathTarget;
        select.value = data.nodes.indexOf(endpoint);
        select.dispatchEvent(new Event("change"));
      }

      function pathMessage(message, error = false) {
        browserList.replaceChildren();
        const paragraph = document.createElement("p");
        paragraph.className = "browser-empty";
        paragraph.textContent = message;
        if (error) paragraph.setAttribute("role", "alert");
        browserList.append(paragraph);
        document.querySelector("#browser-count").textContent = "Path selection";
      }

      function showPathControls() {
        if (!pathOptionsReady) {
          const options = data.nodes.map((node, index) => ({ node, index }))
            .sort((a, b) => displayName(a.node.name).localeCompare(displayName(b.node.name)) ||
              variantKey(a.node.variant).localeCompare(variantKey(b.node.variant)));
          for (const select of [pathSource, pathTarget]) {
            options.forEach(({node, index}) => select.add(new Option(
              `${displayName(node.name)} / ${describeVariant(node)}`, index)));
          }
          pathControls.querySelector("fieldset").disabled = false;
          pathOptionsReady = true;
        }
        if (pathBatch) {
          renderPathList();
          showPath(selectedPath);
        } else {
          selectionType.textContent = "Path selection";
          detail.replaceChildren();
          pathMessage("Click a start and end on the graph, or choose exact variants in the right panel.");
          markPathEndpoints();
          setPathPicking(pathSource.value === "" ? "source" : "target");
          fit();
        }
      }

      for (const select of [pathSource, pathTarget]) select.addEventListener("change", () => {
        stopPathSearch();
        clearPathView();
        pathBatch = null;
        detail.replaceChildren();
        markPathEndpoints();
        setPathPicking(select === pathSource ? "target" : "source");
        if (pathSource.value !== "" && pathTarget.value !== "") pathControls.requestSubmit();
        else pathMessage("Choose the other endpoint on the graph or in the right panel.");
      });

      function stopPathSearch() {
        transitionRequest++;
        pathWorker?.terminate();
        pathWorker = null;
        pathControls.querySelector("fieldset").disabled = false;
        pathCancel.hidden = true;
      }
      pathCancel.addEventListener("click", () => {
        stopPathSearch();
        pathMessage("Search cancelled. Choose endpoints and find paths again.");
      });

      pathControls.addEventListener("submit", event => {
        event.preventDefault();
        stopPathSearch();
        clearPathView();
        markPathEndpoints();
        detail.replaceChildren();
        selectionType.textContent = "Paths";
        pathBatch = null;
        const source = Number(pathSource.value), target = Number(pathTarget.value);
        if (source === target) {
          pathMessage("Choose two different exact variants.");
          applyScope();
          fit();
          return;
        }
        pathMessage("Computing paths and comparing certificates…");
        pathControls.querySelector("fieldset").disabled = true;
        pathCancel.hidden = false;
        const worker = new Worker("./assets/path-worker.js", {type: "module"});
        pathWorker = worker;
        worker.onmessage = ({data: message}) => {
          if (pathWorker !== worker) return;
          stopPathSearch();
          if (message.error) { pathMessage(message.error, true); return; }
          pathBatch = message.result;
          pathFront.checked = true;
          selectedPath = pathBatch.polynomial_comparison.front[0];
          if (!pathBatch.paths.length) {
            pathBatch = null;
            pathMessage("No witness-capable reduction path connects these exact variants.");
            applyScope();
            fit();
          } else {
            transition(() => {
              renderPathList();
              showPath(selectedPath);
            });
          }
        };
        worker.onerror = () => {
          if (pathWorker !== worker) return;
          stopPathSearch();
          pathMessage("Cannot load the path engine. Reload this release and try again.", true);
        };
        worker.postMessage({graph: data, source, target, limit: Number(pathLimit.value)});
      });

      function pathStatus(index) {
        const winner = pathBatch.polynomial_comparison.pairs.find(pair => pair.dominates?.loser === index);
        return winner ? `Dominated by Path ${winner.dominates.winner}` : "Candidate front";
      }

      function renderPathList() {
        if (!pathBatch) return;
        const comparison = pathBatch.polynomial_comparison;
        document.querySelector("#browser-count").textContent =
          `${comparison.front.length} candidates · ${pathBatch.paths.length} paths compared${pathBatch.truncated ? " · more paths exist" : ""}`;
        browserList.replaceChildren();
        pathBatch.paths.forEach((route, i) => {
          if (pathFront.checked && !comparison.front.includes(i + 1)) return;
          const button = document.createElement("button");
          button.className = "browser-item";
          button.dataset.path = i + 1;
          button.setAttribute("aria-current", String(selectedPath === i + 1));
          button.textContent = `Path ${i + 1} · ${route.steps} reductions`;
          const status = document.createElement("small");
          status.textContent = pathStatus(i + 1);
          button.append(status);
          browserList.append(button);
        });
      }

      pathFront.addEventListener("change", () => {
        if (!pathBatch) return;
        if (pathFront.checked && !pathBatch.polynomial_comparison.front.includes(selectedPath)) {
          selectedPath = pathBatch.polynomial_comparison.front[0];
        }
        showPath(selectedPath);
        renderPathList();
      });

      function explainDirection(direction) {
        if (direction.status === "Unknown") {
          const reason = direction.missing_obligation;
          return `Unresolved: ${reason.detail || (reason.kind === "validity" ?
            "no regular, positive counterexample found across this batch" : reason.kind.replaceAll("_", " "))}`;
        }
        const proof = direction.certificate;
        if (proof.kind === "identity") return "Yes: equal after exact expansion.";
        if (proof.kind === "zero") return "Yes: the numerator is identically zero.";
        const monomial = powers => Object.entries(powers)
          .map(([name, exponent]) => exponent === "1" ? name : `${name}^${exponent}`).join(" × ") || "1";
        if (proof.kind === "newton") return "Yes: positive-support coverage\n" + proof.coverage.map(item =>
          `${monomial(item.exponent)} ≤ ${item.weights.map(term => `${term.weight} · (${monomial(term.exponent)})`).join(" + ")}`).join("\n");
        return "No: a regular, positive counterexample for every certificate in this batch\n" +
          Object.entries(proof.curve).map(([name, value]) =>
            `${name} = ${value.scale}${value.exponent === "0" ? "" : ` · t^${value.exponent}`}`).join(", ");
      }

      function showPath(index) {
        clearPathView();
        const route = pathBatch.paths[index - 1];
        const steps = [route.path[0].from, ...route.path.map(edge => edge.to)];
        new Set(steps.map(endpoint => endpoint.name)).forEach(name => {
          if (!expanded.has(name)) toggleVariants(cy.getElementById(name));
        });
        restoreOverview();
        cy.nodes().removeClass("core-hidden");
        const nodes = cy.collection(steps.map(endpoint => cy.getElementById(graphEndpointId(endpoint))));
        const edges = cy.collection(route.path.map(edge => cy.getElementById(
          `edge:${graphEndpointId(edge.from)}:${graphEndpointId(edge.to)}`)));
        relax(visibleElements());
        cy.elements().addClass("faded");
        steps.forEach(endpoint => cy.getElementById(graphEndpointId(endpoint)).data("routeLabel",
          `${displayName(endpoint.name)}\n${Object.values(endpoint.variant).join(" · ")}`));
        nodes.removeClass("faded").addClass("route-node selected");
        nodes.ancestors().removeClass("faded").addClass("route-family");
        edges.removeClass("faded").addClass("route-edge");
        markPathEndpoints();
        canvasNote.textContent = `Path ${index} · ${pathStatus(index)} · start: blue, end: orange`;
        fit(nodes.union(edges), 1.2);
        selectionType.textContent = "Path comparison";
        detail.replaceChildren();
        const heading = document.createElement("h2");
        heading.textContent = `Path ${index}`;
        detail.append(heading);
        const status = document.createElement("p");
        status.textContent = pathStatus(index);
        detail.append(status);
        const list = document.createElement("ol");
        list.className = "path-route";
        steps.forEach(endpoint => {
          const item = document.createElement("li");
          item.textContent = displayName(endpoint.name);
          const variant = document.createElement("small");
          variant.textContent = describeVariant(endpoint);
          item.append(variant);
          list.append(item);
        });
        detail.append(list);
        const profile = pathBatch.polynomial_comparison.profiles[index - 1];
        Object.entries(profile.fields).forEach(([name, field]) => {
          const block = document.createElement("div");
          block.className = "path-field";
          const title = document.createElement("strong");
          title.textContent = name;
          const formula = document.createElement("code");
          formula.textContent = field.certificate === null ? `Unavailable: ${field.accuracy.reason.detail || field.accuracy.reason.kind.replaceAll("_", " ")}` :
            `${field.accuracy.kind === "exact" ? "=" : "≤"} ${field.certificate}`;
          block.append(title, formula);
          detail.append(block);
        });
        routeDownload = URL.createObjectURL(new Blob([JSON.stringify(route, null, 2)], {type:"application/json"}));
        const download = document.createElement("a");
        download.href = routeDownload;
        download.download = `reduction-path-${index}.json`;
        download.className = "path-download";
        download.textContent = "Download route for pred reduce";
        detail.append(download);
        const label = document.createElement("label");
        label.textContent = "Compare with";
        const select = document.createElement("select");
        select.className = "path-compare-select";
        select.setAttribute("aria-label", "Compare with path");
        select.add(new Option("Choose another path", ""));
        pathBatch.paths.forEach((_, i) => {
          if (i + 1 !== index && (!pathFront.checked || pathBatch.polynomial_comparison.front.includes(i + 1)))
            select.add(new Option(`Path ${i + 1}`, i + 1));
        });
        const proof = document.createElement("div");
        select.addEventListener("change", () => {
          proof.replaceChildren();
          if (!select.value) return;
          const other = Number(select.value);
          const pair = pathBatch.polynomial_comparison.pairs.find(pair => pair.paths.includes(index) && pair.paths.includes(other));
          const verdict = document.createElement("p");
          verdict.textContent = pair.dominates ? `Path ${pair.dominates.winner} dominates Path ${pair.dominates.loser}` :
            pair.verdict === "Unknown" ? "Unresolved — this pair does not eliminate either path" : pair.verdict;
          proof.append(verdict);
          Object.entries(pair.fields).forEach(([name, directions]) => {
            const block = document.createElement("details");
            block.className = "path-proof";
            const summary = document.createElement("summary");
            summary.textContent = `${name}: ${directions.forward.status} / ${directions.reverse.status}`;
            const text = document.createElement("pre");
            text.textContent = `Path ${pair.paths[0]} ≤O Path ${pair.paths[1]}\n${explainDirection(directions.forward)}\n\n` +
              `Path ${pair.paths[1]} ≤O Path ${pair.paths[0]}\n${explainDirection(directions.reverse)}`;
            block.append(summary, text);
            proof.append(block);
          });
        });
        label.append(select);
        detail.append(label, proof);
        const note = document.createElement("p");
        note.className = "path-note";
        note.textContent = "Smaller certificates are better. Unknown comparisons retain candidates. Exact-cancellation families are excluded; this is not a per-instance runtime guarantee.";
        detail.append(note);
        inspector.scrollTop = 0;
      }

      function renderBrowser() {
        if (browseMode === "paths") { renderPathList(); return; }
        const query = browserSearch.value.trim().toLowerCase();
        const fragment = document.createDocumentFragment();
        let count = 0;
        function item(title, subtitle, selected) {
          const button = document.createElement("button");
          button.className = "browser-item";
          button.setAttribute("aria-current", String(selected));
          button.textContent = title;
          if (subtitle) {
            const detail = document.createElement("small");
            detail.textContent = subtitle;
            button.append(detail);
          }
          fragment.append(button);
          return button;
        }
        const variantText = (node) => Object.entries(node.variant)
          .map(([key, value]) => `${key}: ${value}`).join(", ");
        if (browseMode === "problems") {
          [...families.values()].sort((a, b) => displayName(a.name).localeCompare(displayName(b.name)))
            .forEach((family) => {
              const schema = schemas.get(family.name);
              const haystack = [family.name, displayName(family.name), schema?.description,
                ...(schema?.aliases || []), ...family.variants.map(variantText)].join(" ").toLowerCase();
              if (!haystack.includes(query) && aliases.get(query.toUpperCase()) !== family.name) return;
              count++;
              const button = item(displayName(family.name), family.variants.length > 1 ? `${family.variants.length} variants` : "", selectedProblem === family.name);
              button.dataset.family = family.name;
              if (family.variants.length > 1) button.setAttribute("aria-expanded", String(expanded.has(family.name)));
              if (expanded.has(family.name)) family.variants.forEach((variant) => {
                const child = item(variantText(variant), "", selectedProblem === variantId(variant));
                child.classList.add("variant-item");
                child.dataset.variant = variantId(variant);
              });
            });
        } else {
          data.edges.forEach((rule, index) => {
            const source = data.nodes[rule.source], target = data.nodes[rule.target];
            const title = `${displayName(source.name)} → ${displayName(target.name)}`;
            const sourceVariant = variantText(source), targetVariant = variantText(target);
            const subtitle = sourceVariant && targetVariant ? `${sourceVariant} → ${targetVariant}`
              : sourceVariant ? `Source: ${sourceVariant}` : targetVariant ? `Target: ${targetVariant}` : "";
            const haystack = `${title} ${subtitle} ${source.name} ${target.name}`.toLowerCase();
            const alias = aliases.get(query.toUpperCase());
            if (!haystack.includes(query) && !(alias && [source.name, target.name].includes(alias))) return;
            count++;
            item(title, subtitle, selectedRules.has(index)).dataset.rule = index;
          });
        }
        document.querySelector("#browser-count").textContent = `${count} ${browseMode}`;
        if (!count) {
          const empty = document.createElement("p");
          empty.className = "browser-empty";
          empty.textContent = "No matches. Try another search.";
          fragment.append(empty);
        }
        // Re-rendering replaces the focused button; keyboard focus follows its counterpart.
        const focused = browserList.contains(document.activeElement) && document.activeElement.dataset;
        browserList.replaceChildren(fragment);
        if (focused) [...browserList.querySelectorAll("button")].find((button) =>
          ["family", "variant", "rule"].every((key) => button.dataset[key] === focused[key]))?.focus();
      }

      browserSearch.addEventListener("input", renderBrowser);
      document.querySelectorAll("[data-browse]").forEach((button) =>
        button.addEventListener("click", () => setBrowseMode(button.dataset.browse)));
      browserList.addEventListener("click", (event) => {
        const button = event.target.closest("button");
        if (!button) return;
        if (button.dataset.path) {
          selectedPath = Number(button.dataset.path);
          renderPathList();
          showPath(selectedPath);
          return;
        }
        transition(() => {
          if (button.dataset.family) selectNode(cy.getElementById(button.dataset.family));
          else if (button.dataset.variant) selectNode(cy.getElementById(button.dataset.variant));
          else {
            const rule = data.edges[Number(button.dataset.rule)];
            [...expanded].forEach((name) => toggleVariants(cy.getElementById(name)));
            restoreOverview();
            const source = data.nodes[rule.source], target = data.nodes[rule.target];
            new Set([source.name, target.name]).forEach((name) => toggleVariants(cy.getElementById(name)));
            const from = expanded.has(source.name) ? variantId(source) : source.name;
            const to = expanded.has(target.name) ? variantId(target) : target.name;
            const edge = cy.getElementById(`edge:${from}:${to}`);
            const endpoints = edge.connectedNodes();
            const groups = endpoints.ancestors().union(endpoints).union(endpoints.ancestors().children());
            groups.removeClass("core-hidden");
            arrangeNeighborhood(groups, edge);
            selectEdge(edge);
          }
        });
      });

      function applySearch() {
        if (browseMode === "paths") {
          cy.elements().removeClass("faded search-match");
          const query = search.value.trim().toLowerCase();
          const matches = cy.nodes().not("[?isVariant]").filter(node =>
            aliases.get(query.toUpperCase()) === node.id() ||
            `${node.id()} ${displayName(node.id())}`.toLowerCase().includes(query));
          if (query) matches.addClass("search-match");
          fit(query && matches.nonempty() ? matches.union(matches.children()) : visibleElements());
          return;
        }
        [...expanded].forEach((name) => toggleVariants(cy.getElementById(name)));
        restoreOverview();
        selectedProblem = "";
        selectedRules.clear();
        renderBrowser();
        const query = search.value.trim().toLowerCase();
        cy.elements().removeClass("selected neighbor faded search-match");
        selectionType.textContent = "Details";
        detail.replaceChildren();
        applyScope();
        if (!query) {
          fit();
          return;
        }
        const matches = cy.nodes().filter((node) => {
          const schema = schemas.get(node.id());
          return (
            aliases.get(query.toUpperCase()) === node.id() ||
            [
              node.id(),
              displayName(node.id()),
              schema?.description,
              ...(schema?.aliases || []),
            ]
              .filter(Boolean)
              .join(" ")
              .toLowerCase()
              .includes(query)
          );
        });
        cy.nodes().removeClass("core-hidden").addClass("faded");
        cy.edges().addClass("faded");
        matches.removeClass("faded").addClass("search-match");
        matches.connectedEdges().removeClass("faded");
        matches.connectedEdges().connectedNodes().removeClass("faded");
        if (matches.length) fit(matches.union(matches.connectedEdges()));
      }

      cy.on("tap", "node", event => transition(() => {
        if (browseMode === "paths") pickPathNode(event.target);
        else selectNode(event.target);
      }));
      cy.on("tap", "edge", event => {
        if (browseMode !== "paths") transition(() => selectEdge(event.target));
      });
      cy.on("tap", event => {
        if (event.target === cy && browseMode !== "paths") transition(clearSelection);
      });
      cy.on("mouseover", "node", (event) => {
        event.target.addClass("hovered");
        event.target.connectedEdges().addClass("direction");
        if (event.target.data("isVariant")) {
          tooltip.textContent = `${displayName(event.target.parent().id())} · ${event.target.data("label")}`;
          tooltip.style.display = "block";
          return;
        }
        const family = families.get(event.target.id());
        tooltip.textContent = `${displayName(family.name)} · ${family.variants.length} variant${family.variants.length === 1 ? "" : "s"} · ${family.rules} reductions`;
        tooltip.style.display = "block";
      });
      cy.on("mouseover", "edge", (event) => {
        event.target.addClass("hovered");
        const edge = event.target;
        tooltip.textContent = `${edge.source().data("label")} → ${edge.target().data("label")}${edge.hasClass("route-edge") ? "" : ` · ${edge.data("count")} concrete rule${edge.data("count") === 1 ? "" : "s"}`}`;
        tooltip.style.display = "block";
      });
      cy.on("mousemove", "node, edge", (event) => {
        tooltip.style.left = `${event.originalEvent.clientX + 14}px`;
        tooltip.style.top = `${event.originalEvent.clientY + 14}px`;
      });
      cy.on("mouseout", "node, edge", (event) => {
        event.target.removeClass("hovered");
        if (event.target.isNode()) event.target.connectedEdges().removeClass("direction");
        tooltip.style.display = "none";
      });

      document.querySelectorAll("[data-scope]").forEach((button) =>
        button.addEventListener("click", () => {
          scope = button.dataset.scope;
          document.querySelectorAll("[data-scope]").forEach((item) =>
            item.setAttribute("aria-pressed", item === button),
          );
          search.value = "";
          transition(clearSelection);
        }),
      );
      document.querySelector("#reset-graph").addEventListener("click", () => {
        search.value = "";
        transition(clearSelection);
      });
      const workspace = document.querySelector(".graph-workspace");
      const divider = document.querySelector(".graph-divider");
      const maxPanelWidth = () => (workspace.clientWidth - document.querySelector(".graph-browser").offsetWidth) * 0.6;
      function resizePanel(width) {
        const size = Math.max(240, Math.min(maxPanelWidth(), width));
        workspace.style.setProperty("--inspector-width", `${size}px`);
      }
      function describeDivider() {
        divider.setAttribute("aria-valuenow", Math.round(inspector.getBoundingClientRect().width));
        divider.setAttribute("aria-valuemax", Math.round(maxPanelWidth()));
      }
      describeDivider();
      divider.addEventListener("pointerdown", (event) => {
        if (event.button !== 0) return;
        if (finishMotion) finishMotion();
        event.preventDefault();
        divider.focus();
        divider.setPointerCapture(event.pointerId);
        divider.classList.add("dragging");
      });
      divider.addEventListener("pointermove", (event) => {
        if (divider.hasPointerCapture(event.pointerId)) {
          resizePanel(workspace.getBoundingClientRect().right - event.clientX - 4);
        }
      });
      divider.addEventListener("pointerup", (event) => {
        if (divider.hasPointerCapture(event.pointerId)) divider.releasePointerCapture(event.pointerId);
      });
      divider.addEventListener("lostpointercapture", () => divider.classList.remove("dragging"));
      divider.addEventListener("keydown", (event) => {
        const widths = { ArrowLeft: inspector.clientWidth + 20,
          ArrowRight: inspector.clientWidth - 20, Home: 240, End: maxPanelWidth() };
        if (!(event.key in widths)) return;
        event.preventDefault();
        if (finishMotion) finishMotion();
        resizePanel(widths[event.key]);
      });
      const canvas = document.querySelector(".graph-canvas");
      let canvasWidth = canvas.clientWidth;
      let canvasHeight = canvas.clientHeight;
      new ResizeObserver(([entry]) => {
        const { width, height } = entry.contentRect;
        if (width === canvasWidth && height === canvasHeight) return;
        cy.resize();
        cy.panBy({ x: (width - canvasWidth) / 2, y: (height - canvasHeight) / 2 });
        canvasWidth = width;
        canvasHeight = height;
        describeDivider();
        updateLabels();
      }).observe(canvas);
      search.addEventListener("input", () => transition(applySearch));
      search.addEventListener("keydown", (event) => {
        if (event.key !== "Enter") return;
        transition(() => {
          applySearch();
          const match = cy.nodes(".search-match").first();
          if (match.nonempty()) {
            if (browseMode === "paths") pickPathNode(match);
            else selectNode(match);
          }
        });
      });
      document.addEventListener("keydown", (event) => {
        const typing =
          /INPUT|TEXTAREA|SELECT/.test(event.target.tagName) ||
          event.target.isContentEditable;
        if (event.key === "/" && !typing) {
          event.preventDefault();
          search.focus();
        }
        if (event.key !== "Escape") return;
        if (!typing) {
          search.value = "";
          transition(clearSelection);
          return;
        }
        // Escape in a field resets only that field, through its own input handler.
        event.preventDefault();
        if (event.target.value) {
          event.target.value = "";
          event.target.dispatchEvent(new Event("input", { bubbles: true }));
        }
        event.target.blur();
      });

      applyScope();
      renderBrowser();
      requestAnimationFrame(() => {
        fit();
        document.querySelector("#cy").removeAttribute("aria-busy");
        workspace.inert = false;
        search.disabled = false;
      });
    } catch (error) {
      document.querySelector(".graph-canvas-note").textContent = `Unable to load graph: ${error.message}`;
      throw error;
    }
})();

/* The site is a static client of the same registry exports as the book and paper. */
(() => {
  "use strict";
  const data = window.REDUCTIONS;
  const main = document.querySelector("main");
  const homeHTML = main.innerHTML;
  const baseTitle = "Problem Reductions";
  const repo = "https://github.com/CodingThrust/problem-reductions";
  const categories = {
    graph: "Graph theory",
    formula: "Logic & formulas",
    set: "Set systems",
    algebraic: "Algebra",
    misc: "Other structures",
  };
  const aliases = {
    MIS: "MaximumIndependentSet",
    MVC: "MinimumVertexCover",
    SAT: "Satisfiability",
    "3SAT": "KSatisfiability",
    "3-SAT": "KSatisfiability",
  };
  let filter = "all";
  let query = "";
  let toastTimer;
  let focusSearchAfterNavigation = false;
  const escape = (value) =>
    String(value ?? "").replace(
      /[&<>"']/g,
      (c) =>
        ({
          "&": "&amp;",
          "<": "&lt;",
          ">": "&gt;",
          '"': "&quot;",
          "'": "&#39;",
        })[c],
    );
  const nameOf = (name) =>
    data.schemas.find((s) => s.name === name)?.display_name ||
    name
      .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
      .replace(/([A-Z])([A-Z][a-z])/g, "$1 $2");
  const variantKey = (node) =>
    Object.entries(node.variant)
      .sort(([a], [b]) => a.localeCompare(b))
      .map(([k, v]) => `${k}=${v}`)
      .join(",");
  const variantLabel = (node) =>
    Object.entries(node.variant)
      .map(([k, v]) => `${k}: ${v}`)
      .join(" · ") || "Default variant";
  const problemHref = (name, node) =>
    `#problem/${encodeURIComponent(name)}${node ? `?variant=${encodeURIComponent(variantKey(node))}` : ""}`;
  const ruleHref = (edge) =>
    `#reduction/${encodeURIComponent(data.nodes[edge.source].name)}/${encodeURIComponent(data.nodes[edge.target].name)}?from=${encodeURIComponent(variantKey(data.nodes[edge.source]))}&to=${encodeURIComponent(variantKey(data.nodes[edge.target]))}`;
  const schemaOf = (name) => data.schemas.find((s) => s.name === name);
  const description = (name) =>
    schemaOf(name)?.description ||
    "Explore the registered variants, reduction contracts, and implementation of this computational problem.";
  const apiHref = (path) => `./api/problemreductions/${path}`;
  const moduleName = (edge) => edge.doc_path.split("/").at(-2);
  const sourceHref = (edge) => `${repo}/blob/main/${edge.source_path}`;
  const families = [...new Set(data.nodes.map((n) => n.name))].sort();
  const variantsOf = (name) =>
    data.nodes
      .map((n, i) => ({ ...n, index: i }))
      .filter((n) => n.name === name);
  const edgesOf = (name) =>
    data.edges.filter(
      (e) =>
        data.nodes[e.source].name === name ||
        data.nodes[e.target].name === name,
    );
  const featuredEdge = data.edges.find(
    (e) =>
      data.nodes[e.source].name === "MinimumVertexCover" &&
      data.nodes[e.target].name === "MaximumIndependentSet",
  );

  function notify(message) {
    const toast = document.querySelector(".toast");
    toast.textContent = message;
    toast.classList.add("visible");
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => toast.classList.remove("visible"), 2800);
  }

  function graphArt() {
    const width = 630,
      height = 430;
    const highlights = {
      MaximumIndependentSet: [310, 242, "Independent Set"],
      MinimumVertexCover: [452, 155, "Vertex Cover"],
      Satisfiability: [145, 162, "Satisfiability"],
      KSatisfiability: [220, 74, "k-SAT"],
      MaximumSetPacking: [449, 314, "Set Packing"],
      QUBO: [158, 335, "QUBO"],
      ILP: [330, 370, "ILP"],
      Coloring: [92, 249, "Coloring"],
    };
    const positions = {};
    families.forEach((name, i) => {
      const angle = i * 2.39996323;
      const radius = Math.sqrt((i + 1) / families.length);
      positions[name] = highlights[name] || [
        315 + Math.cos(angle) * radius * 287,
        218 + Math.sin(angle) * radius * 191,
      ];
    });
    const seen = new Set();
    const lines = data.edges
      .map((e) => {
        const from = data.nodes[e.source].name,
          to = data.nodes[e.target].name;
        if (from === to) return "";
        const key = [from, to].sort().join("/");
        if (seen.has(key)) return "";
        seen.add(key);
        const [x1, y1] = positions[from],
          [x2, y2] = positions[to];
        const featured =
          [from, to].includes("MinimumVertexCover") &&
          [from, to].includes("MaximumIndependentSet");
        return `<line class="network-edge ${featured ? "featured-edge" : ""}" x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}"/>`;
      })
      .join("");
    const nodes = [...families]
      .sort((a, b) => Number(!!highlights[a]) - Number(!!highlights[b]))
      .map((name) => {
        const [x, y] = positions[name];
        const isFocus = [
          "MaximumIndependentSet",
          "MinimumVertexCover",
        ].includes(name);
        const label = highlights[name]?.[2];
        if (!label)
          return `<circle cx="${x}" cy="${y}" r="2.8" fill="var(--graph-node)" stroke="var(--paper)" stroke-width="1" aria-hidden="true"/>`;
        return `<a class="network-node ${label ? "major" : ""} ${isFocus ? "focus" : ""}" href="${problemHref(name)}" aria-label="Explore ${escape(nameOf(name))}"><title>${escape(nameOf(name))}</title><circle cx="${x}" cy="${y}" r="${isFocus ? 8 : label ? 5.5 : 3}"/>${label ? `<text x="${x}" y="${y + (isFocus ? 28 : 21)}" text-anchor="middle">${escape(label)}</text>` : ""}</a>`;
      })
      .join("");
    return `<svg viewBox="0 0 ${width} ${height}" aria-label="Problem-level overview; connections can involve different variants"><g aria-hidden="true">${lines}</g>${nodes}</svg>`;
  }

  const cyclePoints = [
    [150, 35],
    [231, 94],
    [200, 190],
    [100, 190],
    [69, 94],
  ];
  function cycleSVG(mode = "independent", labels = false) {
    const selected = mode === "independent" ? [0, 2] : [1, 3, 4];
    return `<svg viewBox="0 0 300 225" role="img" aria-label="Five-vertex cycle with ${mode === "independent" ? "an independent set of two" : "a vertex cover of three"} highlighted">${cyclePoints
      .map(([x, y], i) => {
        const next = cyclePoints[(i + 1) % cyclePoints.length];
        return `<line x1="${x}" y1="${y}" x2="${next[0]}" y2="${next[1]}" stroke="var(--diagram-line)" stroke-width="1.5"/>`;
      })
      .join(
        "",
      )}${cyclePoints.map(([x, y], i) => `<circle data-selected="${selected.includes(i)}" cx="${x}" cy="${y}" r="${labels ? 15 : 10}" fill="${selected.includes(i) ? "var(--diagram-selected)" : "var(--paper)"}" stroke="${selected.includes(i) ? "var(--diagram-selected)" : "var(--diagram-line)"}" stroke-width="1.5"/>${labels ? `<text x="${x}" y="${y + 4}" text-anchor="middle" font-family="DM Sans, sans-serif" font-size="11" fill="${selected.includes(i) ? "var(--on-accent)" : "var(--diagram-label)"}">${i}</text>` : ""}`).join("")}</svg>`;
  }

  function complementArt() {
    const graph = (offset, selected) =>
      `<g transform="translate(${offset},20) scale(.67)">${cyclePoints.map(([x, y], i) => `<line x1="${x}" y1="${y}" x2="${cyclePoints[(i + 1) % 5][0]}" y2="${cyclePoints[(i + 1) % 5][1]}" stroke="var(--diagram-line)" stroke-width="2"/>`).join("")}${cyclePoints.map(([x, y], i) => `<circle cx="${x}" cy="${y}" r="13" fill="${selected.includes(i) ? "var(--diagram-selected)" : "var(--paper)"}" stroke="var(--diagram-line)" stroke-width="1.5"/>`).join("")}</g>`;
    return `<svg viewBox="0 0 520 245" role="img" aria-label="A vertex cover of size three becomes an independent set of size two by complementing the selection">${graph(15, [1, 3, 4])}${graph(305, [0, 2])}<path d="M235 102h48m-6-6 6 6-6 6" fill="none" stroke="var(--diagram-line)" stroke-width="1.5"/><text x="260" y="133" text-anchor="middle" font-family="monospace" font-size="10" fill="var(--muted)">V ∖ S</text><g font-family="DM Sans, sans-serif" text-anchor="middle" fill="var(--diagram-label)"><text x="115" y="201" font-size="13">Minimum Vertex Cover</text><text x="405" y="201" font-size="13">Maximum Independent Set</text><text x="115" y="224" font-size="10" fill="var(--muted)">3 selected vertices</text><text x="405" y="224" font-size="10" fill="var(--muted)">2 selected vertices</text></g></svg>`;
  }

  function miniArt(name) {
    if (name === "MaximumIndependentSet") return cycleSVG();
    if (name === "Satisfiability")
      return '<svg viewBox="0 0 300 112" role="img" aria-label="Example Boolean clauses"><g font-family="Instrument Serif, Georgia, serif" font-style="italic" font-size="25" text-anchor="middle" fill="var(--diagram-label)"><text x="150" y="39">(x₁ ∨ x₂ ∨ ¬x₃)</text><text x="150" y="68" font-size="19" fill="var(--muted)">∧</text><text x="150" y="97">(¬x₁ ∨ x₃ ∨ x₄)</text></g></svg>';
    return '<svg viewBox="0 0 300 112" role="img" aria-label="A quadratic objective"><text x="150" y="68" text-anchor="middle" font-family="Instrument Serif, Georgia, serif" font-style="italic" font-size="40" fill="var(--diagram-label)">min xᵀQx</text><text x="150" y="93" text-anchor="middle" font-family="monospace" font-size="10" fill="var(--muted)">x ∈ {0, 1}ⁿ</text></svg>';
  }

  function hydrateHome() {
    document.querySelector("#hero-graph").innerHTML = graphArt();
    document.querySelector("#problem-showcase").innerHTML = [
      "MaximumIndependentSet",
      "Satisfiability",
      "QUBO",
    ]
      .filter((name) => families.includes(name))
      .map((name) => {
        const variants = variantsOf(name);
        return `<a class="problem-card" href="${problemHref(name)}"><span class="context-label">${escape(categories[variants[0].category] || "Computational problem")}</span><h3>${escape(nameOf(name))}</h3><p>${escape(description(name))}.</p><div class="mini-art">${miniArt(name)}</div><div class="card-bottom"><span>${variants.length} variant${variants.length === 1 ? "" : "s"} · ${edgesOf(name).length} incident reductions</span><span aria-hidden="true">↗</span></div></a>`;
      })
      .join("");
    document.querySelector("#featured-art").innerHTML = complementArt();
    if (featuredEdge) {
      document.querySelector("#featured-result-link").href =
        ruleHref(featuredEdge);
      document.querySelector(".graph-feature").href = ruleHref(featuredEdge);
    }
  }

  function atlasPage() {
    main.innerHTML = /* HTML */ `<div class="wrap page-header">
        <div class="breadcrumbs">
          <a href="#home">Home</a><span>/</span><span>The atlas</span>
        </div>
        <h1>Find your next <em>connection.</em></h1>
        <p>
          Explore ${families.length} problem families and their implemented
          reductions. Every connection has explicit variants, size bounds, and
          code you can inspect.
        </p>
        <div class="atlas-controls">
          <label class="atlas-search"
            ><span aria-hidden="true">⌕</span
            ><input
              id="atlas-search"
              type="search"
              placeholder="Search problems, aliases, or descriptions…"
              aria-label="Search problems"
              value="${escape(query)}"
              autocomplete="off"
            /><kbd>/</kbd></label
          ><span class="result-count" role="status" aria-live="polite"></span>
          <a class="text-link" href="./reduction-graph.html"
            >Find a reduction path <span aria-hidden="true">↗</span></a
          >
        </div>
      </div>
      <div class="wrap atlas-layout">
        <aside class="filters" aria-label="Filter problems by category">
          <p class="context-label">Input structure</p>
          ${[["all", "All problems"], ...Object.entries(categories)].map(([key, label]) => `<button class="filter-button" data-filter="${key}" aria-pressed="${filter === key}"><span>${label}</span><span>${key === "all" ? families.length : families.filter((name) => variantsOf(name)[0].category === key).length}</span></button>`).join("")}
        </aside>
        <div class="atlas-results" id="atlas-results"></div>
      </div>`;
    renderResults();
    document
      .querySelector("#atlas-search")
      .addEventListener("input", (event) => {
        query = event.target.value;
        renderResults();
      });
    document.querySelectorAll("[data-filter]").forEach((button) =>
      button.addEventListener("click", () => {
        filter = button.dataset.filter;
        document
          .querySelectorAll("[data-filter]")
          .forEach((b) => b.setAttribute("aria-pressed", b === button));
        renderResults();
      }),
    );
  }

  function renderResults() {
    const search = query.trim().toLowerCase();
    const alias = aliases[query.trim().toUpperCase()] || "";
    const results = families.filter((name) => {
      const schema = schemaOf(name);
      return (
        (filter === "all" || variantsOf(name)[0].category === filter) &&
        (!search ||
          name === alias ||
          `${name} ${nameOf(name)} ${description(name)} ${(schema?.aliases || []).join(" ")}`
            .toLowerCase()
            .includes(search))
      );
    });
    document.querySelector(".result-count").textContent =
      `${results.length} of ${families.length} problem families`;
    document.querySelector("#atlas-results").innerHTML = results.length
      ? results
          .map((name) => {
            const variants = variantsOf(name);
            return `<a class="atlas-row" href="${problemHref(name)}"><div><h2>${escape(nameOf(name))}</h2><p>${escape(description(name))}</p><div class="atlas-row-meta"><span>${escape(categories[variants[0].category] || variants[0].category)}</span><span>${variants.length} variant${variants.length === 1 ? "" : "s"}</span><span>${edgesOf(name).length} connections</span></div></div><span aria-hidden="true">↗</span></a>`;
          })
          .join("")
      : `<div class="empty-state"><h2>No matching problems.</h2><p>Try a broader term or explore another input structure.</p><button class="button secondary" id="reset-search">Reset search and filters</button></div>`;
    document.querySelector("#reset-search")?.addEventListener("click", () => {
      query = "";
      filter = "all";
      atlasPage();
      document.querySelector("#atlas-search").focus();
    });
  }

  function relation(edge, currentIndex) {
    const source = data.nodes[edge.source],
      target = data.nodes[edge.target];
    const outgoing = edge.source === currentIndex;
    const other = outgoing ? target : source;
    return `<a class="relation-link" href="${ruleHref(edge)}"><div class="relation-title"><span>${outgoing ? "→ " : "← "}${escape(nameOf(other.name))}</span><span aria-hidden="true">↗</span></div><p>${escape(variantLabel(other))}</p><p class="capability">${[edge.witness && "Witness recovery", edge.aggregate && "Aggregate value", edge.turing && "Turing reduction"].filter(Boolean).join(" · ") || "See reduction contract"}</p></a>`;
  }

  function demoPanel() {
    return `<div class="demo-panel"><div class="demo-toolbar"><span>Unit-weight example · five-vertex cycle</span><div class="segmented" aria-label="Choose highlighted solution"><button data-demo="independent" aria-pressed="true">Independent set</button><button data-demo="cover" aria-pressed="false">Vertex cover</button></div></div><div id="demo-graph">${cycleSVG("independent", true)}</div><div class="demo-caption" role="status" aria-live="polite"><span id="demo-description">No two selected vertices share an edge.</span><strong id="demo-value">Maximum size: 2</strong></div></div>`;
  }

  function bindDemo() {
    document.querySelectorAll("[data-demo]").forEach((button) =>
      button.addEventListener("click", () => {
        const mode = button.dataset.demo;
        document
          .querySelectorAll("[data-demo]")
          .forEach((b) => b.setAttribute("aria-pressed", b === button));
        document.querySelector("#demo-graph").innerHTML = cycleSVG(mode, true);
        document.querySelector("#demo-description").textContent =
          mode === "independent"
            ? "No two selected vertices share an edge."
            : "Every edge touches a selected vertex.";
        document.querySelector("#demo-value").textContent =
          mode === "independent" ? "Maximum size: 2" : "Minimum size: 3";
      }),
    );
  }

  function codePanel(command) {
    return `<div class="code-panel"><div class="terminal-header"><button class="copy-button" data-copy="${escape(command)}" aria-label="Copy command">Copy</button></div><pre>${escape(command)}</pre></div>`;
  }

  function problemPage(name, params) {
    const variants = variantsOf(name);
    if (!variants.length) return notFound();
    const requested = params.get("variant");
    if (
      requested !== null &&
      !variants.some((n) => variantKey(n) === requested)
    )
      return notFound();
    const node =
      variants.find((n) => variantKey(n) === requested) ||
      variants.find(
        (n) => n.variant.graph === "SimpleGraph" && n.variant.weight === "One",
      ) ||
      variants[0];
    const schema = schemaOf(name);
    const incoming = data.edges.filter((e) => e.target === node.index);
    const outgoing = data.edges.filter((e) => e.source === node.index);
    const mis = name === "MaximumIndependentSet";
    document.title = `${nameOf(name)} — ${baseTitle}`;
    main.innerHTML = /* HTML */ `<div class="wrap page-header">
        <div class="breadcrumbs">
          <a href="#home">Home</a><span>/</span><a href="#atlas">Atlas</a
          ><span>/</span><span>${escape(nameOf(name))}</span>
        </div>
        <h1>${escape(nameOf(name))}<span class="brand-period">.</span></h1>
        <p>${escape(description(name))}.</p>
        <div class="variant-control">
          <label for="variant-select">Explore a concrete variant</label
          ><select id="variant-select">
            ${variants.map((n) => `<option value="${escape(variantKey(n))}" ${n.index === node.index ? "selected" : ""}>${escape(variantLabel(n))}</option>`).join("")}
          </select>
        </div>
      </div>
      <div class="wrap reading-layout">
        <div class="reading-content">
          <section class="reading-section">
            <h2>
              ${mis ? "A simple question. A rich search space." : "The problem"}
            </h2>
            ${mis ? '<p>Given a graph G = (V, E), choose a set of vertices such that no two chosen vertices are adjacent. Maximize the total weight of the chosen vertices. With unit weights, this is the largest independent set.</p><div class="formula">maximize ∑ wᵥ xᵥ<small>subject to xᵤ + xᵥ ≤ 1 for every edge (u, v), with xᵥ ∈ {0, 1}.</small></div>' : `<p>${escape(description(name))}. The fields below define the instance accepted by the implementation. Consult the paper for its mathematical definition and the API for its full contract.</p>`}${mis && node.variant.graph === "SimpleGraph" ? demoPanel() : ""}
          </section>
          <section class="reading-section">
            <h2>Connections for this variant</h2>
            <p>
              These are direct registered reductions for
              <strong>${escape(variantLabel(node))}</strong>. Changing the
              variant can change which connections are available.
            </p>
            <h3>
              Reduce this problem to
              <span class="result-count">${outgoing.length}</span>
            </h3>
            ${outgoing.map((e) => relation(e, node.index)).join("") || '<p class="notice">No outgoing reduction is registered for this exact variant.</p>'}
            <h3>
              Reduce other problems here
              <span class="result-count">${incoming.length}</span>
            </h3>
            ${incoming.map((e) => relation(e, node.index)).join("") || '<p class="notice">No incoming reduction is registered for this exact variant.</p>'}
          </section>
          <section class="reading-section">
            <h2>Instance structure</h2>
            ${schema?.fields?.length ? `<div class="table-scroll"><table class="schema-table"><thead><tr><th>Field</th><th>Type</th><th>Meaning</th></tr></thead><tbody>${schema.fields.map((f) => `<tr><td><code>${escape(f.name)}</code></td><td><code>${escape(f.type_name)}</code></td><td>${escape(f.description)}</td></tr>`).join("")}</tbody></table></div>` : "<p>See the API reference for the instance schema.</p>"}
          </section>
          <section class="reading-section">
            <h2>Explore from your terminal</h2>
            ${codePanel(`pred show ${name}`)}
            <p class="notice">
              Install the CLI with
              <code>cargo install problemreductions-cli</code>. The command
              shows the problem catalog; reduction availability depends on the
              selected variant.
            </p>
          </section>
        </div>
        <aside class="reading-aside">
          <dl class="metadata">
            <div>
              <dt>Input structure</dt>
              <dd>${escape(categories[node.category] || node.category)}</dd>
            </div>
            <div>
              <dt>Variant</dt>
              <dd>${escape(variantLabel(node))}</dd>
            </div>
            <div>
              <dt>Registered complexity</dt>
              <dd><code>${escape(node.complexity || "See API")}</code></dd>
            </div>
            <div>
              <dt>Direct connections</dt>
              <dd>${incoming.length} incoming · ${outgoing.length} outgoing</dd>
            </div>
          </dl>
          <a href="${escape(apiHref(node.api_path))}"
            >API reference <span>↗</span></a
          ><a href="./reductions.pdf">Definitions &amp; proofs <span>↗</span></a
          ><a href="#atlas">Back to the atlas <span>→</span></a>
          <p>
            Generated from the library registry. Complexity expressions describe
            registered worst-case bounds; see the cited algorithms in the paper.
          </p>
        </aside>
      </div>`;
    document
      .querySelector("#variant-select")
      .addEventListener("change", (event) => {
        const selected = variants.find(
          (n) => variantKey(n) === event.target.value,
        );
        location.hash = problemHref(name, selected);
      });
    bindDemo();
  }

  function rulePage(sourceName, targetName, params) {
    const edge = data.edges.find(
      (e) =>
        data.nodes[e.source].name === sourceName &&
        data.nodes[e.target].name === targetName &&
        variantKey(data.nodes[e.source]) === (params.get("from") || "") &&
        variantKey(data.nodes[e.target]) === (params.get("to") || ""),
    );
    if (!edge) return notFound();
    const source = data.nodes[edge.source],
      target = data.nodes[edge.target];
    const complement =
      ["MinimumVertexCover", "MaximumIndependentSet"].includes(sourceName) &&
      ["MinimumVertexCover", "MaximumIndependentSet"].includes(targetName) &&
      sourceName !== targetName;
    const vcToMis = sourceName === "MinimumVertexCover";
    document.title = `${nameOf(sourceName)} → ${nameOf(targetName)} — ${baseTitle}`;
    const capabilities = [
      edge.witness && "Witness recovery",
      edge.aggregate && "Aggregate value",
      edge.turing && "Turing reduction",
    ]
      .filter(Boolean)
      .join(" · ");
    const testCommand = edge.test_path
      ? `cargo test --lib ${moduleName(edge)}`
      : "cargo test --lib";
    main.innerHTML = /* HTML */ `<div class="wrap page-header">
        <div class="breadcrumbs">
          <a href="#home">Home</a><span>/</span><a href="#atlas">Atlas</a
          ><span>/</span><span>Reduction record</span>
        </div>
        <span class="pill"
          >${complement ? "Classical reduction" : "Registered reduction"} ·
          Implemented</span
        >
        <h1>
          ${escape(nameOf(sourceName))}<br /><em
            >→ ${escape(nameOf(targetName))}</em
          >
        </h1>
        <p>
          ${complement ? "Keep the graph. Complement the solution. A foundational connection between two views of the same combinatorial structure." : "An executable construction connecting two concrete problem variants, with explicit size bounds and a recoverable contract."}
        </p>
      </div>
      <div class="wrap reading-layout">
        <div class="reading-content">
          <section class="reading-section">
            <h2>The construction</h2>
            ${complement ? `<p>The target uses the same graph and vertex weights. ${vcToMis ? "Solve Maximum Independent Set on that graph, then take the complement of its vertex selection to recover a minimum vertex cover." : "Solve Minimum Vertex Cover on that graph, then take the complement of its vertex selection to recover a maximum independent set."}</p><div class="formula">S ↔ V ∖ S<small>A set is independent exactly when its complement is a vertex cover.</small></div><p>For weighted instances, the two objective values sum to the total vertex weight. Maximizing the independent-set weight therefore minimizes the complementary cover weight.</p>${demoPanel()}` : `<p>This rule maps <a class="text-link" href="${problemHref(sourceName, source)}">${escape(nameOf(sourceName))}</a> to <a class="text-link" href="${problemHref(targetName, target)}">${escape(nameOf(targetName))}</a>. The implementation and paper describe the mapping, its preconditions, and solution extraction.</p><a class="button secondary" href="${sourceHref(edge)}">Read the construction <span>↗</span></a>`}
          </section>
          <section class="reading-section">
            <h2>What the rule guarantees</h2>
            <div class="evidence-grid">
              <div class="evidence-item">
                <span class="context-label">Source variant</span>
                <h3>${escape(nameOf(sourceName))}</h3>
                <p>${escape(variantLabel(source))}</p>
                <a href="${problemHref(sourceName, source)}"
                  >Inspect source problem ↗</a
                >
              </div>
              <div class="evidence-item">
                <span class="context-label">Target variant</span>
                <h3>${escape(nameOf(targetName))}</h3>
                <p>${escape(variantLabel(target))}</p>
                <a href="${problemHref(targetName, target)}"
                  >Inspect target problem ↗</a
                >
              </div>
            </div>
            <h3>Target size bounds</h3>
            <div class="table-scroll">
              <table class="schema-table">
                <thead>
                  <tr>
                    <th>Target parameter</th>
                    <th>Bound in source parameters</th>
                  </tr>
                </thead>
                <tbody>
                  ${(edge.overhead || []).map((o) => `<tr><td><code>${escape(o.field)}</code></td><td><code>${escape(o.formula)}</code></td></tr>`).join("")}
                </tbody>
              </table>
            </div>
            <p class="notice">
              Registry overheads are asymptotic upper bounds. Read the
              constructor for exact instance
              sizes.${complement ? " This construction preserves the number of vertices and edges exactly." : ""}
            </p>
          </section>
          <section class="reading-section">
            <h2>Follow the evidence</h2>
            <p>
              Implementation, mathematical reasoning, and test evidence are
              distinct parts of the record.
            </p>
            <div class="evidence-grid">
              <div class="evidence-item">
                <h3>Executable Rust construction</h3>
                <p>
                  Inspect the transformation and its extraction contract
                  directly.
                </p>
                <a href="${sourceHref(edge)}">Read source ↗</a>
              </div>
              <div class="evidence-item">
                <h3>Definitions and proof</h3>
                <p>
                  Read the mathematical treatment in the project's compiled
                  paper.
                </p>
                <a href="./reductions.pdf">Open the paper ↗</a>
              </div>
              <div class="evidence-item">
                <h3>Inspect and run the tests</h3>
                <p>
                  Tests exercise finite instances. This page does not report a
                  live test run.
                </p>
                <a
                  href="${repo}/${edge.test_path ? "blob/main/" + edge.test_path : "tree/main/src/unit_tests/rules"}"
                  >${edge.test_path ? "Inspect test source" : "Browse the test suite"}
                  ↗</a
                >
              </div>
              <div class="evidence-item">
                <h3>
                  ${complement ? "Established mathematical result" : "Implemented in the library"}
                </h3>
                <p>
                  ${complement ? "This is a classical reduction, presented as an example of an inspectable result." : "Registration establishes implementation availability. It does not establish research novelty."}
                  Formal verification is not asserted.
                </p>
              </div>
            </div>
            ${complement ? "<details><summary>Why does complementation preserve optimality?</summary><p>If S is independent, no edge has both endpoints in S, so every edge has an endpoint in V ∖ S. Conversely, if V ∖ S covers every edge, no edge can have both endpoints in S. Since w(S) + w(V ∖ S) = w(V), maximizing one objective minimizes the other.</p></details>" : ""}
          </section>
          <section class="reading-section">
            <h2>Reproduce the implementation checks</h2>
            <p>From a checkout of the repository:</p>
            ${codePanel(testCommand)}
            <p class="notice">
              ${edge.test_path ? "The command filters library tests by the rule module." : "This rule uses shared test coverage; the command runs the library suite."}
              Check the reported test count and results. Tests support the
              implementation; they do not replace a general proof.
            </p>
          </section>
        </div>
        <aside class="reading-aside">
          <dl class="metadata">
            <div>
              <dt>Status</dt>
              <dd>Implemented</dd>
            </div>
            <div>
              <dt>Capabilities</dt>
              <dd>${escape(capabilities || "See contract")}</dd>
            </div>
            <div>
              <dt>Construction</dt>
              <dd>
                ${complement ? "Graph preserved; solution complemented" : "See implementation"}
              </dd>
            </div>
            <div>
              <dt>Provenance</dt>
              <dd>
                ${complement ? "Classical reduction" : "Library registry"}
              </dd>
            </div>
          </dl>
          <a href="${sourceHref(edge)}">Implementation <span>↗</span></a
          ><a href="${escape(apiHref(edge.api_path))}"
            >API contract <span>↗</span></a
          ><a href="./reductions.pdf">Read the paper <span>↗</span></a>
          <p>
            Research records should make claims traceable. Test execution status
            and agent experiment histories are not yet published here.
          </p>
        </aside>
      </div>`;
    bindDemo();
  }

  function notFound() {
    document.title = `Not found — ${baseTitle}`;
    main.innerHTML =
      '<div class="wrap section empty-state"><h1>This connection is not in the atlas.</h1><p>The link may refer to a different registry version. Search the current atlas to find its available variants.</p><a href="#atlas" class="button primary">Explore the atlas →</a></div>';
  }

  function render() {
    const hash = location.hash.slice(1) || "home";
    const [path, search = ""] = hash.split("?");
    let parts;
    try {
      parts = path.split("/").map(decodeURIComponent);
    } catch {
      notFound();
      return;
    }
    const params = new URLSearchParams(search);
    const isHome = [
      "home",
      "research",
      "infrastructure",
      "featured",
      "connections",
      "main",
    ].includes(parts[0]);
    document.title = `${baseTitle} — A new route through hard problems`;
    if (isHome) {
      main.innerHTML = homeHTML;
      hydrateHome();
    } else if (parts[0] === "atlas") atlasPage();
    else if (parts[0] === "problem") problemPage(parts[1], params);
    else if (parts[0] === "reduction") rulePage(parts[1], parts[2], params);
    else notFound();
    document.querySelectorAll("[data-nav]").forEach((link) => {
      const active =
        link.dataset.nav === parts[0] ||
        (["problem", "reduction"].includes(parts[0]) &&
          link.dataset.nav === "atlas");
      if (active) link.setAttribute("aria-current", "page");
      else link.removeAttribute("aria-current");
    });
    main.classList.remove("page-enter");
    requestAnimationFrame(() => main.classList.add("page-enter"));
    if (isHome && !["home", "main"].includes(parts[0]))
      requestAnimationFrame(() =>
        document.getElementById(parts[0])?.scrollIntoView(),
      );
    else window.scrollTo({ top: 0, behavior: "instant" });
  }

  function openSearch() {
    if (location.hash !== "#atlas") {
      focusSearchAfterNavigation = true;
      location.hash = "atlas";
    } else document.querySelector("#atlas-search")?.focus();
  }
  document
    .querySelector(".search-trigger")
    .addEventListener("click", openSearch);
  document.addEventListener("keydown", (event) => {
    const typing =
      /INPUT|TEXTAREA|SELECT/.test(event.target.tagName) ||
      event.target.isContentEditable;
    if (
      ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") ||
      (event.key === "/" && !typing)
    ) {
      event.preventDefault();
      openSearch();
    }
  });
  document.addEventListener("click", async (event) => {
    const button = event.target.closest("[data-copy]");
    if (!button) return;
    try {
      await navigator.clipboard.writeText(button.dataset.copy);
      notify("Command copied to clipboard");
    } catch {
      notify("Select the command to copy it. Clipboard access is unavailable.");
    }
  });
  window.addEventListener("hashchange", () => {
    render();
    if (focusSearchAfterNavigation) {
      document.querySelector("#atlas-search")?.focus();
      focusSearchAfterNavigation = false;
    } else {
      main.focus({ preventScroll: true });
    }
  });
  render();
})();

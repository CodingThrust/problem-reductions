/* The site is a static client of the same registry exports as the book and paper. */
(() => {
  "use strict";
  let data = window.REDUCTIONS;
  let fullData;
  const main = document.querySelector("main");
  const homeHTML = main.innerHTML;
  history.scrollRestoration = "manual";
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
      return '<img src="./assets/sat.svg" alt="Example Boolean clauses" width="300" height="112">';
    return '<img src="./assets/qubo.svg" alt="A quadratic objective" width="300" height="112">';
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
    }
  }

  function atlasPage() {
    main.innerHTML = /* HTML */ `<div class="wrap page-header detail-header">
        <h1>Find your next connection.</h1>
        <p>
          Explore ${families.length} problem families and their implemented
          reductions. Every connection has explicit variants, size bounds, and
          code you can inspect.
        </p>
      </div>
      <div class="wrap atlas-layout">
        <aside class="filters" aria-label="Filter problems by category">
          <p class="context-label">Input structure</p>
          ${[["all", "All problems"], ...Object.entries(categories)].map(([key, label]) => `<button class="filter-button" data-filter="${key}" aria-pressed="${filter === key}"><span>${label}</span><span>${key === "all" ? families.length : families.filter((name) => variantsOf(name)[0].category === key).length}</span></button>`).join("")}
        </aside>
        <div class="atlas-catalog">
          <div class="atlas-controls">
            <label class="atlas-search">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="1.5" aria-hidden="true">
                <circle cx="10" cy="10" r="6"></circle>
                <path d="m15 15 5 5"></path>
              </svg>
              <input
                id="atlas-search"
                type="search"
                placeholder="Search problems, aliases, or descriptions…"
                aria-label="Search problems"
                value="${escape(query)}"
                autocomplete="off"
              /><kbd>/</kbd></label
            ><span class="result-count" role="status" aria-live="polite"></span>
            <a class="text-link" href="./graph.html"
              >Explore the reduction graph <span aria-hidden="true">↗</span></a
            >
          </div>
          <div class="atlas-results" id="atlas-results"></div>
        </div>
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
    }).sort((a, b) => nameOf(a).localeCompare(nameOf(b)));
    document.querySelector(".result-count").textContent =
      `${results.length} problem${results.length === 1 ? "" : "s"}`;
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
    const variant = Object.keys(other.variant).length ? variantLabel(other) : "";
    return `<a class="relation-link" href="${ruleHref(edge)}"><div class="relation-main"><div class="relation-title"><span>${outgoing ? "→ " : ""}${escape(nameOf(other.name))}${outgoing ? "" : " →"}</span></div>${variant ? `<p>${escape(variant)}</p>` : ""}</div>${edge.turing ? '<span class="capability">Turing reduction</span>' : ""}</a>`;
  }

  function codePanel(command) {
    return `<div class="code-panel"><div class="terminal-header"><button class="copy-button" data-copy="${escape(command)}" aria-label="Copy command">Copy</button></div><pre>${escape(command)}</pre></div>`;
  }

  function openReferenceProblem(key) {
    const [name, variant] = key.split("/");
    location.hash = `#problem/${encodeURIComponent(name)}${variant ? `?variant=${encodeURIComponent(variant)}` : ""}`;
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
    document.title = `${nameOf(name)} — ${baseTitle}`;
    main.innerHTML = /* HTML */ `<div class="wrap page-header detail-header">
        <div class="breadcrumbs">
          <a href="#atlas">← Back</a>
        </div>
        <h1>${escape(nameOf(name))}</h1>
        <div class="variant-control">
          <label for="variant-select">Variant</label
          ><select id="variant-select">
            ${variants.map((n) => `<option value="${escape(variantKey(n))}" ${n.index === node.index ? "selected" : ""}>${escape(variantLabel(n))}</option>`).join("")}
          </select>
        </div>
      </div>
      <div class="wrap reading-layout">
        <div class="reading-content">
          <div class="reference-tools" aria-label="Problem tools">
            <button aria-expanded="false" aria-controls="problem-connections">Connections <span>${incoming.length + outgoing.length}</span></button>
            <button aria-expanded="false" aria-controls="problem-fields">Instance fields <span>${schema?.fields?.length || 0}</span></button>
            <button aria-expanded="false" aria-controls="problem-commands">CLI commands</button>
          </div>
          <section id="problem-connections" class="reference-tool-panel" aria-label="Connections" hidden>
            <section class="connection-group" aria-label="Incoming reductions">
              <header><h3>Incoming <span>${incoming.length}</span></h3><span>Other problems → ${escape(nameOf(name))}</span></header>
              ${incoming.map((e) => relation(e, node.index)).join("")}
            </section>
            <section class="connection-group" aria-label="Outgoing reductions">
              <header><h3>Outgoing <span>${outgoing.length}</span></h3><span>${escape(nameOf(name))} → Other problems</span></header>
              ${outgoing.map((e) => relation(e, node.index)).join("")}
            </section>
          </section>
          <section id="problem-fields" class="reference-tool-panel" aria-label="Instance fields" hidden>
            ${schema?.fields?.length ? `<div class="table-scroll"><table class="schema-table"><thead><tr><th>Field</th><th>Type</th><th>Meaning</th></tr></thead><tbody>${schema.fields.map((f) => `<tr><td><code>${escape(f.name)}</code></td><td><code>${escape(f.type_name)}</code></td><td>${escape(f.description)}</td></tr>`).join("")}</tbody></table></div>` : "<p>See the API reference for the instance schema.</p>"}
          </section>
          <section id="problem-commands" class="reference-tool-panel" aria-label="CLI commands" hidden>
            ${codePanel(`pred show ${name}`)}
            <p class="notice">
              Install the CLI with
              <code>cargo install problemreductions-cli</code>. The command
              shows the problem catalog; reduction availability depends on the
              selected variant.
            </p>
          </section>
          <section id="reference-detail" class="reading-section reference-detail"></section>
        </div>
        <aside class="reading-aside">
          <dl class="metadata">
            <div>
              <dt>Input structure</dt>
              <dd>${escape(categories[node.category] || node.category)}</dd>
            </div>
            <div>
              <dt>Registered complexity</dt>
              <dd class="math-expression">${node.complexity_mathml}</dd>
            </div>
            <div>
              <dt>Direct connections</dt>
              <dd>${incoming.length} incoming · ${outgoing.length} outgoing</dd>
            </div>
          </dl>
          <a href="${repo}/blob/main/${schema.source_path}">Open implementation <span>↗</span></a>
          <a href="${escape(apiHref(node.api_path))}"
            >Open API reference <span>↗</span></a
          ><a href="./reductions.pdf">Open PDF reference <span>↗</span></a
          ><a href="#atlas">Back to the atlas <span>←</span></a>
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
    window.renderDetails(document.querySelector("#reference-detail"), "", "",
      `problem:${name}/${variantKey(node)}`, `problem:${name}`, openReferenceProblem);
  }

  function rulePage(sourceName, targetName, params) {
    const edge = data.edges.find(
      (e) =>
        data.nodes[e.source].name === sourceName &&
        data.nodes[e.target].name === targetName &&
        (!params.has("from") || variantKey(data.nodes[e.source]) === params.get("from")) &&
        (!params.has("to") || variantKey(data.nodes[e.target]) === params.get("to")),
    );
    if (!edge) return notFound();
    const source = data.nodes[edge.source],
      target = data.nodes[edge.target];
    document.title = `${nameOf(sourceName)} → ${nameOf(targetName)} — ${baseTitle}`;
    main.innerHTML = /* HTML */ `<div class="wrap page-header detail-header">
        <div class="breadcrumbs">
          <a href="#atlas">← Back</a>
        </div>
        <h1>${escape(nameOf(sourceName))} → ${escape(nameOf(targetName))}</h1>
      </div>
      <div class="wrap reading-layout">
        <div class="reading-content">
          <dl class="rule-endpoints">
            <div><dt>Source</dt><dd class="rule-endpoint"><a href="${problemHref(sourceName, source)}">${escape(nameOf(sourceName))} <span aria-hidden="true">↗</span></a>${Object.keys(source.variant).length ? `<span>${escape(variantLabel(source))}</span>` : ""}</dd></div>
            <div><dt>Target</dt><dd class="rule-endpoint"><a href="${problemHref(targetName, target)}">${escape(nameOf(targetName))} <span aria-hidden="true">↗</span></a>${Object.keys(target.variant).length ? `<span>${escape(variantLabel(target))}</span>` : ""}</dd></div>
          </dl>
          <section id="rule-parameters" aria-label="Parameters">
            <h2>Parameters</h2>
            ${edge.parameters.map((p) => `<div class="parameter-relation"><code>${escape(p.field)}</code><span class="parameter-operator">${p.contract === "unavailable" ? ":" : {exact: "=", upper_bound: "≤"}[p.contract]}</span>${p.contract === "unavailable" ? `<span>${escape(p.reason)}</span>` : p.mathml}</div>`).join("")}
          </section>
          <section id="reference-detail" class="reading-section reference-detail"></section>
        </div>
        <aside class="reading-aside">
          ${edge.turing ? '<p class="rule-kind">Turing reduction</p>' : ""}
          <a href="${sourceHref(edge)}">Open implementation <span>↗</span></a
          ><a href="${escape(apiHref(edge.api_path))}"
            >Open API reference <span>↗</span></a
          ><a href="./reductions.pdf">Open PDF reference <span>↗</span></a>
          <a href="${repo}/${edge.test_path ? "blob/main/" + edge.test_path : "tree/main/src/unit_tests/rules"}">Test source <span>↗</span></a>
        </aside>
      </div>`;
    window.renderDetails(document.querySelector("#reference-detail"), "", "",
      `rule:${sourceName}/${variantKey(source)}->${targetName}/${variantKey(target)}`,
      `rule:${sourceName}->${targetName}`, openReferenceProblem);
  }

  function notFound() {
    document.title = `Not found — ${baseTitle}`;
    main.innerHTML =
      '<div class="wrap section empty-state"><h1>This connection is not in the atlas.</h1><p>The link may refer to a different registry version. Search the current atlas to find its available variants.</p><a href="#atlas" class="button primary">Explore the atlas →</a></div>';
  }

  async function render() {
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
    if (["atlas", "problem", "reduction"].includes(parts[0])) {
      try {
        fullData ||= fetch("./assets/atlas-data.json").then(response => {
          if (!response.ok) throw new Error("Cannot load atlas. Reload the page to get the current release.");
          return response.json();
        });
        data = await fullData;
        if ((location.hash.slice(1) || "home") !== hash) return;
      } catch (error) {
        fullData = null;
        main.replaceChildren(Object.assign(document.createElement("p"), {textContent: error.message}));
        main.firstChild.setAttribute("role", "alert");
        return;
      }
    }
    const isHome = [
      "home",
      "research",
      "infrastructure",
      "featured",
      "connections",
      "main",
    ].includes(parts[0]);
    document.title = baseTitle;
    if (isHome) {
      main.innerHTML = homeHTML;
      hydrateHome();
    } else if (parts[0] === "atlas") atlasPage();
    else if (parts[0] === "open-problems") {
      document.title = `Open problems — ${baseTitle}`;
      main.innerHTML = '<div class="wrap page-header"><h1>Open problems</h1><p>To be released.</p></div>';
    }
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
        (location.hash.slice(1) || "home") === hash && document.getElementById(parts[0])?.scrollIntoView(),
      );
    else window.scrollTo({ top: 0, behavior: "instant" });
  }

  function openSearch() {
    if (location.hash !== "#atlas") {
      focusSearchAfterNavigation = true;
      location.hash = "atlas";
    } else document.querySelector("#atlas-search")?.focus();
  }
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
    const tool = event.target.closest('.reference-tools button');
    if (tool) {
      const open = tool.getAttribute('aria-expanded') === 'false';
      tool.parentElement.querySelectorAll('button').forEach((button) => {
        const selected = button === tool && open;
        button.setAttribute('aria-expanded', String(selected));
        document.getElementById(button.getAttribute('aria-controls')).hidden = !selected;
      });
      return;
    }
    const button = event.target.closest("[data-copy]");
    if (!button) return;
    try {
      await navigator.clipboard.writeText(button.dataset.copy);
      notify("Command copied to clipboard");
    } catch {
      notify("Select the command to copy it. Clipboard access is unavailable.");
    }
  });
  window.addEventListener("hashchange", async () => {
    await render();
    if (focusSearchAfterNavigation) {
      document.querySelector("#atlas-search")?.focus();
      focusSearchAfterNavigation = false;
    } else {
      main.focus({ preventScroll: true });
    }
  });
  render();
})();

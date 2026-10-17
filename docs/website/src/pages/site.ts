import { loadDetailsIndex, renderDetails } from './details.ts';
import { validateAtlasSummary, validateAtlasRecord } from '../lib/atlas-data.mts';
import type { AtlasSummary, AtlasRecord, RecordEdge, SummaryEdge, Variant } from '../generated/atlas-data.validator.mjs';

declare global { interface Window { REDUCTIONS: unknown } }

/* The site is a static client of the same registry exports as the book and paper. */
(() => {
  "use strict";
  const $ = <T extends HTMLElement = HTMLElement>(selector: string) => document.querySelector<T>(selector)!;
  const main = $('main');
  function showError(error: unknown) {
    const alert = Object.assign(document.createElement('p'), {textContent: error instanceof Error ? error.message : String(error)});
    alert.setAttribute('role', 'alert');
    main.replaceChildren(alert);
  }
  let summary: AtlasSummary;
  try { summary = validateAtlasSummary(window.REDUCTIONS); }
  catch (error) { showError(error); return; }
  const records = new Map<string, Promise<AtlasRecord>>();
  const names = new Map(summary.schemas.map(schema => [schema.name, schema]));
  let atlasRows: Map<string, Element> | undefined;
  // Indexed references below rely on the validated summary/record contracts.
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
  const aliases: Record<string, string> = {
    MIS: "MaximumIndependentSet",
    MVC: "MinimumVertexCover",
    SAT: "Satisfiability",
    "3SAT": "KSatisfiability",
    "3-SAT": "KSatisfiability",
  };
  let filter = "all";
  let query = "";
  let toastTimer: ReturnType<typeof setTimeout> | undefined;
  let focusSearchAfterNavigation = false;
  const escape = (value: unknown) =>
    String(value ?? "").replace(
      /[&<>"']/g,
      (c) =>
        ({
          "&": "&amp;",
          "<": "&lt;",
          ">": "&gt;",
          '"': "&quot;",
          "'": "&#39;",
        } as Record<string, string>)[c]!,
    );
  const nameOf = (name: string) =>
    names.get(name)?.display_name ||
    name
      .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
      .replace(/([A-Z])([A-Z][a-z])/g, "$1 $2");
  const variantKey = (node: {variant: Variant}) =>
    Object.entries(node.variant)
      .sort(([a], [b]) => a.localeCompare(b))
      .map(([k, v]) => `${k}=${v}`)
      .join(",");
  const variantLabel = (node: {variant: Variant}) =>
    Object.entries(node.variant)
      .map(([k, v]) => `${k}: ${v}`)
      .join(" · ") || "Default variant";
  const problemHref = (name: string, node?: {variant: Variant}) =>
    `#problem/${encodeURIComponent(name)}${node ? `?variant=${encodeURIComponent(variantKey(node))}` : ""}`;
  const ruleHref = (edge: SummaryEdge, nodes: AtlasSummary['nodes'] | AtlasRecord['nodes'] = summary.nodes) =>
    `#reduction/${encodeURIComponent(nodes[edge.source]!.name)}/${encodeURIComponent(nodes[edge.target]!.name)}?from=${encodeURIComponent(variantKey(nodes[edge.source]!))}&to=${encodeURIComponent(variantKey(nodes[edge.target]!))}`;
  const schemaOf = (name: string) => summary.schemas.find((s) => s.name === name);
  const description = (name: string) =>
    schemaOf(name)?.description ||
    "Explore the registered variants, reduction contracts, and implementation of this computational problem.";
  const apiHref = (path: string) => `./api/problemreductions/${path}`;
  const sourceHref = (edge: RecordEdge) => `${repo}/blob/main/${edge.source_path}`;
  const families = [...new Set(summary.nodes.map((n) => n.name))].sort();
  const variantsOf = (name: string, data: AtlasRecord) =>
    data.nodes
      .map((n, i) => ({ ...n, index: i }))
      .filter((n) => n.name === name);
  const edgesOf = (name: string) =>
    summary.edges.filter(
      (e) =>
        summary.nodes[e.source]!.name === name ||
        summary.nodes[e.target]!.name === name,
    );
  const featuredEdge = summary.edges.find(
    (e) =>
      summary.nodes[e.source]!.name === "MinimumVertexCover" &&
      summary.nodes[e.target]!.name === "MaximumIndependentSet",
  );
  const catalog = families.map(name => {
    const variants = summary.nodes.filter(node => node.name === name), category = variants[0]!.category;
    return {name, category,
      search: `${name} ${nameOf(name)} ${description(name)} ${(names.get(name)?.aliases || []).join(' ')}`.toLowerCase(),
      html: `<a class="atlas-row" href="${problemHref(name)}"><div><h2>${escape(nameOf(name))}</h2><p>${escape(description(name))}</p><div class="atlas-row-meta"><span>${escape(categories[category] || category)}</span><span>${variants.length} variant${variants.length === 1 ? '' : 's'}</span><span>${edgesOf(name).length} connection${edgesOf(name).length === 1 ? '' : 's'}</span></div></div><span aria-hidden="true">↗</span></a>`};
  }).sort((a, b) => nameOf(a.name).localeCompare(nameOf(b.name)));

  function notify(message: string) {
    const toast = $(".toast");
    toast.textContent = message;
    toast.classList.add("visible");
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => toast.classList.remove("visible"), 2800);
  }

  function graphArt() {
    const width = 630,
      height = 430;
    const highlights: Record<string, [number, number, string]> = {
      MaximumIndependentSet: [310, 242, "Independent Set"],
      MinimumVertexCover: [452, 155, "Vertex Cover"],
      Satisfiability: [145, 162, "Satisfiability"],
      KSatisfiability: [220, 74, "k-SAT"],
      MaximumSetPacking: [449, 314, "Set Packing"],
      QUBO: [158, 335, "QUBO"],
      ILP: [330, 370, "ILP"],
      Coloring: [92, 249, "Coloring"],
    };
    const positions: Record<string, [number, number, string?]> = {};
    families.forEach((name, i) => {
      const angle = i * 2.39996323;
      const radius = Math.sqrt((i + 1) / families.length);
      positions[name] = highlights[name] || [
        315 + Math.cos(angle) * radius * 287,
        218 + Math.sin(angle) * radius * 191,
      ];
    });
    const seen = new Set<string>();
    const lines = summary.edges
      .map((e) => {
        const from = summary.nodes[e.source]!.name,
          to = summary.nodes[e.target]!.name;
        if (from === to) return "";
        const key = [from, to].sort().join("/");
        if (seen.has(key)) return "";
        seen.add(key);
        const [x1, y1] = positions[from]!,
          [x2, y2] = positions[to]!;
        const featured =
          [from, to].includes("MinimumVertexCover") &&
          [from, to].includes("MaximumIndependentSet");
        return `<line class="network-edge ${featured ? "featured-edge" : ""}" x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}"/>`;
      })
      .join("");
    const nodes = [...families]
      .sort((a, b) => Number(!!highlights[a]) - Number(!!highlights[b]))
      .map((name) => {
        const [x, y] = positions[name]!;
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
  ] as const;
  function cycleSVG(mode = "independent", labels = false) {
    const selected = mode === "independent" ? [0, 2] : [1, 3, 4];
    return `<svg viewBox="0 0 300 225" role="img" aria-label="Five-vertex cycle with ${mode === "independent" ? "an independent set of two" : "a vertex cover of three"} highlighted">${cyclePoints
      .map(([x, y], i) => {
        const next = cyclePoints[(i + 1) % cyclePoints.length]!;
        return `<line x1="${x}" y1="${y}" x2="${next[0]}" y2="${next[1]}" stroke="var(--diagram-line)" stroke-width="1.5"/>`;
      })
      .join(
        "",
      )}${cyclePoints.map(([x, y], i) => `<circle data-selected="${selected.includes(i)}" cx="${x}" cy="${y}" r="${labels ? 15 : 10}" fill="${selected.includes(i) ? "var(--diagram-selected)" : "var(--paper)"}" stroke="${selected.includes(i) ? "var(--diagram-selected)" : "var(--diagram-line)"}" stroke-width="1.5"/>${labels ? `<text x="${x}" y="${y + 4}" text-anchor="middle" font-family="DM Sans, sans-serif" font-size="11" fill="${selected.includes(i) ? "var(--on-accent)" : "var(--diagram-label)"}">${i}</text>` : ""}`).join("")}</svg>`;
  }

  function complementArt() {
    const graph = (offset: number, selected: number[]) =>
      `<g transform="translate(${offset},20) scale(.67)">${cyclePoints.map(([x, y], i) => `<line x1="${x}" y1="${y}" x2="${cyclePoints[(i + 1) % 5]![0]}" y2="${cyclePoints[(i + 1) % 5]![1]}" stroke="var(--diagram-line)" stroke-width="2"/>`).join("")}${cyclePoints.map(([x, y], i) => `<circle cx="${x}" cy="${y}" r="13" fill="${selected.includes(i) ? "var(--diagram-selected)" : "var(--paper)"}" stroke="var(--diagram-line)" stroke-width="1.5"/>`).join("")}</g>`;
    return `<svg viewBox="0 0 520 245" role="img" aria-label="A five-cycle with a vertex cover of three filled vertices, and the two remaining vertices forming an independent set">${graph(15, [1, 3, 4])}${graph(305, [0, 2])}<path d="M232 102h54m-6-6 6 6-6 6" fill="none" stroke="var(--diagram-line)" stroke-width="1.5"/><g font-family="DM Sans, sans-serif" text-anchor="middle" fill="var(--diagram-label)"><text x="259" y="130" font-size="12" fill="var(--muted)">keep the</text><text x="259" y="145" font-size="12" fill="var(--muted)">others</text><text x="115" y="196" font-size="14" font-weight="600">Minimum Vertex Cover</text><text x="405" y="196" font-size="14" font-weight="600">Maximum Independent Set</text><text x="115" y="219" font-size="12" fill="var(--muted)">every edge touches a filled vertex</text><text x="405" y="219" font-size="12" fill="var(--muted)">no two filled vertices share an edge</text></g></svg>`;
  }

  function miniArt(name: string) {
    if (name === "MaximumIndependentSet") return cycleSVG();
    if (name === "Satisfiability")
      return '<img src="./assets/sat.svg" alt="Example Boolean clauses" width="300" height="112">';
    return '<img src="./assets/qubo.svg" alt="A quadratic objective" width="300" height="112">';
  }

  function hydrateHome() {
    $("#hero-graph").innerHTML = graphArt();
    $("#problem-showcase").innerHTML = [
      "MaximumIndependentSet",
      "Satisfiability",
      "QUBO",
    ]
      .filter((name) => families.includes(name))
      .map((name) => {
        const variants = summary.nodes.filter(node => node.name === name);
        return `<a class="problem-card" href="${problemHref(name)}"><span class="context-label">${escape(categories[variants[0]!.category] || "Computational problem")}</span><h3>${escape(nameOf(name))}</h3><p>${escape(description(name))}.</p><div class="mini-art">${miniArt(name)}</div><div class="card-bottom"><span>${variants.length} variant${variants.length === 1 ? "" : "s"} · ${edgesOf(name).length} incident reductions</span><span aria-hidden="true">↗</span></div></a>`;
      })
      .join("");
    $("#featured-art").innerHTML = complementArt();
    if (featuredEdge) {
      $<HTMLAnchorElement>("#featured-result-link").href =
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
          ${[["all", "All problems"], ...Object.entries(categories)].map(([key, label]) => `<button class="filter-button" data-filter="${key}" aria-pressed="${filter === key}"><span>${label}</span><span>${key === "all" ? catalog.length : catalog.filter(entry => entry.category === key).length}</span></button>`).join("")}
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
    $("#atlas-search")
      .addEventListener("input", (event) => {
        query = (event.currentTarget as HTMLInputElement).value;
        renderResults();
      });
    document.querySelectorAll<HTMLElement>("[data-filter]").forEach((button) =>
      button.addEventListener("click", () => {
        filter = button.dataset.filter!;
        document
          .querySelectorAll<HTMLElement>("[data-filter]")
          .forEach((b) => b.setAttribute("aria-pressed", String(b === button)));
        renderResults();
      }),
    );
  }

  function renderResults() {
    if (!atlasRows) {
      const rows = document.createElement('div');
      rows.innerHTML = catalog.map(entry => entry.html).join('');
      atlasRows = new Map(catalog.map((entry, i) => [entry.name, rows.children[i]!]));
    }
    const search = query.trim().toLowerCase();
    const alias = aliases[query.trim().toUpperCase()] || "";
    const results = catalog.filter(entry =>
      (filter === 'all' || entry.category === filter) &&
      (!search || entry.name === alias || entry.search.includes(search)));
    $(".result-count").textContent =
      `${results.length} problem${results.length === 1 ? "" : "s"}`;
    const box = $('#atlas-results');
    if (results.length) box.replaceChildren(...results.map(entry => atlasRows!.get(entry.name)!));
    else box.innerHTML = `<div class="empty-state"><h2>No matching problems.</h2><p>Try a broader term or explore another input structure.</p><button class="button secondary" id="reset-search">Reset search and filters</button></div>`;
    $("#reset-search")?.addEventListener("click", () => {
      query = "";
      filter = "all";
      atlasPage();
      $("#atlas-search").focus();
    });
  }

  function relation(edge: RecordEdge, currentIndex: number, data: AtlasRecord) {
    const source = data.nodes[edge.source]!,
      target = data.nodes[edge.target]!;
    const outgoing = edge.source === currentIndex;
    const other = outgoing ? target : source;
    const variant = Object.keys(other.variant).length ? variantLabel(other) : "";
    return `<a class="relation-link" href="${ruleHref(edge, data.nodes)}"><div class="relation-main"><div class="relation-title"><span>${outgoing ? "→ " : ""}${escape(nameOf(other.name))}${outgoing ? "" : " →"}</span></div>${variant ? `<p>${escape(variant)}</p>` : ""}</div>${edge.turing ? '<span class="capability">Turing reduction</span>' : ""}</a>`;
  }

  function codePanel(command: string) {
    return `<div class="code-panel"><div class="terminal-header"><button class="copy-button" data-copy="${escape(command)}" aria-label="Copy command">Copy</button></div><pre>${escape(command)}</pre></div>`;
  }

  function openReferenceProblem(key: string) {
    const [name, variant] = key.split("/") as [string, string?];
    location.hash = `#problem/${encodeURIComponent(name)}${variant ? `?variant=${encodeURIComponent(variant)}` : ""}`;
  }

  function problemPage(name: string, params: URLSearchParams, data: AtlasRecord) {
    const variants = variantsOf(name, data);
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
      variants[0]!;
    const schema = data.schemas[0];
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
          <div class="reference-tools" role="group" aria-label="Problem tools">
            <button aria-expanded="false" aria-controls="problem-connections">Connections <span>${incoming.length + outgoing.length}</span></button>
            <button aria-expanded="false" aria-controls="problem-fields">Instance fields <span>${schema?.fields?.length || 0}</span></button>
            <button aria-expanded="false" aria-controls="problem-commands">CLI commands</button>
          </div>
          <section id="problem-connections" class="reference-tool-panel" aria-label="Connections" hidden>
            <section class="connection-group" aria-label="Incoming reductions">
              <header><h3>Incoming <span>${incoming.length}</span></h3><span>Other problems → ${escape(nameOf(name))}</span></header>
              ${incoming.map((e) => relation(e, node.index, data)).join("")}
            </section>
            <section class="connection-group" aria-label="Outgoing reductions">
              <header><h3>Outgoing <span>${outgoing.length}</span></h3><span>${escape(nameOf(name))} → Other problems</span></header>
              ${outgoing.map((e) => relation(e, node.index, data)).join("")}
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
    $("#variant-select")
      .addEventListener("change", (event) => {
        const selected = variants.find(
          (n) => variantKey(n) === (event.currentTarget as HTMLSelectElement).value,
        );
        location.hash = problemHref(name, selected);
      });
    renderDetails($("#reference-detail"), "", "",
      `problem:${name}/${variantKey(node)}`, `problem:${name}`, openReferenceProblem);
  }

  function rulePage(sourceName: string, targetName: string | undefined, params: URLSearchParams, data: AtlasRecord) {
    const edge = data.edges.find(
      (e) =>
        data.nodes[e.source]!.name === sourceName &&
        data.nodes[e.target]!.name === targetName &&
        (!params.has("from") || variantKey(data.nodes[e.source]!) === params.get("from")) &&
        (!params.has("to") || variantKey(data.nodes[e.target]!) === params.get("to")),
    );
    if (!edge) return notFound();
    const source = data.nodes[edge.source]!,
      target = data.nodes[edge.target]!;
    document.title = `${nameOf(sourceName)} → ${nameOf(target.name)} — ${baseTitle}`;
    main.innerHTML = /* HTML */ `<div class="wrap page-header detail-header">
        <div class="breadcrumbs">
          <a href="#atlas">← Back</a>
        </div>
        <h1>${escape(nameOf(sourceName))} → ${escape(nameOf(target.name))}</h1>
      </div>
      <div class="wrap reading-layout">
        <div class="reading-content">
          <dl class="rule-endpoints">
            <div><dt>Source</dt><dd class="rule-endpoint"><a href="${problemHref(sourceName, source)}">${escape(nameOf(sourceName))} <span aria-hidden="true">↗</span></a>${Object.keys(source.variant).length ? `<span>${escape(variantLabel(source))}</span>` : ""}</dd></div>
            <div><dt>Target</dt><dd class="rule-endpoint"><a href="${problemHref(target.name, target)}">${escape(nameOf(target.name))} <span aria-hidden="true">↗</span></a>${Object.keys(target.variant).length ? `<span>${escape(variantLabel(target))}</span>` : ""}</dd></div>
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
    renderDetails($("#reference-detail"), "", "",
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
    const [path, search = ""] = hash.split("?") as [string, string?];
    let parts: [string, ...string[]];
    try {
      parts = path.split("/").map(decodeURIComponent) as [string, ...string[]];
    } catch {
      notFound();
      return;
    }
    const params = new URLSearchParams(search);
    let record: AtlasRecord | undefined;
    if (["atlas", "problem", "reduction"].includes(parts[0])) loadDetailsIndex().catch(() => {});
    if (["problem", "reduction"].includes(parts[0])) {
      const name = parts[1];
      if (!name || !families.includes(name)) { notFound(); return; }
      try {
        if (!records.has(name)) {
          records.set(name, fetch(`./assets/problem-data/${encodeURIComponent(name)}.json`).then(response => {
            if (!response.ok) throw new Error("Cannot load atlas. Reload the page to get the current release.");
            return response.json();
          }).then((value: unknown) => validateAtlasRecord(value, name)));
        }
        record = await records.get(name)!;
        if ((location.hash.slice(1) || "home") !== hash) return;
      } catch (error) {
        records.delete(name);
        showError(error);
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
    else if (parts[0] === "open-questions") {
      location.replace("./open-questions/");
      return;
    }
    else if (parts[0] === "problem") problemPage(parts[1]!, params, record!);
    else if (parts[0] === "reduction") rulePage(parts[1]!, parts[2], params, record!);
    else notFound();
    document.querySelectorAll<HTMLElement>("[data-nav]").forEach((link) => {
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
    } else $("#atlas-search")?.focus();
  }
  document.addEventListener("keydown", (event) => {
    const target = event.target as HTMLElement;
    const typing =
      /INPUT|TEXTAREA|SELECT/.test(target.tagName) ||
      target.isContentEditable;
    if (
      ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") ||
      (event.key === "/" && !typing)
    ) {
      event.preventDefault();
      openSearch();
    }
  });
  document.addEventListener("click", async (event) => {
    if (!(event.target instanceof Element)) return;
    const tool = event.target.closest('.reference-tools button');
    if (tool) {
      const open = tool.getAttribute('aria-expanded') === 'false';
      tool.parentElement!.querySelectorAll('button').forEach((button) => {
        const selected = button === tool && open;
        button.setAttribute('aria-expanded', String(selected));
        document.getElementById(button.getAttribute('aria-controls')!)!.hidden = !selected;
      });
      return;
    }
    const button = event.target.closest<HTMLElement>("[data-copy]");
    if (!button) return;
    try {
      await navigator.clipboard.writeText(button.dataset.copy!);
      notify("Command copied to clipboard");
    } catch {
      notify("Select the command to copy it. Clipboard access is unavailable.");
    }
  });
  window.addEventListener("hashchange", async () => {
    await render();
    if (focusSearchAfterNavigation) {
      $("#atlas-search")?.focus();
      focusSearchAfterNavigation = false;
    } else {
      main.focus({ preventScroll: true });
    }
  });
  render();
})();

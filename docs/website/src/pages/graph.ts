import { graphStartup } from './graph-startup.ts';
import { renderDetails } from './details.ts';
import { GraphScene, type SceneNode, type SceneItem, type SceneEdge } from "./graph-scene.ts";
import { growth, pathDegree, sizeBadge, compareDegrees } from "../lib/graph-growth.mts";
import { CLASSES, edgeGrowth, activeRules, components, reach, familyRules, type ClassId, type RuleEdge, type Scope, type FamilyRule } from '../lib/graph-structure.mts';
import { validatePathReply, type PathRequest, type PathReply } from '../lib/path-worker-data.mts';

import type { Endpoint, Route, Step, WasmPathResult } from '../generated/wasm-paths.mjs';
import type { WasmComparisonResult } from '../generated/wasm-comparison.mjs';
import type { GraphSceneData } from '../generated/graph-scene.mjs';

type Side = 'source' | 'target';
interface Family { name: string; category: GraphSceneData['nodes'][number]['category']; variants: number[]; nbrs: Set<string> }
type PathResult = WasmPathResult & { polynomial_comparison?: WasmComparisonResult; order?: number[] };
interface State { source: number | null; target: number | null; limit: number; result: PathResult | null;
  selected: number | null; picked: boolean; showSlower: boolean; busy: boolean }

type Bounds = ReturnType<GraphScene['labelBox']>;

(async () => {
"use strict";
performance.mark("graph:script", {detail: {eager_worker: Boolean(graphStartup.worker)}});
const $ = <T extends HTMLElement = HTMLElement>(s: string) => document.querySelector<T>(s)!;
const el = <T extends keyof HTMLElementTagNameMap>(tag: T, cls?: string, text?: string) => { const n = document.createElement(tag); if (cls) n.className = cls; if (text != null) n.textContent = text; return n; };
const D = window.REDUCTIONS = await graphStartup.scene;
performance.mark("graph:scene-parsed");
const defaults = Object.fromEntries(D.nodes.flatMap((n, i) => n.default ? [[n.name, i]] : []));
const aliasMap = D.aliases;
const reduced = matchMedia("(prefers-reduced-motion: reduce)");
const phone = matchMedia("(max-width: 760px)");
// Keep the minimum supported 320px graph viewport beside the 392px and 456px side-panel areas.
const modalDetails = matchMedia("(max-width: 1167px)");

// The build validates scene references; the engine owns path correctness.
/* ---------- Registry model ---------- */
const display = (name: string) => name.replace(/([a-z0-9])([A-Z])/g, "$1 $2").replace(/([A-Z])([A-Z][a-z])/g, "$1 $2");
const families = new Map<string, Family>();
D.nodes.forEach((n, i) => {
  if (!families.has(n.name)) families.set(n.name, { name: n.name, category: n.category, variants: [], nbrs: new Set() });
  families.get(n.name)!.variants.push(i);
});
const famEdges = new Map<string, {s: string; t: string}>();
D.edges.forEach((e) => {
  const s = D.nodes[e.source]!.name, t = D.nodes[e.target]!.name;
  if (s === t) return;
  famEdges.set(s + "→" + t, { s, t });
  families.get(s)!.nbrs.add(t); families.get(t)!.nbrs.add(s);
});
const multi = (name: string) => families.get(name)!.variants.length > 1;
const keyOf = (node: Endpoint) => node.name + "|" + Object.entries(node.variant).sort().map(([k, v]) => k + "=" + v).join(",");
const indexOf = new Map(D.nodes.map((n, i) => [keyOf(n), i]));

/* Keyed variant text, e.g. graph: KingsSubgraph  weight: i64 */
function kvNode(variant: Endpoint['variant']) {
  const span = el("span", "kv");
  Object.entries(variant).forEach(([k, v], i) => { if (i) span.append("   "); span.append(el("i", "", k + ": "), v); });
  return span;
}

const prettyField = (f: string) => f.replace(/^num_/, "").replaceAll("_", " ");

/* ---------- Graph ---------- */
const nodeSize = (deg: number) => Math.max(6, Math.min(22, 6 + 2.4 * Math.sqrt(deg)));
const pos = new Map<string, {x: number; y: number}>();
for (const f of families.values()) pos.set(f.name, { x: D.layout[f.name]!.x * 1.6, y: D.layout[f.name]!.y * 1.6 });
const cat = { graph: "#a9d68a", formula: "#8cc4c7", set: "#e0a988", algebraic: "#e2cf73", misc: "#c39bd3" };
const scene = new GraphScene($("#cy"), [...families.values()].map(f => ({id: f.name, label: display(f.name),
  size: nodeSize(f.nbrs.size), deg: f.nbrs.size, color: cat[f.category] || cat.misc, ...pos.get(f.name)!})), [...famEdges.values()]);
performance.mark("graph:renderer-created");
performance.mark("graph:constructed");

function freeRect() {
  const W = innerWidth, H = innerHeight, top = $(".site-header").getBoundingClientRect().bottom;
  if (phone.matches) return { x: 0, y: top, w: W, h: Math.max(1, $(".panel").getBoundingClientRect().top - top) };
  const right = $("#details-drawer").hidden || modalDetails.matches ? W : $("#details-drawer").getBoundingClientRect().left - 16;
  return { x: 392, y: top, w: Math.max(1, right - 392), h: H - top };
}
scene.visibleRect = freeRect;
function frame(eles: SceneItem[], maxZoom = 1.3, animate = !reduced.matches) {
  if (!eles || !eles.length) return;
  const r = freeRect(), b = scene.bounds(eles), pad = 90;
  const zoom = Math.max(0.1, Math.min((r.w - 2 * pad) / Math.max(b.w, 1), (r.h - 2 * pad) / Math.max(b.h, 1), maxZoom));
  const pan = { x: r.x + r.w / 2 - (b.x1 + b.x2) / 2 * zoom, y: r.y + r.h / 2 - (b.y1 + b.y2) / 2 * zoom };
  scene.frame(zoom, pan, animate);
}

/* Labels keep a constant screen size; important labels are placed first and any label that would collide is skipped. */
let labelRaf = 0;
const scheduleLabels = () => { cancelAnimationFrame(labelRaf); labelRaf = requestAnimationFrame(placeLabels); };
function placeLabels() {
  const r = freeRect(), boxes: Bounds[] = [];
  const box = (n: SceneNode) => scene.labelBox(n);
  const important = (n: SceneNode) => ["route", "start", "end", "hover", "focused", "reach"].some(c => scene.has(n, c));
  const fits = (b: Bounds) => b.x1 > r.x + 4 && b.x2 < r.x + r.w - 4 && b.y1 > r.y + 4 && b.y2 < r.y + r.h - 4;
  const hit = (b: Bounds) => boxes.some(o => b.x1 < o.x2 + 6 && b.x2 > o.x1 - 6 && b.y1 < o.y2 + 4 && b.y2 > o.y1 - 4);
  scene.classes(scene.nodes, "named");
  scene.nodes.filter(important).forEach(n => boxes.push(box(n)));
  const named: SceneNode[] = [];
  const cap = lensOn && lens.focus == null && lens.group == null ? 12 : Infinity;
  scene.nodes.filter(n => !important(n) && !scene.has(n, "dim")).sort((a,b) => b.deg-a.deg).forEach(n => {
    const b = box(n);
    if (named.length < cap && fits(b) && !hit(b)) { boxes.push(b); named.push(n); }
  });
  scene.classes(named, "", "named");
  placePop();
}
scene.onChange = scheduleLabels;
scene.onDrag = () => { scheduleLabels(); placePop(); };
scene.onHover = (n, over) => {
  scene.classes([n], over ? "" : "hover", over ? "hover" : "");
  if (lensOn && lens.focus == null && lens.group == null) {
    const nbrs = families.get(n.id)!.nbrs;
    scene.classes(scene.edges, "lit quiet");
    if (over) {
      scene.edges.forEach(e => scene.classes([e], "", e.source === n || e.target === n ? "lit" : "quiet"));
      scene.nodes.forEach(o => scene.classes([o], "near", o !== n && nbrs.has(o.id) ? "near" : ""));
    } else scene.classes(scene.nodes, "near");
  }
  if (!state.result && !lensOn) {
    if (over) {
      scene.classes(scene.elements.filter(e => e !== n && !families.get(n.id)!.nbrs.has(e.id)), "", "dim");
      scene.classes(scene.edges.filter(e => e.source === n || e.target === n), "dim", "nbr");
    } else scene.classes(scene.elements, "dim nbr");
  }
  scheduleLabels();
};

/* ---------- State ---------- */
let lensOn = false;
const state: State = { source: null, target: null, limit: 20, result: null, selected: null, picked: false, showSlower: false, busy: false };

/* Variant options: one row of pills per variant key. Picking a value keeps the other keys when such a variant exists. */
function focusedVariantGroup(root: HTMLElement) {
  const active = document.activeElement;
  return active && root.contains(active) ? active.closest('[role="group"]')?.getAttribute('aria-label') : null;
}
function restoreVariantGroup(root: HTMLElement, name: string | null | undefined) {
  if (name) [...root.querySelectorAll('[role="group"]')].find(group => group.getAttribute('aria-label') === name)
    ?.querySelector<HTMLButtonElement>('button[aria-pressed="true"]')?.focus({preventScroll: true});
}
function variantOptions(container: HTMLElement, index: number, onPick: (index: number) => void) {
  const focused = focusedVariantGroup(container);
  container.replaceChildren();
  const fam = families.get(D.nodes[index]!.name)!;
  const cur = D.nodes[index]!.variant;
  const keys = [...new Set(fam.variants.flatMap((i) => Object.keys(D.nodes[i]!.variant)))];
  keys.forEach((k) => {
    const values = [...new Set(fam.variants.map((i) => D.nodes[i]!.variant[k]).filter(Boolean))];
    if (values.length < 2) return;
    const row = el("div", "opt-row"); row.setAttribute("role", "group"); row.setAttribute("aria-label", k);
    row.append(el("span", "opt-key", k));
    values.forEach((v) => {
      const b = el("button", "opt", v); b.type = "button";
      b.setAttribute("aria-pressed", String(cur[k] === v));
      b.addEventListener("pointerdown", (ev) => ev.preventDefault());
      b.addEventListener("click", (ev) => {
        ev.stopPropagation();
        const score = (i: number) => Object.entries(cur).filter(([kk, vv]) => kk !== k && D.nodes[i]!.variant[kk] === vv).length + (i === defaults[fam.name]! ? 0.5 : 0);
        onPick(fam.variants.filter((i) => D.nodes[i]!.variant[k] === v).sort((a, b) => score(b) - score(a))[0]!);
      });
      row.append(b);
    });
    container.append(row);
  });
  container.hidden = !container.childElementCount;
  restoreVariantGroup(container, focused);
}

function syncEnds() {
  for (const side of ["source", "target"] as const) {
    const input = $<HTMLInputElement>("#" + side), opts = $("#" + side + "-options"), i = state[side];
    if (document.activeElement !== input) input.value = i == null ? "" : display(D.nodes[i]!.name);
    if (i == null) { opts.hidden = true; opts.replaceChildren(); } else variantOptions(opts, i, (j) => setEnd(side, j));
  }
  $("#swap").style.top = ($(".end.to").offsetTop - 15) + "px";
}

function setEnd(side: Side, index: number) {
  state[side] = index;
  closePop();
  syncEnds();
  if (state.source != null && state.target != null) run();
  else {
    state.result = null; paintGraph(); renderResults();
    frame([scene.byId.get(D.nodes[index]!.name)!], 1.2);
    if (side === "source") $("#target").focus();
  }
}

/* ---------- Search fields ---------- */
let suggestFor: Side | null = null, suggestRows: Family[] = [], active = 0;
function matches(q: string) {
  q = q.trim().toLowerCase();
  const all = [...families.values()];
  if (!q) return all.sort((a, b) => b.nbrs.size - a.nbrs.size).slice(0, 7);
  const rank = (f: Family) => {
    const al = (aliasMap[f.name] || []).map((a) => a.toLowerCase()), name = display(f.name).toLowerCase();
    if (al.includes(q)) return 0;
    if (name.startsWith(q)) return 1;
    if (name.includes(q) || f.name.toLowerCase().includes(q)) return 2;
    if (al.some((a) => a.startsWith(q))) return 3;
    return 9;
  };
  return all.map((f) => [rank(f), f] as const).filter(([r]) => r < 9).sort((a, b) => a[0] - b[0] || b[1].nbrs.size - a[1].nbrs.size).slice(0, 7).map(([, f]) => f);
}
function showSuggest(side: Side) {
  suggestFor = side;
  const input = $<HTMLInputElement>("#" + side), box = $("#suggest");
  suggestRows = matches(input.value === (state[side] == null ? "" : display(D.nodes[state[side]]!.name)) ? "" : input.value);
  active = 0;
  box.replaceChildren();
  if (!suggestRows.length) {
    const empty = el("div", "note", "No matching problem");
    empty.setAttribute("role", "option"); empty.setAttribute("aria-disabled", "true"); box.append(empty);
  }
  $("#search-status").textContent = suggestRows.length ? "" : "No matching problem";
  suggestRows.forEach((f, i) => {
    const b = el("button"); b.type = "button"; b.setAttribute("role", "option"); b.id = "opt-" + i;
    b.append(el("span", "", display(f.name)));
    const al = (aliasMap[f.name] || [])[0]; if (al) b.append(el("small", "", al));
    b.addEventListener("pointerdown", (e) => { e.preventDefault(); pick(i); });
    box.append(b);
  });
  const end = input.closest<HTMLElement>(".end")!;
  box.style.top = (end.offsetTop + 44) + "px";
  box.hidden = false; input.setAttribute("aria-expanded", "true");
  markActive();
}
function markActive() {
  [...$("#suggest").querySelectorAll("button")].forEach((b, i) => b.setAttribute("aria-selected", String(i === active)));
  for (const side of ["source", "target"] as const) $("#" + side).removeAttribute("aria-activedescendant");
  if (suggestFor && suggestRows.length) $("#" + suggestFor).setAttribute("aria-activedescendant", "opt-" + active);
}
function hideSuggest() {
  $("#suggest").hidden = true; $("#search-status").textContent = "";
  for (const side of ["source", "target"] as const) {
    $("#" + side).setAttribute("aria-expanded", "false"); $("#" + side).removeAttribute("aria-activedescendant");
  }
  suggestFor = null;
}
function pick(i: number) { const f = suggestRows[i]; if (!f) return; const side = suggestFor!; hideSuggest(); $("#" + side).blur(); setEnd(side, defaults[f.name]!); }
for (const side of ["source", "target"] as const) {
  const input = $<HTMLInputElement>("#" + side);
  input.addEventListener("focus", () => { input.select(); showSuggest(side); });
  input.addEventListener("input", () => showSuggest(side));
  input.addEventListener("blur", () => setTimeout(() => {
    if (document.activeElement !== input && suggestFor === side) hideSuggest();
    syncEnds();
  }, 0));
  input.addEventListener("keydown", (e) => {
    if (e.key === "ArrowDown" || e.key === "ArrowUp") { e.preventDefault(); active = (active + (e.key === "ArrowDown" ? 1 : -1) + suggestRows.length) % Math.max(1, suggestRows.length); markActive(); }
    else if (e.key === "Enter") { e.preventDefault(); pick(active); }
    else if (e.key === "Escape") { hideSuggest(); input.blur(); }
  });
}
$("#swap").addEventListener("click", () => { [state.source, state.target] = [state.target, state.source]; syncEnds(); if (state.source != null && state.target != null) run(); });

/* ---------- Node popover ---------- */
let popNode: SceneNode | null = null, popAnchor: SceneItem | null = null, popIndex: number | null = null;
function routeVariantOf(name: string) {
  const r = state.result?.paths[state.selected! - 1]; if (!r) return null;
  const hit = endpoints(r).find((e) => e.name === name);
  return hit ? indexOf.get(keyOf(hit)) : null;
}
function openPop(node: SceneNode, {keyboard = false} = {}) {
  popNode = popAnchor = node;
  const name = node.id;
  popIndex = [state.source, state.target].find((i) => i != null && D.nodes[i]!.name === name) ?? routeVariantOf(name) ?? defaults[name]!;
  renderPop();
  if (keyboard) $("#pop button").focus({preventScroll: true});
}
function renderPop() {
  const pop = $("#pop"), focusGroup = focusedVariantGroup(pop);
  pop.replaceChildren();
  const title = el("h2", "", display(popNode!.id)); title.id = "pop-title"; pop.append(title);
  const opts = el("div", "options"); variantOptions(opts, popIndex!, (j) => { popIndex = j; renderPop(); }); pop.append(opts);
  const acts = el("div", "pop-actions");
  const a = el("button", "b-start", "Start"), b = el("button", "b-end", "End");
  a.type = b.type = "button";
  a.addEventListener("click", () => setEnd("source", popIndex!)); b.addEventListener("click", () => setEnd("target", popIndex!));
  acts.append(a, b); pop.append(acts);
  const details = el("button", "pop-details", "Details"); details.type = "button";
  details.onclick = () => showProblem(D.nodes[popIndex!]!); pop.append(details);
  if (lensOn) pop.append(popLens(popIndex!));
  pop.hidden = false; placePop();
  restoreVariantGroup(pop, focusGroup);
}
function placePop() {
  if (!popAnchor || $("#pop").hidden) return;
  const pop = $("#pop"), p = scene.position(popAnchor), r = freeRect();
  pop.style.maxHeight = Math.max(1, r.h - 24) + "px";
  const w = pop.offsetWidth, h = pop.offsetHeight;
  let x = p.x + 20; if (x + w > r.x + r.w - 12) x = p.x - 20 - w;
  pop.style.left = Math.max(r.x + 12, x) + "px";
  pop.style.top = Math.max(r.y + 12, Math.min(p.y - h / 2, r.y + r.h - h - 12)) + "px";
}
function closePop(restoreFocus = false) {
  const opener = popNode?.shape;
  popNode = popAnchor = null; $("#pop").hidden = true;
  if (restoreFocus) opener?.focus({preventScroll: true});
}
scene.onNodeTap = openPop;
scene.onBackgroundTap = () => closePop();
document.addEventListener("keydown", (e) => {
  if (e.key === "Escape") { closePop(true); closeDetails(); }
  if (e.key === "/" && (!modalDetails.matches || $("#details-drawer").hidden) && !document.activeElement!.matches("input, textarea, [contenteditable]")) { e.preventDefault(); $(state.source == null ? "#source" : "#target").focus(); }
});

/* ---------- On-demand articles ---------- */
const variantKey = (variant: Endpoint['variant']) => Object.entries(variant).sort().map(([k, v]) => k + "=" + v).join(",");
const detailId = (endpoint: Endpoint) => endpoint.name + "/" + variantKey(endpoint.variant);
let drawerOpener: HTMLElement | SVGElement | null = null;
const drawerBackground = new Map<HTMLElement, boolean>();
function syncDrawerModal() {
  const drawer = $("#details-drawer"), modal = modalDetails.matches && !drawer.hidden;
  if (modal) {
    drawer.setAttribute("aria-modal", "true");
    for (const node of document.body.children) {
      if (!(node instanceof HTMLElement) || node === drawer || drawerBackground.has(node)) continue;
      drawerBackground.set(node, node.inert); node.inert = true;
    }
    if (!drawer.contains(document.activeElement)) $("#close-details").focus({preventScroll: true});
  } else {
    drawer.removeAttribute("aria-modal");
    for (const [node, inert] of drawerBackground) node.inert = inert;
    drawerBackground.clear();
  }
}
function showDetails(title: string, key: string, parentKey: string) {
  if (state.result) state.picked = true;
  const drawer = $("#details-drawer"), body = $("#drawer-body"), atlas = $<HTMLAnchorElement>("#drawer-atlas");
  const opening = drawer.hidden;
  if (opening) drawerOpener = $("#pop").contains(document.activeElement) ? popNode!.shape : document.activeElement as HTMLElement | SVGElement | null;
  closePop(); drawer.hidden = false; $(".dock").hidden = true;
  // The drawer narrows the free canvas; reframe only when it would cover part of the selected route.
  if (opening && !modalDetails.matches && state.result && state.selected) {
    const r = freeRect(), b = scene.bounds(routeEls(state.selected), true);
    if (b.x1 < r.x || b.x2 > r.x + r.w || b.y1 < r.y || b.y2 > r.y + r.h) frame(routeEls(state.selected), 1.3);
  }
  $("#details-title").textContent = title; body.scrollTop = 0;
  const endpoint = (value: string) => value.split("/").map(encodeURIComponent);
  if (key.startsWith("problem:")) {
    const [name, variant] = endpoint(key.slice(8));
    atlas.href = `./index.html#problem/${name}${variant ? `?variant=${variant}` : ""}`;
  } else {
    const [source, target] = key.slice(5).split("->").map(endpoint);
    atlas.href = `./index.html#reduction/${source![0]}/${target![0]}?from=${source![1]}&to=${target![1]}`;
  }
  renderDetails(body, "", "", key, parentKey, (problem) => {
    const [name, variant] = problem.split("/") as [string, string?];
    const index = variant == null ? defaults[name]! : D.nodes.findIndex((n) => n.name === name && variantKey(n.variant) === variant);
    if (index == null || index < 0) return;
    showProblem(D.nodes[index]!); pulseAndFrame(name);
  });
  $("#close-details").focus({preventScroll: true}); syncDrawerModal(); scheduleLabels(); placePop();
}
function showProblem(endpoint: Endpoint) {
  showDetails(display(endpoint.name), "problem:" + detailId(endpoint), "problem:" + endpoint.name);
}
function showRule(step: Step) {
  showDetails(display(step.from.name) + " → " + display(step.to.name),
    `rule:${detailId(step.from)}->${detailId(step.to)}`, `rule:${step.from.name}->${step.to.name}`);
}
function closeDetails() {
  if ($("#details-drawer").hidden) return;
  $("#details-drawer").hidden = true; $(".dock").hidden = false;
  syncDrawerModal();
  if (drawerOpener?.isConnected) drawerOpener.focus({preventScroll: true});
  scheduleLabels();
}
function pulseAndFrame(name: string) {
  const node = scene.byId.get(name)!; scene.classes([node], "", "pulse");
  setTimeout(() => scene.classes([node], "pulse"), 1000); frame([node], 1.6);
}
$("#close-details").onclick = closeDetails;
scene.onEdgeTap = (edge) => {
  if (lensOn) { openEdgePop(edge); return; }
  if (!scene.has(edge, "route")) return;
  const step = state.result?.paths[state.selected! - 1]!.path.find((s) =>
    s.from.name === edge.source.id && s.to.name === edge.target.id);
  if (step) showRule(step);
};
modalDetails.addEventListener("change", syncDrawerModal);
$("#details-drawer").addEventListener("keydown", (e) => {
  if (!modalDetails.matches || e.key !== "Tab") return;
  const items = [...$("#details-drawer").querySelectorAll<HTMLElement>('a[href], button, summary, [tabindex="0"]')].filter((n) => n.getClientRects().length);
  const first = items[0], last = items.at(-1);
  if (e.shiftKey && document.activeElement === first) { e.preventDefault(); last!.focus(); }
  else if (!e.shiftKey && document.activeElement === last) { e.preventDefault(); first!.focus(); }
});

/* ---------- Path engine ---------- */
let worker = graphStartup.worker, reqId = 0, clock = 0;
graphStartup.worker = null;
const cache = new Map<string, PathResult>();
function stopWorker() { worker?.terminate(); worker = null; }
function compute(source: number, target: number, limit: number, onPaths: (result: PathResult) => void): Promise<PathResult> {
  const key = `${source}-${target}-${limit}`, cached = cache.get(key);
  if (cached) {
    cache.delete(key); cache.set(key, cached);
    onPaths(cached);
    if (cached.polynomial_comparison || !cached.paths.length) return Promise.resolve(cached);
  }
  const request: PathRequest = cached
    ? {kind: 'compare', source, target, limit,
      cached: {paths: cached.paths, truncated: cached.truncated}}
    : {kind: 'search', source, target, limit};
  return new Promise<PathResult>((resolve, reject) => {
    try { worker ||= new Worker(new URL('../workers/path-worker.ts', import.meta.url), { type: "module" }); } catch (err) { reject(err); return; }
    const engine = worker;
    let result = cached;
    worker.onmessage = ({ data }: MessageEvent<PathReply>) => {
      if (worker !== engine) return;
      try {
        const started = performance.now();
        const reply = validatePathReply(data, request, result);
        performance.measure(`graph:${reply.phase}-validation`, {start: started});
        performance.mark(`graph:${reply.phase}-received`);
        if ('error' in reply) { stopWorker(); reject(new Error(reply.error)); return; }
        if (reply.phase === "paths") {
          result = reply.result; cache.set(key, result); onPaths(result);
          if (cache.size > 8) cache.delete(cache.keys().next().value!);
          if (result.paths.length) return;
        } else {
          result!.polynomial_comparison = reply.result;
        }
        resolve(result!);
      } catch (error) {
        stopWorker(); reject(error);
      }
    };
    worker.onerror = (ev) => { ev.preventDefault(); if (worker !== engine) return; stopWorker(); reject(new Error("engine")); };
    performance.mark("graph:request-posted");
    worker.postMessage(request);
  });
}
function orderPaths(result: PathResult) {
  const c = result.polynomial_comparison;
  const degrees = c?.profiles.map(pathDegree), front = new Set(c?.front);
  result.order = result.paths.map((_, i) => i + 1).sort((a, b) =>
    (c ? (Number(front.has(b)) - Number(front.has(a))) || compareDegrees(degrees![a - 1], degrees![b - 1]) : 0) ||
    result.paths[a - 1]!.steps - result.paths[b - 1]!.steps || a - b);
}
async function run() {
  const id = ++reqId;
  if (state.busy) stopWorker();
  clearInterval(clock);
  state.result = null; state.selected = null; state.picked = false; state.showSlower = false; state.busy = true;
  paintGraph(); renderResults();
  frame([scene.byId.get(D.nodes[state.source!]!.name)!, scene.byId.get(D.nodes[state.target!]!.name)!], 1.1);
  if (state.source === state.target) { state.busy = false; renderResults("same"); return; }
  const started = Date.now();
  clock = setInterval(() => { const s = $("#elapsed"); if (s) s.textContent = Math.round((Date.now() - started) / 1000) + " s"; }, 1000);
  try {
    const result = await compute(state.source!, state.target!, state.limit, (paths) => {
      if (id !== reqId || !paths.paths.length) return;
      orderPaths(paths); state.result = paths;
      select(paths.order![0]!);
      performance.mark("graph:routes-rendered");
    });
    if (id !== reqId) return;
    clearInterval(clock); state.busy = false;
    if (!result.paths.length) { renderResults("none"); paintGraph(); return; }
    orderPaths(result); state.result = result;
    // Until the user picks a route, show the best one; afterwards the comparison only re-ranks rows.
    if (state.picked) { paintGraph(); renderResults(); } else select(result.order![0]!);
    performance.mark("graph:comparison-rendered");
  } catch (err) {
    if (id !== reqId) return;
    clearInterval(clock); state.busy = false; state.result = null; state.selected = null; renderResults("error"); paintGraph();
  }
}
function cancel() {
  reqId++; clearInterval(clock); stopWorker(); state.busy = false; state.result = null; state.selected = null; renderResults("cancelled"); paintGraph();
}

/* ---------- Path display ---------- */
function endpoints(route: Route) { return [route.path[0].from, ...route.path.map((s) => s.to)]; }
function routeEls(n: number) {
  const ids = new Set<string>();
  state.result!.paths[n - 1]!.path.forEach((s) => { ids.add(s.from.name); ids.add(s.to.name); if (s.from.name !== s.to.name) ids.add(s.from.name + "→" + s.to.name); });
  return [...ids].map(id => scene.byId.get(id)!);
}
function paintGraph() {
  if (lensOn) return;
  scene.classes(scene.elements, "dim ghost route start end nbr");
  scene.nodes.forEach(n => { n.routeLabel = display(n.id); });
  if (state.result && state.selected) {
    scene.classes(scene.elements, "", "dim");
    (state.result!.polynomial_comparison?.front || []).forEach(n => scene.classes(routeEls(n), "dim", "ghost"));
    const selected = routeEls(state.selected);
    scene.classes(selected, "dim ghost", "route");
    // Selected paths stay above the rest of the scene for reliable edge hit testing.
    selected.filter(n => !n.label).forEach(n => scene.world.insertBefore(n.shape, scene.nodes[0]!.shape));
    const labelled = new Set();
    endpoints(state.result!.paths[state.selected! - 1]!).forEach(e => {
      if (!multi(e.name) || labelled.has(e.name)) return;
      labelled.add(e.name);
      const def = D.nodes[defaults[e.name]!]!.variant;
      const diff = Object.entries(e.variant).filter(([k,v]) => def[k] !== v).map(([k,v]) => k + ": " + v);
      if (diff.length) (scene.byId.get(e.name)! as SceneNode).routeLabel = display(e.name) + "\n" + diff.join("  ");
    });
  } else if (state.source != null && state.target != null) scene.classes(scene.elements, "", "dim");
  for (const side of ["source", "target"] as const) if (state[side] != null)
    scene.classes([scene.byId.get(D.nodes[state[side]]!.name)!], "dim", side === "source" ? "start" : "end");
  scene.nodes.forEach(node => node.shape.setAttribute('aria-label', scene.describe(node)));
  scheduleLabels();
}
function select(n: number) {
  state.selected = n;
  paintGraph(); renderResults();
  const focused = scene.nodeOf(document.activeElement);
  if (focused) scene.reveal(focused);
  else frame(routeEls(n), 1.3);
}

/* Problem families along a route; consecutive variants of one family are a single stop. */
function familyChain(route: Route) {
  const names: string[] = []; endpoints(route).forEach((e) => { if (names.at(-1) !== e.name) names.push(e.name); });
  return names;
}
/* The ends are already in the search box, so a row says only what is in between; the timeline lists every stop. */
function chain(route: Route) {
  const names = familyChain(route), via = names.slice(1, -1);
  const span = el("span", "chain", via.length ? `via ${display(via[0]!)}${via.length > 1 ? ` +${via.length - 1}` : ""}` : "Direct");
  span.title = names.map(display).join(" → ");
  return span;
}
function pathRow(n: number, slower: boolean) {
  const c = state.result!.polynomial_comparison, route = state.result!.paths[n - 1]!, open = state.selected === n;
  const wrap = el("div");
  const b = el("button", "path" + (slower ? " slower" : "")); b.type = "button";
  b.setAttribute("aria-expanded", String(open)); b.dataset.path = String(n);
  const size = el("span", "size" + (c ? "" : " pending"), c ? sizeBadge(pathDegree(c.profiles[n - 1]!)) : "");
  size.title = c && pathDegree(c.profiles[n - 1]!) == null ? "Size unavailable for one or more parameters" : "Largest growth of the reduced problem";
  const line = el("div", "path-line"); line.append(chain(route), size);
  b.append(line);
  b.addEventListener("click", () => { state.picked = true; if (state.selected !== n) select(n); });
  wrap.append(b);
  if (open) wrap.append(pathDetail(n));
  return wrap;
}
function pathDetail(n: number) {
  const route = state.result!.paths[n - 1]!, profile = state.result!.polynomial_comparison?.profiles[n - 1]!;
  const d = el("div", "detail");
  const ol = el("ol", "steps");
  // Consecutive steps inside one family are variant conversions: show the family once with each variant it passes through.
  const groups: {name: string; variants: Endpoint['variant'][]}[] = [];
  endpoints(route).forEach((e) => { if (groups.at(-1)?.name === e.name) groups.at(-1)!.variants.push(e.variant); else groups.push({ name: e.name, variants: [e.variant] }); });
  groups.forEach((e, i) => {
    const li = el("li"), btn = el("button"); btn.type = "button";
    btn.append(el("span", "name", display(e.name)));
    if (multi(e.name)) e.variants.forEach((v) => btn.append(kvNode(v)));
    btn.addEventListener("click", () => { showProblem({name: e.name, variant: e.variants[0]!}); pulseAndFrame(e.name); });
    li.append(el("span", "dot"), btn); ol.append(li);
    if (i < groups.length - 1) {
      const next = groups[i + 1]!, rule = el("button", "rule-link", "rule"); rule.type = "button";
      rule.title = `Reduction rule: ${display(e.name)} to ${display(next.name)}`;
      rule.onclick = () => showRule({from: {name: e.name, variant: e.variants.at(-1)!}, to: {name: next.name, variant: next.variants[0]!}});
      li.append(rule);
    }
  });
  const dl = el("dl", "growth");
  Object.entries(profile?.fields || {}).forEach(([name, f]) => dl.append(el("dt", "", prettyField(name)), el("dd", "", growth(f).lead)));
  const label = "Copy route for pred reduce --via";
  const copy = el("button", "copy link", label); copy.type = "button";
  copy.addEventListener("click", async () => {
    state.picked = true;
    const json = JSON.stringify({ steps: route.steps, path: route.path }, null, 2);
    try { await navigator.clipboard.writeText(json); copy.textContent = "Copied"; setTimeout(() => (copy.textContent = label), 1500); }
    catch { const t = el("textarea", "copy-text"); t.readOnly = true; t.value = json; copy.after(t); t.select(); }
  });
  d.append(ol);
  if (dl.children.length) d.append(el("p", "growth-title", "Result size"), dl);
  d.append(copy);
  return d;
}
function examples() {
  const c = el("div", "chips");
  ([["SAT → QUBO", "Satisfiability", "QUBO"], ["MIS → QUBO", "MaximumIndependentSet", "QUBO"], ["Factoring → ILP", "Factoring", "ILP"], ["HC → QUBO", "HamiltonianCircuit", "QUBO"]] as const).forEach(([label, s, t]) => {
    const b = el("button", "chip", label); b.type = "button";
    b.addEventListener("click", () => { state.source = defaults[s]!; state.target = defaults[t]!; state.limit = 20; syncEnds(); run(); });
    c.append(b);
  });
  return c;
}
function renderResults(status?: 'same' | 'none' | 'error' | 'cancelled') {
  const box = $("#results"); box.replaceChildren();
  if (state.source == null || state.target == null) {
    const n = el("div", "note"); n.append(el("p", "", "Pick a start and an end problem, by searching or by clicking the graph. Try:"), examples()); box.append(n); return;
  }
  if (state.busy && !state.result) {
    const s = el("div", "summary"), left = el("span", "", "Comparing paths "), t = el("span", "", ""); t.id = "elapsed"; left.append(t);
    const c = el("button", "link", "Cancel"); c.type = "button"; c.addEventListener("click", cancel); s.append(left, c);
    box.append(s); for (let i = 0; i < 4; i++) box.append(el("div", "skeleton")); return;
  }
  const msg = status && { same: "Start and end are the same problem variant.", none: "No reduction path connects these two. Try another variant of either problem.",
    error: "Cannot load the path engine. Reload this release and try again.", cancelled: "Search cancelled." }[status];
  if (msg) {
    const n = el("div", "note"); n.append(el("p", "", msg));
    if (status === "error") n.append(examples());
    if (status === "cancelled") { const r = el("button", "link", "Search again"); r.type = "button"; r.onclick = run; n.append(r); }
    box.append(n); return;
  }
  if (!state.result) return;
  const c = state.result!.polynomial_comparison;
  // Merge same-chain variants by rank, retaining the exact route currently selected.
  const seen = new Set(), chainKey = (n: number) => endpoints(state.result!.paths[n - 1]!).map((e) => e.name).filter((x, i, a) => x !== a[i - 1]).join(">");
  const selectedChain = chainKey(state.selected!);
  const shown = state.result.order!.filter((n) => {
    const k = chainKey(n);
    if (seen.has(k) || (k === selectedChain && n !== state.selected)) return false;
    seen.add(k); return true;
  });
  const onFront = new Set(c?.front), front = shown.filter((n) => !c || onFront.has(n)), slower = shown.filter((n) => c && !onFront.has(n));
  const s = el("div", "summary"), left = el("span");
  if (c) left.append(el("strong", "", `${front.length} best route${front.length === 1 ? "" : "s"}`), ` of ${shown.length}`);
  else { const elapsed = el("span"); elapsed.id = "elapsed"; left.append("Comparing paths ", elapsed); }
  s.append(left);
  if (state.busy) {
    const cancelButton = el("button", "link", "Cancel"); cancelButton.type = "button"; cancelButton.onclick = cancel; s.append(cancelButton);
  } else if (state.result.truncated && state.limit < 999) {
    const more = el("button", "link", "Search more"); more.type = "button";
    more.addEventListener("click", () => { state.limit = state.limit < 50 ? 50 : state.limit < 100 ? 100 : 999; run(); });
    s.append(more);
  }
  box.append(s);
  front.forEach((n) => box.append(pathRow(n, false)));
  if (slower.length && !front.length) slower.forEach((n) => box.append(pathRow(n, true)));
  else if (slower.length) {
    const open = state.showSlower || slower.includes(state.selected!);
    const t = el("button", "link more", open ? "Hide slower routes" : `Show ${slower.length} slower routes`); t.type = "button";
    t.addEventListener("click", () => { state.showSlower = !open; if (!state.showSlower && slower.includes(state.selected!)) { select(front[0]!); return; } renderResults(); });
    box.append(t);
    if (open) slower.forEach((n) => box.append(pathRow(n, true)));
  }
}


/* ---------- Overhead lens ---------- */
/* Analysis runs on concrete variants. A family edge only groups the exact variant rules behind it. */
const rules = D.edges as RuleEdge[];
interface Lens { scope: Scope; limit: ClassId; focus: number | null; dir: 'down' | 'up'; group: number | null }
const lens: Lens = { scope: 'witness', limit: 'unknown', focus: null, dir: 'down', group: null };
const classOrder = CLASSES.map(c => c.id);
const SCOPES: readonly { id: Scope; name: string; hint: string }[] = [
  { id: 'witness', name: 'Witness', hint: 'Rules that map solutions back to the source. Paths mode searches this scope.' },
  { id: 'aggregate', name: 'Value', hint: 'Rules that also map completed values (optimum, count, decision).' },
  { id: 'turing', name: 'Turing', hint: 'Multi-query rules, such as binary search over a decision bound.' },
];
const vtext = (i: number) => Object.entries(D.nodes[i]!.variant).map(([k, v]) => k + ": " + v).join("  ") || "default";
const vname = (i: number) => display(D.nodes[i]!.name) + (multi(D.nodes[i]!.name) ? " · " + vtext(i) : "");
const badge = (cls: ClassId) => {
  const c = CLASSES.find(x => x.id === cls)!, b = el("span", "gc g-" + cls, c.label);
  b.title = c.name; return b;
};
const nontrivial = (groups: number[][]) => groups.filter(g => g.length > 1).sort((a, b) => b.length - a.length);
function lensModel() {
  const active = activeRules(rules, { scope: lens.scope, limit: lens.limit });
  const links = active.map(i => rules[i]!), fam = familyRules(D.nodes, rules, active);
  const groups = nontrivial(components(D.nodes.length, links).groups);
  const scoped = activeRules(rules, { scope: lens.scope, limit: "unknown" });
  const counts = new Map<ClassId, number>();
  scoped.forEach(i => { const c = edgeGrowth(rules[i]!).cls; counts.set(c, (counts.get(c) ?? 0) + 1); });
  const baseline = nontrivial(components(D.nodes.length, scoped.map(i => rules[i]!)).groups).length;
  return { active, links, fam, groups, scoped, counts, baseline };
}
let model = lensModel();
const ruleClass = (i: number) => edgeGrowth(rules[i]!).cls;
const ruleSteps = (i: number) => ({ from: { name: D.nodes[rules[i]!.source]!.name, variant: D.nodes[rules[i]!.source]!.variant },
  to: { name: D.nodes[rules[i]!.target]!.name, variant: D.nodes[rules[i]!.target]!.variant } });

function reachOf(focus: number) {
  const variants = reach(D.nodes.length, model.links, focus, lens.dir);
  const byFamily = new Map<string, number[]>();
  variants.forEach(v => { const n = D.nodes[v]!.name; (byFamily.get(n) ?? byFamily.set(n, []).get(n)!).push(v); });
  byFamily.delete(D.nodes[focus]!.name);
  return { variants, byFamily };
}

function paintLens() {
  const edgeClasses = classOrder.map(c => "e-" + c).join(" ");
  scene.classes(scene.elements, "dim ghost route start end nbr off cyc reach pick hub lit quiet near " + edgeClasses);
  const linked = new Set<string>(), cyc = new Set<string>();
  model.fam.forEach(f => { linked.add(f.source); linked.add(f.target); });
  model.groups.forEach(g => g.forEach(v => cyc.add(D.nodes[v]!.name)));
  scene.nodes.forEach(n => { n.routeLabel = display(n.id); });
  const picked = lens.group != null ? new Set(model.groups[lens.group]) : null;
  const focus = lens.focus != null ? { ...reachOf(lens.focus), v: lens.focus } : null;
  const inside = (v: number) => picked ? picked.has(v) : focus ? v === focus.v || focus.variants.has(v) : true;
  const lit = new Set<string>();
  scene.edges.forEach(edge => {
    const f = model.fam.get(edge.id);
    if (!f) { scene.classes([edge], "", "off"); return; }
    scene.classes([edge], "", "e-" + f.cls);
    if (!picked && !focus && (edge.source.id === "ILP" || edge.target.id === "ILP")) scene.classes([edge], "", "hub");
    if ((picked || focus) && !f.rules.some(i => inside(rules[i]!.source) && inside(rules[i]!.target))) scene.classes([edge], "", "dim");
    else if (picked || focus) { lit.add(edge.source.id); lit.add(edge.target.id); scene.classes([edge], "", "lit"); }
  });
  scene.nodes.forEach(node => {
    const members = families.get(node.id)!.variants;
    if (cyc.has(node.id)) scene.classes([node], "", "cyc");
    if (picked || focus) {
      if (members.some(inside) && (lit.has(node.id) || (focus && members.includes(focus.v)) || members.some(v => picked?.has(v)))) scene.classes([node], "", "reach");
      else scene.classes([node], "", "dim");
    } else if (!linked.has(node.id)) scene.classes([node], "", "dim");
  });
  scene.nodes.forEach(node => node.shape.setAttribute('aria-label', scene.describe(node) + (cyc.has(node.id) ? ', in a mutually reachable group' : '')));
  scheduleLabels();
}
/* Frame the dense middle of the map; a few far-flung isolated problems should not shrink everything else. */
function frameCore() {
  const linked = scene.nodes.filter(n => families.get(n.id)!.nbrs.size > 1);
  const cx = linked.reduce((a, n) => a + n.x, 0) / linked.length, cy = linked.reduce((a, n) => a + n.y, 0) / linked.length;
  const near = linked.map(n => [Math.hypot(n.x - cx, n.y - cy), n] as const).sort((a, b) => a[0] - b[0]);
  frame(near.slice(0, Math.ceil(near.length * .85)).map(([, n]) => n), 1.6);
}
function refreshLens() { model = lensModel(); if (lens.group != null && lens.group >= model.groups.length) lens.group = null; paintLens(); renderLens(); }

const short = (name: string) => aliasMap[name]?.[0] ?? display(name);
const pct = (n: number, d: number) => d ? Math.round(100 * n / d) : 0;

/* The distribution bar doubles as legend and filter: each segment keeps every class up to itself. */
function costCard(box: HTMLElement) {
  const m = model, total = [...m.counts.values()].reduce((a, b) => a + b, 0), limitAt = classOrder.indexOf(lens.limit);
  const mild = classOrder.slice(0, classOrder.indexOf("quadratic") + 1).reduce((a, c) => a + (m.counts.get(c) ?? 0), 0);
  const card = el("section", "lcard cost");
  const hero = el("div", "hero-n"); hero.append(el("span", "num", String(pct(mild, total))), el("span", "of", "%"));
  const title = el("h2", "eyebrow", "Overhead of a reduction"); title.title = "Total degree of the target sizes when every source size grows together. A visualization convention, not a runtime bound.";
  card.append(title, hero,
    el("p", "cap", "of reductions are at most quadratic"));
  const bar = el("div", "dist"); bar.setAttribute("role", "radiogroup"); bar.setAttribute("aria-label", "Show reductions up to this growth class");
  const legend = el("div", "dist-legend");
  CLASSES.forEach((c, i) => {
    const n = m.counts.get(c.id) ?? 0;
    if (!n) return;
    const on = i <= limitAt, name = `${c.name}: ${n} reductions (${pct(n, total)}%)`;
    const seg = el("button", "seg-bar g-" + c.id + (on ? " on" : "")); seg.type = "button"; seg.style.flexGrow = String(n);
    seg.setAttribute("role", "radio"); seg.setAttribute("aria-checked", String(c.id === lens.limit)); seg.setAttribute("aria-label", name); seg.title = name; seg.dataset.key = "limit-" + c.id;
    seg.addEventListener("click", () => { lens.limit = c.id; lens.focus = null; refreshLens(); });
    bar.append(seg);
    const key = el("button", "dist-key g-" + c.id + (on ? " on" : ""), ""); key.type = "button"; key.tabIndex = -1; key.setAttribute("aria-hidden", "true");
    key.append(el("b", "", c.label), el("span", "", String(n)));
    key.addEventListener("click", () => seg.click());
    legend.append(key);
  });
  card.append(bar, legend);
  const seg = el("div", "seg"); seg.setAttribute("role", "radiogroup"); seg.setAttribute("aria-label", "Rule scope");
  SCOPES.forEach(x => {
    const b = el("button", "", x.name); b.type = "button"; b.setAttribute("role", "radio"); b.setAttribute("aria-checked", String(x.id === lens.scope)); b.title = x.hint;
    b.addEventListener("click", () => { lens.scope = x.id; lens.focus = null; lens.group = null; refreshLens(); });
    seg.append(b);
  });
  card.append(seg);
  box.append(card);
}

function groupsCard(box: HTMLElement) {
  const m = model, card = el("section", "lcard");
  const head = el("div", "lcard-head"); head.append(el("h2", "eyebrow", "Mutually reducible"));
  const count = el("span", "count", m.baseline === m.groups.length ? String(m.groups.length) : `${m.groups.length} of ${m.baseline}`);
  count.title = "Groups of variants that reduce to each other"; head.append(count); card.append(head);
  const list = el("div", "groups");
  m.groups.forEach((g, gi) => {
    const families = [...new Set(g.map(v => D.nodes[v]!.name))], open = lens.group === gi;
    const b = el("button", "group"); b.type = "button"; b.dataset.key = "group-" + gi; b.setAttribute("aria-pressed", String(open));
    b.append(el("span", "group-names", families.map(short).join(" · ")), el("span", "group-size", String(g.length)));
    b.title = g.map(vname).join("\n");
    b.addEventListener("click", () => { lens.group = open ? null : gi; lens.focus = null; paintLens(); renderLens(); if (!open) frame(scene.nodes.filter(n => scene.has(n, "reach")), 1.2); });
    list.append(b);
    if (open) { const d = el("ul", "group-detail"); g.slice().sort((a, c) => a - c).forEach(v => d.append(el("li", "", vname(v)))); list.append(d); }
  });
  if (!m.groups.length) list.append(el("p", "cap", "None"));
  card.append(list); box.append(card);
}

function focusCard(box: HTMLElement) {
  const f = lens.focus!, { variants, byFamily } = reachOf(f), card = el("section", "lcard focus");
  const head = el("div", "lcard-head"); head.append(el("h2", "eyebrow", lens.dir === "down" ? "Reduces to" : "Reduced from"));
  const clear = el("button", "link", "Clear"); clear.type = "button"; clear.addEventListener("click", () => { lens.focus = null; paintLens(); renderLens(); });
  head.append(clear); card.append(head);
  card.append(el("p", "focus-name", display(D.nodes[f]!.name)), el("p", "cap", `${multi(D.nodes[f]!.name) ? vtext(f) + " · " : ""}${variants.size} variants`));
  const items = el("div", "reach-list");
  [...byFamily].sort((a, b) => b[1].length - a[1].length || a[0].localeCompare(b[0])).forEach(([name, vs]) => {
    const b = el("button", "reach-row"); b.type = "button"; b.append(el("span", "", display(name)), el("small", "", multi(name) ? `${vs.length}/${families.get(name)!.variants.length}` : ""));
    b.title = "Compare routes in Paths";
    b.addEventListener("click", () => { const pair = lens.dir === "down" ? [f, vs[0]!] : [vs[0]!, f]; setTab("paths"); state.source = pair[0]!; state.target = pair[1]!; syncEnds(); run(); });
    items.append(b);
  });
  card.append(items); box.append(card);
}

function renderLens() {
  const box = $("#lens"), hadFocus = document.activeElement?.closest("#lens") ? (document.activeElement as HTMLElement).dataset.key : null;
  box.replaceChildren();
  if (lens.focus != null) focusCard(box);
  costCard(box); groupsCard(box);
  if (hadFocus) box.querySelector<HTMLElement>(`[data-key="${hadFocus}"]`)?.focus({ preventScroll: true });
}

/* Node popover section: reach and outgoing rules for the chosen variant. */
function popLens(v: number) {
  const wrap = el("div", "pop-lens");
  const down = reach(D.nodes.length, model.links, v, "down").size, up = reach(D.nodes.length, model.links, v, "up").size;
  const row = el("div", "pop-reach");
  for (const [dir, label, n] of [["down", "Downstream", down], ["up", "Upstream", up]] as const) {
    const b = el("button", "", `${label} ${n}`); b.type = "button"; b.title = `${n} variants ${dir === "down" ? "reachable from" : "that reach"} this one`;
    b.setAttribute("aria-pressed", String(lens.focus === v && lens.dir === dir));
    b.addEventListener("click", () => {
      lens.focus = v; lens.dir = dir; lens.group = null; closePop(); paintLens(); renderLens();
      frame(scene.nodes.filter(n => scene.has(n, "reach")), 1.2);
    });
    row.append(b);
  }
  wrap.append(row);
  const out = [...model.fam.values()].filter(f => f.source === D.nodes[v]!.name);
  if (out.length) {
    out.sort((a, b) => classOrder.indexOf(a.cls) - classOrder.indexOf(b.cls)).forEach(f => {
      const b = el("button", "pop-rule"); b.type = "button"; b.append(badge(f.cls), el("span", "", display(f.target)));
      b.addEventListener("click", () => openEdgePop(scene.byId.get(f.source + "→" + f.target) as SceneEdge));
      wrap.append(b);
    });
  }
  return wrap;
}

/* Edge inspector: every variant rule behind a family link, with exact and bounded formulas kept apart. */
function openEdgePop(edge: SceneEdge) {
  const f = model.fam.get(edge.id);
  if (!f) return;
  popNode = null; popAnchor = edge;
  const pop = $("#pop"); pop.replaceChildren();
  const title = el("h2", "", display(f.source) + " → " + display(f.target)); title.id = "pop-title"; pop.append(title);
  [...f.rules].sort((a, b) => classOrder.indexOf(ruleClass(a)) - classOrder.indexOf(ruleClass(b))).forEach(i => {
    const rule = rules[i]!, g = edgeGrowth(rule), card = el("div", "rule-card");
    const head = el("button", "rule-head"); head.type = "button";
    head.append(badge(g.cls), el("span", "rule-ends", `${vtext(rule.source)} → ${vtext(rule.target)}`));
    head.title = "Open the rule article"; head.addEventListener("click", () => showRule(ruleSteps(i) as Step));
    card.append(head);
    const dl = el("dl", "growth");
    g.fields.forEach(x => { dl.append(el("dt", "", prettyField(x.name)), el("dd", "", x.kind === "unavailable" ? "unavailable" : x.lead)); });
    card.append(dl);
    pop.append(card);
  });
  pop.hidden = false; placePop();
}

function setTab(name: "paths" | "overhead") {
  lensOn = name === "overhead";
  for (const id of ["paths", "overhead"] as const) {
    const tab = $("#tab-" + id); tab.setAttribute("aria-selected", String(id === name)); tab.tabIndex = id === name ? 0 : -1;
    $("#view-" + id).hidden = id !== name;
  }
  document.body.classList.toggle("lens", lensOn);
  closePop();
  if (lensOn) { model = lensModel(); paintLens(); renderLens(); frameCore(); }
  else { scene.classes(scene.elements, "dim ghost route start end nbr off cyc reach pick hub lit quiet near " + classOrder.map(c => "e-" + c).join(" ")); paintGraph(); }
}
for (const id of ["paths", "overhead"] as const) {
  $("#tab-" + id).addEventListener("click", () => setTab(id));
  $("#tab-" + id).addEventListener("keydown", (e) => {
    if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
    const other = id === "paths" ? "overhead" : "paths"; setTab(other); $("#tab-" + other).focus();
  });
}

/* ---------- Dock and resize ---------- */
const zoomBy = (k: number) => { const r = freeRect(); scene.stop(); scene.zoomAt(scene.zoom * k, { x: r.x + r.w / 2, y: r.y + r.h / 2 }); };
$("#zoom-in").onclick = () => zoomBy(1.3);
$("#zoom-out").onclick = () => zoomBy(1 / 1.3);
$("#fit").onclick = () => frame(scene.elements, 1);
addEventListener("resize", () => { scheduleLabels(); placePop(); });
new ResizeObserver(() => {
  document.documentElement.style.setProperty("--header-height", $(".site-header").getBoundingClientRect().height + "px");
  scheduleLabels();
}).observe($(".site-header"));

/* ---------- Start on a real example ---------- */
state.source = defaults.Satisfiability!; state.target = defaults.QUBO!;
syncEnds(); frame(scene.elements, 1, false); run();
performance.mark("graph:interactive");
document.fonts?.ready.then(() => {
  scene.nodes.forEach(n => { n.labelKey = null; });
  const focused = scene.nodes.find(node => scene.has(node, 'focused'));
  if (focused) scene.reveal(focused);
  scheduleLabels();
});
})().catch(() => {
  graphStartup.worker?.terminate();
  graphStartup.worker = null;
  document.querySelector('#results')!.textContent = 'Cannot load graph. Reload this release and try again.';
});

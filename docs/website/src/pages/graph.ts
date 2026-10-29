import { graphStartup } from './graph-startup.ts';
import { renderDetails } from './details.ts';
import { GraphScene, type SceneNode, type SceneItem, type SceneEdge, type LabelSide } from "./graph-scene.ts";
import { growth, pathDegree, sizeBadge, compareDegrees } from "../lib/graph-growth.mts";
import { CLASSES, growthClass, edgeGrowth, activeRules, components, reach, familyRules, depthsFrom, type ClassId, type RuleEdge } from '../lib/graph-structure.mts';
import { validatePathReply, type PathRequest, type PathReply } from '../lib/path-worker-data.mts';
import { createSimulation, settleSimulation, settled, type Point, type Simulation } from '../lib/force-layout.mts';

import type { Endpoint, Route, Step, WasmPathResult } from '../generated/wasm-paths.mjs';
import type { WasmComparisonResult } from '../generated/wasm-comparison.mjs';

type Side = 'source' | 'target';
interface Family { name: string; variants: number[]; nbrs: Set<string> }
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
  if (!families.has(n.name)) families.set(n.name, { name: n.name, variants: [], nbrs: new Set() });
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

/* Route existence is measured on concrete variants, independent of display filters.
   It does not certify polynomial reductions or registered solver capabilities. */
type Status = "both" | "toilp" | "fromsat" | "none";
type FamilyStatus = Status | "mixed";
const HUBS = new Set(["ILP", "QUBO"]);
const STATUS: Record<FamilyStatus, { color: string; label: string; detail: string }> = {
  both: { color: "#6aa64f", label: "Route from SAT · Route to ILP", detail: "A route leads from SAT to this variant and from this variant to ILP" },
  toilp: { color: "#4c93cf", label: "Route to ILP", detail: "A route leads to ILP; no route from SAT is registered" },
  fromsat: { color: "#d4688a", label: "Route from SAT", detail: "A route leads from SAT; no route to ILP is registered" },
  none: { color: "#5b665e", label: "No SAT / ILP route", detail: "No route from SAT or to ILP is registered" },
  mixed: { color: "#a39ac2", label: "Varies by variant", detail: "Variants have different routes; select a variant to inspect them" },
};
const HUB_COLOR = "#e8d56b";
const variantIds = D.nodes.map((_, i) => i), witnessLinks = D.edges.filter(e => e.witness && !e.turing);
const fromSat = depthsFrom(variantIds, witnessLinks, variantIds.filter(i => D.nodes[i]!.name === "Satisfiability"));
const toIlp = depthsFrom(variantIds, witnessLinks.map(e => ({ source: e.target, target: e.source })), variantIds.filter(i => D.nodes[i]!.name === "ILP"));
const variantStatus = new Map(variantIds.map(i => [i,
  (fromSat.get(i) != null ? toIlp.get(i) != null ? "both" : "fromsat" : toIlp.get(i) != null ? "toilp" : "none") as Status]));
// A family gets a single status only when all its variants agree.
const familyStatus = new Map([...families.values()].map(f => {
  const statuses = new Set(f.variants.map(i => variantStatus.get(i)!));
  return [f.name, statuses.size === 1 ? variantStatus.get(f.variants[0]!)! : "mixed"] as [string, FamilyStatus];
}));
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
const scene = new GraphScene($("#cy"), [...families.values()].map(f => ({id: f.name, label: display(f.name),
  size: nodeSize(f.nbrs.size), deg: f.nbrs.size, color: HUBS.has(f.name) ? HUB_COLOR : STATUS[familyStatus.get(f.name)!].color, ...pos.get(f.name)!})), [...famEdges.values()]);
scene.classes(scene.nodes.filter(n => HUBS.has(n.id)), "", "hub-node");
performance.mark("graph:renderer-created");
performance.mark("graph:constructed");

function freeRect() {
  const W = innerWidth, H = innerHeight, top = $(".site-header").getBoundingClientRect().bottom;
  const panel = $(".panel").getBoundingClientRect();
  if (phone.matches) return { x: 0, y: top, w: W, h: Math.max(1, panel.top - top) };
  const right = $("#details-drawer").hidden || modalDetails.matches ? W : $("#details-drawer").getBoundingClientRect().left - 16;
  // A collapsed panel is only its search bar, so the canvas runs under it to the left edge.
  const left = collapsed() ? 0 : 392, below = collapsed() ? panel.bottom : top;
  return { x: left, y: below, w: Math.max(1, right - left), h: H - below };
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
const SIDES: LabelSide[] = ["below", "above", "right", "left"];
// Labels along a straightened route sit across the chain, never on its own edges.
let routeSides = SIDES;
let labelRaf = 0;
const scheduleLabels = () => { cancelAnimationFrame(labelRaf); labelRaf = requestAnimationFrame(placeLabels); };
function placeLabels() {
  const r = freeRect(), boxes: Bounds[] = [];
  const important = (n: SceneNode) => ["route", "start", "end", "hover", "focused", "reach"].some(c => scene.has(n, c));
  const overlaps = (b: Bounds, o: Bounds) => b.x1 < o.x2 + 6 && b.x2 > o.x1 - 6 && b.y1 < o.y2 + 4 && b.y2 > o.y1 - 4;
  const hit = (b: Bounds) => boxes.some(o => overlaps(b, o));
  // Discs of the problems in focus are obstacles too, so their labels never cover each other's nodes.
  const discs = scene.nodes.filter(important).map(n => {
    const p = scene.position(n), d = n.size / 2 * scene.zoom;
    return { n, b: { x1: p.x - d, x2: p.x + d, y1: p.y - d, y2: p.y + d } };
  });
  const clear = (n: SceneNode, b: Bounds) => !hit(b) && !discs.some(o => o.n !== n && overlaps(b, o.b));
  // The first side that is clear; an important label falls back to below.
  const box = (n: SceneNode) => {
    for (const side of scene.has(n, "route") ? routeSides : SIDES) { const b = scene.labelBox(n, side); if (clear(n, b)) return b; }
    return scene.labelBox(n);
  };
  const fits = (b: Bounds) => b.x1 > r.x + 4 && b.x2 < r.x + r.w - 4 && b.y1 > r.y + 4 && b.y2 < r.y + r.h - 4;
  scene.classes(scene.nodes, "named");
  scene.nodes.filter(important).forEach(n => boxes.push(box(n)));
  const named: SceneNode[] = [];
  const cap = exploring && !ex.sel && !ex.query ? (phone.matches ? 6 : 12) : Infinity;
  scene.nodes.filter(n => !important(n) && !scene.has(n, "dim") && !scene.has(n, "off")).sort((a,b) => b.deg-a.deg).forEach(n => {
    const b = box(n);
    if (named.length < cap && fits(b) && clear(n, b)) { boxes.push(b); named.push(n); }
  });
  scene.classes(named, "", "named");
  placePop();
}
scene.onChange = scheduleLabels;
scene.onDrag = node => { cancelLayout(); scheduleLabels(); placePop(); if (!reduced.matches) grab(node); };
scene.onDragEnd = node => { if (physics) { physics.pin(node.id, null); physics.alphaTarget = 0; } };
scene.onHover = (n, over) => {
  scene.classes([n], over ? "" : "hover", over ? "hover" : "");
  if (exploring && !ex.sel && !ex.query) {
    const nbrs = families.get(n.id)!.nbrs;
    scene.classes(scene.edges, "lit quiet");
    if (over) {
      scene.edges.forEach(e => scene.classes([e], "", e.source === n || e.target === n ? "lit" : "quiet"));
      scene.nodes.forEach(o => scene.classes([o], "near", o !== n && nbrs.has(o.id) ? "near" : ""));
    } else scene.classes(scene.nodes, "near");
  }
  if (!state.result && !exploring) {
    if (over) {
      scene.classes(scene.elements.filter(e => e !== n && !families.get(n.id)!.nbrs.has(e.id)), "", "dim");
      scene.classes(scene.edges.filter(e => e.source === n || e.target === n), "dim", "nbr");
    } else scene.classes(scene.elements, "dim nbr");
  }
  scheduleLabels();
};

/* ---------- State ---------- */
let exploring = true;
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
scene.onNodeTap = (node, options) => exploring ? selectProblem(defaults[node.id]!) : openPop(node, options);
scene.onBackgroundTap = () => { closePop(); if (exploring) collapse(); };
document.addEventListener("keydown", (e) => {
  if (e.key === "Escape") { closePop(true); closeDetails(); if (exploring && !document.activeElement!.matches("input")) collapse(); }
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
    if (b.x1 < r.x || b.x2 > r.x + r.w || b.y1 < r.y || b.y2 > r.y + r.h) frame(routeEls(state.selected), 2.4);
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
  if (exploring) { const f = [...model.fam.values()].find(x => x.source === edge.source.id && x.target === edge.target.id); if (f) selectRules(f.source + "→" + f.target); return; }
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
function startWorker(): Worker {
  if (!worker) {
    const engine = worker = new Worker(new URL('../workers/path-worker.ts', import.meta.url), { type: "module" });
    engine.onerror = event => {
      event.preventDefault();
      if (worker === engine) stopWorker();
    };
  }
  return worker;
}
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
    const engine = startWorker();
    let result = cached;
    engine.onmessage = ({ data }: MessageEvent<PathReply>) => {
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
    engine.onerror = (ev) => { ev.preventDefault(); if (worker !== engine) return; stopWorker(); reject(new Error("engine")); };
    performance.mark("graph:request-posted");
    engine.postMessage(request);
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
  if (exploring) return;
  scene.classes(scene.elements, "dim route start end nbr " + classOrder.map(c => "e-" + c).join(" "));
  scene.nodes.forEach(n => { n.routeLabel = display(n.id); });
  if (state.result && state.selected) {
    scene.classes(scene.elements, "", "dim");
    const selected = routeEls(state.selected);
    scene.classes(selected, "dim", "route");
    // Each route edge takes the growth colour of the step it carries, matching the timeline badges.
    state.result!.paths[state.selected - 1]!.path.forEach(step => {
      if (step.from.name !== step.to.name) scene.classes([scene.byId.get(step.from.name + "→" + step.to.name)!], "", "e-" + stepGrowth(step).cls);
    });
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
  arrangeRoute(state.result && state.selected ? familyChain(state.result.paths[state.selected - 1]!) : []);
  scene.nodes.forEach(node => node.shape.setAttribute('aria-label', scene.describe(node)));
  scheduleLabels();
}
function select(n: number) {
  state.selected = n;
  paintGraph(); renderResults();
  const focused = scene.nodeOf(document.activeElement);
  if (focused) scene.reveal(focused);
  else frame(routeEls(n), 2.4);
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
  // Consecutive points of one family are variant conversions: show the family once with each variant it passes through.
  // Every step, conversions included, carries its rule's growth on the line of the point it leaves.
  const points = endpoints(route), groups: {name: string; points: number[]}[] = [];
  points.forEach((e, k) => { if (groups.at(-1)?.name === e.name) groups.at(-1)!.points.push(k); else groups.push({ name: e.name, points: [k] }); });
  const stepLink = (k: number) => {
    const step = route.path[k]!, g = stepGrowth(step), badge = overhead(g.cls, g.accuracy), rule = el("button", "rule-link"); rule.type = "button";
    const what = step.from.name === step.to.name ? `${display(step.from.name)} variant change` : `${display(step.from.name)} to ${display(step.to.name)}`;
    rule.title = `Reduction rule: ${what}. ${badge.title}`;
    rule.onclick = () => showRule(step);
    rule.append(el("span", "sr-only", `Rule ${what}, `), badge);
    return rule;
  };
  const last = points.length - 1;
  groups.forEach((g) => {
    const li = el("li"), btn = el("button"); btn.type = "button";
    btn.append(el("span", "name", display(g.name)));
    btn.addEventListener("click", () => { showProblem(points[g.points[0]!]!); pulseAndFrame(g.name); });
    li.append(el("span", "dot"), btn);
    if (!multi(g.name)) { if (g.points.at(-1)! < last) li.append(stepLink(g.points.at(-1)!)); }
    else g.points.forEach((k) => {
      const line = el("button", "variant-line"); line.type = "button"; line.append(kvNode(points[k]!.variant));
      line.addEventListener("click", () => { showProblem(points[k]!); pulseAndFrame(g.name); });
      li.append(line);
      if (k < last) li.append(stepLink(k));
    });
    ol.append(li);
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
  const onFront = new Set(c?.front);
  const front = c ? shown.filter((n) => onFront.has(n)) : shown, slower = c ? shown.filter((n) => !onFront.has(n)) : [];
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
    const selectedSlower = !onFront.has(state.selected!), open = state.showSlower || selectedSlower;
    const t = el("button", "link more", open ? "Hide slower routes" : `Show ${slower.length} slower routes`); t.type = "button";
    t.addEventListener("click", () => { state.showSlower = !open; if (!state.showSlower && selectedSlower) { select(front[0]!); return; } renderResults(); });
    box.append(t);
    if (open) slower.forEach((n) => box.append(pathRow(n, true)));
  }
}


/* ---------- Explore ---------- */
/* Analysis runs on concrete variants. A family link only groups the exact variant rules behind it. */
const rules = D.edges as RuleEdge[];
// Routes follow solution-mapping rules; a registry has at most one per exact variant pair.
const stepRule = new Map(rules.flatMap((r, i) => r.witness ? [[r.source + ">" + r.target, i] as const] : []));
const stepGrowth = (step: Step) => {
  const r = stepRule.get(indexOf.get(keyOf(step.from)) + ">" + indexOf.get(keyOf(step.to)));
  return r == null ? { cls: "unknown" as ClassId, accuracy: "exact" as const } : edgeGrowth(rules[r]!);
};
type Reach = "direct" | "down" | "up";
type Selection = { kind: "problem"; variant: number; reach: Reach; dir?: "out" | "in" } | { kind: "rules"; key: string; from?: number; index?: number } | null;
/* `open`: the browse lists are showing; `view`: which problems the map lays out. */
type View = "all" | "nohub" | "fromsat";
interface Explore { turing: boolean; limit: ClassId; tab: "problems" | "rules"; query: string; sel: Selection; all: boolean; open: boolean; view: View }
const ex: Explore = { turing: false, limit: "unknown", tab: "problems", query: "", sel: null, all: false, open: false, view: "all" };
const VIEWS: Record<View, { label: string; keep: (name: string) => boolean }> = {
  all: { label: "All rules", keep: () => true },
  nohub: { label: "Without ILP & QUBO", keep: name => !HUBS.has(name) },
  fromsat: { label: "With routes from SAT", keep: name => families.get(name)!.variants.some(i => fromSat.get(i) != null) },
};
const inView = (name: string) => VIEWS[ex.view].keep(name);
/* With nothing to show, the panel folds to its search bar and the graph takes the page. */
const collapsed = () => exploring && !ex.open && !ex.sel && !ex.query.trim();
const classOrder = CLASSES.map(c => c.id);
const CLEAN = "turing dim route start end nbr off reach sel hub lit quiet near " + classOrder.map(c => "e-" + c).join(" ");
const vtext = (i: number) => Object.entries(D.nodes[i]!.variant).map(([k, v]) => k + ": " + v).join("  ") || "default";
const badge = (cls: ClassId) => {
  const c = CLASSES.find(x => x.id === cls)!, b = el("span", "gc g-" + cls, c.label);
  b.title = c.name; return b;
};
const famText = (name: string) => [display(name), name, ...(aliasMap[name] ?? [])].map(t => t.toLowerCase());
const TURING_HINT = "Turing reduction: solves an optimization problem by asking its yes/no version (is there a solution of value at least k?) several times, with binary search on k. Routes skip these because they need many solver calls.";
const matchesQuery = (name: string) => { const q = ex.query.trim().toLowerCase(); return !q || famText(name).some(t => t.includes(q)); };
const pluralize = (n: number, word: string) => `${n} ${word}${n === 1 ? "" : "s"}`;

function buildModel() {
  /* Solution-mapping rules always; Turing rules only when asked for. */
  const inView = (limit: ClassId) => activeRules(rules, { scope: ex.turing ? ["witness", "turing"] : "witness", limit });
  const active = inView(ex.limit), links = active.map(i => rules[i]!);
  const fam = familyRules(D.nodes, rules, active), groups = components(D.nodes.length, links);
  const counts = new Map<ClassId, number>();
  inView("unknown").forEach(i => { const c = edgeGrowth(rules[i]!).cls; counts.set(c, (counts.get(c) ?? 0) + 1); });
  const out = new Map<number, number[]>(), inn = new Map<number, number[]>();
  active.forEach(i => {
    const r = rules[i]!;
    (out.get(r.source) ?? out.set(r.source, []).get(r.source)!).push(i);
    (inn.get(r.target) ?? inn.set(r.target, []).get(r.target)!).push(i);
  });
  const degree = new Map<string, number>();
  fam.forEach(f => { degree.set(f.source, (degree.get(f.source) ?? 0) + 1); degree.set(f.target, (degree.get(f.target) ?? 0) + 1); });
  const core = new Set([...degree].sort((a, b) => b[1] - a[1]).slice(0, 40).map(([n]) => n));
  return { active, links, fam, groups, counts, out, inn, degree, core };
}
let model = buildModel();
const ruleClass = (i: number) => edgeGrowth(rules[i]!).cls;
const endpoint = (i: number) => ({ name: D.nodes[i]!.name, variant: D.nodes[i]!.variant });
const ruleStep = (i: number): Step => ({ from: endpoint(rules[i]!.source), to: endpoint(rules[i]!.target) });
const reachOf = (v: number, dir: "down" | "up") => reach(D.nodes.length, model.links, v, dir);

/* Frame the dense middle; a few far-flung isolated problems should not shrink everything else. */
function frameAll(animate = !reduced.matches) {
  const linked = scene.nodes.filter(n => families.get(n.id)!.nbrs.size > 1 && inView(n.id));
  const cx = linked.reduce((a, n) => a + n.x, 0) / linked.length, cy = linked.reduce((a, n) => a + n.y, 0) / linked.length;
  const near = linked.map(n => [Math.hypot(n.x - cx, n.y - cy), n] as const).sort((a, b) => a[0] - b[0]);
  frame(near.slice(0, Math.ceil(near.length * (phone.matches ? .5 : .85))).map(([, n]) => n), 1.6, animate);
}

/* Canvas state follows the selection; with none, rules are faint and ILP's are fainter still. */
function paintExplore() {
  scene.classes(scene.elements, CLEAN);
  scene.nodes.forEach(n => { n.routeLabel = display(n.id); });
  const sel = ex.sel, q = ex.query.trim();
  let litRules: Set<number> | null = null, litNodes: Set<string> | null = null, selected = "";
  if (sel?.kind === "problem") {
    const v = sel.variant, near = new Set<number>([v]);
    selected = D.nodes[v]!.name;
    if (sel.reach === "direct") {
      litRules = new Set([...(model.out.get(v) ?? []), ...(model.inn.get(v) ?? [])]);
      litRules.forEach(i => { near.add(rules[i]!.source); near.add(rules[i]!.target); });
    } else {
      reachOf(v, sel.reach).forEach(w => near.add(w));
      litRules = new Set(model.active.filter(i => near.has(rules[i]!.source) && near.has(rules[i]!.target)));
    }
    litNodes = new Set([...near].map(i => D.nodes[i]!.name));
  } else if (sel) {
    const f = ruleGroup(sel.key);
    if (f) { litRules = new Set(f.rules); litNodes = new Set([f.source, f.target]); }
  } else if (q) litNodes = new Set([...families.keys()].filter(matchesQuery));
  scene.edges.forEach(edge => {
    const f = model.fam.get(edge.id);
    if (!f || !inView(f.source) || !inView(f.target)) { scene.classes([edge], "", "off"); return; }
    scene.classes([edge], "", "e-" + f.cls + (f.rules.every(i => rules[i]!.turing) ? " turing" : ""));
    if (litRules) scene.classes([edge], "", f.rules.some(i => litRules.has(i)) ? "lit" : "dim");
    else if (litNodes) scene.classes([edge], "", litNodes.has(f.source) && litNodes.has(f.target) ? "lit" : "dim");
    else if (HUBS.has(edge.source.id) || HUBS.has(edge.target.id)) scene.classes([edge], "", "hub");
    else if (!(model.core.has(f.source) && model.core.has(f.target))) scene.classes([edge], "", "quiet");
  });
  scene.nodes.forEach(node => {
    if (!inView(node.id)) scene.classes([node], "", "off");
    else if (litNodes) scene.classes([node], "", litNodes.has(node.id) ? "reach" : "dim");
    else if (!model.degree.has(node.id)) scene.classes([node], "", "dim");
    if (node.id === selected) scene.classes([node], "", "sel");
    node.shape.setAttribute("aria-label", scene.describe(node));
  });
  scheduleLabels();
}

// Force-layout neighbourhoods are compact, so a selection may zoom in further than the whole map.
function focusLit() { frame(scene.nodes.filter(n => scene.has(n, "reach")), 2.4); }
/* The active rules between two problems. Variant rules inside one family have no graph edge, so they are found by name. */
function ruleGroup(key: string) {
  const f = model.fam.get(key);
  if (f) return f;
  const [source, target] = key.split("→"), group = model.active.filter(i => D.nodes[rules[i]!.source]!.name === source && D.nodes[rules[i]!.target]!.name === target);
  return group.length ? { source: source!, target: target!, rules: group } : undefined;
}
function selectProblem(variant: number, reach: Reach = "direct", focus = true) {
  if (!inView(D.nodes[variant]!.name)) setView("all");
  ex.sel = { kind: "problem", variant, reach }; ex.all = false; closeDetails();
  paintExplore(); renderExplore();
  if (focus) focusLit();
}
function selectRules(key: string, from?: number, index?: number) {
  ex.sel = { kind: "rules", key, ...(from == null ? {} : { from }), ...(index == null ? {} : { index }) };
  paintExplore(); renderExplore(); focusLit();
}
/* Leaving a card goes back to the lists it was reached from. */
function clearSelection() { ex.sel = null; ex.open = true; paintExplore(); renderExplore(); }
/* Back to the bare map: no selection, no lists. A typed query stays, since the list is its answer. */
function collapse() {
  if (!ex.sel && !ex.open) return;
  ex.sel = null; ex.open = false; closeDetails(); paintExplore(); renderExplore();
}
function refreshExplore() { model = buildModel(); paintExplore(); renderExplore(); renderLegend(); }

$("#turing-toggle").title = TURING_HINT;
$("#turing-toggle").addEventListener("click", () => { ex.turing = !ex.turing; ex.sel = null; refreshExplore(); });
/* Each view keeps a subset and lays it out with the same force model as the published map, so taking the hubs
   away shows the structure they hide. */
const published = new Map(scene.nodes.map(n => [n.id, {x: n.x, y: n.y}]));
let layout: AbortController | null = null;
function cancelLayout() { layout?.abort(); layout = null; }
function finishLayout(sim: Simulation, apply: (positions: Map<string, Point>) => void) {
  cancelLayout();
  const pending = layout = new AbortController();
  void settleSimulation(sim, pending.signal).then(done => {
    if (!done) return;
    layout = null;
    apply(sim.positions());
  });
}
function setView(view: View) {
  if (view === ex.view) return;
  cancelLayout(); scene.settle();
  const before = new Map(scene.nodes.map(n => [n.id, {x: n.x, y: n.y}]));
  ex.view = view; ex.sel = null; physics = null;
  paintExplore(); renderExplore(); renderLegend();
  const apply = (target: ReadonlyMap<string, Point>) => {
    scene.nodes.forEach(n => Object.assign(n, target.get(n.id) ?? before.get(n.id)));
    frameAll();
    if (reduced.matches) scene.updatePositions(); else scene.morph(n => before.get(n.id)!);
  };
  if (view === "all") apply(published);
  else finishLayout(createSimulation(physicsInput(before)), apply);
}
/* ---------- Live physics ---------- */
/* A dragged problem pulls its neighbours along. The loop stops once the model is at rest. */
let physics: Simulation | null = null, physicsFrame = 0;
const physicsInput = (start: ReadonlyMap<string, Point>) => ({ ids: scene.nodes.filter(n => inView(n.id)).map(n => n.id), hubs: HUBS,
  links: scene.edges.map(e => [e.source.id, e.target.id] as const), radius: (id: string) => nodeSize(families.get(id)!.nbrs.size) / 2, start });
function kick() { if (!physicsFrame) physicsFrame = requestAnimationFrame(stepPhysics); }
function stepPhysics() {
  physicsFrame = 0;
  if (!physics) return;
  physics.tick();
  const at = physics.positions();
  scene.nodes.forEach(n => { const p = at.get(n.id); if (p) { n.x = p.x; n.y = p.y; } });
  scene.updatePositions(); scheduleLabels(); placePop();
  if (settled(physics)) physics = null; else kick();
}
/* ---------- Straightened routes ---------- */
/* The selected route is drawn as an evenly spaced chain from its start toward its end, so its own edges never cross.
   The rest of the map relaxes around the pinned chain from the published layout; a gentle start keeps its shape. */
const ROUTE_GAP = 48;
let arranged = "";
function routeLayout(names: string[]) {
  const a = published.get(names[0]!)!, b = published.get(names.at(-1)!)!;
  const d = Math.hypot(b.x - a.x, b.y - a.y), ux = d ? (b.x - a.x) / d : 1, uy = d ? (b.y - a.y) / d : 0;
  // Discs keep a clear gap between them; a long route keeps its ends where they are and spreads the stops between.
  const radius = (id: string) => nodeSize(families.get(id)!.nbrs.size) / 2, at = [0];
  names.slice(1).forEach((id, i) => at.push(at[i]! + radius(names[i]!) + radius(id) + ROUTE_GAP));
  const len = at.at(-1)!, scale = Math.max(1, d / len), mx = (a.x + b.x) / 2, my = (a.y + b.y) / 2;
  const sim = createSimulation(physicsInput(published));
  names.forEach((id, i) => { const t = (at[i]! - len / 2) * scale; sim.pin(id, { x: mx + ux * t, y: my + uy * t }); });
  sim.alpha = 0.3;
  sim.tick();
  // A label below a stop clears the chain only when it runs nearly level; otherwise it goes beside the stop.
  return { sim, sides: Math.abs(uy) > 0.4 * Math.abs(ux) ? ["right", "left", "below", "above"] as LabelSide[] : SIDES };
}
function arrangeRoute(names: string[]) {
  const key = names.join(">");
  if (key === arranged) return;
  arranged = key; physics = null; cancelLayout();
  const before = new Map(scene.nodes.map(n => [n.id, {x: n.x, y: n.y}]));
  const apply = (positions: ReadonlyMap<string, Point>) => {
    scene.nodes.forEach(n => Object.assign(n, positions.get(n.id) ?? before.get(n.id)));
    if (reduced.matches) scene.updatePositions(); else scene.morph(n => before.get(n.id)!);
    scheduleLabels();
  };
  routeSides = SIDES;
  if (names.length < 2) { apply(published); return; }
  const { sim, sides } = routeLayout(names);
  routeSides = sides;
  // Pin the route immediately so its camera and controls work while the surrounding map settles.
  const pinned = sim.positions();
  names.forEach(id => Object.assign(scene.byId.get(id)!, pinned.get(id)));
  if (reduced.matches) scene.updatePositions();
  finishLayout(sim, apply);
}
/* Dragging starts from the drawing as it is, so only the neighbourhood of the dragged problem reacts. */
function grab(node: SceneNode) {
  if (!physics) { physics = createSimulation(physicsInput(new Map(scene.nodes.map(n => [n.id, {x: n.x, y: n.y}])))); physics.alpha = 0; }
  physics.alphaTarget = 0.3;
  physics.pin(node.id, { x: node.x, y: node.y });
  kick();
}
function renderViews() {
  $("#views").replaceChildren(...(Object.keys(VIEWS) as View[]).map(id => {
    const b = el("button", "", VIEWS[id].label); b.type = "button"; b.setAttribute("aria-pressed", String(ex.view === id));
    b.addEventListener("click", () => setView(id)); return b;
  }));
  const counts = new Map<FamilyStatus, number>();
  familyStatus.forEach((st, name) => { if (!HUBS.has(name)) counts.set(st, (counts.get(st) ?? 0) + 1); });
  const swatch = (color: string, label: string, n: number | null, title: string) => {
    const item = el("span", "status-item"); item.title = title; item.style.setProperty("--c", color);
    item.append(el("i"), label); if (n != null) item.append(el("b", "", String(n))); return item;
  };
  $("#status-legend").replaceChildren(
    ...(Object.keys(STATUS) as FamilyStatus[]).map(st => swatch(STATUS[st].color, STATUS[st].label, counts.get(st) ?? 0, st === "mixed" ? STATUS[st].detail : "Every variant: " + STATUS[st].detail)),
    swatch(HUB_COLOR, "Common targets", null, "ILP and QUBO; inspect each variant for its routes"));
}
function renderLegend() {
  renderViews();
  const box = $("#growth-legend"), at = classOrder.indexOf(ex.limit);
  const turing = $("#turing-toggle");
  turing.classList.toggle("on", ex.turing); turing.setAttribute("aria-checked", String(ex.turing));
  box.replaceChildren(...CLASSES.flatMap((c, i) => {
    const n = model.counts.get(c.id) ?? 0;
    if (!n) return [];
    const b = el("button", "legend-chip g-" + c.id + (i <= at ? " on" : "")); b.type = "button"; b.setAttribute("role", "radio");
    b.setAttribute("aria-checked", String(c.id === ex.limit && ex.limit !== "unknown"));
    b.title = `${c.name}: ${n} reductions. Show those up to this growth.`;
    b.append(el("b", "", c.label), el("span", "", String(n)));
    b.addEventListener("click", () => { ex.limit = ex.limit === c.id ? "unknown" : c.id; ex.sel = null; refreshExplore(); });
    return [b];
  }));
}

/* ---------- Explore panel ---------- */
/* Find: search plus two browsable lists. Explore: one place at a time. */
function renderExplore(enter = true) {
  const body = $("#explore-body"), tabs = $("#tabs"), keep = body.scrollTop;
  body.replaceChildren();
  $(".panel").classList.toggle("collapsed", collapsed());
  tabs.hidden = !!ex.sel;
  if (collapsed()) { scheduleLabels(); return; }
  if (ex.sel?.kind === "problem") placeCard(body, ex.sel);
  else if (ex.sel) ruleCard(body, ex.sel);
  else { renderTabs(tabs); list(body); }
  body.scrollTop = enter ? 0 : keep;
  if (enter && !reduced.matches) { body.classList.remove("enter"); void body.offsetWidth; body.classList.add("enter"); }
}
const problemMatches = () => {
  const q = ex.query.trim().toLowerCase();
  const score = (n: string) => !q ? 0 : famText(n).some(t => t === q) ? 0 : famText(n).some(t => t.startsWith(q)) ? 1 : 2;
  return [...families.keys()].filter(matchesQuery).sort((a, b) => score(a) - score(b) || display(a).localeCompare(display(b)));
};
const ruleMatches = () => [...model.fam.values()].filter(f => matchesQuery(f.source) || matchesQuery(f.target))
  .sort((a, b) => display(a.source).localeCompare(display(b.source)) || display(a.target).localeCompare(display(b.target)));
const dir = (sel: Extract<Selection, { kind: "problem" }>) => sel.dir ?? ((model.out.get(sel.variant) ?? []).length || !(model.inn.get(sel.variant) ?? []).length ? "out" : "in");
const letter = (name: string) => display(name)[0]!.toUpperCase();
/* Only what differs from the family's default variant; the default itself needs no label. */
function variantTokens(i: number, all = false) {
  const n = D.nodes[i]!, base = D.nodes[defaults[n.name]!]!.variant, box = el("span", "vars");
  Object.entries(n.variant).filter(([k, v]) => all || base[k] !== v).forEach(([k, v]) => { const t = el("span", "var"); t.append(el("i", "", k), v); box.append(t); });
  return box;
}
function overhead(cls: ClassId, accuracy: "exact" | "upper_bound" = "exact") {
  const c = CLASSES.find(x => x.id === cls)!, b = el("span", "oh g-" + cls, (accuracy === "upper_bound" && cls !== "unknown" ? "≤ " : "") + c.label);
  b.title = `${c.name} growth${cls === "unknown" ? "" : accuracy === "exact" ? ", exact" : ", upper bound"}`;
  return b;
}
const ruleOverhead = (i: number) => { const g = edgeGrowth(rules[i]!); return overhead(g.cls, g.accuracy); };
function topbar(body: HTMLElement, text: string, action: () => void) {
  const bar = el("div", "topbar"), b = el("button", "crumb"); b.type = "button";
  b.innerHTML = '<svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M9 2.5 4.5 7 9 11.5"/></svg>';
  b.append(text); b.addEventListener("click", action); bar.append(b); body.append(bar);
}

/* Lists: one row per thing, grouped by initial so a long list has landmarks. */
function problemRow(name: string) {
  const row = el("button", "row"); row.type = "button"; row.dataset.name = name;
  row.append(el("span", "row-name", display(name)));
  const alias = (aliasMap[name] ?? [])[0]; if (alias) row.append(el("small", "row-alias", alias));
  row.addEventListener("click", () => selectProblem(defaults[name]!));
  return row;
}
function ruleRow(f: { source: string; target: string; cls: ClassId; rules: number[] }) {
  const row = el("button", "row rule"); row.type = "button";
  const text = el("span", "row-stack"); text.append(el("span", "row-name", display(f.source)), el("span", "row-sub", "→ " + display(f.target)));
  row.append(text, overhead(f.cls));
  row.addEventListener("click", () => selectRules(f.source + "→" + f.target));
  return row;
}
function renderTabs(box: HTMLElement) {
  const n = { problems: problemMatches().length, rules: ruleMatches().length };
  box.replaceChildren(...(["problems", "rules"] as const).map(id => {
    const t = el("button", "tab"); t.type = "button"; t.setAttribute("role", "tab"); t.setAttribute("aria-selected", String(ex.tab === id));
    t.append(id === "problems" ? "Problems" : "Reductions", el("span", "", String(n[id])));
    t.addEventListener("click", () => { ex.tab = id; renderExplore(); }); return t;
  }));
}
function list(body: HTMLElement) {
  const grouped = !ex.query.trim();
  const items = ex.tab === "problems" ? problemMatches().map(n => [letter(n), problemRow(n)] as const) : ruleMatches().map(f => [letter(f.source), ruleRow(f)] as const);
  if (!items.length) { body.append(el("p", "empty", "Nothing matches")); return; }
  let last = "";
  items.forEach(([l, row]) => { if (grouped && l !== last) { body.append(el("div", "letter", l)); last = l; } body.append(row); });
}

/* A problem: identity, then variant, then its reductions. */
function reductionRow(i: number, side: "source" | "target", sel: Extract<Selection, { kind: "problem" }>) {
  const r = rules[i]!, other = r[side], row = el("button", "red"); row.type = "button";
  const main = el("span", "red-main"), top = el("span", "red-top");
  top.append(el("span", "red-dir", side === "target" ? "→" : "←"), el("span", "red-name", display(D.nodes[other]!.name)));
  main.append(top); const tokens = variantTokens(other); if (tokens.childElementCount) main.append(tokens);
  if (r.turing) { const k = el("span", "red-kind", "Turing"); k.title = TURING_HINT; main.append(k); }
  row.append(main, ruleOverhead(i));
  row.addEventListener("click", () => selectRules(D.nodes[r.source]!.name + "→" + D.nodes[r.target]!.name, sel.variant, i));
  return row;
}
function reductions(body: HTMLElement, sel: Extract<Selection, { kind: "problem" }>) {
  const v = sel.variant, outs = model.out.get(v) ?? [], ins = model.inn.get(v) ?? [];
  if (!outs.length && !ins.length) { body.append(el("p", "empty", "No reductions at this growth limit")); return; }
  const which = dir(sel), indexes = which === "out" ? outs : ins, side = which === "out" ? "target" : "source", reachId = which === "out" ? "down" : "up";
  const tabs = el("div", "tabs inline"); tabs.setAttribute("role", "tablist"); tabs.setAttribute("aria-label", "Reductions");
  ([["out", "Reduces to", outs.length], ["in", "Reduced from", ins.length]] as const).forEach(([id, text, n]) => {
    const t = el("button", "tab"); t.type = "button"; t.setAttribute("role", "tab"); t.setAttribute("aria-selected", String(which === id)); t.disabled = !n;
    t.append(text, el("span", "", String(n)));
    t.addEventListener("click", () => { sel.dir = id; sel.reach = "direct"; ex.all = false; paintExplore(); renderExplore(false); }); tabs.append(t);
  });
  body.append(tabs);
  const on = sel.reach === reachId, bar = el("div", "trace");
  const toggle = el("button", "link", on ? "Stop tracing" : `Trace everything ${reachId === "down" ? "downstream" : "upstream"}`); toggle.type = "button";
  toggle.addEventListener("click", () => { sel.reach = on ? "direct" : reachId; paintExplore(); renderExplore(false); focusLit(); });
  bar.append(on ? el("span", "", pluralize(reachOf(v, reachId).size, "variant") + " on the graph") : el("span", "", "On the graph"), toggle);
  body.append(bar);
  const rows = el("div", "reds");
  (ex.all ? indexes : indexes.slice(0, 8)).forEach(i => rows.append(reductionRow(i, side, sel)));
  body.append(rows);
  if (indexes.length > 8) {
    const more = el("button", "link more", ex.all ? "Show fewer" : `Show all ${indexes.length}`); more.type = "button";
    more.addEventListener("click", () => { ex.all = !ex.all; renderExplore(false); }); body.append(more);
  }
}
function placeCard(body: HTMLElement, sel: Extract<Selection, { kind: "problem" }>) {
  const v = sel.variant, name = D.nodes[v]!.name;
  topbar(body, ex.tab === "problems" ? "Problems" : "Reductions", clearSelection);
  const head = el("header", "identity");
  head.append(el("h1", "place-name", display(name)));
  const depth = fromSat.get(v), meta = el("p", "place-meta");
  const alias = (aliasMap[name] ?? [])[0]; if (alias) meta.append(el("code", "", alias));
  const st = variantStatus.get(v)!, badge = el("span", "status", STATUS[st].label);
  badge.style.setProperty("--c", STATUS[st].color); badge.title = STATUS[st].detail;
  meta.prepend(badge);
  if (depth != null) meta.append(depth === 0 ? "SAT" : `${pluralize(depth, "step")} from SAT`);
  head.append(meta);
  const acts = el("div", "acts");
  const go = el("button", "act primary", "Directions"); go.type = "button";
  go.addEventListener("click", () => { state.source = v; if (state.target === v) state.target = null; setMode("directions"); if (state.target != null) run(); });
  const more = el("button", "act", "Article"); more.type = "button"; more.addEventListener("click", () => showProblem(D.nodes[v]!));
  acts.append(go, more); head.append(acts); body.append(head);
  if (multi(name)) {
    const sec = el("section", "sec"); sec.append(el("h2", "eyebrow", "Variant"));
    const opts = el("div", "options"); variantOptions(opts, v, j => selectProblem(j, sel.reach, false)); sec.append(opts); body.append(sec);
  }
  const sec = el("section", "sec"); reductions(sec, sel); body.append(sec);
  const group = model.groups.groups[model.groups.comp[v]!]!;
  if (group.length > 1) {
    const wrap = el("details", "fold"), sum = el("summary"); sum.append(el("span", "", "Mutually reducible"), el("small", "", String(group.length - 1)));
    wrap.append(sum);
    const chips = el("div", "chips");
    group.filter(w => w !== v).forEach(w => { const c = el("button", "chip", display(D.nodes[w]!.name) + (multi(D.nodes[w]!.name) ? " · " + vtext(w) : "")); c.type = "button"; c.addEventListener("click", () => selectProblem(w)); chips.append(c); });
    wrap.append(chips); body.append(wrap);
  }
}

/* One reduction: its two ends, how large the result grows, and every variant it covers. */
const sup = (d: string) => [...d].map(c => "⁰¹²³⁴⁵⁶⁷⁸⁹"[Number(c)]).join("");
/* Certificates are sums of `coef * var^k` terms; show them highest degree first, with real minus signs and no unit coefficients. */
function formula(text: string | null) {
  if (text === null) return "unavailable";
  const terms = text.split(" + ").map(term => {
    const factors = term.split(" * "), lead = /^-?[\d./]+$/.test(factors[0]!) ? factors.shift()! : "1";
    const negative = lead.startsWith("-"), coef = negative ? lead.slice(1) : lead;
    const degree = factors.reduce((sum, f) => sum + Number(f.split("^")[1] ?? 1), 0);
    const vars = factors.map(f => f.replace(/^num_/, "").replace(/\^(\d+)$/, (_, d: string) => sup(d))).join("·");
    return { negative, degree, body: !vars ? coef : coef === "1" ? vars : coef + "·" + vars };
  }).sort((a, b) => b.degree - a.degree);
  return terms.map((t, i) => (i ? (t.negative ? " − " : " + ") : t.negative ? "−" : "") + t.body).join("");
}
function ruleCard(body: HTMLElement, sel: Extract<Selection, { kind: "rules" }>) {
  const f = ruleGroup(sel.key);
  const origin = sel.from != null ? display(D.nodes[sel.from]!.name) : ex.tab === "problems" ? "Problems" : "Reductions";
  topbar(body, origin, () => { if (sel.from != null) selectProblem(sel.from, "direct", false); else clearSelection(); });
  if (!f) return;
  const siblings = [...f.rules].sort((a, b) => classOrder.indexOf(ruleClass(a)) - classOrder.indexOf(ruleClass(b)));
  const i = sel.index != null && f.rules.includes(sel.index) ? sel.index : siblings[0]!, rule = rules[i]!, g = edgeGrowth(rule);
  body.append(el("p", "eyebrow lead", "Reduction"));
  const flow = el("div", "flow");
  for (const [side, role] of [["source", "from"], ["target", "to"]] as const) {
    const node = rule[side], end = el("button", "end-node " + role); end.type = "button";
    end.addEventListener("click", () => selectProblem(node, "direct", true));
    const text = el("span", "end-text"); text.append(el("span", "end-name", display(D.nodes[node]!.name)));
    const tokens = variantTokens(node, true); if (tokens.childElementCount) text.append(tokens);
    end.append(el("span", "end-dot"), text); flow.append(end);
    if (role === "from") { const link = el("div", "flow-link"); link.append(overhead(g.cls, g.accuracy), el("span", "", g.cls === "unknown" ? "Growth undetermined" : CLASSES.find(c => c.id === g.cls)!.name + (g.accuracy === "exact" ? ", exact" : ", upper bound"))); flow.append(link); }
  }
  body.append(flow);
  const sec = el("section", "sec"); sec.append(el("h2", "eyebrow", "Size of the result"));
  const dl = el("dl", "fields");
  g.fields.forEach(x => {
    const row = el("div", "field"), field = rule.fields[x.name]!;
    const exact = field.accuracy.kind === "exact", mark = field.accuracy.kind === "unavailable" ? "" : exact ? "=" : "≤";
    const val = el("dd"), text = el("span", "formula");
    text.append(el("span", "mark", mark), formula(field.certificate));
    val.append(text, x.degree === null ? el("span", "oh g-unknown", "?") : overhead(growthClass(x.degree), exact ? "exact" : "upper_bound"));
    row.append(el("dt", "", prettyField(x.name)), val);
    dl.append(row);
  });
  sec.append(dl);
  sec.append(el("p", "foot", "Growth treats every source parameter as scaling together."));
  body.append(sec);
  const acts = el("div", "acts"), art = el("button", "act primary", "Rule article"); art.type = "button"; art.addEventListener("click", () => showRule(ruleStep(i)));
  acts.append(art); body.append(acts);
  if (siblings.length > 1) {
    const vs = el("section", "sec"); vs.append(el("h2", "eyebrow", `Covers ${siblings.length} variants`));
    const rows = el("div", "reds");
    siblings.forEach(j => {
      const r = rules[j]!, row = el("button", "red"); row.type = "button"; row.setAttribute("aria-current", String(j === i));
      const main = el("span", "red-main"); main.append(variantPair(r.source, r.target)); row.append(main, ruleOverhead(j));
      row.addEventListener("click", () => { ex.sel = { ...sel, index: j }; paintExplore(); renderExplore(false); }); rows.append(row);
    });
    vs.append(rows); body.append(vs);
  }
}
function variantPair(a: number, b: number) {
  const box = el("span", "pair");
  [[a, "from"], [b, "to"]].forEach(([i, role]) => {
    const line = el("span", "pair-line " + role), tokens = variantTokens(i as number, true);
    line.append(el("span", "pair-dot"), tokens.childElementCount ? tokens : el("span", "vars", "default")); box.append(line);
  });
  return box;
}
$("#find").addEventListener("focus", () => { if (!ex.open && !ex.sel) { ex.open = true; renderExplore(); } });
$("#find").addEventListener("input", (e) => {
  ex.query = (e.target as HTMLInputElement).value; ex.sel = null; paintExplore(); renderExplore();
});
$("#find").addEventListener("keydown", (e) => {
  if (e.key === "Enter") $<HTMLButtonElement>("#explore-body .row")?.click();
  if (e.key !== "Escape") return;
  if (ex.query) { ex.query = ""; ($("#find") as HTMLInputElement).value = ""; paintExplore(); renderExplore(); }
  else { ($("#find") as HTMLInputElement).blur(); collapse(); }
});

/* ---------- Modes: explore the graph, or ask for directions ---------- */
function setMode(mode: "explore" | "directions") {
  exploring = mode === "explore";
  $(".panel").classList.toggle("collapsed", collapsed());
  $("#view-explore").hidden = !exploring; $("#view-directions").hidden = exploring;
  document.body.classList.toggle("explore", exploring);
  closePop();
  if (exploring) {
    arrangeRoute([]);
    refreshExplore(); frameAll();
    history.replaceState(null, "", location.pathname + location.search);
  } else {
    try { startWorker(); } catch { /* Search reports startup failures when requested. */ }
    setView("all");
    scene.classes(scene.elements, CLEAN);
    syncEnds(); renderResults(); paintGraph();
    history.replaceState(null, "", "#directions");
    if (state.source == null) $("#source").focus({ preventScroll: true }); else if (state.target == null) $("#target").focus({ preventScroll: true });
  }
}
$("#go-directions").addEventListener("click", () => setMode("directions"));
$("#back-explore").addEventListener("click", () => setMode("explore"));

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
if (location.hash === "#directions") {
  exploring = false; $("#view-explore").hidden = true; $("#view-directions").hidden = false; document.body.classList.remove("explore");
  state.source = defaults.Satisfiability!; state.target = defaults.QUBO!;
  syncEnds(); frame(scene.elements, 1, false); run();
} else {
  document.body.classList.add("explore");
  // The camera starts framed; the graph itself unfolds into place.
  refreshExplore(); frameAll(false);
  if (!reduced.matches) scene.unfold();
}
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
  const error = document.querySelector<HTMLElement>('#graph-error')!;
  error.textContent = 'Cannot load graph. Reload this release and try again.';
  error.hidden = false;
});

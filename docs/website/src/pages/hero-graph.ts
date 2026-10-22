/* Animated home-page overview: a small force simulation over problem families, drawn into the
   existing hero SVG classes so the site's palette and node styles stay authoritative. The
   integration follows d3-force (alpha cooling, velocity decay, degree-biased link springs);
   at about two hundred nodes the exact pairwise repulsion is cheaper than a quadtree. */

import { make } from './svg.ts';

export interface HeroNode {
  id: string;
  title: string;
  href: string;
  label?: string;
  // A fixed point in the 630 × 430 view that holds a solver hub in place.
  anchor?: [number, number];
}
export interface HeroGraph {
  nodes: HeroNode[];
  links: [string, string][];
  featured: [string, string];
}

interface SimNode extends HeroNode {
  x: number; y: number; vx: number; vy: number;
  focus: boolean; degree: number; r: number;
  el: SVGGElement; dot: SVGCircleElement; text: SVGTextElement | null;
}
interface SimLink { s: SimNode; t: SimNode; hub: boolean; length: number; el: SVGLineElement }

const WIDTH = 630, HEIGHT = 430, MARGIN = 12;

export function mountHeroGraph(container: HTMLElement, graph: HeroGraph) {
  const reduce = matchMedia("(prefers-reduced-motion: reduce)").matches;
  const svg = make("svg", { viewBox: `0 0 ${WIDTH} ${HEIGHT}`, class: "hero-force",
    "aria-label": "Problem-level overview; connections can involve different variants" });
  const edgeLayer = make("g", { "aria-hidden": "true" }), nodeLayer = make("g");
  svg.append(edgeLayer, nodeLayer);

  const adjacent = new Map(graph.nodes.map(n => [n.id, new Set([n.id])]));
  for (const [a, b] of graph.links) { adjacent.get(a)!.add(b); adjacent.get(b)!.add(a); }
  // Labelled problems draw last so their circles and labels sit above the crowd.
  const ordered = [...graph.nodes].sort((a, b) => Number(!!a.label) - Number(!!b.label));
  const nodes: SimNode[] = ordered.map(node => {
    const focus = graph.featured.includes(node.id), r = focus ? 8 : node.label ? 5.5 : 2.8;
    const el = make("g", { class: "hero-node" });
    let dot: SVGCircleElement, text: SVGTextElement | null = null;
    if (node.label) {
      const link = make("a", { class: `network-node major${focus ? " focus" : ""}${node.anchor ? " hub" : ""}`,
        href: node.href, "aria-label": `Explore ${node.title}` });
      const title = make("title"); title.textContent = node.title;
      dot = make("circle", { r });
      text = make("text", { "text-anchor": "middle" }); text.textContent = node.label;
      link.append(title, dot, text); el.append(link);
    } else {
      dot = make("circle", { r, fill: "var(--graph-node)", stroke: "var(--paper)", "stroke-width": 1, "aria-hidden": "true" });
      el.append(dot);
    }
    nodeLayer.append(el);
    // Start in a tight cloud at the centre; the simulation unfolds it into place.
    const angle = Math.random() * Math.PI * 2, spread = Math.random() * 40;
    return { ...node, x: WIDTH / 2 + Math.cos(angle) * spread, y: HEIGHT / 2 + Math.sin(angle) * spread,
      vx: 0, vy: 0, focus, degree: adjacent.get(node.id)!.size - 1, r, el, dot, text };
  });
  const byId = new Map(nodes.map(n => [n.id, n]));
  const links: SimLink[] = graph.links.map(([a, b]) => {
    const s = byId.get(a)!, t = byId.get(b)!, hub = !!(s.anchor || t.anchor), featured = s.focus && t.focus;
    const el = make("line", { class: `network-edge${featured ? " featured-edge" : ""}` });
    edgeLayer.append(el);
    return { s, t, hub, length: hub ? 80 : featured ? 64 : 24, el };
  });

  // The dragged node is pinned under the pointer while the rest of the layout reacts.
  let dragged: SimNode | null = null, pin: { x: number; y: number } | null = null, moved = false;
  let alpha = 1, alphaTarget = 0;
  const alphaMin = 0.002, alphaDecay = 1 - Math.pow(alphaMin, 1 / 320), velocityDecay = 0.42;
  function tick() {
    alpha += (alphaTarget - alpha) * alphaDecay;
    for (const { s, t, length, hub } of links) {
      let dx = t.x + t.vx - s.x - s.vx, dy = t.y + t.vy - s.y - s.vy;
      const len = Math.hypot(dx, dy) || 1e-6;
      const k = ((len - length) / len) * alpha * (hub ? 0.05 : 0.5);
      dx *= k; dy *= k;
      const bias = s.degree / (s.degree + t.degree);
      t.vx -= dx * bias; t.vy -= dy * bias; s.vx += dx * (1 - bias); s.vy += dy * (1 - bias);
    }
    for (let i = 0; i < nodes.length; i++) {
      const a = nodes[i]!;
      for (let j = i + 1; j < nodes.length; j++) {
        const b = nodes[j]!;
        const dx = b.x - a.x, dy = b.y - a.y, d2 = Math.max(dx * dx + dy * dy, 1);
        // Labelled problems repel each other harder so their names do not pile up.
        const push = (a.label && b.label ? -700 : -18) * alpha / d2;
        a.vx += dx * push; a.vy += dy * push; b.vx -= dx * push; b.vy -= dy * push;
        const min = a.r + b.r + 2;
        if (d2 < min * min) {
          const d = Math.sqrt(d2), shift = ((min - d) / d) * 0.35;
          a.vx -= dx * shift; a.vy -= dy * shift; b.vx += dx * shift; b.vy += dy * shift;
        }
      }
    }
    for (const n of nodes) {
      const [cx, cy] = n.anchor ?? [WIDTH / 2, HEIGHT / 2];
      const pull = n.anchor ? 0.3 : n.degree ? 0.035 : 0.07;
      n.vx += (cx - n.x) * pull * alpha;
      n.vy += (cy - n.y) * pull * 1.4 * alpha;
      if (n === dragged && pin) { n.x = pin.x; n.y = pin.y; n.vx = n.vy = 0; continue; }
      n.x = Math.min(WIDTH - MARGIN, Math.max(MARGIN, n.x + (n.vx *= 1 - velocityDecay)));
      n.y = Math.min(HEIGHT - MARGIN, Math.max(MARGIN, n.y + (n.vy *= 1 - velocityDecay)));
    }
  }
  function draw() {
    for (const { s, t, el } of links) {
      el.setAttribute("x1", s.x.toFixed(1)); el.setAttribute("y1", s.y.toFixed(1));
      el.setAttribute("x2", t.x.toFixed(1)); el.setAttribute("y2", t.y.toFixed(1));
    }
    for (const n of nodes) {
      n.dot.setAttribute("cx", n.x.toFixed(1)); n.dot.setAttribute("cy", n.y.toFixed(1));
      if (n.text) { n.text.setAttribute("x", n.x.toFixed(1)); n.text.setAttribute("y", (n.y + (n.focus ? 28 : 21)).toFixed(1)); }
    }
  }

  // Animate only while the hero is attached and on screen; requestAnimationFrame already sleeps in hidden tabs.
  let frame = 0, visible = true;
  const observer = new IntersectionObserver(([entry]) => {
    visible = !!entry?.isIntersecting;
    if (visible) restart(); else { cancelAnimationFrame(frame); frame = 0; }
  });
  function loop() {
    frame = 0;
    if (!svg.isConnected) { observer.disconnect(); return; }
    tick(); draw();
    if (alpha > alphaMin || alphaTarget > 0) restart();
  }
  function restart() { if (!frame && !reduce && visible) frame = requestAnimationFrame(loop); }

  // Hover lights a problem's direct reductions and names it; the rest of the network recedes.
  // Classes change only when the hovered problem changes, not on every pointer move.
  const hoverLabel = make("text", { class: "hero-force-label", "text-anchor": "middle" });
  svg.append(hoverLabel);
  let focused: SimNode | null = null;
  function focus(node: SimNode | null) {
    if (node !== focused) {
      focused = node;
      svg.classList.toggle("is-focused", !!node);
      const near = node ? adjacent.get(node.id)! : null;
      for (const n of nodes) n.el.classList.toggle("is-near", !!near?.has(n.id));
      for (const l of links) l.el.classList.toggle("is-near", !!node && (l.s === node || l.t === node));
      hoverLabel.textContent = node && !node.label ? node.title : "";
    }
    if (node && !node.label) { hoverLabel.setAttribute("x", node.x.toFixed(1)); hoverLabel.setAttribute("y", (node.y - 10).toFixed(1)); }
  }
  const point = (event: PointerEvent) => {
    const m = svg.getScreenCTM();
    if (!m) return { x: 0, y: 0 };
    const p = new DOMPoint(event.clientX, event.clientY).matrixTransform(m.inverse());
    return { x: p.x, y: p.y };
  };
  const nearest = (event: PointerEvent) => {
    const { x, y } = point(event);
    let best: SimNode | null = null, bestD = 14 * 14;
    for (const n of nodes) { const d = (n.x - x) ** 2 + (n.y - y) ** 2; if (d < Math.max(bestD, n.r * n.r)) { bestD = d; best = n; } }
    return best;
  };
  svg.addEventListener("pointermove", event => {
    if (dragged) {
      pin = point(event); moved = true;
      if (reduce) { dragged.x = pin.x; dragged.y = pin.y; draw(); }
      focus(dragged);
      return;
    }
    const node = nearest(event);
    svg.style.cursor = node ? (node.label ? "pointer" : "grab") : "";
    focus(node);
  });
  svg.addEventListener("pointerleave", () => { if (!dragged) focus(null); });
  svg.addEventListener("pointerdown", event => {
    const node = nearest(event);
    if (!node || event.button !== 0) return;
    dragged = node; moved = false;
    pin = { x: node.x, y: node.y };
    svg.setPointerCapture(event.pointerId);
    alphaTarget = 0.25; restart();
  });
  const release = () => {
    if (!dragged) return;
    dragged = null; pin = null; alphaTarget = 0;
  };
  svg.addEventListener("pointerup", release);
  svg.addEventListener("pointercancel", release);
  // A drag must not follow the labelled node's link.
  svg.addEventListener("click", event => { if (moved) { event.preventDefault(); moved = false; } }, true);

  container.replaceChildren(svg);
  observer.observe(svg);
  if (!reduce) { draw(); restart(); return; }
  // Without motion the layout settles off-screen in short slices, so loading never blocks input.
  svg.style.visibility = "hidden";
  const settle = () => {
    for (let i = 0; i < 24 && alpha > alphaMin; i++) tick();
    if (alpha > alphaMin && svg.isConnected) { setTimeout(settle); return; }
    alpha = 0; draw(); svg.style.visibility = "";
  };
  settle();
}

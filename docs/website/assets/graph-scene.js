// The scene is small and already laid out at build time; SVG owns hit testing and painting.
window.GraphScene = class {
  constructor(container, nodes, edges) {
    const make = (tag, attrs = {}) => {
      const node = document.createElementNS('http://www.w3.org/2000/svg', tag);
      for (const [key, value] of Object.entries(attrs)) node.setAttribute(key, value);
      return node;
    };
    this.zoom = 1; this.pan = {x: 0, y: 0}; this.animation = 0;
    this.svg = make('svg', {class: 'graph-scene', role: 'group',
      'aria-label': 'Problem families and directed reductions. Arrows move between spatial neighbours. Page Up and Page Down visit every problem. Home and End jump to the first and last problem. Enter or Space opens controls.'});
    this.world = make('g');
    const defs = make('defs'), marker = make('marker', {id: 'route-arrow', viewBox: '0 -4 8 8', refX: 7, refY: 0,
      markerWidth: 6, markerHeight: 6, orient: 'auto', markerUnits: 'userSpaceOnUse'});
    marker.append(make('path', {d: 'M0,-3 L7,0 L0,3 Z', fill: 'var(--accent)'}));
    defs.append(marker); this.svg.append(defs, this.world); container.append(this.svg);
    this.nodes = nodes.map(data => {
      const shape = make('g', {class: 'scene-node', 'data-node': data.id, role: 'button', tabindex: -1,
        'aria-haspopup': 'dialog', 'aria-label': data.label});
      const halo = make('circle', {class: 'node-halo', r: data.size / 2 + 6});
      const circle = make('circle', {class: 'node-disc', r: data.size / 2, fill: data.color});
      const label = make('text', {class: 'node-label', 'text-anchor': 'middle'});
      shape.append(halo, circle, label);
      return {...data, shape, label, labelText: data.label, routeLabel: data.label, labelKey: null, width: 0, height: 0};
    });
    this.byId = new Map(this.nodes.map(n => [n.id, n]));
    this.edges = edges.map(({s, t}) => {
      const edge = {id: s + '→' + t, source: this.byId.get(s), target: this.byId.get(t),
        shape: make('line', {class: 'scene-edge', 'data-edge': s + '→' + t, 'aria-hidden': 'true'})};
      this.byId.set(edge.id, edge); this.world.append(edge.shape); return edge;
    });
    for (const node of this.nodes) this.world.append(node.shape);
    this.elements = [...this.nodes, ...this.edges];
    this.measure = document.createElement('canvas').getContext('2d');
    this.updatePositions();

    const point = e => { const r = this.svg.getBoundingClientRect(); return {x: e.clientX - r.x, y: e.clientY - r.y}; };
    const pointers = new Map();
    let gesture;
    this.svg.addEventListener('pointerdown', e => {
      if (e.button !== 0) return;
      this.stop();
      const p = point(e); pointers.set(e.pointerId, p);
      const target = e.target.closest('[data-node], [data-edge]');
      gesture = {item: target && this.byId.get(target.dataset.node || target.dataset.edge),
        moved: pointers.size > 1, start: p};
      this.svg.setPointerCapture(e.pointerId);
    });
    this.svg.addEventListener('pointermove', e => {
      if (!pointers.has(e.pointerId)) return;
      const before = [...pointers.values()], old = pointers.get(e.pointerId), p = point(e);
      pointers.set(e.pointerId, p);
      const dx = p.x - old.x, dy = p.y - old.y;
      if (pointers.size > 1) {
        const after = [...pointers.values()];
        const center = a => ({x: (a[0].x + a[1].x) / 2, y: (a[0].y + a[1].y) / 2});
        const distance = a => Math.hypot(a[0].x - a[1].x, a[0].y - a[1].y);
        const a = center(before), b = center(after), previous = distance(before);
        if (previous) this.zoomAt(this.zoom * distance(after) / previous, a);
        this.setViewport(this.zoom, {x: this.pan.x + b.x - a.x, y: this.pan.y + b.y - a.y});
        gesture.item = null; gesture.moved = true;
      } else if (Math.hypot(p.x - gesture.start.x, p.y - gesture.start.y) > 3 || gesture.moved) {
        gesture.moved = true;
        if (gesture.item?.label) {
          this.moveNode(gesture.item, dx, dy);
        } else this.setViewport(this.zoom, {x: this.pan.x + dx, y: this.pan.y + dy});
      }
    });
    const release = e => {
      if (!pointers.has(e.pointerId)) return;
      pointers.delete(e.pointerId);
      if (e.type === 'pointerup' && !gesture.moved) {
        if (gesture.item?.label) this.onNodeTap?.(gesture.item);
        else if (gesture.item) this.onEdgeTap?.(gesture.item);
        else this.onBackgroundTap?.();
      }
      if (pointers.size) gesture.moved = true;
    };
    this.svg.addEventListener('pointerup', release);
    this.svg.addEventListener('pointercancel', release);
    // Roving tabindex: one Tab stop for the whole graph, arrows move between spatial neighbours.
    this.tabStop = this.nodes[0];
    this.tabStop.shape.setAttribute('tabindex', 0);
    this.svg.addEventListener('focusin', e => {
      const node = this.nodeOf(e.target);
      if (!node) return;
      this.stop();
      this.setTabStop(node);
      node.shape.classList.add('focused');
      node.shape.setAttribute('aria-label', this.describe(node));
      this.reveal(node);
      this.onChange?.();
    });
    this.svg.addEventListener('focusout', e => {
      this.nodeOf(e.target)?.shape.classList.remove('focused');
      this.onChange?.();
    });
    this.svg.addEventListener('click', e => {
      const node = this.nodeOf(e.target);
      if (node && e.detail === 0) this.onNodeTap?.(node, {keyboard: true});
    });
    this.svg.addEventListener('keydown', e => {
      const node = this.nodeOf(e.target);
      if (!node || e.altKey || e.ctrlKey || e.metaKey) return;
      if (e.key === ' ') { e.preventDefault(); return; }
      if (e.key === 'Enter') { e.preventDefault(); this.onNodeTap?.(node, {keyboard: true}); return; }
      const index = this.nodes.indexOf(node);
      const next = {Home: 0, End: this.nodes.length - 1,
        PageDown: (index + 1) % this.nodes.length,
        PageUp: (index + this.nodes.length - 1) % this.nodes.length}[e.key];
      if (next != null) { e.preventDefault(); this.nodes[next].shape.focus(); return; }
      const direction = {ArrowRight: [1, 0], ArrowLeft: [-1, 0], ArrowDown: [0, 1], ArrowUp: [0, -1]}[e.key];
      if (!direction) return;
      e.preventDefault();
      this.neighbour(node, direction)?.shape.focus();
    });
    this.svg.addEventListener('keyup', e => {
      const node = this.nodeOf(e.target);
      if (node && e.key === ' ' && !e.altKey && !e.ctrlKey && !e.metaKey) {
        e.preventDefault(); this.onNodeTap?.(node, {keyboard: true});
      }
    });
    this.svg.addEventListener('wheel', e => {
      e.preventDefault(); this.stop(); this.zoomAt(this.zoom * Math.exp(-e.deltaY * .002), point(e));
    }, {passive: false});
    for (const [event, over] of [['pointerover', true], ['pointerout', false]]) this.svg.addEventListener(event, e => {
      const target = e.target.closest('[data-node]');
      if (target && !target.contains(e.relatedTarget)) this.onHover?.(this.byId.get(target.dataset.node), over);
    });
  }

  nodeOf(target) {
    const shape = target.closest?.('[data-node]');
    return shape && this.byId.get(shape.dataset.node);
  }
  setTabStop(node) {
    if (this.tabStop === node) return;
    this.tabStop.shape.setAttribute('tabindex', -1);
    node.shape.setAttribute('tabindex', 0);
    this.tabStop = node;
  }
  // The spoken name includes whatever the route painting says about the node.
  describe(node) {
    const roles = [['start', 'start of the route'], ['end', 'end of the route'], ['route', 'on the selected route']]
      .filter(([name]) => this.has(node, name)).map(([, text]) => text);
    return [node.routeLabel.replace('\n', ', '), ...roles].join(', ');
  }
  // Nearest node inside a 90° cone around the arrow direction, measured on screen-independent world coordinates.
  neighbour(node, [dx, dy]) {
    let best, bestScore = Infinity;
    for (const other of this.nodes) {
      if (other === node) continue;
      const along = (other.x - node.x) * dx + (other.y - node.y) * dy;
      const across = Math.abs((other.x - node.x) * dy - (other.y - node.y) * dx);
      if (along <= 0 || across > along) continue;
      const score = along + 2 * across;
      if (score < bestScore) { best = other; bestScore = score; }
    }
    return best;
  }
  // Keyboard focus must never land off-screen: pan the minimum distance to bring the node into view.
  reveal(node) {
    const box = this.svg.getBoundingClientRect(), p = this.position(node);
    const r = this.visibleRect?.() || {x: 0, y: 0, w: box.width, h: box.height};
    this.labelBox(node);
    const radius = (node.size / 2 + 6) * this.zoom;
    const side = Math.min(Math.max(48, node.width / 2 + 12, radius + 12), r.w / 2);
    const top = Math.min(Math.max(48, radius + 12), r.h / 2);
    const bottom = Math.min(Math.max(48, node.size / 2 * this.zoom + node.height + 17), r.h / 2);
    const shift = (value, low, high) => value < low ? low - value : value > high ? high - value : 0;
    const x = shift(p.x, r.x + side, r.x + r.w - side), y = shift(p.y, r.y + top, r.y + r.h - bottom);
    if (x || y) this.setViewport(this.zoom, {x: this.pan.x + x, y: this.pan.y + y});
  }
  has(item, name) { return item.shape.classList.contains(name); }
  classes(items, remove, add = '') {
    const removed = remove.split(' ').filter(Boolean), added = add.split(' ').filter(Boolean);
    for (const item of items) {
      for (const name of removed) if (this.has(item, name)) item.shape.classList.remove(name);
      for (const name of added) if (!this.has(item, name)) item.shape.classList.add(name);
    }
  }
  updatePositions() {
    for (const node of this.nodes) node.shape.setAttribute('transform', `translate(${node.x} ${node.y})`);
    for (const edge of this.edges) {
      const a = edge.source, b = edge.target, d = Math.hypot(b.x-a.x, b.y-a.y) || 1, r = b.size/2 + 3;
      edge.shape.setAttribute('x1', a.x); edge.shape.setAttribute('y1', a.y);
      edge.shape.setAttribute('x2', b.x - (b.x-a.x)*r/d); edge.shape.setAttribute('y2', b.y - (b.y-a.y)*r/d);
    }
  }
  moveNode(node, dx, dy) {
    this.stop();
    node.x += dx / this.zoom; node.y += dy / this.zoom;
    this.updatePositions(); this.onDrag?.(node); this.onChange?.();
  }
  position(item) {
    const x = item.label ? item.x : (item.source.x + item.target.x) / 2;
    const y = item.label ? item.y : (item.source.y + item.target.y) / 2;
    return {x: x * this.zoom + this.pan.x, y: y * this.zoom + this.pan.y};
  }
  bounds(items, rendered = false) {
    const nodes = items.flatMap(n => n.label ? [n] : [n.source, n.target]);
    const x1 = Math.min(...nodes.map(n => n.x - n.size/2)), x2 = Math.max(...nodes.map(n => n.x + n.size/2));
    const y1 = Math.min(...nodes.map(n => n.y - n.size/2)), y2 = Math.max(...nodes.map(n => n.y + n.size/2));
    return rendered ? {x1: x1*this.zoom+this.pan.x, x2: x2*this.zoom+this.pan.x,
      y1: y1*this.zoom+this.pan.y, y2: y2*this.zoom+this.pan.y} : {x1, x2, y1, y2, w: x2-x1, h: y2-y1};
  }
  setViewport(zoom, pan) {
    this.zoom = zoom; this.pan = pan;
    this.world.setAttribute('transform', `translate(${pan.x} ${pan.y}) scale(${zoom})`);
    this.onChange?.();
  }
  zoomAt(zoom, p) {
    zoom = Math.max(.1, Math.min(3, zoom));
    const scale = zoom / this.zoom;
    this.setViewport(zoom, {x: p.x-(p.x-this.pan.x)*scale, y: p.y-(p.y-this.pan.y)*scale});
  }
  stop() { cancelAnimationFrame(this.animation); this.animation = 0; }
  frame(zoom, pan, animate) {
    this.stop();
    if (!animate) { this.setViewport(zoom, pan); performance.mark('graph:camera-settled'); return; }
    const start = performance.now(), z = this.zoom, p = this.pan;
    const step = now => {
      const t = Math.min(1, (now-start)/450), ease = t < .5 ? 4*t*t*t : 1-(-2*t+2)**3/2;
      this.setViewport(z+(zoom-z)*ease, {x:p.x+(pan.x-p.x)*ease, y:p.y+(pan.y-p.y)*ease});
      this.animation = t < 1 ? requestAnimationFrame(step) : 0;
      if (t === 1) performance.mark('graph:camera-settled');
    };
    this.animation = requestAnimationFrame(step);
  }
  labelBox(node) {
    const important = ['route', 'start', 'end', 'hover', 'focused'].some(c => this.has(node, c));
    const size = important ? 12 : 11, text = important ? node.routeLabel : node.labelText;
    const key = size + ':' + text;
    if (node.labelKey !== key) {
      node.labelKey = key;
      this.measure.font = `500 ${size}px "DM Sans", sans-serif`;
      const lines = [];
      for (const paragraph of text.split('\n')) {
        let line = '';
        for (const word of paragraph.split(' ')) {
          if (line && this.measure.measureText(line+' '+word).width > 180) { lines.push(line); line = word; }
          else line += (line ? ' ' : '') + word;
        }
        lines.push(line);
      }
      node.width = Math.max(...lines.map(line => this.measure.measureText(line).width));
      node.height = lines.length * size * 1.25;
      node.label.replaceChildren(...lines.map((line, i) => {
        const span = document.createElementNS('http://www.w3.org/2000/svg', 'tspan');
        span.setAttribute('x', '0'); span.setAttribute('y', 5+size+i*size*1.25); span.textContent = line; return span;
      }));
      node.label.setAttribute('font-size', size);
    }
    node.label.setAttribute('transform', `translate(0 ${node.size/2}) scale(${1/this.zoom})`);
    const p = this.position(node), top = p.y + node.size/2*this.zoom + 5;
    return {x1:p.x-node.width/2-3, x2:p.x+node.width/2+3, y1:top-3, y2:top+node.height+3};
  }
};

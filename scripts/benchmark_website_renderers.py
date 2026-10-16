#!/usr/bin/env python3
"""Compare actual registry geometry in Cytoscape Canvas and a native SVG prototype.

This renderer-only probe draws all labels and pans 120 frames. It does not
implement the website's interaction/selection/drawer behavior, so it cannot
establish application parity. Requires the same Playwright as test_website.py.
"""
import argparse
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import re
import tempfile
from threading import Thread

from playwright.sync_api import sync_playwright

# Both renderers use the same geometry, labels, font, edges, and pan trajectory.
PROBE = r"""
(async()=>{
 const D=window.REDUCTIONS, families=new Map(), links=new Map();
 for(const n of D.nodes)if(!families.has(n.name))families.set(n.name,{name:n.name,neighbors:new Set()});
 for(const e of D.edges){const s=D.nodes[e.source].name,t=D.nodes[e.target].name;if(s===t)continue;families.get(s).neighbors.add(t);families.get(t).neighbors.add(s);links.set(s+"→"+t,{s,t});}
 const nodes=[...families.values()].map(n=>({...n,x:D.layout[n.name].x*1.6,y:D.layout[n.name].y*1.6,size:Math.max(6,Math.min(22,6+2.4*Math.sqrt(n.neighbors.size)))}));
 const byName=new Map(nodes.map(n=>[n.name,n]));
 const xs=nodes.map(n=>n.x),ys=nodes.map(n=>n.y),xmin=Math.min(...xs),xmax=Math.max(...xs),ymin=Math.min(...ys),ymax=Math.max(...ys);
 const z=Math.min((innerWidth-100)/(xmax-xmin),(innerHeight-100)/(ymax-ymin)),px=innerWidth/2-z*(xmin+xmax)/2,py=innerHeight/2-z*(ymin+ymax)/2;
 if(document.body.dataset.renderer==='canvas'){
  const cy=cytoscape({container:document.querySelector('#scene'),layout:{name:'preset'},textureOnViewport:true,pixelRatio:1,
   elements:[...nodes.map(n=>({data:{id:n.name,label:n.name,size:n.size},position:{x:n.x,y:n.y}})),...[...links.values()].map(e=>({data:{id:e.s+'→'+e.t,source:e.s,target:e.t}}))],
   style:[{selector:'node',style:{width:'data(size)',height:'data(size)','background-color':'#a9d68a',label:'data(label)',color:'#d8ddd2','font-size':11,'font-family':'DM Sans','text-valign':'bottom','text-margin-y':5}},{selector:'edge',style:{'curve-style':'straight',width:.7,'line-color':'#d8ddd2',opacity:.09}}]});
  cy.viewport({zoom:z,pan:{x:px,y:py}});window.moveGraph=k=>cy.pan({x:px+80*Math.sin(k/10),y:py+50*Math.cos(k/10)});
 }else{
  const ns='http://www.w3.org/2000/svg',make=(tag,attrs)=>{const e=document.createElementNS(ns,tag);for(const[k,v]of Object.entries(attrs))e.setAttribute(k,v);return e;};
  const svg=make('svg',{width:'100%',height:'100%'}),g=make('g',{});svg.append(g);document.querySelector('#scene').append(svg);
  for(const{s,t}of links.values()){const a=byName.get(s),b=byName.get(t);g.append(make('line',{x1:a.x,y1:a.y,x2:b.x,y2:b.y,stroke:'#d8ddd2','stroke-width':.7,opacity:.09}));}
  for(const n of nodes){g.append(make('circle',{cx:n.x,cy:n.y,r:n.size/2,fill:'#a9d68a'}));const text=make('text',{x:n.x,y:n.y+n.size/2+16,fill:'#d8ddd2','font-size':11,'font-family':'DM Sans','text-anchor':'middle'});text.textContent=n.name;g.append(text);}
  window.moveGraph=k=>g.setAttribute('transform',`translate(${px+80*Math.sin(k/10)},${py+50*Math.cos(k/10)}) scale(${z})`);window.moveGraph(0);
 }
 await document.fonts.ready;await new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r)));window.rendererReady=performance.now();window.nodeCount=nodes.length;window.edgeCount=links.size;
})()
"""


class PreviewServer(ThreadingHTTPServer):
    request_queue_size = 128


class QuietHandler(SimpleHTTPRequestHandler):
    def log_message(self, *args):
        pass


def benchmark(book):
    html = (book / 'graph.html').read_text()
    assets = re.search(r'src="([^"?]+)graph-data.js', html)[1]
    rows = []
    with tempfile.TemporaryDirectory(prefix='renderer-benchmark-') as directory:
        root = Path(directory)
        (root / 'releases').symlink_to(book / 'releases', target_is_directory=True)
        (root / 'probe.js').write_text(PROBE)
        for kind in ['canvas', 'svg']:
            canvas = f'<script src="{assets}cytoscape.min.js" defer></script>' if kind == 'canvas' else ''
            (root / f'{kind}.html').write_text(f"""<!doctype html><html><head>
<meta charset="utf-8"><meta http-equiv="Content-Security-Policy"
 content="default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'">
<link rel="stylesheet" href="{assets}graph.css">{canvas}
<script src="{assets}graph-data.js" defer></script><script src="./probe.js" defer></script>
</head><body data-renderer="{kind}"><div id="scene" style="position:fixed;inset:0"></div></body></html>""")
        server = PreviewServer(('127.0.0.1', 0), partial(QuietHandler, directory=directory))
        thread = Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            with sync_playwright() as playwright:
                browser = playwright.chromium.launch(args=['--no-proxy-server', '--proxy-bypass-list=*'])
                for slow in [False, True]:
                    for kind in ['canvas', 'svg']:
                        context = browser.new_context(viewport={'width': 1440, 'height': 1000})
                        page = context.new_page()
                        errors = []
                        page.on('pageerror', lambda error: errors.append(str(error)))
                        session = context.new_cdp_session(page)
                        session.send('Performance.enable')
                        if slow:
                            session.send('Network.enable')
                            session.send('Network.emulateNetworkConditions', {'offline': False,
                                'latency': 150, 'downloadThroughput': 200000, 'uploadThroughput': 93750})
                            session.send('Emulation.setCPUThrottlingRate', {'rate': 4})
                        page.goto(f'http://127.0.0.1:{server.server_port}/{kind}.html')
                        page.wait_for_function('() => Boolean(window.rendererReady)', timeout=20000)
                        metrics = lambda: {m['name']: m['value'] for m in session.send('Performance.getMetrics')['metrics']}
                        before = metrics()
                        frames = page.evaluate("""async () => {
                            const times = []; let previous = performance.now();
                            for (let i = 0; i < 120; i++) {
                                await new Promise(requestAnimationFrame);
                                const now = performance.now(); times.push(now - previous); previous = now;
                                moveGraph(i);
                            }
                            return times.slice(1);
                        }""")
                        after = metrics()
                        session.send('HeapProfiler.collectGarbage')
                        memory = session.send('Runtime.getHeapUsage')
                        dom = session.send('Memory.getDOMCounters')
                        row = page.evaluate("""({ready_ms:rendererReady,nodes:nodeCount,edges:edgeCount,
                            bytes:performance.getEntriesByType('resource').reduce((s,r)=>s+r.transferSize,0)})""")
                        assert not errors, errors
                        assert row['nodes'] > 0 and row['edges'] > 0
                        assert len(frames) == 119
                        row.update(renderer=kind, slow=slow,
                            frame_p95_ms=sorted(frames)[int(.95*len(frames))],
                            script_ms=1000*(after['ScriptDuration']-before['ScriptDuration']),
                            task_ms=1000*(after['TaskDuration']-before['TaskDuration']),
                            js_heap_mib=memory['usedSize']/1048576, dom_nodes=dom['nodes'])
                        if rows:
                            assert (row['nodes'],row['edges']) == (rows[0]['nodes'],rows[0]['edges'])
                        rows.append(row)
                        print(json.dumps(row), flush=True)
                        context.close()
                browser.close()
        finally:
            server.shutdown()
            thread.join()
            server.server_close()
    return rows


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('book', nargs='?', default='book', type=Path)
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    results = benchmark(args.book.resolve())
    if args.output:
        args.output.write_text(json.dumps(results, indent=2) + '\n')

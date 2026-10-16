"""Security and release contracts for the static website build."""

from pathlib import Path
import json
import itertools
import math
import re
import subprocess
import tempfile
import unittest

from playwright.sync_api import expect, sync_playwright

from build_graph_details import SafeDetailHTML
import build_website
from finalize_website import finalize
from check_website import serve_website

ROOT = Path(__file__).resolve().parents[1]


class WebsiteBuildTests(unittest.TestCase):
    def test_frontend_build_emits_typescript_for_both_pages(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source, output = root / 'source', root / 'output'
            source.mkdir()
            style = ':root { --grid: rgba(237, 242, 232, .045); } body { background: #0b110d; } '
            style += 'output { color: rgb(1, 2, 3); background: '
            style += 'radial-gradient(ellipse at 62% 48%, transparent 35%, rgba(0,0,0,.4)), '
            style += 'radial-gradient(circle at 1px 1px, var(--grid) 1px, transparent 0) 0 0 / 24px 24px; '
            style += 'display: inline-block; width: 40px; height: 40px; }'
            (source / 'style.css').write_text(style)
            for page, value in [('index', 21), ('graph', 22)]:
                (source / f'{page}.html').write_text('<!doctype html><html><head><link rel="stylesheet" href="./style.css"></head><body>'
                    f'<output></output><script type="module" src="./{page}.ts"></script></body></html>')
                (source / f'{page}.ts').write_text(f'const value: number = {value}; '
                    'document.querySelector("output")!.textContent = String(value * 2);')
            result = subprocess.run(['node', str(ROOT / 'scripts/build_frontend.mjs'), str(source), str(output)],
                                    capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            (source / 'index.ts').unlink()
            (source / 'graph.ts').unlink()
            finalize(output)
            with serve_website(output) as base, sync_playwright() as p:
                browser = p.chromium.launch()
                try:
                    page = browser.new_page()
                    for name, value in [('index', '42'), ('graph', '44')]:
                        page.set_content(f'<!doctype html><html><head><style>{style}</style></head><body><output>{value}</output></body></html>')
                        reference = page.locator('output').screenshot()
                        page.goto(base + name + '.html')
                        expect(page.locator('output')).to_have_text(value)
                        expect(page.locator('output')).to_have_css('color', 'rgb(1, 2, 3)')
                        self.assertEqual(page.locator('output').screenshot(), reference,
                                         'The build changed rendered CSS pixels')
                finally:
                    browser.close()

    def test_strict_type_checking_rejects_errors_without_emitting_javascript(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'main.ts'
            config = root / 'tsconfig.json'
            config.write_text(json.dumps({'extends': str(ROOT / 'tsconfig.json'), 'include': ['main.ts']}))
            source.write_text('function double(value: number) { return value * 2; }\ndouble(21);\n')
            command = [str(ROOT / 'node_modules/.bin/tsc'), '--project', str(config)]
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            source.write_text('function double(value) { return value * 2; }\ndouble(21);\n')
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0, 'Strict checking accepted an implicit any parameter')
            self.assertFalse((root / 'main.js').exists(), 'The separate checker emitted runtime code')

    def test_problem_records_preserve_variants_and_incident_endpoints(self):
        nodes = [{'name': name, 'variant': {'weight': weight}} for name, weight in
                 [('Neighbor', 'One'), ('Source', 'One'), ('Target', 'One'), ('Source', 'i64'), ('Remote', 'One'), ('Source', 'f64')]]
        edges = [{'source': source, 'target': target} for source, target in [(1, 2), (3, 0), (0, 4)]]
        schemas = [{'name': name} for name in ['Neighbor', 'Source', 'Target', 'Remote']]
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            build_website.build_problem_records(output, nodes, edges, schemas)
            record = json.loads((output / 'Source.json').read_text())
        self.assertEqual(record['schemas'], [{'name': 'Source'}])
        self.assertEqual({node['variant']['weight'] for node in record['nodes'] if node['name'] == 'Source'}, {'One', 'i64', 'f64'})
        self.assertNotIn('Remote', [node['name'] for node in record['nodes']])
        self.assertEqual([(record['nodes'][edge['source']]['name'], record['nodes'][edge['source']]['variant']['weight'],
                           record['nodes'][edge['target']]['name']) for edge in record['edges']],
                         [('Source', 'One', 'Target'), ('Source', 'i64', 'Neighbor')])

    def test_question_board_keeps_assets_and_registry_provenance(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'board'
            (source / 'assets').mkdir(parents=True)
            (source / 'assets/site.js').write_text('document.body.dataset.ready = "true";')
            (source / 'assets/site.css').write_text('body { color: green; }')
            (source / 'index.html').write_text('<html><head><link rel="stylesheet" href="assets/site.css">'
                '<script src="assets/site.js" defer></script></head><body>Questions</body></html>')
            registry = json.dumps({'questions': [{'id': 'Q001', 'solutions': [
                {'materials': [{'path': 'assets/proof.svg'}]}]}]}) + '\n'
            (source / 'registry.json').write_text(registry)
            output = root / 'site/open-questions'
            build_website.publish_open_questions(source, output)
            html = (output / 'index.html').read_text()
            self.assertIn("script-src 'self'", html)
            for url in re.findall(r'(?:src|href)="([^"]+)"', html):
                self.assertTrue((output / url.split('?')[0]).is_file(), url)
            self.assertEqual((output / 'registry.json').read_text(), registry)
            self.assertEqual((source / 'registry.json').read_text(), registry)

    def test_documentation_rejects_active_content(self):
        for markup in ['<script>alert(1)</script>', '<img onerror="alert(1)">',
                       '<a href="java&#10;script:alert(1)">bad</a>',
                       '<iframe src="https://example.com"></iframe>',
                       '<img src="https://example.com/track">']:
            with self.subTest(markup=markup), self.assertRaises(ValueError):
                SafeDetailHTML().feed(markup)
        SafeDetailHTML().feed('<p><a href="https://example.com">Reference</a>'
                              '<svg><use href="#glyph" /></svg></p>')

    def test_release_urls_and_csp_cover_inline_scripts(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'assets').mkdir()
            (root / 'assets/app.js').write_text('fetch("./assets/data.json")')
            (root / 'assets/data.json').write_text('{}')
            (root / 'index.html').write_text('<html><head><script>window.ready=true;</script>'
                '<script src="./assets/app.js"></script></head><body></body></html>')
            finalize(root)
            html = (root / 'index.html').read_text()
            self.assertIn("script-src 'self'", html)
            self.assertNotIn('<script>window.ready', html)
            scripts = re.findall(r'<script[^>]*src="([^"]+)"', html)
            self.assertEqual(len(scripts), 2)
            for script in scripts:
                self.assertTrue((root / script.split('?')[0]).is_file())
                self.assertIn('/assets/', script)
                self.assertIn('releases/', script)
            app = (root / scripts[1].split('?')[0]).read_text()
            self.assertIn('releases/', app)
            self.assertFalse((root / 'assets').exists())

    def test_graph_layout_is_reproducible_and_separates_node_circles(self):
        command = ['node', str(ROOT / 'scripts/generate_website_graph_layout.js'),
                   str(ROOT / 'docs/src/reductions/reduction_graph.json')]
        first, second = (subprocess.run(command, check=True, capture_output=True).stdout
                         for _ in range(2))
        self.assertEqual(first, second)
        positions = json.loads(first)
        graph = json.loads((ROOT / 'docs/src/reductions/reduction_graph.json').read_text())
        neighbors = {node['name']: set() for node in graph['nodes']}
        for edge in graph['edges']:
            a, b = (graph['nodes'][edge[end]]['name'] for end in ('source', 'target'))
            if a != b:
                neighbors[a].add(b)
                neighbors[b].add(a)
        radius = lambda name: max(6, min(22, 6 + 2.4 * math.sqrt(len(neighbors[name])))) / 2 + 9
        for a, b in itertools.combinations(positions, 2):
            distance = math.hypot(positions[a]['x'] - positions[b]['x'], positions[a]['y'] - positions[b]['y']) * 1.6
            self.assertGreaterEqual(distance, radius(a) + radius(b) - 0.1, (a, b))


if __name__ == '__main__':
    unittest.main()

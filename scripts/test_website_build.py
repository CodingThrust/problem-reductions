"""Security and release contracts for the static website build."""

from pathlib import Path
import json
import itertools
import math
import re
import subprocess
import tempfile
import unittest

from build_graph_details import SafeDetailHTML
import build_website
from finalize_website import finalize

ROOT = Path(__file__).resolve().parents[1]


class WebsiteBuildTests(unittest.TestCase):
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

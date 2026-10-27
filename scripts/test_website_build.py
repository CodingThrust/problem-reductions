"""Security and release contracts for the static website build."""

from pathlib import Path
import json
import itertools
import math
import re
import subprocess
import shutil
import tempfile
import unittest

from playwright.sync_api import expect, sync_playwright

from build_graph_details import SafeDetailHTML
import build_website
from finalize_website import finalize
from check_website import serve_website

ROOT = Path(__file__).resolve().parents[1]


class WebsiteBuildTests(unittest.TestCase):
    def test_clean_generation_produces_types_and_tracks_schema_changes(self):
        schemas = subprocess.run([str(ROOT / 'target/debug/examples/export_frontend_schema')],
                                 capture_output=True, text=True, check=True).stdout
        json.loads(schemas)
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / 'workspace with spaces'
            (root / 'scripts').mkdir(parents=True)
            (root / 'docs/website/src/lib').mkdir(parents=True)
            (root / 'node_modules').symlink_to(ROOT / 'node_modules', target_is_directory=True)
            for name in ['scripts/generate_website_contracts.mjs',
                         'docs/website/src/lib/path-worker-data.mts',
                         *[f'docs/website/{name}.schema.json' for name in
                           ['atlas-data', 'graph-details', 'graph-scene', 'native-exports', 'path-graph']]]:
                shutil.copyfile(ROOT / name, root / name)
            command = ['node', str(root / 'scripts/generate_website_contracts.mjs')]
            result = subprocess.run(command, input=schemas, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            source = root / 'consumer.mts'
            source.write_text('''
import type {GraphSceneData} from './docs/website/src/generated/graph-scene.mjs';
import type {WasmPathResult} from './docs/website/src/generated/wasm-paths.mjs';
import type {PathRequest, PathReply} from './docs/website/src/lib/path-worker-data.mts';
export function position(scene: GraphSceneData) { return scene.layout.Alpha.x; }
const result: WasmPathResult = {paths: [], truncated: false};
const request: PathRequest = {kind: 'search', source: 0, target: 1, limit: 1};
const reply: PathReply = {phase: 'paths', result};
// @ts-expect-error Rust serializes the truncation marker as a boolean.
result.truncated = 'yes';
// @ts-expect-error Comparison messages must carry comparison results.
const wrongReply: PathReply = {phase: 'comparison', result};
// @ts-expect-error Cached comparison requests require their engine result.
const wrongRequest: PathRequest = {kind: 'compare', source: 0, target: 1, limit: 1};
''')
            typecheck = [str(ROOT / 'node_modules/.bin/tsc'), '--strict', '--noEmit',
                         '--target', 'ES2022', '--module', 'NodeNext', '--moduleResolution', 'NodeNext',
                         '--allowImportingTsExtensions', '--skipLibCheck', str(source)]
            result = subprocess.run(typecheck, cwd=root, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            schema_path = root / 'docs/website/graph-scene.schema.json'
            schema = json.loads(schema_path.read_text())
            schema['required'].remove('layout')
            schema_path.write_text(json.dumps(schema))
            result = subprocess.run(command, input=schemas, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            result = subprocess.run(typecheck, cwd=root, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("'scene.layout' is possibly 'undefined'", result.stdout)

    def test_frontend_lint_checks_typescript_control_flow(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'docs/website/src/route.ts'
            source.parent.mkdir(parents=True)
            config = root / 'eslint.config.mjs'
            config.write_text(f'export {{default}} from "{(ROOT / "eslint.config.mjs").as_uri()}";\n')
            source.write_text('export function route(value: number) { switch (value) { '
                              'case 1: return "first"; case 1: return "second"; default: return "other"; } }')
            result = subprocess.run([str(ROOT / 'node_modules/.bin/eslint'), str(source), '--format', 'json'],
                                    cwd=root, capture_output=True, text=True)
            self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
            self.assertIn('no-duplicate-case', [message['ruleId'] for report in json.loads(result.stdout)
                                               for message in report['messages']])

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
            (source / 'shared.ts').write_text('export function double(value: number) { return value * 2; }\n'
                                             'document.documentElement.dataset.shared = "loaded";\n')
            for page, value in [('index', 21), ('graph', 22)]:
                (source / f'{page}.html').write_text('<!doctype html><html><head><link rel="stylesheet" href="./style.css"></head><body>'
                    f'<output></output><script type="module" src="./{page}.ts"></script></body></html>')
                (source / f'{page}.ts').write_text('import { double } from "./shared";\n'
                    f'const value: number = {value}; '
                    'document.querySelector("output")!.textContent = String(double(value));')
            result = subprocess.run(['node', str(ROOT / 'scripts/build_frontend.mjs'), str(source), str(output)],
                                    capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            for path in source.glob('*.ts'):
                path.unlink()
            finalize(output)
            with serve_website(output) as base, sync_playwright() as p:
                browser = p.chromium.launch()
                try:
                    page = browser.new_page()
                    scripts = []
                    page.on('request', lambda request: scripts.append(request.url.split('?')[0])
                            if request.url.split('?')[0].endswith('.js') else None)
                    for name, value in [('index', '42'), ('graph', '44')]:
                        page.set_content(f'<!doctype html><html><head><style>{style}</style></head><body><output>{value}</output></body></html>')
                        reference = page.locator('output').screenshot()
                        scripts.clear()
                        page.goto(base + name + '.html')
                        expect(page.locator('output')).to_have_text(value)
                        expect(page.locator('output')).to_have_css('color', 'rgb(1, 2, 3)')
                        self.assertEqual(page.locator('output').screenshot(), reference,
                                         'The build changed rendered CSS pixels')
                        self.assertEqual(len(scripts), len(set(scripts)), 'A preloaded module was downloaded twice')
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
        nodes = [{'name': name, 'variant': {'weight': weight}, 'category': 'misc', 'default': weight == 'One',
                  'parameter_names': [], 'complexity': '1', 'complexity_mathml': build_website.expression_mathml('1'),
                  'doc_path': f'models/misc/struct.{name}.html', 'api_path': f'models/misc/struct.{name}.html'} for name, weight in
                 [('Neighbor', 'One'), ('Source', 'One'), ('Target', 'One'), ('Source', 'i64'), ('Remote', 'One'), ('Source', 'f64')]]
        edges = [{'source': source, 'target': target, 'parameters': [], 'parameter_contract_error': None,
                  'witness': True, 'aggregate': False, 'turing': False, 'doc_path': 'rules/example/index.html',
                  'source_path': 'src/rules/example.rs', 'test_path': None, 'api_path': 'rules/trait.ReduceTo.html'}
                 for source, target in [(1, 2), (3, 0), (0, 4)]]
        schemas = [{'name': name, 'category': 'misc', 'description': '', 'fields': [],
                    'module_path': f'problemreductions::models::misc::{name.lower()}',
                    'source_path': f'src/models/misc/{name.lower()}.rs'} for name in ['Neighbor', 'Source', 'Target', 'Remote']]
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            build_website.build_problem_records(output, nodes, edges, schemas)
            record = json.loads((output / 'Source.json').read_text())
            nodes[0]['category'] = 123
            with self.assertRaises(subprocess.CalledProcessError):
                build_website.build_problem_records(output, nodes, edges, schemas)
        self.assertEqual(record['schemas'], [schemas[1]])
        self.assertEqual({node['variant']['weight'] for node in record['nodes'] if node['name'] == 'Source'}, {'One', 'i64', 'f64'})
        self.assertNotIn('Remote', [node['name'] for node in record['nodes']])
        self.assertEqual([(record['nodes'][edge['source']]['name'], record['nodes'][edge['source']]['variant']['weight'],
                           record['nodes'][edge['target']]['name']) for edge in record['edges']],
                         [('Source', 'One', 'Target'), ('Source', 'i64', 'Neighbor')])

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
        # The default disc's 2px stroke extends 1px beyond its geometric radius.
        radius = lambda name: max(6, min(22, 6 + 2.4 * math.sqrt(len(neighbors[name])))) / 2 + 1
        for a, b in itertools.combinations(positions, 2):
            distance = math.hypot(positions[a]['x'] - positions[b]['x'], positions[a]['y'] - positions[b]['y']) * 1.6
            self.assertGreaterEqual(distance, radius(a) + radius(b), (a, b))


if __name__ == '__main__':
    unittest.main()

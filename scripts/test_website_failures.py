"""Real HTTP failure/retry checks using mutable copies of the production assets."""

from functools import partial
import gzip
import json
import os
from pathlib import Path
import re
import shutil
import tempfile
import time
from threading import Event, Thread
import unittest

from playwright.sync_api import expect

from check_website import Handler, PreviewServer
import test_website


class UncachedHandler(Handler):
    def do_GET(self):
        if self.path.endswith('/graph-scene.json.gz'):
            self.server.scene_release.wait(timeout=3)
        if self.path.endswith('/path-selection_bg.wasm.gz'):
            self.server.engine_requested.set()
            self.server.engine_release.wait(timeout=3)
        super().do_GET()

    def end_headers(self):
        # Each retry exercises the repaired bytes, rather than a cached bad response.
        self.send_header('Cache-Control', 'no-store')
        super().end_headers()


class WebsiteFailureTests(test_website.WebsiteTests):
    @classmethod
    def setUpClass(cls):
        source = os.environ.get('WEBSITE_SITE_DIR')
        if not source:
            raise unittest.SkipTest('Use check_website.py to supply the production artifact with WEBSITE_SITE_DIR')
        cls.source = Path(source)
        super().setUpClass()

    def setUp(self):
        super().setUp()
        temporary = tempfile.TemporaryDirectory(prefix='problem-reductions-failure-')
        self.addCleanup(temporary.cleanup)
        self.site = Path(temporary.name)
        for name in ['index.html', 'graph.html']:
            shutil.copyfile(self.source / name, self.site / name)
        release = re.search(r'releases/\d+', (self.site / 'graph.html').read_text())[0]
        shutil.copytree(self.source / release, self.site / release)
        self.assets = self.site / release / 'assets'
        server = PreviewServer(('127.0.0.1', 0), partial(UncachedHandler, directory=str(self.site)))
        self.engine_requested = server.engine_requested = Event()
        self.engine_release = server.engine_release = Event()
        self.engine_release.set()
        self.scene_release = server.scene_release = Event()
        self.scene_release.set()
        thread = Thread(target=server.serve_forever, daemon=True)
        thread.start()
        self.base = f'http://127.0.0.1:{server.server_port}/'
        def close():
            self.engine_release.set()
            self.scene_release.set()
            server.shutdown()
            thread.join()
            server.server_close()
        self.addCleanup(close)

    def assert_engine_recovery(self, name, damaged):
        resource = self.assets / name
        original = resource.read_bytes()
        if damaged is None:
            resource.unlink()
        else:
            resource.write_bytes(damaged)
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results')).to_contain_text('Cannot load the path engine', timeout=1000)
        expect(self.page.locator('.skeleton, .path')).to_have_count(0)
        expect(self.page.get_by_role('button', name='Cancel', exact=True)).to_have_count(0)
        resource.write_bytes(original)
        self.page.get_by_role('button', name='SAT → QUBO', exact=True).click()
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        route = self.copy_route()
        self.assertEqual(route['path'][0]['from']['name'], 'Satisfiability')
        self.assertEqual(route['path'][-1]['to']['name'], 'QUBO')

    def test_graph_recovers_after_missing_worker(self):
        self.assert_engine_recovery('path-worker.js', None)

    def test_graph_recovers_after_invalid_wasm(self):
        self.assert_engine_recovery('path-selection_bg.wasm.gz', gzip.compress(b'not a WASM module', mtime=0))

    def test_graph_recovers_after_corrupt_engine_metadata(self):
        self.assert_engine_recovery('path-graph.json.gz', gzip.compress(b'{invalid JSON', mtime=0))

    def test_graph_growth_preserves_wide_exponents_and_exact_degree_sums(self):
        resource = self.assets / 'path-graph.json.gz'
        graph = json.loads(gzip.decompress(resource.read_bytes()))
        edge = next(edge for edge in graph['edges']
                    if graph['nodes'][edge['source']]['name'] == 'GraphPartitioning'
                    and graph['nodes'][edge['source']]['default']
                    and graph['nodes'][edge['target']]['name'] == 'QUBO'
                    and graph['nodes'][edge['target']]['default'])
        formulas = {
            'num_vars': 'num_vertices^9007199254740993 + num_edges^9007199254740992',
            'num_quadratic_terms': 'num_edges^9007199254740992 * num_vertices',
        }
        for parameter in edge['parameters']:
            parameter['formula'] = formulas[parameter['field']]
            parameter['contract'] = 'exact'
        graph['edges'] = [edge]
        resource.write_bytes(gzip.compress(json.dumps(graph).encode(), mtime=0))
        self.page.goto(self.base + 'graph.html')
        self.choose_endpoint('source', 'Graph Partitioning')
        expect(self.page.locator('#results .summary strong')).to_contain_text('1 best route')
        expect(self.page.locator('.path .size')).to_have_text('n⁹⁰⁰⁷¹⁹⁹²⁵⁴⁷⁴⁰⁹⁹³', timeout=1000)
        summaries = self.page.locator('.growth').evaluate('''dl => Object.fromEntries(
            [...dl.querySelectorAll('dt')].map(term => [term.textContent, term.nextElementSibling.textContent]))''')
        self.assertEqual(summaries['vars'], 'Θ(vertices⁹⁰⁰⁷¹⁹⁹²⁵⁴⁷⁴⁰⁹⁹³)')
        quadratic = summaries['quadratic terms']
        self.assertRegex(quadratic, r'^Θ\([^()]+\)$')
        # Product factors commute; their exact exponents and multiplicity must agree.
        self.assertCountEqual(quadratic.removeprefix('Θ(').removesuffix(')').split('·'),
                              ['edges⁹⁰⁰⁷¹⁹⁹²⁵⁴⁷⁴⁰⁹⁹²', 'vertices'])
        route = self.copy_route()
        self.assertEqual([step['to']['name'] for step in route['path']], ['QUBO'])

    def cancel_pending_engine(self):
        # Hold the real HTTP response so cancellation deterministically precedes path results.
        self.engine_release.clear()
        self.page.goto(self.base + 'graph.html')
        self.assertTrue(self.engine_requested.wait(timeout=1), 'The real WASM request never started')
        expect(self.page.locator('.path')).to_have_count(0)
        self.page.get_by_role('button', name='Cancel', exact=True).click()
        expect(self.page.locator('#results')).to_contain_text('Search cancelled.')
        self.engine_release.set()

    def test_graph_cancel_before_paths_allows_successful_retry(self):
        self.cancel_pending_engine()
        self.page.get_by_role('button', name='Search again', exact=True).click()
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.assertEqual(self.copy_route()['path'][-1]['to']['name'], 'QUBO')

    def test_graph_recovers_after_offline_engine_start(self):
        self.cancel_pending_engine()
        self.context.set_offline(True)
        self.page.get_by_role('button', name='Search again', exact=True).click()
        expect(self.page.locator('#results')).to_contain_text('Cannot load the path engine')
        expect(self.page.get_by_role('button', name='Cancel', exact=True)).to_have_count(0)
        self.context.set_offline(False)
        self.page.get_by_role('button', name='SAT → QUBO', exact=True).click()
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.assertEqual(self.copy_route()['path'][-1]['to']['name'], 'QUBO')

    def assert_detail_recovery(self, resource, message):
        original = resource.read_bytes()
        resource.unlink()
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        def open_details():
            self.page.locator('.scene-node[data-node="MaximumIndependentSet"]').click()
            self.page.locator('#pop .pop-details').click()
        open_details()
        expect(self.page.locator('#drawer-body [role="alert"]')).to_contain_text(message)
        expect(self.page.locator('#drawer-body [aria-busy="true"]')).to_have_count(0)
        resource.write_bytes(original)
        self.page.get_by_role('button', name='Close details', exact=True).click()
        open_details()
        expect(self.page.locator('#drawer-body article')).to_contain_text('wireless network scheduling')
        expect(self.page.locator('#drawer-body [role="alert"]')).to_have_count(0)
        self.assertEqual(self.copy_route()['path'][-1]['to']['name'], 'QUBO')

    def test_graph_details_retry_after_missing_index(self):
        self.assert_detail_recovery(self.assets / 'graph-details.json', 'Cannot load documentation index')

    def test_graph_details_retry_after_missing_article(self):
        manifest = json.loads((self.assets / 'graph-details.json').read_text())
        article = self.site / manifest['entries']['problem:MaximumIndependentSet']
        self.assert_detail_recovery(article, 'Cannot load documentation (404)')

    def test_graph_details_rejects_index_urls_outside_current_articles_and_recovers(self):
        resource = self.assets / 'graph-details.json'
        original = resource.read_bytes()
        manifest = json.loads(original)
        manifest['entries']['problem:MaximumIndependentSet'] = './index.html'
        resource.write_text(json.dumps(manifest))
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        def open_details():
            self.open_problem_popover('MaximumIndependentSet')
            self.page.get_by_role('button', name='Details', exact=True).click()
        open_details()
        expect(self.page.locator('#drawer-body [role="alert"]')).to_contain_text('Invalid documentation index', timeout=1000)
        resource.write_bytes(original)
        self.page.get_by_role('button', name='Close details', exact=True).click()
        open_details()
        expect(self.page.locator('#drawer-body article')).to_contain_text('wireless network scheduling')
        self.assertEqual(self.copy_route()['path'][-1]['to']['name'], 'QUBO')

    def test_graph_rejects_unknown_scene_category_before_rendering(self):
        resource = self.assets / 'graph-scene.json.gz'
        scene = json.loads(gzip.decompress(resource.read_bytes()))
        scene['nodes'][0]['category'] = 'unregistered-category'
        resource.write_bytes(gzip.compress(json.dumps(scene).encode(), mtime=0))
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results')).to_contain_text('Cannot load graph', timeout=1000)
        expect(self.page.locator('svg.graph-scene')).to_have_count(0)

    def test_graph_scene_failure_releases_worker_and_reload_recovers(self):
        resource = self.assets / 'graph-scene.json.gz'
        original = resource.read_bytes()
        resource.unlink()
        # Let the eager worker start before delivering the scene's real 404 response.
        self.scene_release.clear()
        self.page.goto(self.base + 'graph.html')
        self.assertTrue(self.engine_requested.wait(timeout=1), 'The eager engine never requested its WASM')
        self.scene_release.set()
        expect(self.page.locator('#results')).to_contain_text('Cannot load graph', timeout=1000)
        deadline = time.monotonic() + 1
        while self.page.workers and time.monotonic() < deadline:
            self.page.wait_for_timeout(10)
        self.assertEqual(self.page.workers, [], 'A failed graph retained its unused path worker')
        resource.write_bytes(original)
        self.page.reload()
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.assertEqual(self.copy_route()['path'][-1]['to']['name'], 'QUBO')


def load_tests(loader, tests, pattern):
    # Reuse browser fixtures without running the inherited page suite a second time.
    return unittest.TestSuite(WebsiteFailureTests(name) for name in WebsiteFailureTests.__dict__ if name.startswith('test_'))


if __name__ == '__main__':
    unittest.main()

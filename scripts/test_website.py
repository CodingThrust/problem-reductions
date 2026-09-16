#!/usr/bin/env python3
"""Browser checks against a built, served website (see docs/website/README.md)."""

import os
import unittest
from urllib.parse import urljoin

from playwright.sync_api import sync_playwright, expect


class WebsiteTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.playwright = sync_playwright().start()
        cls.browser = cls.playwright.chromium.launch(
            channel=os.environ.get("WEBSITE_BROWSER_CHANNEL") or None
        )
        cls.base = os.environ.get("WEBSITE_BASE_URL", "http://127.0.0.1:3001/")

    @classmethod
    def tearDownClass(cls):
        cls.browser.close()
        cls.playwright.stop()

    def setUp(self):
        self.context = self.browser.new_context(
            viewport={"width": 1440, "height": 1000},
            reduced_motion="reduce",
            permissions=["clipboard-read", "clipboard-write"],
        )
        self.page = self.context.new_page()
        self.page.set_default_timeout(3000)
        self.errors = []
        self.page.on("pageerror", lambda error: self.errors.append(str(error)))

    def tearDown(self):
        self.context.close()
        self.assertEqual(self.errors, [], "Uncaught browser errors")

    def visit(self, route="home"):
        self.page.goto(self.base + "#" + route)

    def test_alias_search_and_reset(self):
        self.visit("atlas")
        self.page.locator("#atlas-search").fill("MIS")
        expect(self.page.locator(".atlas-row")).to_have_count(1)
        expect(self.page.locator(".atlas-row h2")).to_have_text("Maximum Independent Set")
        self.page.locator('[data-filter="algebraic"]').click()
        expect(self.page.locator(".empty-state")).to_be_visible()
        self.page.get_by_role("button", name="Reset search and filters").click()
        expect(self.page.locator("#atlas-search")).to_have_value("")
        self.assertGreater(self.page.locator(".atlas-row").count(), 100)

    def test_main_navigation_returns_home_from_problem(self):
        self.visit('problem/MaximumIndependentSet')
        home = self.page.get_by_role('navigation', name='Main navigation').get_by_role('link', name='Home', exact=True)
        home.click()
        expect(home).to_have_attribute('aria-current', 'page')
        expect(self.page.locator('main .hero')).to_be_visible()

    def test_open_problems_has_its_own_navigation_tab(self):
        self.visit()
        tab = self.page.get_by_role('navigation', name='Main navigation').get_by_role('link', name='Open problems', exact=True)
        tab.click()
        expect(tab).to_have_attribute('aria-current', 'page')
        self.assertEqual(tab.locator('xpath=following-sibling::a').count(), 0)
        expect(tab.locator('span')).to_have_text('↗')
        expect(self.page.get_by_role('heading', name='Open problems', exact=True)).to_be_visible()
        expect(self.page.locator('main')).to_contain_text('To be released.')
        self.page.goto(self.base + 'introduction.html')
        expect(self.page.locator('.sidebar')).not_to_contain_text('Open problems')
        expect(self.page.locator('.sidebar').get_by_text('Research', exact=True)).to_have_count(0)
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        tab = self.page.get_by_role('link', name='Open problems', exact=True)
        self.assertEqual(tab.locator('xpath=following-sibling::a').count(), 0)
        expect(tab.locator('span')).to_have_text('↗')
        tab.click()
        expect(self.page.get_by_role('heading', name='Open problems', exact=True)).to_be_visible()

    def test_home_formulas_load_as_typeset_vectors(self):
        self.visit()
        for name in ['sat', 'qubo']:
            art = self.page.locator(f'.mini-art img[src$="/assets/{name}.svg"]')
            expect(art).to_be_visible()
            self.page.wait_for_function('(name) => { const img = document.querySelector(`.mini-art img[src$="/assets/${name}.svg"]`); return img.complete && img.naturalWidth > 0; }', arg=name)
            expect(art).to_have_css('object-fit', 'contain')
            svg = self.page.request.get(urljoin(self.base, art.get_attribute('src')))
            self.assertEqual(svg.status, 200)
            self.assertIn('<path', svg.text())
            self.assertNotIn('<text', svg.text())

    def test_large_resources_load_only_when_needed(self):
        requests = []
        self.page.on('request', lambda request: requests.append(request.url))
        self.visit()
        expect(self.page.locator('.problem-card').first).to_be_visible()
        self.assertFalse(any('atlas-data.' in url or 'graph-details.' in url for url in requests))
        self.page.locator('[data-nav="atlas"]').click()
        expect(self.page.locator('.atlas-row').first).to_be_visible()
        self.assertEqual(sum('atlas-data.json' in url for url in requests), 1)
        requests.clear()
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#cy canvas').first).to_be_visible()
        self.assertFalse(any('layout-bundle' in url or 'graph-details.' in url for url in requests))
        self.page.locator('#browser-search').fill('Bin Packing')
        self.page.locator('.browser-item').first.click()
        expect(self.page.locator('.typst-detail')).to_contain_text('Definition')
        self.assertEqual(sum('layout-bundle' in url for url in requests), 1)
        self.assertEqual(sum('graph-details.json' in url for url in requests), 1)

    def test_csp_blocks_inline_event_handlers(self):
        self.visit()
        self.page.evaluate('''() => {
            window.blockedScripts = [];
            document.addEventListener('securitypolicyviolation', e => window.blockedScripts.push(e.effectiveDirective));
            const image = document.createElement('img');
            image.setAttribute('onerror', 'window.injected = true');
            document.body.append(image);
            image.dispatchEvent(new Event('error'));
        }''')
        self.page.wait_for_function('() => window.blockedScripts.includes("script-src-attr")')
        self.assertIsNone(self.page.evaluate('window.injected'))

    def test_published_documentation_resources_and_csp(self):
        violations = []
        self.page.on('console', lambda message: violations.append(message.text)
                     if message.type == 'error' else None)
        for path in ['index.html', 'graph.html', 'introduction.html', 'api/problemreductions/index.html']:
            self.page.goto(urljoin(self.base, path))
            expect(self.page.locator('meta[http-equiv="Content-Security-Policy"]')).to_have_count(1)
            self.assertEqual(self.page.locator('script:not([src]):not([type="application/json"])').evaluate_all(
                '(scripts) => scripts.filter(script => script.textContent.trim()).length'), 0)
        self.page.goto(urljoin(self.base, 'introduction.html?search=reduction'))
        expect(self.page.locator('#mdbook-searchresults')).to_contain_text('reduction')
        self.page.goto(urljoin(self.base, 'api/problemreductions/index.html?search=MaximumIndependentSet'))
        expect(self.page.locator('#search')).to_contain_text('MaximumIndependentSet')
        self.assertEqual(violations, [])
        pdf = self.page.request.get(urljoin(self.base, 'reductions.pdf'))
        self.assertEqual(pdf.status, 200)
        self.assertTrue(pdf.body().startswith(b'%PDF-'))

    def test_details_cache_evicts_old_articles(self):
        requests = []
        self.page.on('request', lambda request: requests.append(request.url))
        self.visit('problem/ILP')
        expect(self.page.locator('.typst-detail')).to_contain_text('Definition')
        urls = self.page.evaluate('''async () => {
            const keys = Object.keys(GRAPH_DETAILS.entries).filter(key => key.startsWith('problem:') && key !== 'problem:ILP').slice(0, 25);
            const detail = document.querySelector('#reference-detail');
            for (const key of [...keys, keys.at(-1), keys[0]]) {
                await renderDetails(detail, '', '', key, key, () => {});
            }
            return [keys[0], keys.at(-1)].map(key => new URL(GRAPH_DETAILS.entries[key], location.href).href);
        }''')
        self.assertEqual(requests.count(urls[0]), 2)
        self.assertEqual(requests.count(urls[1]), 1)

    def test_text_sizes_preserve_reading_hierarchy(self):
        self.visit('problem/MaximumIndependentSet')
        for selector, size in [('.metadata dt', 13), ('.reading-aside p', 13),
                               ('.site-header nav', 14), ('.reference-tools button', 14),
                               ('.metadata dd', 15), ('.reading-aside > a', 15),
                               ('.variant-control select', 15), ('.typst-detail p', 16)]:
            expect(self.page.locator(selector).first).to_have_css('font-size', f'{size}px')
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        expect(self.page.locator('.browser-item').first).to_be_visible()
        for selector, size in [('.browser-item small', 13), ('.graph-legend', 13),
                               ('.inspector-heading span', 14), ('.browser-item', 15),
                               ('.graph-detail', 16)]:
            expect(self.page.locator(selector).first).to_have_css('font-size', f'{size}px')

    def test_search_text_cannot_inject_markup(self):
        self.visit("atlas")
        self.page.locator("#atlas-search").fill('<img src=x onerror="window.injected=true">')
        self.page.locator('.wordmark').first.click()
        self.page.locator('[data-nav="atlas"]').click()
        expect(self.page.locator(".empty-state")).to_be_visible()
        self.assertIsNone(self.page.evaluate("window.injected"))
        self.assertEqual(self.page.locator("main img").count(), 0)

    def test_keyboard_search_from_home(self):
        self.visit()
        expect(self.page.get_by_role("button", name="Search the atlas")).to_have_count(0)
        self.page.keyboard.press("Control+k")
        expect(self.page.locator("#atlas-search")).to_be_focused()
        self.page.keyboard.type("QUBO")
        expect(self.page.get_by_role("heading", name="QUBO", exact=True)).to_be_visible()

    def test_variant_changes_connections_and_survives_reload(self):
        self.visit("problem/MaximumIndependentSet")
        expect(self.page.locator("#variant-select")).to_have_value("graph=SimpleGraph,weight=One")
        expect(self.page.locator('.relation-link[href*="MinimumVertexCover"]')).to_have_count(0)
        self.page.locator("#variant-select").select_option("graph=SimpleGraph,weight=i64")
        expect(self.page.locator('.relation-link[href*="MinimumVertexCover"]')).to_have_count(2)
        self.page.reload()
        expect(self.page.locator("#variant-select")).to_have_value("graph=SimpleGraph,weight=i64")

    def test_reduction_evidence_and_typst_proof(self):
        self.visit()
        self.page.locator("#featured-result-link").click()
        expect(self.page.locator("h1")).to_contain_text("Minimum Vertex Cover")
        expect(self.page.locator('.parameter-relation')).to_have_count(2)
        expect(self.page.locator('.parameter-operator')).to_have_text(['=', '='])
        expect(self.page.locator('.typst-detail')).to_contain_text('Proof')
        expect(self.page.locator('.typst-detail')).to_have_attribute(
            'data-content-key', 'rule:MinimumVertexCover->MaximumIndependentSet')
        expect(self.page.locator('.page-header > p')).to_have_count(0)
        expect(self.page.locator('main')).not_to_contain_text('Classical reduction')
        expect(self.page.locator('main')).not_to_contain_text('Graph preserved; solution complemented')
        source = self.page.locator('.rule-endpoint a').first
        expect(source).to_be_visible()
        expect(self.page.locator('.rule-endpoint a [aria-hidden="true"]')).to_have_text(['↗', '↗'])
        expect(self.page.locator('h1 a')).to_have_count(0)
        expect(self.page.locator('.rule-endpoints dt')).to_have_text(['Source', 'Target'])
        self.assertTrue(self.page.evaluate('''() =>
            document.querySelector('#rule-parameters').getBoundingClientRect().bottom <
            document.querySelector('#reference-detail').getBoundingClientRect().top'''))
        expect(self.page.get_by_role('button', name='Parameters', exact=True)).to_have_count(0)
        expect(self.page.locator('#rule-parameters table')).to_have_count(0)
        expect(self.page.locator('#rule-parameters')).to_be_visible()
        source.click()
        expect(self.page.locator("#variant-select")).to_have_value("graph=SimpleGraph,weight=i64")

    def test_only_turing_rules_have_execution_labels(self):
        self.visit('problem/MaximumIndependentSet')
        expect(self.page.locator('.relation-link .capability').first).to_have_text('Turing reduction')
        self.assertTrue(self.page.locator('.relation-link').evaluate_all('''links => links.every(link =>
            !link.textContent.includes('Result recovery') && !link.textContent.includes('See contract'))'''))
        self.visit('reduction/MaximumIndependentSet/DecisionMaximumIndependentSet')
        expect(self.page.locator('.reading-aside .rule-kind')).to_have_text('Turing reduction')
        self.visit('reduction/CircuitSAT/DecisionSpinGlass')
        expect(self.page.locator('.reading-aside .rule-kind')).to_have_count(0)
        expect(self.page.locator('.reading-aside')).not_to_contain_text('Status')
        expect(self.page.locator('.reading-aside')).not_to_contain_text('Capabilities')

    def test_rule_parameter_upper_bounds_use_inequalities(self):
        self.visit('reduction/CircuitSAT/DecisionSpinGlass')
        expect(self.page.locator('.parameter-operator')).to_have_text(['≤', '≤'])
        expect(self.page.locator('.parameter-relation math')).to_have_count(2)
        self.page.set_viewport_size({'width': 390, 'height': 900})
        self.assertFalse(self.page.evaluate('document.documentElement.scrollWidth > innerWidth'))

    def test_clipboard(self):
        self.visit()
        self.page.get_by_role("button", name="Copy installation command").click()
        expect(self.page.locator(".toast")).to_have_text("Command copied to clipboard")
        self.assertEqual(self.page.evaluate("navigator.clipboard.readText()"),
                         "cargo install problemreductions-cli")

    def test_unknown_record_has_recovery(self):
        self.visit("problem/NotARegisteredProblem")
        expect(self.page.get_by_text("This connection is not in the atlas.")).to_be_visible()
        self.page.locator("main .button").click()
        expect(self.page.locator("#atlas-search")).to_be_visible()

    def test_implementation_links_follow_main(self):
        for route, path in [('problem/ILP', 'src/models/algebraic/ilp.rs'),
                            ('problem/MaximumIndependentSet', 'src/models/graph/maximum_independent_set.rs'),
                            ('problem/DecisionMaximumIndependentSet', 'src/models/decision.rs'),
                            ('reduction/MinimumVertexCover/MaximumIndependentSet', 'src/rules/minimumvertexcover_maximumindependentset.rs'),
                            ('reduction/MaximumIndependentSet/DecisionMaximumIndependentSet', 'src/models/decision.rs')]:
            self.visit(route)
            expect(self.page.get_by_role('link', name='Open API reference ↗', exact=True)).to_be_visible()
            expect(self.page.get_by_role('link', name='Open implementation')).to_have_attribute(
                'href', 'https://github.com/CodingThrust/problem-reductions/blob/main/' + path)

    def test_decision_wrapper_links_to_its_public_api(self):
        self.visit("problem/DecisionMinimumVertexCover")
        api = self.page.get_by_role("link", name="Open API reference", exact=False)
        self.assertTrue(api.get_attribute("href").endswith(
            "models/decision/struct.Decision.html"
        ))
        response = self.page.request.get(urljoin(self.page.url, api.get_attribute("href")))
        self.assertEqual(response.status, 200)

    def test_reduction_links_to_a_published_api_contract(self):
        self.visit()
        self.page.locator("#featured-result-link").click()
        contract = self.page.get_by_role("link", name="Open API reference", exact=False)
        response = self.page.request.get(urljoin(self.page.url, contract.get_attribute("href")))
        self.assertEqual(response.status, 200)

    def test_research_anchor_and_back_navigation(self):
        self.visit()
        expect(self.page.get_by_role("navigation", name="Main navigation")
               .get_by_role("link", name="Research", exact=True)).to_have_count(0)
        self.page.get_by_role("link", name="Research loop", exact=False).click()
        expect(self.page.locator("#research")).to_be_in_viewport()
        self.page.go_back()
        expect(self.page.locator(".hero")).to_be_in_viewport()

    def test_documentation_is_reachable(self):
        self.visit()
        self.page.locator(".docs-nav").click()
        self.assertTrue(self.page.url.endswith("/introduction.html"))
        expect(self.page.get_by_role("heading", name="Problem Reductions", exact=True)).to_be_visible()
        self.assertNotIn("{{#include", self.page.locator("body").inner_text())
        self.page.get_by_role('navigation', name='Documentation resources').get_by_role('link', name='Home ↗', exact=True).click()
        expect(self.page.locator('main .hero')).to_be_visible()

    def test_docs_sidebar_drag_resizes_content(self):
        self.page.goto(self.base + 'introduction.html')
        sidebar = self.page.locator('.sidebar')
        handle = self.page.locator('.sidebar-resize-handle')
        before = sidebar.bounding_box()['width']
        content_before = self.page.locator('.content').bounding_box()['x']
        box = handle.bounding_box()
        self.page.mouse.move(box['x'] + box['width'] / 2, box['y'] + 100)
        self.page.mouse.down()
        self.page.mouse.move(box['x'] + 120, box['y'] + 100, steps=8)
        self.page.mouse.up()
        self.assertGreater(sidebar.bounding_box()['width'], before + 100)
        self.assertGreater(self.page.locator('.content').bounding_box()['x'], content_before + 100)
        for width in [150, 420]:
            box = handle.bounding_box()
            self.page.mouse.move(box['x'] + box['width'] / 2, 100)
            self.page.mouse.down()
            self.page.mouse.move(width, 100, steps=8)
            self.page.mouse.up()
            logo = self.page.locator('.docs-brand img')
            bounds = logo.bounding_box()
            self.assertLessEqual(bounds['width'], 204)
            self.assertLessEqual(bounds['x'] + bounds['width'], sidebar.bounding_box()['width'] - 17)
            ratio = logo.evaluate('img => img.naturalWidth / img.naturalHeight')
            self.assertAlmostEqual(bounds['width'] / bounds['height'], ratio, delta=0.02)
            expect(self.page.locator('.docs-brand')).to_have_css('height', '78px')

    def test_graph_motion_expands_and_cleans_up_after_interruption(self):
        self.page.emulate_media(reduced_motion="no-preference")
        self.page.goto(self.base + "graph.html")
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.page.locator('#graph-search').fill('MIS')
        self.page.locator('#graph-search').press('Enter')
        self.page.wait_for_function("() => document.querySelector('#cy')._cyreg.cy.getElementById('MaximumIndependentSet').children().length > 0")
        samples = self.page.evaluate("""async () => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            const node = cy.getElementById('MaximumIndependentSet').children().first();
            const first = {...node.position()};
            await new Promise(resolve => setTimeout(resolve, 400));
            return [first, {...node.position()}];
        }""")
        self.assertNotEqual(samples[0], samples[1], 'Variants should move out from the family')
        self.page.get_by_role('button', name='Reset view').click()
        self.page.locator('#graph-search').fill('ILP')
        self.page.locator('#graph-search').press('Enter')
        self.page.get_by_role('button', name='Reset view').click()
        self.page.wait_for_function("""() =>
            document.querySelector('#cy')._cyreg.cy.nodes('[?isVariant]').length === 0
        """)
        expect(self.page.locator('.graph-inspector')).to_be_visible()

    def test_graph_is_in_primary_navigation_and_opens_details(self):
        self.visit()
        graph_link = self.page.get_by_role("link", name="Graph", exact=True)
        expect(graph_link).to_be_visible()
        graph_link.click()
        self.assertTrue(self.page.url.endswith("/graph.html"))
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.page.locator("#graph-search").fill("MIS")
        self.page.locator("#graph-search").press("Enter")
        expect(self.page.locator(".graph-inspector")).to_be_visible()
        expect(self.page.locator("#selection-type")).to_have_text("Problem")
        variants = self.page.evaluate("""() => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            return cy.getElementById('MaximumIndependentSet').children()
                .map(node => node.id());
        }""")
        self.assertGreater(len(variants), 1)
        self.assertIn('MaximumIndependentSet/graph=SimpleGraph,weight=i64', variants)
        self.assertTrue(self.page.evaluate("""() => {
            const boxes = document.querySelector('#cy')._cyreg.cy.nodes('[?isVariant]')
                .map(node => node.boundingBox());
            return boxes.every((a, i) => boxes.slice(i + 1).every(b =>
                a.x2 <= b.x1 || b.x2 <= a.x1 || a.y2 <= b.y1 || b.y2 <= a.y1));
        }"""), 'Variant nodes and their labels must not overlap')
        self.assertTrue(self.page.evaluate("""() => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            const children = cy.getElementById('MaximumIndependentSet').children();
            return children.connectedEdges().some(edge =>
                edge.source().isChild() && edge.target().isChild());
        }"""))
        expect(self.page.locator('#graph-detail > h2')).to_have_text('Maximum Independent Set')
        self.page.get_by_role("button", name="Reset view").click()
        expect(self.page.locator(".graph-inspector")).to_be_visible()
        self.assertEqual(self.page.evaluate("""() =>
            document.querySelector('#cy')._cyreg.cy
                .getElementById('MaximumIndependentSet').children().length
        """), 0)
        positions = self.page.evaluate("""() =>
            document.querySelector('#cy')._cyreg.cy.nodes()
                .map(node => ({id: node.id(), ...node.position()}))
        """)
        self.page.locator('#graph-search').fill('ILP')
        self.page.locator('#graph-search').press('Enter')
        self.assertEqual(self.page.evaluate("""() => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            const parent = cy.getElementById('ILP');
            const box = parent.boundingBox();
            const neighbors = parent.children().connectedEdges().connectedNodes()
                .difference(parent.children());
            if (parent.children().length !== 4 || neighbors.length <= 50) return ['Missing variants or neighbors'];
            return neighbors.filter(node => {
                    const {x, y} = node.position();
                    return x >= box.x1 && x <= box.x2 && y >= box.y1 && y <= box.y2;
                }).map(node => node.id());
        }"""), [])
        displaced = self.page.evaluate("""() =>
            document.querySelector('#cy')._cyreg.cy.nodes()
                .filter(node => !node.isParent() && !node.data('isVariant'))
                .map(node => ({id: node.id(), ...node.position()}))
        """)
        original = {node['id']: node for node in positions}
        moved = [node for node in displaced if node != original[node['id']]]
        self.assertGreater(len(moved), 0)
        self.assertLess(len(moved), len(displaced), 'Distant nodes keep their positions')
        self.page.get_by_role('button', name='Reset view').click()
        self.assertEqual(self.page.evaluate("""() =>
            document.querySelector('#cy')._cyreg.cy.nodes()
                .map(node => ({id: node.id(), ...node.position()}))
        """), positions)

    def test_graph_details_panel_is_persistent_and_resizable(self):
        self.page.goto(self.base + "graph.html")
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        panel = self.page.locator('.graph-inspector')
        expect(panel).to_be_visible()
        expect(self.page.locator('#graph-detail')).to_be_empty()
        divider = self.page.get_by_role('separator', name='Resize details panel')
        expect(divider).to_have_attribute('aria-valuenow', '320')
        before = self.page.evaluate("""() => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            return {pan: {...cy.pan()}, zoom: cy.zoom(),
                positions: cy.nodes().map(node => ({id: node.id(), ...node.position()}))};
        }""")
        box = divider.bounding_box()
        self.page.mouse.move(box['x'] + box['width'] / 2, box['y'] + 100)
        self.page.mouse.down()
        self.page.mouse.move(box['x'] + box['width'] / 2 - 160, box['y'] + 100)
        expect(divider).to_have_attribute('aria-valuenow', '480')
        after = self.page.evaluate("""() => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            return {pan: {...cy.pan()}, zoom: cy.zoom(),
                positions: cy.nodes().map(node => ({id: node.id(), ...node.position()}))};
        }""")
        self.assertAlmostEqual(after['pan']['x'], before['pan']['x'] - 80)
        self.assertAlmostEqual(after['pan']['y'], before['pan']['y'])
        self.assertEqual(after['zoom'], before['zoom'])
        self.assertEqual(after['positions'], before['positions'])
        self.page.mouse.up()
        divider.press('ArrowRight')
        expect(divider).to_have_attribute('aria-valuenow', '460')
        self.page.locator('#graph-search').fill('MIS')
        self.page.locator('#graph-search').press('Enter')
        self.page.get_by_role('button', name='Reset view').click()
        expect(panel).to_be_visible()
        expect(divider).to_have_attribute('aria-valuenow', '460')
        divider.press('Home')
        expect(divider).to_have_attribute('aria-valuenow', '240')
        divider.press('ArrowRight')
        expect(divider).to_have_attribute('aria-valuenow', '240')
        divider.press('End')
        expect(divider).to_have_attribute('aria-valuenow', divider.get_attribute('aria-valuemax'))

    def test_graph_hint_stays_top_right_when_panel_resizes(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#cy canvas').first).to_be_visible()
        divider = self.page.get_by_role('separator', name='Resize details panel')
        for key in ['Home', 'End']:
            divider.press(key)
            legend = self.page.locator('.graph-legend').bounding_box()
            note = self.page.locator('.graph-canvas-note').bounding_box()
            canvas = self.page.locator('.graph-canvas').bounding_box()
            self.assertLessEqual(note['y'] + note['height'], legend['y'])
            self.assertAlmostEqual(note['y'] - canvas['y'], 18)
            self.assertAlmostEqual(canvas['x'] + canvas['width'] - note['x'] - note['width'], 18)
            for box in [legend, note]:
                self.assertGreaterEqual(box['x'], canvas['x'])
                self.assertLessEqual(box['x'] + box['width'], canvas['x'] + canvas['width'])

    def test_graph_local_view_keeps_selection_and_neighbor_labels_readable(self):
        self.page.goto(self.base + "graph.html")
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.page.locator('[data-family="AcyclicPartition"]').click()
        expect(self.page.locator('#graph-detail > h2')).to_have_text('Acyclic Partition')
        self.assertTrue(self.page.evaluate("""() => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            const selected = cy.getElementById('AcyclicPartition');
            const nodes = selected.union(selected.neighborhood('node'));
            return selected.hasClass('selected') && !selected.hasClass('neighbor') &&
                nodes.every(node => node.visible() && node.style('label') === node.data('label') &&
                    Number(node.style('text-opacity')) === 1 &&
                    parseFloat(node.style('font-size')) * cy.zoom() >= 11.9) && cy.zoom() > 1;
        }"""))

    def test_graph_expanded_problem_keeps_a_compact_neighborhood(self):
        self.page.goto(self.base + "graph.html")
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.page.locator('[data-family="BinPacking"]').click()
        expect(self.page.locator('#graph-detail > h2')).to_have_text('Bin Packing')
        self.assertTrue(self.page.evaluate("""() => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            const family = cy.getElementById('BinPacking');
            const variants = family.children();
            const box = family.renderedBoundingBox();
            return variants.length === 2 && box.w < 500 && box.h < 350 &&
                variants.connectedEdges().every(edge => {
                    const a = edge.source().position(), b = edge.target().position();
                    return Math.hypot(a.x - b.x, a.y - b.y) < 450;
                });
        }"""))

    def test_graph_problem_selection_highlights_and_reveals_list_item(self):
        self.page.goto(self.base + "graph.html")
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.page.evaluate("""() => {
            document.querySelector('#cy')._cyreg.cy.getElementById('QUBO').emit('tap');
        }""")
        selected = self.page.locator('#browser-list [data-family="QUBO"]')
        expect(selected).to_have_attribute('aria-current', 'true')
        expect(selected).to_be_in_viewport()
        expect(selected).to_have_text('QUBO2 variants')
        self.page.evaluate("""() => {
            document.querySelector('#cy')._cyreg.cy.getElementById('QUBO').children().first().emit('tap');
        }""")
        variant = self.page.locator('#browser-list [data-variant][aria-current="true"]')
        expect(variant).to_be_in_viewport()
        expect(selected).to_have_attribute('aria-current', 'false')

    def test_atlas_ilp_prioritizes_documentation_over_metadata(self):
        self.visit('problem/ILP')
        expect(self.page.locator('article.typst-detail')).to_contain_text('Lenstra')
        expect(self.page.locator('.relation-title > span').filter(
            has_text='Expected Retrieval Cost')).to_have_text('Expected Retrieval Cost →')
        self.page.get_by_role('button', name='Connections').click()
        expect(self.page.get_by_role('region', name='Incoming reductions').locator('.relation-link')).to_have_count(4)
        expect(self.page.get_by_role('region', name='Outgoing reductions').locator('.relation-link')).to_have_count(0)
        expect(self.page.locator('#problem-connections')).not_to_contain_text('Default variant')
        self.page.get_by_role('button', name='Connections').click()
        expect(self.page.locator('#reference-detail a[href="./reductions.pdf"]')).to_have_count(0)
        expect(self.page.locator('.reading-aside').get_by_role('link', name='Open PDF reference')).to_have_count(1)
        expect(self.page.locator('.reference-tools [aria-expanded="true"]')).to_have_count(0)
        self.assertTrue(self.page.evaluate('''() =>
            document.querySelector('.reference-tools').getBoundingClientRect().bottom <
            document.querySelector('#reference-detail').getBoundingClientRect().top'''))
        self.page.get_by_role('button', name='Instance fields').click()
        expect(self.page.locator('.schema-table')).to_be_visible()
        self.page.get_by_role('button', name='CLI commands').click()
        expect(self.page.locator('.schema-table')).not_to_be_visible()
        expect(self.page.locator('#problem-commands')).to_contain_text('pred show ILP')
        self.page.get_by_role('button', name='CLI commands').click()
        expect(self.page.locator('#problem-commands')).not_to_be_visible()
        self.page.set_viewport_size({'width': 390, 'height': 900})
        self.assertTrue(self.page.evaluate('''() =>
            document.querySelector('.reading-aside').getBoundingClientRect().top >
            document.querySelector('#reference-detail').getBoundingClientRect().bottom'''))
        self.assertFalse(self.page.evaluate('document.documentElement.scrollWidth > innerWidth'))

    def test_graph_atlas_link_preserves_variant_and_documentation(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.page.locator('[data-family="MaximumIndependentSet"]').click()
        self.page.locator('[data-variant="MaximumIndependentSet/graph=SimpleGraph,weight=i64"]').click()
        expect(self.page.locator('.typst-detail')).to_contain_text('wireless network scheduling')
        content = self.page.locator('article.typst-detail').inner_html()
        self.page.get_by_role('link', name='Open in Atlas').click()
        expect(self.page.locator('#variant-select')).to_have_value('graph=SimpleGraph,weight=i64')
        expect(self.page.locator('.page-header > p')).to_have_count(0)
        expect(self.page.locator('.typst-detail')).to_contain_text('wireless network scheduling')
        self.assertEqual(self.page.locator('article.typst-detail').inner_html(), content)
        self.page.locator('.detail-references summary').click()
        expect(self.page.locator('.detail-references li').first).to_be_visible()

    def test_graph_rule_atlas_link_preserves_exact_endpoints(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.page.locator('[data-browse="rules"]').click()
        rule = self.page.evaluate('''() => REDUCTIONS.edges.findIndex(e =>
            REDUCTIONS.nodes[e.source].name === 'MaximumIndependentSet' &&
            REDUCTIONS.nodes[e.target].name === 'MaximumClique')''')
        self.page.locator(f'[data-rule="{rule}"]').click()
        expect(self.page.locator('article.typst-detail')).to_contain_text('Proof')
        content = self.page.locator('article.typst-detail').inner_html()
        endpoints = self.page.locator('.detail-variant').inner_text()
        self.page.get_by_role('link', name='Open in Atlas').click()
        expect(self.page.locator('article.typst-detail')).to_contain_text('Proof')
        self.assertEqual(self.page.locator('article.typst-detail').inner_html(), content)
        for endpoint in endpoints.split(' → '):
            expect(self.page.locator('.rule-endpoints')).to_contain_text(endpoint.replace(', ', ' · '))

    def test_graph_details_load_on_demand_and_reuse_family_content(self):
        requests = []
        self.page.on('request', lambda request: requests.append(request.url)
                     if '/assets/details/' in request.url else None)
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        expect(self.page.locator('[data-family="AcyclicPartition"] small')).to_have_count(0)
        expect(self.page.locator('[data-family="BinPacking"] small')).to_have_text('2 variants')
        self.assertEqual(requests, [])
        self.page.locator('[data-family="MaximumIndependentSet"]').click()
        expect(self.page.locator('.typst-detail')).to_contain_text('wireless network scheduling')
        self.assertEqual(len(requests), 1)
        self.page.locator('[data-variant="MaximumIndependentSet/graph=SimpleGraph,weight=i64"]').click()
        expect(self.page.locator('.typst-detail')).to_contain_text('wireless network scheduling')
        self.assertEqual(len(requests), 1)
        self.page.locator('.detail-references summary').click()
        expect(self.page.locator('.detail-references li').first).to_be_visible()
        self.assertEqual(len(requests), 2)
        self.assertTrue(self.page.locator('.typst-detail').evaluate('''article =>
            [...article.querySelectorAll('use')].every(use =>
                document.getElementById((use.getAttribute('href') || use.getAttribute('xlink:href')).slice(1)))'''))
        self.page.locator('[data-browse="rules"]').click()
        expect(self.page.locator('#browser-list')).not_to_contain_text('Default variant')

    def test_graph_detail_errors_are_visible_and_retryable(self):
        self.page.route('**/assets/details/problem/BinPacking.html', lambda route: route.fulfill(status=503))
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.page.locator('[data-family="BinPacking"]').click()
        expect(self.page.locator('.typst-detail[role="alert"]')).to_contain_text('503')
        self.page.unroute('**/assets/details/problem/BinPacking.html')
        self.page.locator('[data-variant="BinPacking/weight=i64"]').click()
        expect(self.page.locator('.typst-detail')).to_contain_text('Definition')
        expect(self.page.locator('.typst-detail[role="alert"]')).to_have_count(0)

    def test_graph_details_inherit_typst_content_without_changing_variant(self):
        self.page.goto(self.base + "graph.html")
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.page.locator('[data-family="MaximumIndependentSet"]').click()
        variant = 'MaximumIndependentSet/graph=SimpleGraph,weight=i64'
        self.page.locator(f'[data-variant="{variant}"]').click()
        article = self.page.locator('.typst-detail')
        expect(article).to_have_attribute('data-content-key', 'problem:MaximumIndependentSet')
        expect(article).to_contain_text('wireless network scheduling')
        expect(article.locator('figure > svg')).to_have_count(1)
        expect(self.page.locator('.detail-variant')).to_have_text('graph: SimpleGraph, weight: i64')
        expect(self.page.locator(f'[data-variant="{variant}"]')).to_have_attribute('aria-current', 'true')
        self.assertTrue(article.locator('p').first.locator('.typst-math svg').count() > 0)
        article.locator('a[role="doc-biblioref"]').first.click()
        expect(self.page.locator('.detail-references')).to_have_attribute('open', '')
        self.assertTrue(self.page.locator('.graph-inspector').evaluate(
            '(panel) => panel.scrollWidth <= panel.clientWidth'))

        # A specialized entry takes precedence over the shared family text.
        self.page.route('**/weighted-detail.html', lambda route: route.fulfill(
            content_type='text/html', body='<p>Weighted simple-graph documentation.</p>'))
        self.page.evaluate("""key => {
            window.GRAPH_DETAILS.entries['problem:' + key] = './weighted-detail.html';
            document.querySelector('#cy')._cyreg.cy.getElementById(key).emit('tap');
        }""", variant)
        expect(article).to_have_attribute('data-content-key', 'problem:' + variant)
        expect(article).to_have_text('Weighted simple-graph documentation.')

        self.page.locator('[data-browse="rules"]').click()
        rule = self.page.evaluate("""() => REDUCTIONS.edges.findIndex(rule =>
            REDUCTIONS.nodes[rule.source].name === 'MaximumIndependentSet' &&
            REDUCTIONS.nodes[rule.target].name === 'MaximumClique')""")
        self.page.locator(f'[data-rule="{rule}"]').click()
        expect(article).to_have_attribute('data-content-key', 'rule:MaximumIndependentSet->MaximumClique')
        expect(article).to_contain_text('Solution extraction.')
        expect(article).to_contain_text('pred reduce mis.json')
        expect(self.page.locator('.detail-source')).to_have_count(0)
        self.page.get_by_role('button', name='Reset view').click()
        expect(self.page.locator('#graph-detail')).to_be_empty()

    def test_graph_rule_selection_highlights_matching_list_items(self):
        self.page.goto(self.base + "graph.html")
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        expected = self.page.evaluate("""() => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            const edge = cy.edges().filter(edge => edge.visible()).last();
            const matches = REDUCTIONS.edges.flatMap((rule, index) =>
                REDUCTIONS.nodes[rule.source].name === edge.source().id() &&
                REDUCTIONS.nodes[rule.target].name === edge.target().id() ? [String(index)] : []);
            edge.emit('tap');
            return matches;
        }""")
        selected = self.page.locator('#browser-list [data-rule][aria-current="true"]')
        self.assertEqual(selected.evaluate_all('(items) => items.map(item => item.dataset.rule)'), expected)
        expect(selected.first).to_be_in_viewport()
        self.assertNotEqual(selected.first.evaluate('(item) => getComputedStyle(item).backgroundColor'),
                            self.page.locator('#browser-list [aria-current="false"]').first.evaluate(
                                '(item) => getComputedStyle(item).backgroundColor'))

    def test_graph_browser_lists_real_problems_variants_and_rules(self):
        self.page.goto(self.base + "graph.html")
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        expected = self.page.evaluate("""() => ({
            families: new Set(REDUCTIONS.nodes.map(node => node.name)).size,
            rules: REDUCTIONS.edges.length,
            rule: REDUCTIONS.edges.findIndex(rule =>
                REDUCTIONS.nodes[rule.source].name === 'MaximumIndependentSet' &&
                REDUCTIONS.nodes[rule.target].name === 'MaximumClique')
        })""")
        expect(self.page.locator('#browser-list [data-family]')).to_have_count(expected['families'])
        self.page.locator('#browser-search').fill('MIS')
        expect(self.page.locator('#browser-list [data-family="MaximumIndependentSet"]')).to_be_visible()
        self.page.locator('#browser-list [data-family="MaximumIndependentSet"]').click()
        expect(self.page.locator('#browser-list [data-variant]')).to_have_count(8)
        variant = 'MaximumIndependentSet/graph=SimpleGraph,weight=i64'
        self.page.locator(f'#browser-list [data-variant="{variant}"]').click()
        expect(self.page.locator(f'#browser-list [data-variant="{variant}"]')).to_have_attribute('aria-current', 'true')
        viewport = self.page.evaluate("""() => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            return {zoom: cy.zoom(), pan: cy.pan()};
        }""")
        self.page.locator('[data-browse="rules"]').click()
        expect(self.page.locator('#browser-list [data-rule]')).to_have_count(expected['rules'])
        self.assertEqual(self.page.evaluate("""() => {
            const cy = document.querySelector('#cy')._cyreg.cy;
            return {zoom: cy.zoom(), pan: cy.pan()};
        }"""), viewport)
        self.page.locator(f'[data-rule="{expected["rule"]}"]').click()
        self.assertTrue(self.page.evaluate("""index => {
            const rule = REDUCTIONS.edges[index];
            const key = node => node.name + '/' + Object.entries(node.variant)
                .sort(([a], [b]) => a.localeCompare(b)).map(([k,v]) => `${k}=${v}`).join(',');
            const edge = document.querySelector('#cy')._cyreg.cy.edges('.selected');
            return edge.length === 1 && edge.source().id() === key(REDUCTIONS.nodes[rule.source]) &&
                edge.target().id() === key(REDUCTIONS.nodes[rule.target]);
        }""", expected['rule']))
        same_family_rule = self.page.evaluate("""() => REDUCTIONS.edges.findIndex(rule =>
            REDUCTIONS.nodes[rule.source].name === 'MaximumIndependentSet' &&
            REDUCTIONS.nodes[rule.target].name === 'MaximumIndependentSet')""")
        self.page.locator(f'[data-rule="{same_family_rule}"]').click()
        self.assertTrue(self.page.evaluate("""() => {
            const edge = document.querySelector('#cy')._cyreg.cy.edges('.selected');
            return edge.length === 1 && edge.source().id() !== edge.target().id() &&
                edge.source().parent().id() === 'MaximumIndependentSet' &&
                edge.target().parent().id() === 'MaximumIndependentSet';
        }"""))
        expect(self.page.locator('.typst-detail')).to_contain_text('Proof')
        self.page.locator('#browser-search').fill('no-such-reduction')
        expect(self.page.locator('.browser-empty')).to_be_visible()

    def test_graph_overview_is_stable_and_labels_do_not_overlap(self):
        self.page.goto(self.base + "graph.html")
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        positions = self.page.evaluate("""() =>
            document.querySelector('#cy')._cyreg.cy.nodes()
                .map(node => ({id: node.id(), ...node.position()}))
        """)
        self.page.reload()
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.assertEqual(self.page.evaluate("""() =>
            document.querySelector('#cy')._cyreg.cy.nodes()
                .map(node => ({id: node.id(), ...node.position()}))
        """), positions)
        self.page.wait_for_function("""() =>
            document.querySelector('#cy')._cyreg.cy.nodes('.named').length > 0
        """)
        self.assertTrue(self.page.evaluate("""() => {
            const labels = document.querySelector('#cy')._cyreg.cy.nodes('.named')
                .map(node => node.renderedBoundingBox({includeNodes: false, includeLabels: true}));
            return labels.every((a, i) => labels.slice(i + 1).every(b =>
                a.x2 <= b.x1 || b.x2 <= a.x1 || a.y2 <= b.y1 || b.y2 <= a.y1));
        }"""))


def responsive_check(width):
    def test(self):
        self.page.set_viewport_size({"width": width, "height": 900})
        for route in ["home", "atlas", "problem/MaximumIndependentSet"]:
            with self.subTest(route=route):
                self.visit(route)
                self.assertFalse(self.page.evaluate(
                    "document.documentElement.scrollWidth > innerWidth"
                ), f"Horizontal overflow at {width}px on {route}")
        self.page.goto(self.base + "graph.html")
        expect(self.page.locator("#cy canvas").first).to_be_visible()
        self.assertFalse(self.page.evaluate(
            "document.documentElement.scrollWidth > innerWidth"
        ), f"Horizontal overflow at {width}px on graph.html")
        self.page.locator("#graph-search").fill("MIS")
        self.page.locator("#graph-search").press("Enter")
        self.assertFalse(self.page.evaluate(
            "document.documentElement.scrollWidth > innerWidth"
        ), f"Horizontal overflow at {width}px with graph details open")
    return test


for viewport in [320, 390, 768, 1440]:
    setattr(WebsiteTests, f"test_responsive_{viewport}", responsive_check(viewport))


if __name__ == "__main__":
    unittest.main(verbosity=2)

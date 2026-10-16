#!/usr/bin/env python3
"""Browser checks against a built, served website (see docs/website/README.md)."""

import os
import re
import json
from pathlib import Path
import subprocess
import sys
import unittest
import xml.etree.ElementTree as ET
from urllib.parse import urljoin

from playwright.sync_api import sync_playwright, expect


def require_open_questions(request, base, *, required):
    response = request.get(urljoin(base, 'open-questions/registry.json'))
    if response.status == 404 and not required:
        raise unittest.SkipTest('Open Question Board absent; include its build with --open-questions')
    if response.status != 200:
        raise AssertionError(f'Open Question Board registry is required (HTTP {response.status})')


class WebsiteTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.playwright = sync_playwright().start()
        engine = os.environ.get('WEBSITE_BROWSER', 'chromium')
        if engine not in ['chromium', 'firefox', 'webkit']:
            raise ValueError('WEBSITE_BROWSER must be chromium, firefox or webkit')
        options = {'channel': os.environ.get('WEBSITE_BROWSER_CHANNEL') or None} if engine == 'chromium' else {}
        if engine == 'firefox':
            options['firefox_user_prefs'] = {'network.proxy.type': 0}
        cls.browser = getattr(cls.playwright, engine).launch(**options)
        print('Website browser:', engine, cls.browser.version, flush=True)
        cls.base = os.environ.get("WEBSITE_BASE_URL", "http://127.0.0.1:3001/")

    @classmethod
    def tearDownClass(cls):
        cls.browser.close()
        cls.playwright.stop()

    def setUp(self):
        self.context = self.browser.new_context(
            viewport={"width": 1440, "height": 1000},
            reduced_motion="reduce",
            permissions=["clipboard-read", "clipboard-write"] if self.browser.browser_type.name == 'chromium' else [],
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

    def read_clipboard(self):
        if self.browser.browser_type.name != 'webkit':
            return self.page.evaluate('navigator.clipboard.readText()')
        # WebKit denies script reads here; verify the real clipboard with a native paste.
        active = self.page.evaluate_handle('document.activeElement').as_element()
        editor = self.page.evaluate_handle('''() => {
            const editor = document.createElement('textarea');
            editor.style = 'position:fixed;top:0;left:0;z-index:99999';
            document.body.append(editor); editor.focus(); return editor;
        }''').as_element()
        try:
            self.page.keyboard.press('Meta+V' if sys.platform == 'darwin' else 'Control+V')
            self.page.wait_for_function('node => node.value.length > 0', arg=editor, timeout=1000)
            return editor.input_value()
        finally:
            editor.evaluate('node => node.remove()')
            if active and active.evaluate('node => node.isConnected'):
                active.focus()

    def assert_accessible(self, state):
        source = Path(__file__).resolve().parents[1] / 'node_modules/axe-core/axe.min.js'
        self.page.evaluate(source.read_text())
        report = self.page.evaluate('''async () => {
            const result = await axe.run(document);
            const summarize = (rules) => rules.map(rule => ({id: rule.id, impact: rule.impact,
                help_url: rule.helpUrl, nodes: rule.nodes.map(node => ({target: node.target, reason: node.failureSummary}))}));
            return {engine: result.testEngine, environment: result.testEnvironment,
                violations: summarize(result.violations), needs_review: summarize(result.incomplete)};
        }''')
        self.assertEqual(report['engine']['version'], '4.14.0')
        print('Accessibility:', json.dumps({'state': state, **report}), flush=True)
        blocking = [rule for rule in report['violations']
                    if rule['impact'] in ['serious', 'critical'] or rule['id'] in
                    ['aria-allowed-role', 'heading-order', 'landmark-no-duplicate-banner', 'landmark-unique']]
        self.assertEqual(blocking, [], json.dumps(blocking, indent=2))

    def test_accessibility_atlas(self):
        self.visit('atlas')
        expect(self.page.locator('.atlas-row').first).to_be_visible()
        self.assert_accessible('atlas')

    def test_accessibility_atlas_details(self):
        self.visit('problem/MaximumIndependentSet')
        expect(self.page.locator('.typst-detail')).to_contain_text('wireless network scheduling')
        with self.subTest('named problem tools'):
            expect(self.page.get_by_role('group', name='Problem tools', exact=True)).to_be_visible(timeout=500)
        with self.subTest('article follows the page heading'):
            expect(self.page.get_by_role('heading', name='Definition', level=2, exact=True)).to_be_visible(timeout=500)
        self.assert_accessible('atlas-details')

    def test_problem_code_block_is_reachable_and_scrolls_with_keyboard(self):
        self.visit('problem/MaximumIndependentSet')
        expect(self.page.locator('.typst-detail')).to_contain_text('wireless network scheduling')
        code = self.page.locator('.typst-detail pre').first
        self.page.locator('#main').focus()
        for _ in range(40):
            self.page.keyboard.press('Tab')
            if code.evaluate('(node) => node === document.activeElement'):
                break
        else:
            self.fail('Scrollable code could not be reached with Tab')
        self.page.evaluate('() => document.fonts.ready')
        self.assertGreater(code.evaluate('(node) => node.scrollHeight - node.clientHeight'), 0)
        before = code.evaluate('(node) => node.scrollTop')
        self.page.keyboard.press('ArrowUp' if before > 0 else 'ArrowDown', delay=100)
        self.page.wait_for_function('({node, before}) => node.scrollTop !== before',
                                    arg={'node': code.element_handle(), 'before': before}, timeout=500)

    def test_accessibility_atlas_empty(self):
        self.visit('atlas')
        self.page.locator('#atlas-search').fill('NoSuchRegisteredProblem')
        expect(self.page.locator('.empty-state')).to_be_visible()
        self.assert_accessible('atlas-empty')

    def test_accessibility_graph(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.assert_accessible('graph')

    def test_accessibility_graph_empty_suggestions(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.page.locator('#source').fill('NoSuchRegisteredProblem')
        expect(self.page.locator('#suggest')).to_contain_text('No matching problem')
        self.assert_accessible('graph-empty-suggestions')

    def test_accessibility_graph_popover(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.page.locator('.scene-node[data-node="Satisfiability"]').focus()
        self.page.keyboard.press('Enter')
        self.assert_accessible('graph-popover')

    def test_accessibility_graph_desktop_drawer(self):
        self.page.goto(self.base + 'graph.html')
        self.page.locator('.steps button').first.click()
        expect(self.page.locator('#drawer-body article')).to_contain_text('Definition')
        expect(self.page.get_by_role('banner')).to_have_count(1, timeout=500)
        self.assert_accessible('graph-desktop-drawer')

    def test_accessibility_graph_mobile_drawer(self):
        self.page.set_viewport_size({'width': 390, 'height': 844})
        self.page.goto(self.base + 'graph.html')
        self.page.locator('.steps button').first.click()
        expect(self.page.locator('#drawer-body article')).to_contain_text('Definition')
        self.assert_accessible('graph-mobile-drawer')

    def test_graph_combobox_clears_active_option_on_close_and_empty_results(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        source = self.page.locator('#source')
        source.fill('MIS')
        expect(source).to_have_attribute('aria-activedescendant', 'opt-0')
        source.press('Escape')
        self.assertIsNone(source.get_attribute('aria-activedescendant'))
        source.fill('NoSuchRegisteredProblem')
        self.assertIsNone(source.get_attribute('aria-activedescendant'))
        expect(self.page.get_by_role('status')).to_contain_text('No matching problem', timeout=500)
        expect(self.page.locator('#suggest [role="option"]')).to_have_attribute('aria-disabled', 'true', timeout=500)

    def test_graph_endpoint_variant_repaint_preserves_keyboard_focus(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.choose_endpoint('source', 'MIS')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        weight = self.page.locator('#source-options [aria-label="weight"]').get_by_role('button', name='i64', exact=True)
        weight.focus()
        weight.press('Enter')
        expect(weight).to_have_attribute('aria-pressed', 'true')
        expect(weight).to_be_focused(timeout=500)

    def test_graph_popover_metadata_has_readable_contrast(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.page.locator('.scene-node[data-node="MaximumIndependentSet"]').focus()
        self.page.keyboard.press('Enter')
        expect(self.page.locator('#pop')).to_be_visible()
        samples = self.page.locator('#pop .opt-key').evaluate_all('''nodes => nodes.map(node => {
            const canvas = document.createElement('canvas'); canvas.width = canvas.height = 1;
            const ctx = canvas.getContext('2d');
            const rgba = color => { ctx.clearRect(0, 0, 1, 1); ctx.fillStyle = color;
                ctx.fillRect(0, 0, 1, 1); return Array.from(ctx.getImageData(0, 0, 1, 1).data); };
            return {text: node.textContent, foreground: rgba(getComputedStyle(node).color),
                background: rgba(getComputedStyle(node.closest('.pop')).backgroundColor)};
        })''')
        self.assertGreater(len(samples), 0)
        def luminance(rgba):
            self.assertEqual(rgba[3], 255, 'This contrast fixture requires opaque colors')
            channels = [value / 255 for value in rgba[:3]]
            linear = [value / 12.92 if value <= 0.04045 else ((value + 0.055) / 1.055) ** 2.4
                      for value in channels]
            return sum(weight * value for weight, value in zip([0.2126, 0.7152, 0.0722], linear))
        for sample in samples:
            high, low = sorted([luminance(sample['foreground']), luminance(sample['background'])], reverse=True)
            self.assertGreaterEqual((high + 0.05) / (low + 0.05), 4.5, sample['text'])

    def test_graph_footnotes_have_link_roles_and_restore_reference_focus(self):
        self.page.goto(self.base + 'graph.html')
        self.page.locator('.steps button').first.click()
        reference = self.page.locator('#drawer-body article a[role="doc-noteref"]').first
        expect(reference).to_be_visible(timeout=500)
        origin = reference.locator('..').get_attribute('id')
        target = reference.get_attribute('href')[1:]
        note = self.page.locator(f'#drawer-body [id="{target}"]')
        backlink = note.locator(f'a[role="doc-backlink"][href="#{origin}"]')
        expect(backlink).to_be_visible(timeout=500)
        reference.click()
        expect(note).to_be_focused(timeout=500)
        backlink.click()
        expect(reference).to_be_focused(timeout=500)

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

    def test_home_navigation_opens_question_board(self):
        require_open_questions(self.page.request, self.base,
                               required=bool(os.environ.get('CI')) or os.environ.get('WEBSITE_REQUIRE_OPEN_QUESTIONS') == '1')
        self.visit()
        tab = self.page.get_by_role('navigation', name='Main navigation').get_by_role('link', name='Open questions', exact=True)
        tab.click()
        expect(self.page).to_have_url(self.base + 'open-questions/')
        expect(self.page.get_by_role('heading', name='Open Question Zoo', exact=True)).to_be_visible()
        self.page.get_by_role('link', name='All questions', exact=False).click()
        self.page.get_by_role('searchbox', name='Search the collection').fill('Q001')
        expect(self.page.locator('.question-item:visible')).to_have_count(1)
        expect(self.page.locator('.question-item:visible .question-id')).to_have_text('Q001')
        self.page.locator('.question-item:visible .question-link').click()
        self.page.get_by_role('link', name='Submit a solution', exact=True).click()
        expect(self.page.get_by_label('Registry entry', exact=True)).to_have_value('Q001')
        registry = self.page.request.get(self.base + 'open-questions/registry.json').json()
        question = next(entry for entry in registry['questions'] if entry['id'] == 'Q001')
        prompt = self.page.get_by_role('textbox', name='Agent prompt').input_value()
        self.assertIn(f"registry/{question['slug']}.json", prompt)
        self.assertIn('https://github.com/GiggleLiu/autoresearch-gadgets', prompt)
        self.page.get_by_role('button', name='Copy prompt', exact=True).click()
        self.assertEqual(self.read_clipboard(), prompt)

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
        self.assertFalse(any('atlas-data.' in url or '/problem-data/' in url for url in requests))
        self.page.locator('#atlas-search').fill('MIS')
        self.page.locator('.atlas-row').click()
        expect(self.page.locator('.typst-detail')).to_contain_text('wireless network scheduling')
        self.assertEqual(sum('/problem-data/MaximumIndependentSet.json' in url for url in requests), 1)
        self.assertEqual(sum('graph-details.json' in url for url in requests), 1)
        self.assertTrue(self.page.evaluate('''() => {
            const resources = performance.getEntriesByType('resource');
            const index = resources.find(r => r.name.includes('graph-details.json'));
            const record = resources.find(r => r.name.includes('/problem-data/'));
            return index.startTime < record.responseEnd;
        }'''))
        requests.clear()
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#cy svg.graph-scene').first).to_be_visible()
        self.assertFalse(any('layout-bundle' in url or 'graph-details.' in url for url in requests))
        self.assertFalse(self.page.evaluate('Boolean(window.GRAPH_DETAILS)'))
        self.open_problem_popover('BinPacking')
        self.page.get_by_role('button', name='Details', exact=True).click()
        expect(self.page.locator('.typst-detail')).to_contain_text('Definition')
        self.assertFalse(any('layout-bundle' in url for url in requests))
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
        expect(self.page.locator("#cy svg.graph-scene").first).to_be_visible()
        for selector, size in [('.brand', 22), ('.end input', 16), ('.path .chain', 14)]:
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
        self.assertEqual(self.read_clipboard(),
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

    def test_legacy_reduction_graph_url_redirects(self):
        self.page.goto(self.base + 'reduction-graph.html')
        self.page.wait_for_url(self.base + 'graph.html')
        expect(self.page.locator("#cy svg.graph-scene").first).to_be_visible()

    def test_legacy_question_board_url_redirects(self):
        require_open_questions(self.page.request, self.base,
                               required=bool(os.environ.get('CI')) or os.environ.get('WEBSITE_REQUIRE_OPEN_QUESTIONS') == '1')
        self.page.goto(self.base + 'open-problems.html')
        self.page.wait_for_url(self.base + 'open-questions/')
        expect(self.page.get_by_role('heading', name='Open Question Zoo', exact=True)).to_be_visible()

    def test_legacy_open_questions_hash_opens_question_board(self):
        require_open_questions(self.page.request, self.base,
                               required=bool(os.environ.get('CI')) or os.environ.get('WEBSITE_REQUIRE_OPEN_QUESTIONS') == '1')
        self.visit('open-questions')
        expect(self.page).to_have_url(self.base + 'open-questions/')
        expect(self.page.get_by_role('heading', name='Open Question Zoo', exact=True)).to_be_visible()

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
        logo = self.page.locator('.docs-brand img')
        response = self.page.request.get(urljoin(self.page.url, logo.get_attribute('src')))
        self.assertEqual(response.status, 200)
        viewbox = ET.fromstring(response.text()).attrib['viewBox'].split()
        ratio = float(viewbox[2]) / float(viewbox[3])
        for width in [150, 420]:
            box = handle.bounding_box()
            self.page.mouse.move(box['x'] + box['width'] / 2, 100)
            self.page.mouse.down()
            self.page.mouse.move(width, 100, steps=8)
            self.page.mouse.up()
            bounds = logo.bounding_box()
            self.assertLessEqual(bounds['width'], 204)
            self.assertLessEqual(bounds['x'] + bounds['width'], sidebar.bounding_box()['width'] - 17)
            self.assertAlmostEqual(bounds['width'] / bounds['height'], ratio, delta=0.02)
            expect(self.page.locator('.docs-brand')).to_have_css('height', '78px')

    def test_atlas_ilp_prioritizes_documentation_over_metadata(self):
        self.visit('problem/ILP?variant=bounds=general,coefficient=f64,variable=bool')
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

    def choose_endpoint(self, side, query):
        self.page.locator('#' + side).fill(query)
        self.page.locator('#' + side).press('Enter')

    def open_problem_popover(self, name):
        # Initial path selection moves the camera; wait before positioning the node for a click.
        self.page.wait_for_function("() => !!document.querySelector('.path') && !document.querySelector('.size.pending')")
        self.page.get_by_role('button', name='Show whole graph', exact=True).click()
        self.page.locator(f'[data-node="{name}"] .node-disc').click()
        expect(self.page.locator('#pop')).to_be_visible()

    def copy_route(self):
        self.page.locator('.copy').click()
        return json.loads(self.read_clipboard())

    def observe_path_worker(self):
        # Observe real worker messages without replacing or delaying the engine.
        self.page.add_init_script("""(() => {
            const NativeWorker = window.Worker;
            window.pathMessages = [];
            window.pathWorkerStarts = 0;
            window.pathWorkers = [];
            window.Worker = class extends NativeWorker {
                constructor(...args) {
                    super(...args);
                    window.pathWorkerStarts++;
                    window.pathWorkers.push(this);
                }
                set onmessage(handler) {
                    super.onmessage = handler;
                    if (this.observing) return;
                    this.observing = true;
                    // Register after the application's handler, including eagerly started workers.
                    this.addEventListener('message', e => {
                        window.pathMessages.push(e.data);
                        window.onPathMessage?.(e.data);
                    });
                }
                get onmessage() { return super.onmessage; }
            };
        })()""")

    def test_graph_brand_returns_home(self):
        self.visit()
        self.page.get_by_role('link', name='Graph', exact=True).click()
        expect(self.page.locator('#cy svg.graph-scene').first).to_be_visible()
        self.page.get_by_role('link', name='Reduction paths', exact=True).click()
        expect(self.page.locator('main .hero')).to_be_visible()

    def test_graph_pan_zoom_and_node_drag_preserve_route(self):
        self.page.goto(self.base + 'graph.html')
        route = self.copy_route()
        self.page.get_by_role('button', name='Show whole graph', exact=True).click()
        node = self.page.locator('[data-node="Satisfiability"] .node-disc')
        center = lambda: node.evaluate('e => {const r=e.getBoundingClientRect(); return {x:r.x+r.width/2,y:r.y+r.height/2,r:r.width/2}}')
        before = center()
        self.page.mouse.move(1300, 930)
        self.page.mouse.down()
        self.page.mouse.move(1330, 950, steps=3)
        self.page.mouse.up()
        moved = center()
        self.assertAlmostEqual(moved['x'] - before['x'], 30, delta=1)
        self.assertAlmostEqual(moved['y'] - before['y'], 20, delta=1)
        self.page.get_by_role('button', name='Zoom in', exact=True).click()
        zoomed = center()
        self.assertAlmostEqual(zoomed['r'] / moved['r'], 1.3, delta=.02)
        self.page.mouse.move(zoomed['x'], zoomed['y'])
        self.page.mouse.down()
        self.page.mouse.move(zoomed['x'] + 50, zoomed['y'] + 40, steps=5)
        self.page.mouse.up()
        dragged = center()
        self.assertAlmostEqual(dragged['x'] - zoomed['x'], 50, delta=1)
        self.assertAlmostEqual(dragged['y'] - zoomed['y'], 40, delta=1)
        self.assertEqual(self.copy_route(), route)

    def test_graph_pan_buttons_move_view_without_changing_route(self):
        self.page.goto(self.base + 'graph.html')
        route = self.copy_route()
        world = self.page.locator('svg.graph-scene > g')
        matrix = lambda: world.evaluate('e => {const m=e.getCTM(); return {z:m.a,x:m.e,y:m.f}}')
        node = self.page.locator('.scene-node[data-node="Satisfiability"]')
        position = node.get_attribute('transform')
        for direction, dx, dy in [('left', -80, 0), ('up', 0, -80), ('right', 80, 0), ('down', 0, 80)]:
            before = matrix()
            button = self.page.get_by_role('button', name=f'Pan graph {direction}', exact=True)
            expect(button).to_be_visible(timeout=500)
            button.click()
            after = matrix()
            self.assertAlmostEqual(after['x'] - before['x'], dx, delta=1)
            self.assertAlmostEqual(after['y'] - before['y'], dy, delta=1)
            self.assertEqual(after['z'], before['z'])
            self.assertEqual(node.get_attribute('transform'), position)
        self.assertEqual(self.copy_route(), route)

    def test_graph_move_buttons_reposition_node_and_connected_edge(self):
        self.page.goto(self.base + 'graph.html')
        route = self.copy_route()
        self.open_problem_popover('Satisfiability')
        node = self.page.locator('[data-node="Satisfiability"] .node-disc')
        other = self.page.locator('[data-node="QUBO"] .node-disc')
        center = lambda item: item.evaluate('e => {const r=e.getBoundingClientRect(); return {x:r.x+r.width/2,y:r.y+r.height/2}}')
        edge = self.page.locator('.scene-edge.route[data-edge^="Satisfiability→"]').first
        self.assertEqual(edge.count(), 1)
        other_before = center(other)
        for direction, dx, dy in [('left', -40, 0), ('up', 0, -40), ('right', 40, 0), ('down', 0, 40)]:
            before = center(node)
            button = self.page.get_by_role('button', name=f'Move problem {direction}', exact=True)
            expect(button).to_be_visible(timeout=500)
            button.focus()
            button.press('Enter')
            expect(button).to_be_focused(timeout=500)
            after = center(node)
            self.assertAlmostEqual(after['x'] - before['x'], dx, delta=1)
            self.assertAlmostEqual(after['y'] - before['y'], dy, delta=1)
            endpoint = edge.evaluate('e => {const m=e.getCTM(); return {x:e.x1.baseVal.value*m.a+m.e,y:e.y1.baseVal.value*m.d+m.f}}')
            self.assertAlmostEqual(endpoint['x'], after['x'], delta=1)
            self.assertAlmostEqual(endpoint['y'], after['y'], delta=1)
            self.assertEqual(center(other), other_before)
        self.page.keyboard.press('Escape')
        self.assertEqual(self.copy_route(), route)

    def test_graph_mobile_navigation_controls_are_visible_and_operable(self):
        self.context.close()
        self.context = self.browser.new_context(viewport={'width': 320, 'height': 640},
                                                has_touch=True, reduced_motion='reduce')
        self.page = self.context.new_page()
        self.page.set_default_timeout(3000)
        self.page.on('pageerror', lambda error: self.errors.append(str(error)))
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        controls = self.page.get_by_role('navigation', name='Graph navigation', exact=True)
        expect(controls).to_be_visible(timeout=500)
        panel = self.page.locator('.panel').bounding_box()
        bounds = controls.bounding_box()
        self.assertGreaterEqual(bounds['x'], 0)
        self.assertLessEqual(bounds['x'] + bounds['width'], 320)
        self.assertLessEqual(bounds['y'] + bounds['height'], panel['y'])
        world = self.page.locator('svg.graph-scene > g')
        zoom = lambda: world.evaluate('e=>e.getCTM().a')
        before = zoom()
        controls.get_by_role('button', name='Zoom in', exact=True).tap()
        self.assertAlmostEqual(zoom() / before, 1.3, delta=.02)
        controls.get_by_role('button', name='Zoom out', exact=True).tap()
        self.assertAlmostEqual(zoom(), before, delta=.001)
        before_pan = world.evaluate('e=>e.getCTM().e')
        controls.get_by_role('button', name='Pan graph left', exact=True).tap()
        self.assertAlmostEqual(world.evaluate('e=>e.getCTM().e') - before_pan, -80, delta=1)
        panned = world.get_attribute('transform')
        controls.get_by_role('button', name='Show whole graph', exact=True).tap()
        self.assertNotEqual(world.get_attribute('transform'), panned)

    def test_graph_small_mobile_popover_controls_remain_reachable(self):
        self.page.set_viewport_size({'width': 320, 'height': 640})
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.page.locator('.scene-node[data-node="MaximumIndependentSet"]').focus()
        self.page.keyboard.press('Enter')
        move = self.page.get_by_role('button', name='Move problem down', exact=True)
        expect(move).to_have_count(1, timeout=500)
        details = self.page.locator('#pop .pop-details')
        for _ in range(20):
            if details.evaluate('e=>e===document.activeElement'):
                break
            self.page.keyboard.press('Tab')
        else:
            self.fail('Popover details cannot be reached from its variant and movement controls')
        bounds = self.page.locator('#pop').bounding_box()
        header = self.page.locator('.site-header').bounding_box()
        panel = self.page.locator('.panel').bounding_box()
        self.assertGreaterEqual(bounds['y'], header['y'] + header['height'])
        self.assertLessEqual(bounds['y'] + bounds['height'], panel['y'])
        expect(details).to_be_in_viewport()
        details.press('Enter')
        expect(self.page.locator('#drawer-body article')).to_contain_text('Definition')

    def test_graph_touch_pinch_keeps_center_and_route(self):
        if self.browser.browser_type.name != 'chromium':
            self.skipTest('Native multipoint injection requires Chromium CDP; physical mobile pinch acceptance remains required')
        self.page.goto(self.base + 'graph.html')
        route = self.copy_route()
        matrix = lambda: self.page.locator('svg.graph-scene > g').evaluate('e => {const m=e.getCTM(); return {z:m.a,x:m.e,y:m.f}}')
        before = matrix()
        session = self.context.new_cdp_session(self.page)
        session.send('Emulation.setTouchEmulationEnabled', {'enabled': True, 'maxTouchPoints': 2})
        def touch(kind, points):
            session.send('Input.dispatchTouchEvent', {'type': kind, 'touchPoints': [
                {'x': x, 'y': y, 'id': i} for i, (x, y) in enumerate(points)]})
        touch('touchStart', [(700, 400), (900, 400)])
        touch('touchMove', [(650, 400), (950, 400)])
        touch('touchEnd', [])
        after = matrix()
        self.assertAlmostEqual(after['z']/before['z'], 1.5, delta=.02)
        self.assertAlmostEqual((800-after['x'])/after['z'], (800-before['x'])/before['z'], delta=1)
        self.assertAlmostEqual((400-after['y'])/after['z'], (400-before['y'])/before['z'], delta=1)
        touch('touchStart', [(1000, 800)])
        touch('touchMove', [(1030, 810)])
        touch('touchCancel', [])
        panned = matrix()
        self.assertAlmostEqual(panned['x']-after['x'], 30, delta=1)
        self.assertAlmostEqual(panned['y']-after['y'], 10, delta=1)
        self.assertEqual(self.copy_route(), route)

    def test_graph_animated_frame_fits_route_and_wheel_zoom_keeps_cursor_anchor(self):
        self.page.emulate_media(reduced_motion='no-preference')
        self.page.goto(self.base + 'graph.html')
        self.page.wait_for_function('''() => {
            const done = performance.getEntriesByName('graph:comparison-rendered').at(-1);
            const camera = performance.getEntriesByName('graph:camera-settled').at(-1);
            return done && camera && camera.startTime >= done.startTime;
        }''')
        self.assertTrue(self.page.locator('.scene-node.route .node-disc').evaluate_all('''nodes => nodes.every(n => {
            const r=n.getBoundingClientRect(); return r.left >= 392 && r.right <= innerWidth && r.top >= 85 && r.bottom <= innerHeight;
        })'''))
        route = self.copy_route()
        matrix = lambda: self.page.locator('svg.graph-scene > g').evaluate('e => {const m=e.getCTM(); return {z:m.a,x:m.e,y:m.f}}')
        before = matrix()
        self.page.mouse.move(1100, 500)
        self.page.mouse.wheel(0, 100)
        self.page.wait_for_function('(z) => document.querySelector("svg.graph-scene > g").getCTM().a < z', arg=before['z'])
        after = matrix()
        self.assertAlmostEqual((1100-after['x'])/after['z'], (1100-before['x'])/before['z'], delta=1)
        self.assertAlmostEqual((500-after['y'])/after['z'], (500-before['y'])/before['z'], delta=1)
        self.assertEqual(self.copy_route(), route)

    def test_graph_has_site_navigation_above_the_canvas(self):
        self.page.goto(self.base + 'graph.html')
        navigation = self.page.get_by_role('navigation', name='Main navigation')
        expect(navigation.get_by_role('link', name='Graph', exact=True)).to_have_attribute('aria-current', 'page')
        expect(self.page.get_by_role('link', name='Problem Reductions home')).to_be_visible()
        self.assertGreaterEqual(self.page.locator('.panel').bounding_box()['y'],
                                navigation.bounding_box()['y'] + navigation.bounding_box()['height'])
        navigation.get_by_role('link', name='Atlas', exact=True).click()
        expect(self.page.locator('#atlas-search')).to_be_visible()

    def test_graph_sat_size_badges_finish_promptly_after_routes_arrive(self):
        self.observe_path_worker()
        self.page.goto(self.base + 'graph.html')
        self.page.wait_for_function("() => pathMessages.some(m => m.phase === 'paths')")
        self.page.wait_for_function("() => pathMessages.some(m => m.phase === 'comparison')", timeout=3000)
        expect(self.page.locator('.size.pending')).to_have_count(0)
        expect(self.page.locator('.path').first.locator('.size')).to_contain_text('n')

    def test_graph_unavailable_parameters_keep_size_badges_unknown(self):
        self.page.goto(self.base + 'graph.html')
        self.choose_endpoint('source', 'MVC')
        self.page.locator('#source-options [aria-label="weight"]').get_by_role('button', name='One', exact=True).click()
        self.choose_endpoint('target', 'LongestCommonSubsequence')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route', timeout=3000)
        expect(self.page.locator('.path').first.locator('.size')).to_have_text('?', timeout=500)

    def test_graph_engine_download_starts_before_graph_script_finishes(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('.path').first).to_be_visible()
        timings = self.page.evaluate('''() => {
            const resources = performance.getEntriesByType('resource');
            return {wasm:resources.find(r => r.name.includes('path-selection_bg.wasm'))?.startTime,
                    graph:resources.find(r => r.name.includes('/graph.js'))?.responseEnd};
        }''')
        self.assertIsNotNone(timings['wasm'])
        self.assertLess(timings['wasm'], timings['graph'])

    def test_graph_worker_starts_before_scene_initialization(self):
        if self.browser.browser_type.name == 'chromium':
            session = self.context.new_cdp_session(self.page)
            session.send('Network.enable')
            session.send('Network.emulateNetworkConditions', {'offline': False, 'latency': 20,
                         'downloadThroughput': 600000, 'uploadThroughput': 600000})
        else:
            print('Instrumentation: CDP network throttling unsupported; worker ordering runs unthrottled', flush=True)
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('.path').first).to_be_visible()
        snapshot = self.page.evaluate('performance.getEntriesByName("graph:script")[0].detail')
        self.assertIsNotNone(snapshot, 'Startup instrumentation must record whether its eager worker exists')
        self.assertIs(snapshot['eager_worker'], True)

    def test_graph_reuses_idle_engine_and_releases_old_cached_queries(self):
        self.observe_path_worker()
        self.page.goto(self.base + 'graph.html')
        self.page.wait_for_function("() => pathMessages.some(m => m.phase === 'comparison')")
        self.choose_endpoint('source', 'MIS')
        self.page.wait_for_function("() => pathMessages.filter(m => m.phase === 'comparison').length === 2")
        self.assertEqual(self.page.evaluate('pathWorkerStarts'), 1)
        self.choose_endpoint('target', 'SAT')
        expect(self.page.locator('#results')).to_contain_text('No reduction path')
        for name in ['MaximumClique', 'MinimumVertexCover', 'MaximumSetPacking', 'Coloring',
                     'MinimumDominatingSet', 'HamiltonianCircuit', 'Partition', 'SubsetSum', 'MaximumMatching']:
            self.choose_endpoint('source', name)
            expect(self.page.locator('#results')).to_contain_text('No reduction path')
        # Revisiting the oldest query recomputes after the bounded cache has evicted it.
        self.choose_endpoint('source', 'SAT')
        before = len(self.page.evaluate('pathMessages'))
        self.choose_endpoint('target', 'QUBO')
        self.page.wait_for_function('(before) => pathMessages.length > before', arg=before)
        expect(self.page.locator('.path').first).to_be_visible()
        expect(self.page.locator('.size.pending')).to_have_count(0)
        self.assertEqual(self.page.evaluate('pathWorkerStarts'), 1)

    def test_graph_path_selection_matches_cli_and_keeps_exact_variants(self):
        self.observe_path_worker()
        self.page.goto(self.base + 'graph.html')
        self.choose_endpoint('source', 'MIS')
        pred = str(Path(__file__).resolve().parents[1] / 'target/debug/pred')
        reference = json.loads(subprocess.run(
            [pred, 'path', 'MIS', 'QUBO', '--compare', '--limit', '20', '--json'],
            check=True, capture_output=True).stdout)
        self.page.wait_for_function("() => pathMessages.some(m => m.phase === 'comparison')")
        messages = self.page.evaluate('pathMessages')
        actual = next(m['result'] for m in messages if m['phase'] == 'paths'
                      and m['result']['paths'][0]['path'][0]['from']['name'] == 'MaximumIndependentSet')
        comparison = next(m['result'] for m in messages if m['phase'] == 'comparison')
        # The page carries only the route itself; the CLI adds per-step and overall parameter metadata.
        route_only = lambda r: {'steps': r['steps'], 'path': [{'from': s['from'], 'to': s['to']} for s in r['path']]}
        self.assertEqual(actual['paths'], [route_only(r) for r in reference['paths']])
        self.assertEqual(actual['truncated'], reference['truncated'])
        expected = reference['polynomial_comparison']
        self.assertEqual(comparison['front'], expected['front'])
        self.assertEqual(comparison['counts'], expected['counts'])
        self.assertNotIn('pairs', comparison)
        for actual_profile, expected_profile in zip(comparison['profiles'], expected['profiles'], strict=True):
            for name, field in actual_profile['fields'].items():
                self.assertEqual(field['certificate'], expected_profile['fields'][name]['certificate'])
                self.assertEqual(field['accuracy'], expected_profile['fields'][name]['accuracy'])
        slower = self.page.get_by_role('button', name=re.compile('^Show .* slower routes$'))
        if slower.count():
            slower.click()
        chains = set()
        expect(self.page.locator('.copy')).to_have_text('Copy route for pred reduce --via')
        for row in self.page.locator('.path').all():
            n = int(row.get_attribute('data-path'))
            route = route_only(reference['paths'][n - 1])
            points = [route['path'][0]['from']] + [step['to'] for step in route['path']]
            names = [v['name'] for i, v in enumerate(points) if not i or v['name'] != points[i - 1]['name']]
            self.assertNotIn(tuple(names), chains)
            chains.add(tuple(names))
            row.click()
            self.assertEqual(self.copy_route(), route)
            self.assertEqual(set(self.page.locator('.scene-node.route').evaluate_all('(nodes) => nodes.map(n => n.dataset.node)')), set(names))
            groups = []
            for point in points:
                if groups and groups[-1][0] == point['name']:
                    groups[-1][1].append(point['variant'])
                else:
                    groups.append((point['name'], [point['variant']]))
            expect(self.page.locator('.steps li')).to_have_count(len(groups))
            for i, (name, variants) in enumerate(groups):
                group = self.page.locator('.steps li').nth(i)
                if len([n for n in self.page.evaluate('REDUCTIONS.nodes') if n['name'] == name]) > 1:
                    expected = ['   '.join(f'{k}: {v}' for k, v in variant.items()) for variant in variants]
                    self.assertEqual(group.locator('.kv').all_text_contents(), expected)
            expect(self.page.get_by_role('button', name='rule', exact=True)).to_have_count(len(groups) - 1)
            self.assertEqual(self.page.locator('.path[aria-expanded="true"]').get_attribute('data-path'), str(n))
        count = len(self.page.evaluate('pathMessages'))
        self.choose_endpoint('source', 'MIS')
        expect(self.page.locator('.size.pending')).to_have_count(0)
        self.assertEqual(len(self.page.evaluate('pathMessages')), count)
        self.page.get_by_role('button', name='Search more', exact=True).click()
        expect(self.page.locator('.path').first).to_be_visible()

    def test_graph_path_route_copy_solves_and_unreachable_query_is_explicit(self):
        import tempfile
        self.page.goto(self.base + 'graph.html')
        self.choose_endpoint('source', 'MIS')
        self.choose_endpoint('target', 'MIS')
        self.page.locator('#target-options [aria-label="weight"]').get_by_role('button', name='i64', exact=True).click()
        route = self.copy_route()
        expect(self.page.locator('.steps li')).to_have_count(1)
        expect(self.page.locator('.steps .kv')).to_have_count(2)
        expect(self.page.get_by_role('button', name='rule', exact=True)).to_have_count(0)
        self.assertEqual(route, {'steps': 1, 'path': [{
            'from': {'name': 'MaximumIndependentSet', 'variant': {'graph': 'SimpleGraph', 'weight': 'One'}},
            'to': {'name': 'MaximumIndependentSet', 'variant': {'graph': 'SimpleGraph', 'weight': 'i64'}}}]})
        pred = str(Path(__file__).resolve().parents[1] / 'target/debug/pred')
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'route.json'
            path.write_text(json.dumps(route))
            instance = subprocess.run([pred, 'create', 'MIS', '--graph', '0-1,1-2,2-3,3-4,4-0', '--json'],
                                      check=True, capture_output=True).stdout
            bundle = subprocess.run([pred, 'reduce', '-', '--via', str(path), '--json'],
                                    input=instance, check=True, capture_output=True).stdout
            result = json.loads(subprocess.run([pred, 'solve', '-', '--solver', 'brute-force', '--json'],
                                input=bundle, check=True, capture_output=True).stdout)
        self.assertEqual(result['evaluation'], 'Max(2)')
        self.assertEqual(sum(result['solution']), 2)
        self.assertTrue(all(not (result['solution'][u] and result['solution'][v])
                            for u, v in [(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]))
        self.choose_endpoint('target', 'Factoring')
        expect(self.page.locator('#results')).to_contain_text('No reduction path connects these two.')
        expect(self.page.locator('.path')).to_have_count(0)

    def test_graph_rows_arrive_before_comparison_and_keep_user_selection(self):
        self.observe_path_worker()
        self.page.add_init_script('''window.onPathMessage = message => {
            if (message.phase !== 'paths') return;
            window.pendingAtPaths = document.querySelectorAll('.size.pending').length;
            const row = document.querySelectorAll('.path')[1];
            row.click();
            window.chosenDuringComparison = row.dataset.path;
        };''')
        self.page.goto(self.base + 'graph.html')
        self.page.wait_for_function("() => pathMessages.some(m => m.phase === 'comparison')")
        self.assertGreater(self.page.evaluate('pendingAtPaths'), 0)
        self.assertEqual(self.page.evaluate('pathMessages.map(m => m.phase)'), ['paths', 'comparison'])
        expect(self.page.locator('.size.pending')).to_have_count(0)
        self.assertEqual(self.page.locator('.path[aria-expanded="true"]').get_attribute('data-path'),
                         self.page.evaluate('chosenDuringComparison'))

    def test_graph_path_search_can_be_cancelled_without_blocking_the_page(self):
        self.observe_path_worker()
        self.page.add_init_script('''window.onPathMessage = message => {
            if (message.phase !== 'paths') return;
            window.onPathMessage = null;
            window.pendingAtCancellation = document.querySelectorAll('.size.pending').length;
            document.querySelector('#results .summary button').click();
        };''')
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results')).to_contain_text('Search cancelled.')
        self.assertGreater(self.page.evaluate('pendingAtCancellation'), 0)
        self.page.locator('#target').fill('MIS')
        expect(self.page.locator('#suggest [role="option"]').first).to_contain_text('Maximum Independent Set')
        self.page.locator('#target').press('Enter')
        expect(self.page.locator('#target')).to_have_value('Maximum Independent Set')

    def test_graph_drawer_opens_from_popover_for_chosen_variant_and_closes_with_escape(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#details-drawer')).to_be_hidden()
        self.open_problem_popover('MaximumIndependentSet')
        self.page.locator('#pop [aria-label="weight"]').get_by_role('button', name='i64', exact=True).click()
        before = self.page.locator('.scene-node.route').evaluate_all('(nodes) => nodes.map(n => n.dataset.node)')
        self.page.get_by_role('button', name='Details', exact=True).click()
        expect(self.page.locator('#details-title')).to_have_text('Maximum Independent Set')
        expect(self.page.locator('#drawer-body article')).to_contain_text('wireless network scheduling')
        expect(self.page.locator('#drawer-atlas')).to_have_attribute('href', re.compile('variant=graph%3DSimpleGraph%2Cweight%3Di64'))
        expect(self.page.locator('.dock')).to_be_hidden()
        # Opening details keeps the route; the camera may only move so the route stays clear of the drawer.
        after = self.page.locator('.scene-node.route').evaluate_all('(nodes) => nodes.map(n => n.dataset.node)')
        self.assertEqual(after, before)
        self.page.wait_for_timeout(600)
        self.assertTrue(self.page.evaluate("""() => {
            const right = document.querySelector('#details-drawer').getBoundingClientRect().left;
            return [...document.querySelectorAll('.scene-node.route .node-disc')]
                .every(n => n.getBoundingClientRect().right <= right);
        }"""))
        self.page.keyboard.press('Escape')
        expect(self.page.locator('#details-drawer')).to_be_hidden()
        expect(self.page.locator('.dock')).to_be_visible()

    def test_graph_drawer_opens_from_step_and_article_problem_links_frame_node(self):
        self.page.goto(self.base + 'graph.html')
        route = self.copy_route()
        self.page.locator('.steps .name').first.click()
        expect(self.page.locator('#details-title')).to_have_text('Satisfiability')
        expect(self.page.locator('#drawer-body article')).to_contain_text('Definition')
        expect(self.page.locator('#drawer-atlas')).to_have_attribute('href', './index.html#problem/Satisfiability')
        # Most articles have no cross-problem links; this one links to Shortest Common Supersequence.
        self.page.keyboard.press('Escape')
        self.open_problem_popover('ShortestCommonSuperstring')
        self.page.get_by_role('button', name='Details', exact=True).click()
        expect(self.page.locator('#details-title')).to_have_text('Shortest Common Superstring')
        expect(self.page.locator('#drawer-body article')).to_contain_text('substring containment')
        selector = self.page.evaluate("""() => [...document.querySelectorAll('#drawer-body article a[href^="#"]')]
            .find(a => GRAPH_DETAILS.anchors[a.hash.slice(1)] === 'problem:ShortestCommonSupersequence')?.getAttribute('href')""")
        self.assertIsNotNone(selector)
        self.page.locator('#drawer-body article a[href="' + selector + '"]').first.click()
        expect(self.page.locator('#details-title')).to_have_text('Shortest Common Supersequence')
        self.assertEqual(self.copy_route(), route)

    def test_graph_drawer_opens_exact_cross_family_rule_from_connector_and_canvas(self):
        self.page.goto(self.base + 'graph.html')
        route = self.copy_route()
        rule = next(step for step in route['path'] if step['from']['name'] != step['to']['name'])
        self.page.get_by_role('button', name='rule', exact=True).first.click()
        expect(self.page.locator('#drawer-body article')).to_contain_text('Proof')
        href = self.page.locator('#drawer-atlas').get_attribute('href')
        from urllib.parse import parse_qs, urlparse
        query = parse_qs(urlparse(href).fragment.partition('?')[2], keep_blank_values=True)
        for side, key in [('from', 'from'), ('to', 'to')]:
            self.assertEqual(query[key], [','.join(f'{k}={v}' for k, v in sorted(rule[side]['variant'].items()))])
        title = self.page.locator('#details-title').inner_text()
        self.page.get_by_role('button', name='Close details', exact=True).click()
        edge = self.page.locator('[data-edge="' + rule['from']['name'] + '→' + rule['to']['name'] + '"]')
        point = edge.evaluate("""e => { const p = new DOMPoint((+e.getAttribute('x1') + +e.getAttribute('x2'))/2,
            (+e.getAttribute('y1') + +e.getAttribute('y2'))/2).matrixTransform(e.getScreenCTM()); return {x:p.x,y:p.y}; }""")
        self.page.mouse.click(point['x'], point['y'])
        expect(self.page.locator('#details-drawer')).to_be_visible()
        expect(self.page.locator('#details-title')).to_have_text(title)
        expect(self.page.locator('#drawer-atlas')).to_have_attribute('href', href)
        self.assertEqual(self.copy_route(), route)

    def test_graph_details_load_lazily_reuse_family_article_and_preserve_atlas_variant(self):
        requests = []
        self.page.on('request', lambda request: requests.append(request.url))
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('.path').first).to_be_visible()
        self.assertFalse(any('graph-details' in url or '/assets/details/' in url for url in requests))
        self.open_problem_popover('MaximumIndependentSet')
        self.page.get_by_role('button', name='Details', exact=True).click()
        expect(self.page.locator('#drawer-body article')).to_contain_text('wireless network scheduling')
        self.assertEqual(sum('graph-details.json' in url for url in requests), 1)
        self.page.keyboard.press('Escape')
        self.open_problem_popover('MaximumIndependentSet')
        self.page.locator('#pop [aria-label="weight"]').get_by_role('button', name='i64', exact=True).click()
        self.page.get_by_role('button', name='Details', exact=True).click()
        expect(self.page.locator('#drawer-body article')).to_have_attribute('data-content-key', 'problem:MaximumIndependentSet')
        self.assertEqual(sum('/details/problem/MaximumIndependentSet.html' in url for url in requests), 1)
        read_content = '''node => ({text: node.textContent,
            formulas: [...node.querySelectorAll('svg, math')].map(item => item.outerHTML),
            links: [...node.querySelectorAll('a')].map(link => ({text: link.textContent,
                href: link.getAttribute('href'), role: link.getAttribute('role')}))})'''
        content = self.page.locator('#drawer-body article').evaluate(read_content)
        expect(self.page.get_by_role('heading', name='Definition', level=3, exact=True)).to_be_visible()
        self.page.get_by_role('link', name='Open in Atlas').click()
        expect(self.page.locator('#variant-select')).to_have_value('graph=SimpleGraph,weight=i64')
        expect(self.page.locator('article.typst-detail')).to_contain_text('wireless network scheduling')
        self.assertEqual(self.page.locator('article.typst-detail').evaluate(read_content), content)
        expect(self.page.get_by_role('heading', name='Definition', level=2, exact=True)).to_be_visible()

    def test_graph_node_names_track_route_and_endpoint_state(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        sat = self.page.locator('.scene-node[data-node="Satisfiability"]')
        expect(sat).to_have_attribute('aria-label', re.compile('start of the route'), timeout=500)
        self.choose_endpoint('source', 'MIS')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        expect(self.page.locator('.scene-node[data-node="MaximumIndependentSet"]')).to_have_attribute(
            'aria-label', re.compile('start of the route'), timeout=500)
        expect(sat).not_to_have_attribute('aria-label', re.compile('start of the route'), timeout=500)

    def test_graph_click_activation_without_pointer_events_opens_controls(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.page.locator('.scene-node[data-node="Satisfiability"]').dispatch_event('click', {'detail': 0})
        expect(self.page.locator('#pop .b-start')).to_be_focused(timeout=500)
        expect(self.page.get_by_role('dialog', name='Satisfiability', exact=True)).to_be_visible(timeout=500)

    def test_graph_space_activation_waits_for_key_release(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.page.locator('.scene-node[data-node="Satisfiability"]').focus()
        self.page.keyboard.down('Space')
        expect(self.page.locator('#pop')).to_be_hidden(timeout=500)
        self.page.keyboard.up('Space')
        expect(self.page.locator('#pop .b-start')).to_be_focused(timeout=500)

    def test_graph_keyboard_visits_disconnected_families_and_both_ends(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        graph = self.page.evaluate('window.REDUCTIONS')
        names = list(dict.fromkeys(node['name'] for node in graph['nodes']))
        linked = {graph['nodes'][edge[side]]['name'] for edge in graph['edges'] for side in ['source', 'target']
                  if graph['nodes'][edge['source']]['name'] != graph['nodes'][edge['target']]['name']}
        isolated = next((name for name in names[1:-1] if name not in linked), None)
        self.assertIsNotNone(isolated, 'The registry must retain an isolated-family navigation fixture')
        node = self.page.locator(f'.scene-node[data-node="{isolated}"]')
        node.focus()
        node.press('PageDown')
        expect(self.page.locator(f'.scene-node[data-node="{names[names.index(isolated) + 1]}"]')).to_be_focused(timeout=500)
        self.page.keyboard.press('PageUp')
        expect(node).to_be_focused(timeout=500)
        self.page.keyboard.press('Home')
        expect(self.page.locator(f'.scene-node[data-node="{names[0]}"]')).to_be_focused(timeout=500)
        self.page.keyboard.press('End')
        expect(self.page.locator(f'.scene-node[data-node="{names[-1]}"]')).to_be_focused(timeout=500)

    def test_graph_keyboard_focus_reveals_a_node_hidden_by_the_search_panel(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.page.get_by_role('button', name='Show whole graph', exact=True).click()
        node = self.page.locator('.scene-node[data-node="BinPacking"]')
        dot = node.locator('.node-disc')
        before = dot.bounding_box()
        self.page.mouse.move(1300, 930)
        self.page.mouse.down()
        self.page.mouse.move(1300 + 200 - before['x'] - before['width'] / 2, 930, steps=2)
        self.page.mouse.up()
        panel = self.page.locator('.panel').bounding_box()
        self.assertLess(dot.bounding_box()['x'], panel['x'] + panel['width'])
        node.focus()
        self.assertGreaterEqual(dot.bounding_box()['x'], panel['x'] + panel['width'] + 12)
        expect(node.locator('.node-label')).to_be_visible(timeout=500)
        expect(node).to_have_css('opacity', '1', timeout=500)

    def test_graph_keyboard_variant_selection_keeps_focus_in_the_same_group(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        node = self.page.locator('.scene-node[data-node="MaximumIndependentSet"]')
        node.focus()
        node.press('Enter')
        weight = self.page.locator('#pop [aria-label="weight"]').get_by_role('button', name='i64', exact=True)
        weight.focus()
        weight.press('Enter')
        expect(weight).to_have_attribute('aria-pressed', 'true')
        expect(weight).to_be_focused(timeout=500)
        self.page.keyboard.press('Escape')
        expect(node).to_be_focused(timeout=500)

    def test_graph_keyboard_popover_enters_controls_and_returns_to_node(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        node = self.page.locator('.scene-node[data-node="Satisfiability"]')
        node.focus()
        node.press('Enter')
        expect(self.page.locator('#pop .b-start')).to_be_focused(timeout=500)
        expect(self.page.get_by_role('dialog', name='Satisfiability', exact=True)).to_be_visible(timeout=500)
        self.page.keyboard.press('Escape')
        expect(self.page.locator('#pop')).to_be_hidden()
        expect(node).to_be_focused(timeout=500)

    def test_graph_details_from_keyboard_popover_returns_to_visible_node(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        node = self.page.locator('.scene-node[data-node="Satisfiability"]')
        node.focus()
        node.press('Enter')
        self.page.locator('#pop .pop-details').focus()
        self.page.keyboard.press('Enter')
        expect(self.page.locator('#close-details')).to_be_focused(timeout=500)
        self.page.keyboard.press('Escape')
        expect(self.page.locator('#details-drawer')).to_be_hidden()
        expect(node).to_be_focused(timeout=500)

    def test_graph_comparison_completion_preserves_existing_node_focus(self):
        self.observe_path_worker()
        self.page.add_init_script("""window.onPathMessage = (message) => {
            if (message.phase === 'paths')
                document.querySelector('.scene-node[data-node="Satisfiability"]').focus();
        };""")
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        expect(self.page.locator('.scene-node[data-node="Satisfiability"]')).to_be_focused(timeout=500)

    def test_graph_startup_camera_keeps_a_focused_isolated_node_visible(self):
        self.page.emulate_media(reduced_motion='no-preference')
        self.observe_path_worker()
        self.page.add_init_script('''window.onPathMessage = message => {
            if (message.phase === 'paths')
                document.querySelector('.scene-node[data-node="BinPacking"]').focus();
        };''')
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        expect(self.page.locator('.scene-node[data-node="BinPacking"]')).to_be_focused(timeout=500)
        obscured = self.page.evaluate('''async () => {
            const node = document.querySelector('.scene-node[data-node="BinPacking"]');
            const end = performance.now() + 600, outside = [];
            do {
                const r = node.getBoundingClientRect(), panel = document.querySelector('.panel').getBoundingClientRect();
                const top = document.querySelector('.site-header').getBoundingClientRect().bottom;
                if (r.left < panel.right || r.right > innerWidth || r.top < top || r.bottom > innerHeight)
                    outside.push({left: r.left, right: r.right, top: r.top, bottom: r.bottom});
                await new Promise(requestAnimationFrame);
            } while (performance.now() < end);
            return outside;
        }''')
        self.assertEqual(obscured, [], 'Automatic route framing obscured the focused problem')

    def test_graph_mobile_drawer_isolates_background_and_restores_focus(self):
        self.page.set_viewport_size({'width': 390, 'height': 844})
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        opener = self.page.locator('.steps button').first
        opener.focus()
        opener.press('Enter')
        expect(self.page.locator('#details-drawer')).to_have_attribute('aria-modal', 'true', timeout=500)
        close = self.page.get_by_role('button', name='Close details', exact=True)
        expect(close).to_be_focused(timeout=500)
        self.page.locator('#source').evaluate('(input) => input.focus()')
        expect(close).to_be_focused(timeout=500)
        for _ in range(40):
            self.page.keyboard.press('Tab')
            self.assertTrue(self.page.evaluate("document.activeElement.closest('#details-drawer') !== null"))
            if close.evaluate('(button) => button === document.activeElement'):
                break
        else:
            self.fail('Tab did not cycle back to the close control')
        self.page.keyboard.press('Escape')
        expect(self.page.locator('#details-drawer')).to_be_hidden()
        expect(opener).to_be_focused(timeout=500)
        self.page.locator('#source').focus()
        expect(self.page.locator('#source')).to_be_focused(timeout=500)

    def test_graph_drawer_changes_modal_behavior_when_resized(self):
        self.page.goto(self.base + 'graph.html')
        expect(self.page.locator('#results .summary strong')).to_contain_text('best route')
        self.page.locator('.steps .name').first.click()
        self.assertIsNone(self.page.locator('#details-drawer').get_attribute('aria-modal'))
        self.page.locator('#source').focus()
        expect(self.page.locator('#source')).to_be_focused(timeout=500)
        for width in [1167, 768, 390]:
            self.page.set_viewport_size({'width': width, 'height': 844})
            expect(self.page.locator('#details-drawer')).to_have_attribute('aria-modal', 'true', timeout=500)
            expect(self.page.locator('#close-details')).to_be_focused(timeout=500)
            self.assertEqual(self.page.locator('#details-drawer').bounding_box(),
                             {'x': 0, 'y': 0, 'width': width, 'height': 844})
            self.page.locator('#source').evaluate('(input) => input.focus()')
            expect(self.page.locator('#close-details')).to_be_focused(timeout=500)
        self.page.set_viewport_size({'width': 1168, 'height': 844})
        expect(self.page.locator('#details-drawer')).not_to_have_attribute('aria-modal', 'true', timeout=500)
        self.page.locator('#source').focus()
        expect(self.page.locator('#source')).to_be_focused(timeout=500)
        self.page.set_viewport_size({'width': 1440, 'height': 1000})
        expect(self.page.locator('#details-drawer')).not_to_have_attribute('aria-modal', 'true', timeout=500)
        self.page.locator('#source').focus()
        expect(self.page.locator('#source')).to_be_focused(timeout=500)

    def test_graph_mobile_drawer_has_no_horizontal_scroll(self):
        self.page.set_viewport_size({'width':390, 'height':844})
        self.page.goto(self.base + 'graph.html')
        self.page.locator('.steps .name').first.click()
        expect(self.page.locator('#drawer-body article')).to_contain_text('Definition')
        expect(self.page.get_by_role('button', name='Close details', exact=True)).to_be_visible()
        self.assertFalse(self.page.evaluate('document.documentElement.scrollWidth > innerWidth'))
        self.assertEqual(self.page.locator('#details-drawer').bounding_box(), {'x':0, 'y':0, 'width':390, 'height':844})
        self.page.keyboard.press('Escape')
        expect(self.page.locator('#details-drawer')).to_be_hidden()


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
        expect(self.page.locator("#cy svg.graph-scene").first).to_be_visible()
        self.assertFalse(self.page.evaluate(
            "document.documentElement.scrollWidth > innerWidth"
        ), f"Horizontal overflow at {width}px on graph.html")
        self.page.locator('.steps .name').first.click()
        expect(self.page.locator('#details-drawer')).to_be_visible()
        self.assertFalse(self.page.evaluate(
            "document.documentElement.scrollWidth > innerWidth"
        ), f"Horizontal overflow at {width}px with graph details open")
    return test


for viewport in [320, 390, 768, 1440]:
    setattr(WebsiteTests, f"test_responsive_{viewport}", responsive_check(viewport))


if __name__ == "__main__":
    unittest.main(verbosity=2)

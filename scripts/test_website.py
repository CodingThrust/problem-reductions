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
        self.page.keyboard.press("Control+k")
        expect(self.page.locator("#atlas-search")).to_be_focused()
        self.page.keyboard.type("QUBO")
        expect(self.page.locator(".atlas-row")).to_have_count(1)

    def test_variant_changes_connections_and_survives_reload(self):
        self.visit("problem/MaximumIndependentSet")
        expect(self.page.locator("#variant-select")).to_have_value("graph=SimpleGraph,weight=One")
        expect(self.page.locator('.relation-link[href*="MinimumVertexCover"]')).to_have_count(0)
        self.page.locator("#variant-select").select_option("graph=SimpleGraph,weight=i32")
        expect(self.page.locator('.relation-link[href*="MinimumVertexCover"]')).to_have_count(2)
        self.page.reload()
        expect(self.page.locator("#variant-select")).to_have_value("graph=SimpleGraph,weight=i32")

    def test_reduction_evidence_and_complement_example(self):
        self.visit()
        self.page.locator("#featured-result-link").click()
        expect(self.page.locator("h1")).to_contain_text("Minimum Vertex Cover")
        expect(self.page.locator("#demo-value")).to_have_text("Maximum size: 2")
        self.page.get_by_role("button", name="Vertex cover", exact=True).click()
        expect(self.page.locator("#demo-value")).to_have_text("Minimum size: 3")
        expect(self.page.locator('#demo-graph circle[data-selected="true"]')).to_have_count(3)
        self.page.get_by_role("button", name="Independent set", exact=True).click()
        expect(self.page.locator('#demo-graph circle[data-selected="true"]')).to_have_count(2)
        self.page.get_by_text("Why does complementation preserve optimality?").click()
        expect(self.page.locator("details")).to_have_attribute("open", "")
        source = self.page.get_by_role("link", name="Inspect source problem")
        source.click()
        expect(self.page.locator("#variant-select")).to_have_value("graph=SimpleGraph,weight=i32")

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

    def test_decision_wrapper_links_to_its_public_api(self):
        self.visit("problem/DecisionMinimumVertexCover")
        api = self.page.get_by_role("link", name="API reference", exact=False)
        self.assertTrue(api.get_attribute("href").endswith(
            "models/decision/struct.Decision.html"
        ))
        response = self.page.request.get(urljoin(self.page.url, api.get_attribute("href")))
        self.assertEqual(response.status, 200)

    def test_reduction_links_to_a_published_api_contract(self):
        self.visit()
        self.page.locator("#featured-result-link").click()
        contract = self.page.get_by_role("link", name="API contract", exact=False)
        response = self.page.request.get(urljoin(self.page.url, contract.get_attribute("href")))
        self.assertEqual(response.status, 200)

    def test_research_anchor_and_back_navigation(self):
        self.visit("atlas")
        self.page.locator('[data-nav="research"]').click()
        expect(self.page.locator("#research")).to_be_in_viewport()
        self.page.go_back()
        expect(self.page.locator("#atlas-search")).to_be_visible()

    def test_legacy_documentation_is_reachable(self):
        self.visit()
        self.page.locator(".docs-nav").click()
        self.assertTrue(self.page.url.endswith("/cli.html"))
        expect(self.page.get_by_role("heading", name="CLI Tool", exact=True)).to_be_visible()
        self.assertNotIn("{{#include", self.page.locator("body").inner_text())


def responsive_check(width):
    def test(self):
        self.page.set_viewport_size({"width": width, "height": 900})
        for route in ["home", "atlas", "problem/MaximumIndependentSet"]:
            with self.subTest(route=route):
                self.visit(route)
                self.assertFalse(self.page.evaluate(
                    "document.documentElement.scrollWidth > innerWidth"
                ), f"Horizontal overflow at {width}px on {route}")
    return test


for viewport in [320, 390, 768, 1440]:
    setattr(WebsiteTests, f"test_responsive_{viewport}", responsive_check(viewport))


if __name__ == "__main__":
    unittest.main(verbosity=2)

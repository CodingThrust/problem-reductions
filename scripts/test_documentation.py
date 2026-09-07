#!/usr/bin/env python3
"""Check built documentation, agent-readable exports, and the real CLI showcase."""

import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import unittest
from urllib.parse import unquote, urljoin, urlparse

from playwright.sync_api import expect, sync_playwright

ROOT = Path(__file__).resolve().parents[1]
BOOK = Path(os.environ.get("DOCS_BUILD_DIR", ROOT / "book"))
PAGES = re.findall(r"\]\(([^)]+\.md)\)", (ROOT / "docs/src/SUMMARY.md").read_text())


class DocumentationArtifacts(unittest.TestCase):
    def test_html_guides_have_valid_local_links(self):
        for filename in PAGES:
            page = BOOK / Path(filename).with_suffix(".html")
            main = re.search(r"<main[^>]*>(.*?)</main>", page.read_text(), re.S).group(1)
            for target in re.findall(r'href="([^"]+)"', main):
                parsed = urlparse(target)
                if parsed.scheme or not parsed.path:
                    continue
                self.assertTrue((page.parent / unquote(parsed.path)).exists(),
                                f"{filename}: {target}")

    def test_markdown_pages_have_expanded_includes_and_valid_local_links(self):
        for filename in [*PAGES, "index.md"]:
            page = BOOK / "markdown" / filename
            text = page.read_text()
            self.assertNotIn("{{#include", text, filename)
            for target in re.findall(r"\]\(([^)]+)\)", text):
                parsed = urlparse(target)
                if parsed.scheme or not parsed.path:
                    continue
                self.assertTrue((page.parent / unquote(parsed.path)).exists(),
                                f"{filename}: {target}")
        self.assertIn("let graph = ReductionGraph::new()", (BOOK / "markdown/rust-paths.md").read_text())

    def test_recording_contains_real_solution_and_does_not_autoplay(self):
        events = [json.loads(line) for line in (ROOT / "docs/src/static/cli-demo.cast").read_text().splitlines()]
        self.assertEqual(events[0]["version"], 2)
        output = "".join(event[2] for event in events[1:])
        self.assertIn("Solver: ilp (via ILP)", output)
        self.assertIn("Solver: brute-force", output)
        self.assertGreaterEqual(output.count("Max(2)"), 3)
        self.assertIn('"autoPlay": false', (ROOT / "docs/src/static/cli-demo.html").read_text())

    def test_demo_replays_and_matches_the_recorded_optimum(self):
        pred = ROOT / "target/debug/pred"
        if not pred.exists():
            self.skipTest("Build the CLI to replay the demo")
        with tempfile.TemporaryDirectory(prefix="pred-docs-test-") as work:
            def run(*args):
                result = subprocess.run([str(pred), *args], cwd=work, capture_output=True,
                                        text=True, timeout=4)
                self.assertEqual(result.returncode, 0, result.stderr)
                return result.stdout
            run("path", "MIS", "ILP")
            run("create", "MIS", "--graph", "0-1,1-2,2-3,3-4,4-0", "-o", "cycle.json")
            run("reduce", "cycle.json", "--to", "ILP", "-o", "reduced.json")
            reduced = json.loads(run("solve", "reduced.json", "--json"))
            direct = json.loads(run("solve", "cycle.json", "--solver", "brute-force", "--json"))
            self.assertEqual(reduced["evaluation"], "Max(2)")
            self.assertEqual(direct["evaluation"], reduced["evaluation"])
            self.assertIn("Max(2)", run("evaluate", "cycle.json", "--config",
                                       ",".join(map(str, reduced["solution"]))))


class DocumentationBrowser(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.playwright = sync_playwright().start()
        cls.browser = cls.playwright.chromium.launch(channel=os.environ.get("WEBSITE_BROWSER_CHANNEL") or None)
        cls.base = os.environ.get("WEBSITE_BASE_URL", "http://127.0.0.1:3001/")

    @classmethod
    def tearDownClass(cls):
        cls.browser.close()
        cls.playwright.stop()

    def setUp(self):
        self.context = self.browser.new_context(viewport={"width": 1440, "height": 1000},
                                               permissions=["clipboard-read", "clipboard-write"])
        self.page = self.context.new_page()
        self.page.set_default_timeout(3000)
        self.errors = []
        self.page.on("pageerror", lambda error: self.errors.append(str(error)))

    def tearDown(self):
        self.context.close()
        self.assertEqual(self.errors, [])

    def visit(self, filename):
        self.page.goto(urljoin(self.base, filename))

    def test_task_navigation_and_markdown(self):
        self.visit("introduction.html")
        self.page.locator("main").get_by_role("link", name="Find a solver", exact=True).click()
        expect(self.page.locator("main h1")).to_have_text("Find a solver")
        href = self.page.get_by_role("navigation", name="Documentation resources").get_by_role("link", name="Markdown", exact=True).get_attribute("href")
        result = self.context.request.get(urljoin(self.page.url, href))
        self.assertEqual(result.status, 200)
        self.assertIn(".claude/skills/find-solver/SKILL.md", result.text())
        self.assertLessEqual(self.page.locator(".docs-brand").bounding_box()["y"] + self.page.locator(".docs-brand").bounding_box()["height"],
                             self.page.locator(".sidebar-scrollbox").bounding_box()["y"] + 1)

    def test_search_finds_a_specific_task(self):
        self.visit("introduction.html")
        self.page.locator("#search-toggle, #mdbook-search-toggle").click()
        self.page.locator("#searchbar, #mdbook-searchbar").press_sequentially("overhead")
        results = self.page.locator("#searchresults, #mdbook-searchresults")
        expect(results).to_contain_text("overhead")
        self.assertGreater(results.locator("a").count(), 0)

    def test_copy_commands(self):
        self.visit("cli-demo.html")
        self.page.locator("main pre").hover()
        self.page.locator("main .clip-button").first.click()
        self.assertIn("pred reduce cycle.json --to ILP", self.page.evaluate("navigator.clipboard.readText()"))

    def test_old_deep_links_reach_the_new_task(self):
        for old, title in [("cli.html#installation", "Install the CLI"),
                           ("design.html#reduction-rules", "Reduction contracts"),
                           ("mcp.html#walkthrough", "MCP example session")]:
            self.visit(old)
            expect(self.page.locator("main h1")).to_have_text(title)

    def test_recording_loads_offline_and_plays(self):
        self.context.set_offline(True)
        self.page.goto((ROOT / "docs/src/static/cli-demo.html").as_uri())
        expect(self.page.locator(".ap-overlay-start")).to_be_visible()
        self.page.locator(".ap-overlay-start").click()
        expect(self.page.locator(".ap-overlay-start")).not_to_be_visible()
        expect(self.page.locator(".ap-control-bar")).to_be_visible()

    def test_mobile_menu_and_embedded_player_fit(self):
        self.page.set_viewport_size({"width": 390, "height": 844})
        self.visit("cli-demo.html")
        frame = self.page.frame_locator(".cli-cast")
        expect(frame.locator(".ap-control-bar")).to_be_visible()
        self.assertTrue(frame.locator("body").evaluate("e => e.scrollHeight <= innerHeight + 2"))
        self.assertTrue(self.page.locator("body").evaluate("e => e.scrollWidth <= innerWidth"))
        self.page.locator("#sidebar-toggle, #mdbook-sidebar-toggle").click()
        expect(self.page.locator(".sidebar").get_by_role("link", name="Start with an agent", exact=True)).to_be_visible()


if __name__ == "__main__":
    unittest.main(verbosity=2)

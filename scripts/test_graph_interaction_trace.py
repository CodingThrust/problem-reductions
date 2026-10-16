"""Real browser evidence for trace capture and keyboard activation measurements."""

import gzip
import json
import os
from pathlib import Path
import tempfile
import unittest

from playwright.sync_api import sync_playwright

from benchmark_graph_interaction import phase, trace_frame_costs, trace_interaction


@unittest.skipUnless(os.environ.get("WEBSITE_BASE_URL"), "Needs an explicitly served production artifact")
class InteractionTraceTests(unittest.TestCase):
    def test_keyboard_activation_is_observed_and_traced(self):
        with sync_playwright() as p, tempfile.TemporaryDirectory() as directory:
            browser = p.chromium.launch()
            try:
                page = browser.new_page(viewport={"width": 1440, "height": 1000}, reduced_motion="reduce")
                page.goto(os.environ["WEBSITE_BASE_URL"] + "graph.html")
                page.wait_for_function("() => performance.getEntriesByName('graph:comparison-rendered').length > 0")
                path = Path(directory) / "activation.json.gz"
                with trace_interaction(page, path):
                    result = phase(page, "keyboard_activation")
                self.assertFalse(page.locator("#pop").is_hidden())
                self.assertEqual(result["raw_samples"]["first_input"]["type"], "keydown")
                self.assertEqual(result["raw_samples"]["event_status"], "supported")
                self.assertTrue(any(event["type"] == "keydown" for event in result["raw_samples"]["events"]))
                entries = json.loads(gzip.decompress(path.read_bytes()))["traceEvents"]
                self.assertTrue(any(e["name"] == "EventDispatch" and e.get("args", {}).get("data", {}).get("type") == "keydown" for e in entries))
                costs = trace_frame_costs(entries, 'keydown')
                self.assertEqual(costs['status'], 'supported')
                self.assertGreater(costs['first_input_script_render_ms'], 0)
            finally:
                browser.close()


if __name__ == "__main__":
    unittest.main()

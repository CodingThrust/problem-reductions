"""Trace accounting and a cold navigation through the real mathematical engine."""

import os
import unittest

from playwright.sync_api import sync_playwright

from benchmark_graph_startup import sample, trace_cpu


class StartupTraceTests(unittest.TestCase):
    def test_nested_trace_slices_are_clipped_and_counted_once_per_thread(self):
        events = [
            {"name": "thread_name", "pid": 1, "tid": 1, "args": {"name": "CrRendererMain"}},
            {"name": "EvaluateScript", "ph": "X", "pid": 1, "tid": 1, "ts": 10000, "dur": 8000},
            {"name": "FunctionCall", "ph": "X", "pid": 1, "tid": 1, "ts": 12000, "dur": 3000},
            {"name": "FunctionCall", "ph": "X", "pid": 1, "tid": 1, "ts": 17000, "dur": 5000},
            {"name": "FunctionCall", "ph": "X", "pid": 1, "tid": 2, "ts": 10000, "dur": 30000},
            {"name": "wasm.ExecuteCompilationUnits", "ph": "X", "pid": 1, "tid": 2, "ts": 11000, "dur": 4000},
            {"name": "wasm.ExecuteCompilationUnits", "ph": "X", "pid": 1, "tid": 3, "ts": 12000, "dur": 2000},
        ]
        result = trace_cpu(events, cutoff=20000)
        self.assertEqual(result["js_execute_main"], 10)
        self.assertEqual(result["wasm_compile_all_threads"], 6)
        self.assertIsNone(result["js_parse_main"])

    def test_missing_trace_categories_are_unavailable(self):
        self.assertTrue(all(value is None for value in trace_cpu([]).values()))


@unittest.skipUnless(os.environ.get("WEBSITE_BASE_URL"), "Needs an explicitly served production artifact")
class ColdNavigationTests(unittest.TestCase):
    def test_cold_navigation_verifies_routes_and_an_actual_search_suggestion(self):
        with sync_playwright() as p:
            row = sample(p, os.environ["WEBSITE_BASE_URL"] + "graph.html", 1, None)
        self.assertEqual(row["verification"]["search_suggestion"], "Maximum Independent Set")
        self.assertGreater(row["verification"]["routes"], 0)
        self.assertEqual(row["instrumentation"]["longtask"], "supported")
        self.assertGreater(row["input_latency_ms"], 0)
        self.assertGreater(row["wasm_memory_bytes"], 0)
        self.assertEqual(len(row["html_sha256"]), 64)
        self.assertTrue(row["verification"]["warm_query_route"].startswith("Maximum Independent Set → "))
        self.assertTrue(row["verification"]["warm_query_route"].endswith("QUBO"))
        self.assertGreater(row["warm_query"]["route_dom_ms"], 0)


if __name__ == "__main__":
    unittest.main()

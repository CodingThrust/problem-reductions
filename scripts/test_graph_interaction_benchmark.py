"""Behavior checks for reported graph interaction measurements."""

import unittest
import benchmark_graph_interaction

from benchmark_graph_interaction import summarize


class InteractionMeasurementTests(unittest.TestCase):
    def test_first_pan_sample_includes_pointerdown_work(self):
        probe = {"events": [{"type": "pointerdown", "start": 12}, {"type": "pointermove", "start": 85}],
                 "frames": [{"start": 10, "duration": 70}, {"start": 80, "duration": 16}],
                 "loaf": [{"start": 10, "duration": 70}]}
        first = benchmark_graph_interaction.first_input(probe, "background_pan")
        self.assertEqual(first, {"type": "pointerdown", "start": 12,
                                 "raf_interval_ms": 70, "loaf_ms": 70})

    def test_run_summary_keeps_unavailable_samples_distinct_from_zero(self):
        result = benchmark_graph_interaction.summarize_runs([None, 0, 0])
        self.assertEqual(result["samples"], 2)
        self.assertEqual(result["unavailable"], 1)
        self.assertEqual(result["median"], 0)
        self.assertEqual(result["p95"], 0)
        self.assertIsNone(result["cv"])
        missing = benchmark_graph_interaction.summarize_runs([None, None])
        self.assertEqual(missing["samples"], 0)
        self.assertIsNone(missing["median"])
        self.assertIsNone(missing["p95"])

    def test_run_summary_reports_independent_sample_variation(self):
        result = benchmark_graph_interaction.summarize_runs([2, 4, 6])
        self.assertEqual(result["samples"], 3)
        self.assertEqual(result["median"], 4)
        self.assertEqual(result["p95"], 6)
        self.assertAlmostEqual(result["cv"], .5)
        single = benchmark_graph_interaction.summarize_runs([4])
        self.assertIsNone(single["cv"])

    def test_first_frame_hitch_is_retained(self):
        result = summarize([80, 12], [])
        self.assertEqual(result["frames"], 2)
        self.assertEqual(result["max_ms"], 80)
        self.assertEqual(result["over_50"], 1)

    def test_unsupported_instrumentation_has_no_long_frame_count(self):
        result = summarize([16], [], "unsupported")
        self.assertEqual(result["loaf_status"], "unsupported")
        self.assertIsNone(result["loaf_count"])
        self.assertIsNone(result["loaf_p95_script_ms"])

    def test_supported_instrumentation_can_report_zero_long_frames(self):
        result = summarize([16], [], "supported")
        self.assertEqual(result["loaf_status"], "supported")
        self.assertEqual(result["loaf_count"], 0)


if __name__ == "__main__":
    unittest.main()

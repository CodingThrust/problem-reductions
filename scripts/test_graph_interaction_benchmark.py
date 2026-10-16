"""Behavior checks for reported graph interaction measurements."""

import unittest

from benchmark_graph_interaction import summarize


class InteractionMeasurementTests(unittest.TestCase):
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

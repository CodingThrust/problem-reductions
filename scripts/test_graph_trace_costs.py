"""Account for nested script/render slices without treating frame waits as work."""

import unittest

from benchmark_graph_interaction import trace_frame_costs


class FrameCostTests(unittest.TestCase):
    def test_first_input_frame_unions_overlapping_script_and_render_slices(self):
        thread = {"pid": 1, "tid": 2}
        events = [{**thread, "name": "thread_name", "args": {"name": "CrRendererMain"}}]
        for start in [0, 10000, 20000]:
            events.append({**thread, "name": "AnimationFrame", "ph": "b", "ts": start,
                           "args": {"animation_frame_timing_info": {}}})
        for name, start, duration, args in [
            ("EventDispatch", 1000, 5000, {"data": {"type": "pointerdown"}}),
            ("Layout", 2000, 2000, {}),
            ("FireAnimationFrame", 5500, 2000, {}),
            ("Paint", 8000, 1500, {}),
            ("Paint", 12000, 500, {}),
        ]:
            events.append({**thread, "name": name, "ph": "X", "ts": start, "dur": duration, "args": args})
        # A worker's work and the long wait between frames do not belong to main-thread slices.
        events.append({"pid": 1, "tid": 3, "name": "FunctionCall", "ph": "X", "ts": 0, "dur": 30000})
        report = trace_frame_costs(events, "pointerdown")
        self.assertEqual(report["status"], "supported")
        self.assertEqual(report["active_script_render_ms"], [8, .5])
        self.assertEqual(report["first_input_script_render_ms"], 8)
        self.assertEqual(report["p95_active_script_render_ms"], 8)

    def test_unknown_trace_format_is_unavailable(self):
        report = trace_frame_costs([], "wheel")
        self.assertEqual(report["status"], "unsupported")
        self.assertIsNone(report["p95_active_script_render_ms"])

    def test_a_first_input_outside_complete_frames_cannot_pass_a_budget(self):
        thread = {"pid": 1, "tid": 2}
        events = [{**thread, "name": "thread_name", "args": {"name": "CrRendererMain"}},
                  {**thread, "name": "EventDispatch", "ph": "X", "ts": 1000, "dur": 70000,
                   "args": {"data": {"type": "wheel"}}}]
        for start in [10000, 20000]:
            events.append({**thread, "name": "AnimationFrame", "ph": "b", "ts": start,
                           "args": {"animation_frame_timing_info": {}}})
        report = trace_frame_costs(events, "wheel")
        self.assertEqual(report["status"], "unsupported")
        self.assertIsNone(report["first_input_script_render_ms"])

if __name__ == "__main__":
    unittest.main()

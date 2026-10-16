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

    def test_first_presentation_uses_matching_frame_id(self):
        thread = {"pid": 1, "tid": 2}
        track = {"local": "frame-track"}
        events = [{**thread, "name": "thread_name", "args": {"name": "CrRendererMain"}},
                  {**thread, "name": "AnimationFrame", "ph": "b", "ts": 0, "id2": track,
                   "args": {"animation_frame_timing_info": {}, "id": "first"}},
                  {**thread, "name": "EventDispatch", "ph": "X", "ts": 5000, "dur": 1000,
                   "args": {"data": {"type": "pointerdown"}}},
                  {**thread, "name": "AnimationFrame", "ph": "e", "ts": 10000, "id2": track},
                  {**thread, "name": "AnimationFrame::Presentation", "ph": "n", "ts": 11000, "args": {"id": "previous"}},
                  {**thread, "name": "AnimationFrame::Presentation", "ph": "n", "ts": 17000, "args": {"id": "first"}},
                  {**thread, "name": "AnimationFrame", "ph": "b", "ts": 20000, "id2": track,
                   "args": {"animation_frame_timing_info": {}, "id": "next"}}]
        report = trace_frame_costs(events, "pointerdown")
        self.assertEqual(report.get("presentation_status"), "supported")
        self.assertEqual(report.get("first_input_frame_to_presentation_ms"), 17)
        self.assertEqual(report.get("first_input_handler_to_presentation_ms"), 12)
        events = [event for event in events if event["name"] != "AnimationFrame::Presentation"]
        missing = trace_frame_costs(events, "pointerdown")
        self.assertEqual(missing["status"], "supported")
        self.assertEqual(missing.get("presentation_status"), "unsupported")
        self.assertIsNone(missing.get("first_input_frame_to_presentation_ms"))

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

#!/usr/bin/env python3
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from project_board_poll import ack_item, process_snapshot


def make_issue_item(item_id: str, number: int, status: str = "Ready") -> dict:
    return {
        "id": item_id,
        "status": status,
        "content": {"type": "Issue", "number": number},
    }


def make_pr_item(item_id: str, number: int, status: str = "Review pool") -> dict:
    return {
        "id": item_id,
        "status": status,
        "content": {"type": "PullRequest", "number": number},
    }


class ProjectBoardPollTests(unittest.TestCase):
    def test_ready_queue_retries_same_item_until_ack(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "ready-state.json"
            snapshot = {
                "items": [
                    make_issue_item("PVTI_1", 101),
                    make_issue_item("PVTI_2", 102),
                ]
            }

            item_id, number = process_snapshot("ready", snapshot, state_file)
            self.assertEqual((item_id, number), ("PVTI_1", 101))

            item_id, number = process_snapshot("ready", snapshot, state_file)
            self.assertEqual((item_id, number), ("PVTI_1", 101))

            ack_item(state_file, "PVTI_1")
            item_id, number = process_snapshot("ready", snapshot, state_file)
            self.assertEqual((item_id, number), ("PVTI_2", 102))

    def test_ready_queue_detects_new_item_after_queue_drops_to_zero(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "ready-state.json"

            item_id, number = process_snapshot(
                "ready",
                {"items": [make_issue_item("PVTI_1", 101)]},
                state_file,
            )
            self.assertEqual((item_id, number), ("PVTI_1", 101))

            ack_item(state_file, "PVTI_1")
            no_item = process_snapshot("ready", {"items": []}, state_file)
            self.assertIsNone(no_item)

            item_id, number = process_snapshot(
                "ready",
                {"items": [make_issue_item("PVTI_2", 102)]},
                state_file,
            )
            self.assertEqual((item_id, number), ("PVTI_2", 102))

    def test_empty_state_file_is_treated_as_no_previous_items(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "ready-state.json"
            state_file.write_text("")

            item_id, number = process_snapshot(
                "ready",
                {"items": [make_issue_item("PVTI_1", 101)]},
                state_file,
            )
            self.assertEqual((item_id, number), ("PVTI_1", 101))

    def test_review_queue_resolves_issue_cards_to_prs(self) -> None:
        def fake_pr_resolver(repo: str, issue_number: int) -> int | None:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            return 570 if issue_number == 117 else None

        def fake_review_fetcher(repo: str, pr_number: int) -> list[dict]:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            if pr_number == 570:
                return [{"user": {"login": "copilot-pull-request-reviewer[bot]"}}]
            return []

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "review-state.json"
            item_id, number = process_snapshot(
                "review",
                {"items": [make_issue_item("PVTI_10", 117, status="Review pool")]},
                state_file,
                repo="CodingThrust/problem-reductions",
                review_fetcher=fake_review_fetcher,
                pr_resolver=fake_pr_resolver,
            )
            self.assertEqual((item_id, number), ("PVTI_10", 570))

    def test_review_fetch_errors_are_not_suppressed(self) -> None:
        def fake_review_fetcher(repo: str, pr_number: int) -> list[dict]:
            raise subprocess.CalledProcessError(42, ["gh", "api"])

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "review-state.json"
            with self.assertRaises(subprocess.CalledProcessError):
                process_snapshot(
                    "review",
                    {"items": [make_pr_item("PVTI_10", 570)]},
                    state_file,
                    repo="CodingThrust/problem-reductions",
                    review_fetcher=fake_review_fetcher,
                )


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from project_board_recover import (
    STATUS_BACKLOG,
    STATUS_DONE,
    STATUS_FINAL_REVIEW,
    STATUS_READY,
    STATUS_REVIEW_POOL,
    all_checks_green,
    build_recovery_plan,
    infer_issue_status,
)


def make_issue(
    number: int,
    *,
    state: str = "OPEN",
    labels: list[str] | None = None,
) -> dict:
    return {
        "number": number,
        "state": state,
        "labels": [{"name": name} for name in (labels or [])],
    }


def make_pr(
    number: int,
    *,
    state: str = "OPEN",
    merged: bool = False,
    checks: list[dict] | None = None,
) -> dict:
    return {
        "number": number,
        "state": state,
        "mergedAt": "2026-03-15T00:00:00Z" if merged else None,
        "statusCheckRollup": checks or [],
    }


class ProjectBoardRecoverTests(unittest.TestCase):
    def test_all_checks_green_accepts_successful_runs_and_statuses(self) -> None:
        pr = make_pr(
            101,
            checks=[
                {
                    "__typename": "CheckRun",
                    "status": "COMPLETED",
                    "conclusion": "SUCCESS",
                },
                {
                    "__typename": "StatusContext",
                    "state": "SUCCESS",
                },
            ],
        )
        self.assertTrue(all_checks_green(pr))

    def test_all_checks_green_rejects_pending_or_failing_checks(self) -> None:
        pending = make_pr(
            101,
            checks=[
                {
                    "__typename": "CheckRun",
                    "status": "IN_PROGRESS",
                    "conclusion": None,
                }
            ],
        )
        failing = make_pr(
            102,
            checks=[
                {
                    "__typename": "StatusContext",
                    "state": "FAILURE",
                }
            ],
        )
        self.assertFalse(all_checks_green(pending))
        self.assertFalse(all_checks_green(failing))

    def test_closed_issue_recovers_to_done(self) -> None:
        status, reason = infer_issue_status(make_issue(10, state="CLOSED"), [])
        self.assertEqual(status, STATUS_DONE)
        self.assertIn("closed", reason)

    def test_good_issue_without_pr_recovers_to_ready(self) -> None:
        status, reason = infer_issue_status(make_issue(10, labels=["Good"]), [])
        self.assertEqual(status, STATUS_READY)
        self.assertIn("Good", reason)

    def test_unchecked_issue_without_pr_recovers_to_backlog(self) -> None:
        status, reason = infer_issue_status(make_issue(10), [])
        self.assertEqual(status, STATUS_BACKLOG)
        self.assertIn("no linked PR", reason)

    def test_issue_with_failure_labels_recovers_to_backlog(self) -> None:
        status, reason = infer_issue_status(
            make_issue(10, labels=["PoorWritten", "Wrong"]),
            [],
        )
        self.assertEqual(status, STATUS_BACKLOG)
        self.assertIn("failure", reason)

    def test_issue_with_open_pr_no_green_checks_recovers_to_review_pool(self) -> None:
        pr = make_pr(200)
        status, reason = infer_issue_status(
            make_issue(10, labels=["Good"]),
            [pr],
        )
        self.assertEqual(status, STATUS_REVIEW_POOL)
        self.assertIn("still implementing", reason)

    def test_issue_with_green_open_pr_recovers_to_final_review(self) -> None:
        pr = make_pr(
            200,
            checks=[
                {
                    "__typename": "CheckRun",
                    "status": "COMPLETED",
                    "conclusion": "SUCCESS",
                }
            ],
        )
        status, reason = infer_issue_status(
            make_issue(10, labels=["Good"]),
            [pr],
        )
        self.assertEqual(status, STATUS_FINAL_REVIEW)
        self.assertIn("green open PR", reason)

    def test_issue_with_non_green_open_pr_stays_in_review_pool(self) -> None:
        pr = make_pr(
            200,
            checks=[
                {
                    "__typename": "CheckRun",
                    "status": "IN_PROGRESS",
                    "conclusion": None,
                }
            ],
        )
        status, reason = infer_issue_status(
            make_issue(10, labels=["Good"]),
            [pr],
        )
        self.assertEqual(status, STATUS_REVIEW_POOL)
        self.assertIn("still implementing", reason)

    def test_issue_with_closed_unmerged_pr_falls_back_to_backlog(self) -> None:
        pr = make_pr(200, state="CLOSED")
        status, reason = infer_issue_status(make_issue(10), [pr])
        self.assertEqual(status, STATUS_BACKLOG)
        self.assertIn("default", reason)

    def test_build_recovery_plan_ignores_non_model_and_non_rule_titles(self) -> None:
        plan = build_recovery_plan(
            {
                "items": [
                    {
                        "id": "PVTI_1",
                        "status": None,
                        "content": {"number": 10, "title": "[Model] Foo"},
                    },
                    {
                        "id": "PVTI_2",
                        "status": None,
                        "content": {"number": 11, "title": "Meta task"},
                    },
                ]
            },
            [
                make_issue(10, labels=["Good"]) | {"title": "[Model] Foo"},
                make_issue(11, labels=["Good"]) | {"title": "Meta task"},
            ],
            [],
        )
        self.assertEqual([entry["issue_number"] for entry in plan], [10])


if __name__ == "__main__":
    unittest.main()

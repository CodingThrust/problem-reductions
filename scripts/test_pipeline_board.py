#!/usr/bin/env python3
import io
import json
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path

from pipeline_board import (
    STATUS_DONE,
    STATUS_FINAL_REVIEW,
    STATUS_IN_PROGRESS,
    STATUS_ON_HOLD,
    STATUS_READY,
    STATUS_REVIEW_POOL,
    STATUS_UNDER_REVIEW,
    ack_item,
    claim_next_entry,
    build_recovery_plan,
    normalize_status_name,
    print_next_item,
    process_snapshot,
    review_candidates,
    select_next_entry,
    status_items,
)


def make_issue_item(
    item_id: str,
    number: int,
    *,
    status: str = "Ready",
    title: str | None = None,
    linked_prs: list[int] | None = None,
) -> dict:
    item = {
        "id": item_id,
        "status": status,
        "content": {
            "type": "Issue",
            "number": number,
            "title": title or f"[Model] Issue {number}",
        },
        "title": title or f"[Model] Issue {number}",
    }
    if linked_prs is not None:
        item["linked pull requests"] = [
            f"https://github.com/CodingThrust/problem-reductions/pull/{pr_number}"
            for pr_number in linked_prs
        ]
    return item


def make_pr_item(item_id: str, number: int, status: str = "Review pool") -> dict:
    return {
        "id": item_id,
        "status": status,
        "content": {"type": "PullRequest", "number": number},
    }


def make_issue(number: int, *, state: str = "OPEN", labels: list[str] | None = None) -> dict:
    return {
        "number": number,
        "state": state,
        "title": f"[Model] Issue {number}",
        "labels": [{"name": label} for label in (labels or [])],
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


def success_check(name: str = "ci") -> dict:
    return {
        "__typename": "CheckRun",
        "name": name,
        "status": "COMPLETED",
        "conclusion": "SUCCESS",
    }


class PipelineBoardPollTests(unittest.TestCase):
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

    def test_review_queue_resolves_issue_cards_to_prs(self) -> None:
        def fake_pr_resolver(repo: str, issue_number: int) -> int | None:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            return 570 if issue_number == 117 else None

        def fake_review_fetcher(repo: str, pr_number: int) -> list[dict]:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            if pr_number == 570:
                return [{"user": {"login": "copilot-pull-request-reviewer[bot]"}}]
            return []

        def fake_pr_state_fetcher(repo: str, pr_number: int) -> str:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(pr_number, 570)
            return "OPEN"

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "review-state.json"
            item_id, number = process_snapshot(
                "review",
                {"items": [make_issue_item("PVTI_10", 117, status="Review pool")]},
                state_file,
                repo="CodingThrust/problem-reductions",
                review_fetcher=fake_review_fetcher,
                pr_resolver=fake_pr_resolver,
                pr_state_fetcher=fake_pr_state_fetcher,
            )
            self.assertEqual((item_id, number), ("PVTI_10", 570))

    def test_review_queue_skips_closed_pr_cards(self) -> None:
        def fake_review_fetcher(repo: str, pr_number: int) -> list[dict]:
            return [{"user": {"login": "copilot-pull-request-reviewer[bot]"}}]

        def fake_pr_state_fetcher(repo: str, pr_number: int) -> str:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(pr_number, 570)
            return "CLOSED"

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "review-state.json"
            no_item = process_snapshot(
                "review",
                {"items": [make_pr_item("PVTI_10", 570)]},
                state_file,
                repo="CodingThrust/problem-reductions",
                review_fetcher=fake_review_fetcher,
                pr_state_fetcher=fake_pr_state_fetcher,
            )
            self.assertIsNone(no_item)

    def test_final_review_queue_resolves_issue_cards_to_open_prs(self) -> None:
        def fake_pr_resolver(repo: str, issue_number: int) -> int | None:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            return 615 if issue_number == 101 else None

        def fake_pr_state_fetcher(repo: str, pr_number: int) -> str:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(pr_number, 615)
            return "OPEN"

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "final-review-state.json"
            item_id, number = process_snapshot(
                "final-review",
                {"items": [make_issue_item("PVTI_20", 101, status="Final review")]},
                state_file,
                repo="CodingThrust/problem-reductions",
                pr_resolver=fake_pr_resolver,
                pr_state_fetcher=fake_pr_state_fetcher,
            )
            self.assertEqual((item_id, number), ("PVTI_20", 615))

    def test_final_review_queue_skips_closed_pr_cards(self) -> None:
        def fake_pr_state_fetcher(repo: str, pr_number: int) -> str:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(pr_number, 621)
            return "CLOSED"

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "final-review-state.json"
            no_item = process_snapshot(
                "final-review",
                {"items": [make_pr_item("PVTI_21", 621, status="Final review")]},
                state_file,
                repo="CodingThrust/problem-reductions",
                pr_state_fetcher=fake_pr_state_fetcher,
            )
            self.assertIsNone(no_item)


class PipelineBoardRecoveryTests(unittest.TestCase):
    def test_recovery_plan_marks_merged_pr_items_done(self) -> None:
        board_data = {
            "items": [
                make_issue_item(
                    "PVTI_1",
                    101,
                    status="Review pool",
                    title="[Model] MinimumFeedbackVertexSet",
                    linked_prs=[615],
                )
            ]
        }
        issues = [make_issue(101, labels=["Good"])]
        prs = [make_pr(615, state="MERGED", merged=True)]

        plan = build_recovery_plan(board_data, issues, prs, pr_reviews={})

        self.assertEqual(len(plan), 1)
        self.assertEqual(plan[0]["proposed_status"], STATUS_DONE)

    def test_recovery_plan_marks_green_copilot_reviewed_prs_final_review(self) -> None:
        board_data = {
            "items": [
                make_issue_item(
                    "PVTI_1",
                    101,
                    status="Review pool",
                    title="[Model] HamiltonianPath",
                    linked_prs=[621],
                )
            ]
        }
        issues = [make_issue(101, labels=["Good"])]
        prs = [make_pr(621, checks=[success_check()])]
        pr_reviews = {621: [{"user": {"login": "copilot-pull-request-reviewer[bot]"}}]}

        plan = build_recovery_plan(board_data, issues, prs, pr_reviews=pr_reviews)

        self.assertEqual(plan[0]["proposed_status"], STATUS_FINAL_REVIEW)

    def test_recovery_plan_marks_open_pr_without_copilot_review_review_pool(self) -> None:
        board_data = {
            "items": [
                make_issue_item(
                    "PVTI_1",
                    101,
                    status="In progress",
                    title="[Model] SteinerTree",
                    linked_prs=[192],
                )
            ]
        }
        issues = [make_issue(101, labels=["Good"])]
        prs = [make_pr(192, checks=[success_check()])]

        plan = build_recovery_plan(board_data, issues, prs, pr_reviews={192: []})

        self.assertEqual(plan[0]["proposed_status"], STATUS_REVIEW_POOL)

    def test_recovery_plan_marks_good_issue_without_pr_ready(self) -> None:
        board_data = {
            "items": [
                make_issue_item(
                    "PVTI_1",
                    101,
                    status="Backlog",
                    title="[Model] ExactCoverBy3Sets",
                )
            ]
        }
        issues = [make_issue(101, labels=["Good"])]

        plan = build_recovery_plan(board_data, issues, prs=[], pr_reviews={})

        self.assertEqual(plan[0]["proposed_status"], STATUS_READY)


class PipelineBoardStatusTests(unittest.TestCase):
    def test_normalize_status_name_accepts_pipeline_aliases(self) -> None:
        self.assertEqual(normalize_status_name("ready"), STATUS_READY)
        self.assertEqual(normalize_status_name("review-pool"), STATUS_REVIEW_POOL)
        self.assertEqual(normalize_status_name("in-progress"), STATUS_IN_PROGRESS)
        self.assertEqual(normalize_status_name("under review"), STATUS_UNDER_REVIEW)
        self.assertEqual(normalize_status_name("on-hold"), STATUS_ON_HOLD)
        self.assertEqual(normalize_status_name("done"), STATUS_DONE)


class PipelineBoardOutputTests(unittest.TestCase):
    def test_claim_next_ready_moves_selected_item_to_in_progress(self) -> None:
        moves: list[tuple[str, str]] = []

        def fake_mover(item_id: str, status: str) -> None:
            moves.append((item_id, status))

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "ready-state.json"
            result = claim_next_entry(
                "ready",
                {
                    "items": [
                        make_issue_item(
                            "PVTI_1",
                            101,
                            title="[Model] ExactCoverBy3Sets",
                        )
                    ]
                },
                state_file,
                mover=fake_mover,
            )

        self.assertEqual(moves, [("PVTI_1", STATUS_IN_PROGRESS)])
        self.assertEqual(
            result,
            {
                "item_id": "PVTI_1",
                "number": 101,
                "issue_number": 101,
                "pr_number": None,
                "status": STATUS_READY,
                "title": "[Model] ExactCoverBy3Sets",
                "claimed": True,
                "claimed_status": STATUS_IN_PROGRESS,
            },
        )

    def test_claim_next_review_moves_selected_item_to_under_review(self) -> None:
        moves: list[tuple[str, str]] = []

        def fake_review_fetcher(repo: str, pr_number: int) -> list[dict]:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(pr_number, 570)
            return [{"user": {"login": "copilot-pull-request-reviewer[bot]"}}]

        def fake_pr_resolver(repo: str, issue_number: int) -> int | None:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(issue_number, 117)
            return 570

        def fake_pr_state_fetcher(repo: str, pr_number: int) -> str:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(pr_number, 570)
            return "OPEN"

        def fake_mover(item_id: str, status: str) -> None:
            moves.append((item_id, status))

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "review-state.json"
            result = claim_next_entry(
                "review",
                {
                    "items": [
                        make_issue_item(
                            "PVTI_10",
                            117,
                            status="Review pool",
                            title="[Model] GraphPartitioning",
                        )
                    ]
                },
                state_file,
                repo="CodingThrust/problem-reductions",
                review_fetcher=fake_review_fetcher,
                pr_resolver=fake_pr_resolver,
                pr_state_fetcher=fake_pr_state_fetcher,
                mover=fake_mover,
            )

        self.assertEqual(moves, [("PVTI_10", STATUS_UNDER_REVIEW)])
        self.assertEqual(
            result,
            {
                "item_id": "PVTI_10",
                "number": 570,
                "issue_number": 117,
                "pr_number": 570,
                "status": STATUS_REVIEW_POOL,
                "title": "[Model] GraphPartitioning",
                "claimed": True,
                "claimed_status": STATUS_UNDER_REVIEW,
            },
        )

    def test_select_next_entry_honors_requested_number(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "ready-state.json"
            entry = select_next_entry(
                "ready",
                {
                    "items": [
                        make_issue_item("PVTI_1", 101, title="[Model] A"),
                        make_issue_item("PVTI_2", 102, title="[Model] B"),
                    ]
                },
                state_file,
                target_number=102,
            )
            self.assertEqual(
                entry,
                {
                    "item_id": "PVTI_2",
                    "number": 102,
                    "issue_number": 102,
                    "pr_number": None,
                    "status": STATUS_READY,
                    "title": "[Model] B",
                },
            )

    def test_select_next_entry_includes_ready_issue_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "ready-state.json"
            entry = select_next_entry(
                "ready",
                {
                    "items": [
                        make_issue_item(
                            "PVTI_1",
                            101,
                            title="[Model] ExactCoverBy3Sets",
                        )
                    ]
                },
                state_file,
            )
            self.assertEqual(
                entry,
                {
                    "item_id": "PVTI_1",
                    "number": 101,
                    "issue_number": 101,
                    "pr_number": None,
                    "status": STATUS_READY,
                    "title": "[Model] ExactCoverBy3Sets",
                },
            )

    def test_select_next_entry_includes_review_metadata(self) -> None:
        def fake_pr_resolver(repo: str, issue_number: int) -> int | None:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(issue_number, 117)
            return 570

        def fake_review_fetcher(repo: str, pr_number: int) -> list[dict]:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(pr_number, 570)
            return [{"user": {"login": "copilot-pull-request-reviewer[bot]"}}]

        def fake_pr_state_fetcher(repo: str, pr_number: int) -> str:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(pr_number, 570)
            return "OPEN"

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "review-state.json"
            entry = select_next_entry(
                "review",
                {
                    "items": [
                        make_issue_item(
                            "PVTI_10",
                            117,
                            status="Review pool",
                            title="[Model] GraphPartitioning",
                        )
                    ]
                },
                state_file,
                repo="CodingThrust/problem-reductions",
                review_fetcher=fake_review_fetcher,
                pr_resolver=fake_pr_resolver,
                pr_state_fetcher=fake_pr_state_fetcher,
            )
            self.assertEqual(
                entry,
                {
                    "item_id": "PVTI_10",
                    "number": 570,
                    "issue_number": 117,
                    "pr_number": 570,
                    "status": STATUS_REVIEW_POOL,
                    "title": "[Model] GraphPartitioning",
                },
            )

    def test_print_next_item_json_emits_rich_payload(self) -> None:
        buffer = io.StringIO()
        with redirect_stdout(buffer):
            rc = print_next_item(
                {
                    "item_id": "PVTI_20",
                    "number": 615,
                    "issue_number": 101,
                    "pr_number": 615,
                    "status": STATUS_FINAL_REVIEW,
                    "title": "[Model] MinimumFeedbackVertexSet",
                },
                mode="final-review",
                fmt="json",
            )

        self.assertEqual(rc, 0)
        self.assertEqual(
            json.loads(buffer.getvalue()),
            {
                "mode": "final-review",
                "item_id": "PVTI_20",
                "number": 615,
                "issue_number": 101,
                "pr_number": 615,
                "status": STATUS_FINAL_REVIEW,
                "title": "[Model] MinimumFeedbackVertexSet",
            },
        )


class PipelineBoardReviewCandidateTests(unittest.TestCase):
    def test_review_candidates_report_ambiguous_issue_cards(self) -> None:
        def fake_review_fetcher(repo: str, pr_number: int) -> list[dict]:
            raise AssertionError("ambiguous cards should not fetch reviews")

        def fake_pr_resolver(repo: str, issue_number: int) -> int | None:
            raise AssertionError("ambiguous cards should not resolve by issue search")

        def fake_pr_info_fetcher(repo: str, pr_number: int) -> dict:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            return {
                170: {"number": 170, "state": "CLOSED", "title": "Superseded LCS model"},
                173: {
                    "number": 173,
                    "state": "OPEN",
                    "title": "Fix #109: Add LCS reduction",
                },
            }[pr_number]

        candidates = review_candidates(
            {
                "items": [
                    make_issue_item(
                        "PVTI_10",
                        108,
                        status="Review pool",
                        title="[Model] LongestCommonSubsequence",
                        linked_prs=[170, 173],
                    )
                ]
            },
            "CodingThrust/problem-reductions",
            fake_review_fetcher,
            fake_pr_resolver,
            fake_pr_info_fetcher,
        )

        self.assertEqual(len(candidates), 1)
        self.assertEqual(
            candidates[0],
            {
                "item_id": "PVTI_10",
                "number": 173,
                "issue_number": 108,
                "pr_number": 173,
                "status": STATUS_REVIEW_POOL,
                "title": "[Model] LongestCommonSubsequence",
                "eligibility": "ambiguous-linked-prs",
                "reason": "multiple linked repo PRs require confirmation",
                "recommendation": 173,
                "linked_repo_prs": [
                    {
                        "number": 170,
                        "state": "CLOSED",
                        "title": "Superseded LCS model",
                    },
                    {
                        "number": 173,
                        "state": "OPEN",
                        "title": "Fix #109: Add LCS reduction",
                    },
                ],
            },
        )

    def test_review_candidates_report_waiting_for_copilot(self) -> None:
        def fake_review_fetcher(repo: str, pr_number: int) -> list[dict]:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(pr_number, 570)
            return []

        def fake_pr_resolver(repo: str, issue_number: int) -> int | None:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(issue_number, 117)
            return 570

        def fake_pr_info_fetcher(repo: str, pr_number: int) -> dict:
            self.assertEqual(repo, "CodingThrust/problem-reductions")
            self.assertEqual(pr_number, 570)
            return {
                "number": 570,
                "state": "OPEN",
                "title": "Fix #117: [Model] GraphPartitioning",
            }

        candidates = review_candidates(
            {
                "items": [
                    make_issue_item(
                        "PVTI_11",
                        117,
                        status="Review pool",
                        title="[Model] GraphPartitioning",
                    )
                ]
            },
            "CodingThrust/problem-reductions",
            fake_review_fetcher,
            fake_pr_resolver,
            fake_pr_info_fetcher,
        )

        self.assertEqual(candidates[0]["eligibility"], "waiting-for-copilot")
        self.assertEqual(candidates[0]["reason"], "open PR #570 waiting for Copilot review")


class PipelineBoardStatusListTests(unittest.TestCase):
    def test_status_items_list_ready_issues(self) -> None:
        items = status_items(
            {
                "items": [
                    make_issue_item(
                        "PVTI_1",
                        101,
                        status="Ready",
                        title="[Model] ExactCoverBy3Sets",
                    ),
                    make_issue_item(
                        "PVTI_2",
                        102,
                        status="In progress",
                        title="[Rule] A to B",
                    ),
                ]
            },
            STATUS_READY,
        )
        self.assertEqual(
            items,
            [
                {
                    "item_id": "PVTI_1",
                    "number": 101,
                    "issue_number": 101,
                    "pr_number": None,
                    "status": STATUS_READY,
                    "title": "[Model] ExactCoverBy3Sets",
                }
            ],
        )

    def test_status_items_list_in_progress_issues(self) -> None:
        items = status_items(
            {
                "items": [
                    make_issue_item(
                        "PVTI_1",
                        101,
                        status="Ready",
                        title="[Model] ExactCoverBy3Sets",
                    ),
                    make_issue_item(
                        "PVTI_2",
                        102,
                        status="In progress",
                        title="[Rule] A to B",
                    ),
                ]
            },
            STATUS_IN_PROGRESS,
        )
        self.assertEqual(
            items,
            [
                {
                    "item_id": "PVTI_2",
                    "number": 102,
                    "issue_number": 102,
                    "pr_number": None,
                    "status": STATUS_IN_PROGRESS,
                    "title": "[Rule] A to B",
                }
            ],
        )


if __name__ == "__main__":
    unittest.main()

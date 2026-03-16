#!/usr/bin/env python3
import io
import json
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path

from unittest.mock import patch

from pipeline_board import (
    STATUS_DONE,
    STATUS_FINAL_REVIEW,
    STATUS_IN_PROGRESS,
    STATUS_ON_HOLD,
    STATUS_READY,
    STATUS_REVIEW_POOL,
    STATUS_UNDER_REVIEW,
    ack_item,
    batch_fetch_issues,
    batch_fetch_prs_with_reviews,
    claim_next_entry,
    build_recovery_plan,
    claim_entry_from_entries,
    eligible_review_candidate_entries,
    normalize_status_name,
    print_next_item,
    process_snapshot,
    review_candidates,
    select_entry_from_entries,
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

    def test_parse_args_accepts_final_review_list_mode(self) -> None:
        import pipeline_board

        args = pipeline_board.parse_args(["list", "final-review", "--repo", "CodingThrust/problem-reductions"])

        self.assertEqual(args.command, "list")
        self.assertEqual(args.mode, "final-review")

    def test_main_lists_final_review_items(self) -> None:
        import pipeline_board

        board_data = {
            "items": [make_issue_item("PVTI_30", 239, status="Final review", title="[Model] BalancedCompleteBipartiteSubgraph")]
        }

        with (
            patch.object(pipeline_board, "fetch_board_items", return_value=board_data) as fetch_board_items,
            patch.object(pipeline_board, "print_candidate_list", return_value=0) as print_candidate_list,
        ):
            rc = pipeline_board.main(
                ["list", "final-review", "--repo", "CodingThrust/problem-reductions", "--format", "json"]
            )

        self.assertEqual(rc, 0)
        fetch_board_items.assert_called_once()
        print_candidate_list.assert_called_once_with(
            "final-review",
            [
                {
                    "number": 239,
                    "issue_number": 239,
                    "pr_number": None,
                    "status": "Final review",
                    "title": "[Model] BalancedCompleteBipartiteSubgraph",
                    "item_id": "PVTI_30",
                }
            ],
            fmt="json",
        )


class ReviewCandidateQueueTests(unittest.TestCase):
    def test_select_entry_from_entries_tracks_pending_until_ack(self) -> None:
        entries = eligible_review_candidate_entries(
            [
                {
                    "item_id": "PVTI_1",
                    "issue_number": 117,
                    "pr_number": 570,
                    "status": "Review pool",
                    "title": "[Model] GraphPartitioning",
                    "eligibility": "eligible",
                },
                {
                    "item_id": "PVTI_2",
                    "issue_number": 118,
                    "pr_number": 571,
                    "status": "Review pool",
                    "title": "[Rule] BinPacking to ILP",
                    "eligibility": "eligible",
                },
            ]
        )

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "review-candidates.json"

            first = select_entry_from_entries(entries, state_file)
            self.assertEqual(first["item_id"], "PVTI_1")
            self.assertEqual(first["pr_number"], 570)

            retry = select_entry_from_entries(entries, state_file)
            self.assertEqual(retry["item_id"], "PVTI_1")

            ack_item(state_file, "PVTI_1")
            second = select_entry_from_entries(entries, state_file)
            self.assertEqual(second["item_id"], "PVTI_2")
            self.assertEqual(second["pr_number"], 571)

    def test_claim_entry_from_entries_moves_selected_review_item(self) -> None:
        entries = eligible_review_candidate_entries(
            [
                {
                    "item_id": "PVTI_11",
                    "issue_number": 117,
                    "pr_number": 570,
                    "status": "Review pool",
                    "title": "[Model] GraphPartitioning",
                    "eligibility": "eligible",
                }
            ]
        )
        moves: list[tuple[str, str]] = []

        with tempfile.TemporaryDirectory() as tmpdir:
            state_file = Path(tmpdir) / "review-candidates.json"
            claimed = claim_entry_from_entries(
                "review",
                entries,
                state_file,
                mover=lambda item_id, status: moves.append((item_id, status)),
            )

        self.assertEqual(claimed["item_id"], "PVTI_11")
        self.assertEqual(claimed["claimed_status"], STATUS_UNDER_REVIEW)
        self.assertEqual(moves, [("PVTI_11", STATUS_UNDER_REVIEW)])


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


class ReviewCandidatesBatchTests(unittest.TestCase):
    def test_review_candidates_uses_batch_fetcher(self) -> None:
        """When batch_pr_fetcher is provided, individual fetchers are NOT called."""

        def fail_review_fetcher(repo: str, pr_number: int) -> list[dict]:
            raise AssertionError("should not be called when batch is available")

        def fail_pr_info_fetcher(repo: str, pr_number: int) -> dict:
            raise AssertionError("should not be called when batch is available")

        def fake_batch_pr_fetcher(
            repo: str, pr_numbers: list[int]
        ) -> dict[int, dict]:
            return {
                570: {
                    "number": 570,
                    "state": "OPEN",
                    "title": "Fix #117",
                    "url": "https://github.com/o/r/pull/570",
                    "reviews": [
                        {
                            "author": {"login": "copilot-pull-request-reviewer"},
                            "state": "COMMENTED",
                        },
                    ],
                }
            }

        candidates = review_candidates(
            {
                "items": [
                    make_pr_item("PVTI_1", 570, status="Review pool"),
                ]
            },
            "CodingThrust/problem-reductions",
            fail_review_fetcher,
            None,
            fail_pr_info_fetcher,
            batch_pr_fetcher=fake_batch_pr_fetcher,
        )

        self.assertEqual(len(candidates), 1)
        self.assertEqual(candidates[0]["eligibility"], "eligible")

    def test_review_candidates_batch_falls_back_for_resolved_prs(self) -> None:
        """pr_resolver results are not in the batch cache, so individual fetchers are used."""
        resolve_called = []

        def fake_pr_resolver(repo: str, issue_number: int) -> int | None:
            resolve_called.append(issue_number)
            return 580

        def fake_review_fetcher(repo: str, pr_number: int) -> list[dict]:
            return [
                {
                    "author": {"login": "copilot-pull-request-reviewer"},
                    "state": "COMMENTED",
                }
            ]

        def fake_pr_info_fetcher(repo: str, pr_number: int) -> dict:
            return {"number": 580, "state": "OPEN", "title": "Fix #120"}

        def fake_batch_pr_fetcher(
            repo: str, pr_numbers: list[int]
        ) -> dict[int, dict]:
            # No linked PRs known ahead of time for this issue
            return {}

        candidates = review_candidates(
            {
                "items": [
                    make_issue_item(
                        "PVTI_2", 120, status="Review pool", title="[Model] Foo"
                    ),
                ]
            },
            "CodingThrust/problem-reductions",
            fake_review_fetcher,
            fake_pr_resolver,
            fake_pr_info_fetcher,
            batch_pr_fetcher=fake_batch_pr_fetcher,
        )

        self.assertEqual(len(candidates), 1)
        self.assertEqual(candidates[0]["eligibility"], "eligible")
        self.assertEqual(resolve_called, [120])


class BatchFetchTests(unittest.TestCase):
    def test_batch_fetch_prs_with_reviews_builds_correct_query(self) -> None:
        fake_response = {
            "data": {
                "repository": {
                    "pr_42": {
                        "number": 42,
                        "state": "OPEN",
                        "title": "Fix foo",
                        "url": "https://github.com/o/r/pull/42",
                        "reviews": {
                            "nodes": [
                                {
                                    "author": {"login": "copilot-pull-request-reviewer"},
                                    "state": "COMMENTED",
                                },
                            ]
                        },
                    },
                    "pr_99": {
                        "number": 99,
                        "state": "CLOSED",
                        "title": "Old PR",
                        "url": "https://github.com/o/r/pull/99",
                        "reviews": {"nodes": []},
                    },
                }
            }
        }

        with patch(
            "pipeline_board.run_gh", return_value=json.dumps(fake_response)
        ) as mock_gh:
            result = batch_fetch_prs_with_reviews(
                "CodingThrust/problem-reductions", [42, 99]
            )

        self.assertEqual(set(result.keys()), {42, 99})
        self.assertEqual(result[42]["state"], "OPEN")
        self.assertEqual(
            result[42]["reviews"][0]["author"]["login"],
            "copilot-pull-request-reviewer",
        )
        self.assertEqual(result[99]["state"], "CLOSED")
        self.assertEqual(result[99]["reviews"], [])

        mock_gh.assert_called_once()
        call_args = mock_gh.call_args[0]
        self.assertEqual(call_args[0], "api")
        self.assertEqual(call_args[1], "graphql")

    def test_batch_fetch_prs_with_reviews_empty_list(self) -> None:
        result = batch_fetch_prs_with_reviews(
            "CodingThrust/problem-reductions", []
        )
        self.assertEqual(result, {})

    def test_batch_fetch_issues_builds_correct_query(self) -> None:
        fake_response = {
            "data": {
                "repository": {
                    "issue_42": {
                        "number": 42,
                        "title": "[Model] Foo",
                        "body": "## Definition\n...",
                        "state": "OPEN",
                        "url": "https://github.com/o/r/issues/42",
                        "labels": {"nodes": [{"name": "Model"}]},
                        "comments": {"nodes": [{"body": "looks good"}]},
                    },
                }
            }
        }

        with patch(
            "pipeline_board.run_gh", return_value=json.dumps(fake_response)
        ) as mock_gh:
            result = batch_fetch_issues("CodingThrust/problem-reductions", [42])

        self.assertEqual(set(result.keys()), {42})
        self.assertEqual(result[42]["title"], "[Model] Foo")
        self.assertEqual(result[42]["state"], "OPEN")
        self.assertEqual(result[42]["labels"], [{"name": "Model"}])
        self.assertEqual(result[42]["comments"], [{"body": "looks good"}])
        mock_gh.assert_called_once()

    def test_batch_fetch_issues_empty_list(self) -> None:
        result = batch_fetch_issues("CodingThrust/problem-reductions", [])
        self.assertEqual(result, {})


if __name__ == "__main__":
    unittest.main()

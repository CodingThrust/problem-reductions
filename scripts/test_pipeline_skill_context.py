#!/usr/bin/env python3
import io
import json
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock

import pipeline_skill_context


class PipelineSkillContextTests(unittest.TestCase):
    def test_parse_args_review_pipeline_defaults(self) -> None:
        args = pipeline_skill_context.parse_args(
            [
                "review-pipeline",
                "--repo",
                "CodingThrust/problem-reductions",
            ]
        )

        self.assertEqual(args.command, "review-pipeline")
        self.assertEqual(args.repo, "CodingThrust/problem-reductions")
        self.assertIsNone(args.pr)
        self.assertEqual(
            args.state_file,
            Path("/tmp/problemreductions-review-state.json"),
        )
        self.assertEqual(args.format, "json")

    def test_parse_args_final_review_with_explicit_values(self) -> None:
        args = pipeline_skill_context.parse_args(
            [
                "final-review",
                "--repo",
                "CodingThrust/problem-reductions",
                "--pr",
                "615",
                "--state-file",
                "/tmp/custom-final-review-state.json",
                "--format",
                "text",
            ]
        )

        self.assertEqual(args.command, "final-review")
        self.assertEqual(args.repo, "CodingThrust/problem-reductions")
        self.assertEqual(args.pr, 615)
        self.assertEqual(
            args.state_file,
            Path("/tmp/custom-final-review-state.json"),
        )
        self.assertEqual(args.format, "text")

    def test_parse_args_review_implementation_defaults(self) -> None:
        args = pipeline_skill_context.parse_args(
            [
                "review-implementation",
                "--format",
                "text",
            ]
        )

        self.assertEqual(args.command, "review-implementation")
        self.assertEqual(args.repo_root, Path("."))
        self.assertIsNone(args.kind)
        self.assertEqual(args.format, "text")

    def test_parse_args_project_pipeline_with_explicit_values(self) -> None:
        args = pipeline_skill_context.parse_args(
            [
                "project-pipeline",
                "--repo",
                "CodingThrust/problem-reductions",
                "--issue",
                "117",
                "--repo-root",
                "/tmp/repo",
                "--format",
                "text",
            ]
        )

        self.assertEqual(args.command, "project-pipeline")
        self.assertEqual(args.repo, "CodingThrust/problem-reductions")
        self.assertEqual(args.issue, 117)
        self.assertEqual(args.repo_root, Path("/tmp/repo"))
        self.assertEqual(args.format, "text")

    def test_emit_result_prints_sorted_json_for_all_formats(self) -> None:
        expected_output = '{\n  "a": 2,\n  "b": 1\n}\n'

        for fmt in ["json"]:
            with self.subTest(fmt=fmt):
                stdout = io.StringIO()
                with redirect_stdout(stdout):
                    pipeline_skill_context.emit_result({"b": 1, "a": 2}, fmt)
                self.assertEqual(stdout.getvalue(), expected_output)

    def test_emit_result_prints_final_review_text_report(self) -> None:
        result = {
            "skill": "final-review",
            "status": "ready",
            "selection": {
                "item_id": "PVTI_22",
                "pr_number": 615,
                "issue_number": 117,
                "title": "[Model] GraphPartitioning",
            },
            "pr": {
                "number": 615,
                "title": "Fix #117: [Model] GraphPartitioning",
                "url": "https://github.com/CodingThrust/problem-reductions/pull/615",
                "comments": {
                    "counts": {
                        "human_inline_comments": 1,
                        "human_issue_comments": 2,
                        "human_linked_issue_comments": 1,
                        "human_reviews": 1,
                    }
                },
                "issue_context_text": "Issue #117: Add GraphPartitioning model",
            },
            "prep": {
                "ready": False,
                "checkout": {"worktree_dir": "/tmp/final-pr-615"},
                "merge": {"status": "conflicted", "conflicts": ["src/models/graph_partitioning.rs"]},
            },
            "review_context": {
                "subject": {"kind": "model", "name": "GraphPartitioning"},
                "whitelist": {"ok": True, "violations": []},
                "completeness": {"ok": False, "missing": ["paper_display_name"]},
                "changed_files": ["src/models/graph_partitioning.rs", "docs/paper/reductions.typ"],
                "diff_stat": "2 files changed, 30 insertions(+), 2 deletions(-)",
            },
        }

        stdout = io.StringIO()
        with redirect_stdout(stdout):
            pipeline_skill_context.emit_result(result, "text")

        rendered = stdout.getvalue()
        self.assertIn("# Final Review Packet", rendered)
        self.assertIn("- PR: #615", rendered)
        self.assertIn("- Board item: `PVTI_22`", rendered)
        self.assertIn("## Recommendation Seed", rendered)
        self.assertIn("- Suggested mode: conflicted-review", rendered)
        self.assertIn("## Deterministic Checks", rendered)
        self.assertIn("- Completeness: fail", rendered)
        self.assertIn("- `paper_display_name`", rendered)
        self.assertIn("## Changed Files", rendered)

    def test_emit_result_prints_review_pipeline_text_report(self) -> None:
        result = {
            "skill": "review-pipeline",
            "status": "ready",
            "selection": {
                "item_id": "PVTI_11",
                "pr_number": 570,
                "issue_number": 117,
                "title": "[Model] GraphPartitioning",
            },
            "pr": {
                "number": 570,
                "title": "Fix #117: [Model] GraphPartitioning",
                "url": "https://github.com/CodingThrust/problem-reductions/pull/570",
                "comments": {
                    "counts": {
                        "copilot_inline_comments": 2,
                        "human_inline_comments": 1,
                        "human_issue_comments": 1,
                        "human_linked_issue_comments": 1,
                    }
                },
                "issue_context_text": "Issue #117: Add GraphPartitioning model",
                "ci": {"state": "failure", "failing": 1, "pending": 0},
                "codecov": {"found": True, "patch_coverage": 84.21},
            },
            "prep": {
                "ready": False,
                "checkout": {"worktree_dir": "/tmp/review-pr-570"},
                "merge": {"status": "conflicted", "conflicts": ["src/models/graph_partitioning.rs"]},
            },
        }

        stdout = io.StringIO()
        with redirect_stdout(stdout):
            pipeline_skill_context.emit_result(result, "text")

        rendered = stdout.getvalue()
        self.assertIn("# Review Pipeline Packet", rendered)
        self.assertIn("- PR: #570", rendered)
        self.assertIn("- Board item: `PVTI_11`", rendered)
        self.assertIn("## Recommendation Seed", rendered)
        self.assertIn("- Suggested mode: conflicted-fix", rendered)
        self.assertIn("- Copilot inline comments: 2", rendered)
        self.assertIn("- CI state: failure", rendered)
        self.assertIn("## Merge Prep", rendered)
        self.assertIn("- Worktree: `/tmp/review-pr-570`", rendered)
        self.assertIn("## Linked Issue Context", rendered)

    def test_emit_result_prints_review_implementation_text_report(self) -> None:
        result = {
            "skill": "review-implementation",
            "status": "ready",
            "git": {
                "repo_root": "/tmp/repo",
                "base_sha": "abc123",
                "head_sha": "def456",
            },
            "review_context": {
                "scope": {
                    "review_type": "model",
                    "models": [
                        {
                            "path": "src/models/graph/graph_partitioning.rs",
                            "problem_name": "GraphPartitioning",
                        }
                    ],
                    "rules": [],
                    "changed_files": [
                        "src/models/graph/graph_partitioning.rs",
                        "src/unit_tests/models/graph/graph_partitioning.rs",
                    ],
                },
                "subject": {"kind": "model", "name": "GraphPartitioning"},
                "changed_files": [
                    "src/models/graph/graph_partitioning.rs",
                    "src/unit_tests/models/graph/graph_partitioning.rs",
                ],
                "diff_stat": "2 files changed, 40 insertions(+)",
                "whitelist": {"ok": True, "skipped": False},
                "completeness": {"ok": False, "skipped": False, "missing": ["paper_display_name"]},
            },
            "current_pr": {
                "repo": "CodingThrust/problem-reductions",
                "pr_number": 615,
                "title": "Fix #117: [Model] GraphPartitioning",
                "linked_issue_number": 117,
                "issue_context_text": "# Add GraphPartitioning\n\nNeed canonical example.",
            },
        }

        stdout = io.StringIO()
        with redirect_stdout(stdout):
            pipeline_skill_context.emit_result(result, "text")

        rendered = stdout.getvalue()
        self.assertIn("# Review Implementation Packet", rendered)
        self.assertIn("- Base SHA: `abc123`", rendered)
        self.assertIn("- Review type: model", rendered)
        self.assertIn("- Name: GraphPartitioning", rendered)
        self.assertIn("- PR: #615", rendered)
        self.assertIn("- Linked issue: #117", rendered)
        self.assertIn("## Deterministic Checks", rendered)
        self.assertIn("- Completeness: fail", rendered)
        self.assertIn("## Linked Issue Context", rendered)

    def test_emit_result_prints_project_pipeline_text_report(self) -> None:
        result = {
            "skill": "project-pipeline",
            "status": "ready",
            "repo": "CodingThrust/problem-reductions",
            "existing_problems": ["BinPacking", "ILP", "GraphColoring"],
            "requested_issue": None,
            "ready_issues": [
                {
                    "item_id": "PVTI_1",
                    "issue_number": 117,
                    "title": "[Model] GraphPartitioning",
                    "kind": "model",
                    "eligible": True,
                    "blocking_reason": None,
                    "pending_rule_count": 2,
                    "summary": "Partition graph vertices into balanced groups.",
                },
                {
                    "item_id": "PVTI_2",
                    "issue_number": 130,
                    "title": "[Rule] MultivariateQuadratic to ILP",
                    "kind": "rule",
                    "eligible": False,
                    "blocking_reason": 'model "MultivariateQuadratic" not yet implemented on main',
                    "pending_rule_count": 0,
                    "source_problem": "MultivariateQuadratic",
                    "target_problem": "ILP",
                    "summary": "Linearize quadratic constraints.",
                },
            ],
            "in_progress_issues": [
                {
                    "issue_number": 129,
                    "title": "[Model] MultivariateQuadratic",
                }
            ],
        }

        stdout = io.StringIO()
        with redirect_stdout(stdout):
            pipeline_skill_context.emit_result(result, "text")

        rendered = stdout.getvalue()
        self.assertIn("# Project Pipeline Packet", rendered)
        self.assertIn("- Bundle status: ready", rendered)
        self.assertIn("- Ready issues: 2", rendered)
        self.assertIn("- In progress issues: 1", rendered)
        self.assertIn("## Eligible Ready Issues", rendered)
        self.assertIn("- #117 [Model] GraphPartitioning", rendered)
        self.assertIn("- Pending rules unblocked: 2", rendered)
        self.assertIn("## Blocked Ready Issues", rendered)
        self.assertIn('model "MultivariateQuadratic" not yet implemented on main', rendered)

    def test_build_status_result_normalizes_empty_state(self) -> None:
        self.assertEqual(
            pipeline_skill_context.build_status_result(
                "review-pipeline",
                status="empty",
            ),
            {
                "skill": "review-pipeline",
                "status": "empty",
            },
        )

    def test_build_status_result_normalizes_manual_choice_state(self) -> None:
        options = [{"item_id": "PVTI_1", "pr_number": 173}]

        self.assertEqual(
            pipeline_skill_context.build_status_result(
                "review-pipeline",
                status="needs-user-choice",
                options=options,
                recommendation=173,
            ),
            {
                "skill": "review-pipeline",
                "status": "needs-user-choice",
                "options": options,
                "recommendation": 173,
            },
        )

    def test_main_review_pipeline_emits_ready_bundle_shape(self) -> None:
        result = {
            "skill": "review-pipeline",
            "status": "ready",
            "selection": {"item_id": "PVTI_1", "pr_number": 173},
            "prep": {"ready": True},
            "pr": {"number": 173},
        }

        with mock.patch.object(
            pipeline_skill_context,
            "build_review_pipeline_context",
            return_value=result,
        ) as builder:
            stdout = io.StringIO()
            with redirect_stdout(stdout):
                exit_code = pipeline_skill_context.main(
                    [
                        "review-pipeline",
                        "--repo",
                        "CodingThrust/problem-reductions",
                    ]
                )

        builder.assert_called_once_with(
            repo="CodingThrust/problem-reductions",
            pr_number=None,
            state_file=Path("/tmp/problemreductions-review-state.json"),
        )
        self.assertEqual(exit_code, 0)
        self.assertEqual(json.loads(stdout.getvalue()), result)

    def test_main_final_review_emits_ready_bundle_shape(self) -> None:
        result = {
            "skill": "final-review",
            "status": "ready",
            "selection": {"item_id": "PVTI_2", "pr_number": 615},
            "prep": {"ready": True},
            "pr": {"number": 615},
            "review_context": {"files": ["src/lib.rs"]},
        }

        with mock.patch.object(
            pipeline_skill_context,
            "build_final_review_context",
            return_value=result,
        ) as builder:
            stdout = io.StringIO()
            with redirect_stdout(stdout):
                exit_code = pipeline_skill_context.main(
                    [
                        "final-review",
                        "--repo",
                        "CodingThrust/problem-reductions",
                        "--pr",
                        "615",
                    ]
                )

        builder.assert_called_once_with(
            repo="CodingThrust/problem-reductions",
            pr_number=615,
            state_file=Path("/tmp/problemreductions-final-review-state.json"),
        )
        self.assertEqual(exit_code, 0)
        self.assertEqual(json.loads(stdout.getvalue()), result)

    def test_main_review_implementation_emits_ready_bundle_shape(self) -> None:
        result = {
            "skill": "review-implementation",
            "status": "ready",
            "git": {"base_sha": "abc123", "head_sha": "def456"},
            "review_context": {"subject": {"kind": "generic"}},
            "current_pr": None,
        }

        with mock.patch.object(
            pipeline_skill_context,
            "build_review_implementation_context",
            return_value=result,
        ) as builder:
            stdout = io.StringIO()
            with redirect_stdout(stdout):
                exit_code = pipeline_skill_context.main(
                    [
                        "review-implementation",
                        "--repo-root",
                        ".",
                    ]
                )

        builder.assert_called_once_with(
            repo_root=Path("."),
            kind=None,
            name=None,
            source=None,
            target=None,
        )
        self.assertEqual(exit_code, 0)
        self.assertEqual(json.loads(stdout.getvalue()), result)

    def test_main_project_pipeline_emits_ready_bundle_shape(self) -> None:
        result = {
            "skill": "project-pipeline",
            "status": "ready",
            "ready_issues": [{"issue_number": 117}],
        }

        with mock.patch.object(
            pipeline_skill_context,
            "build_project_pipeline_context",
            return_value=result,
        ) as builder:
            stdout = io.StringIO()
            with redirect_stdout(stdout):
                exit_code = pipeline_skill_context.main(
                    [
                        "project-pipeline",
                        "--repo",
                        "CodingThrust/problem-reductions",
                        "--issue",
                        "117",
                    ]
                )

        builder.assert_called_once_with(
            repo="CodingThrust/problem-reductions",
            issue_number=117,
            repo_root=Path("."),
        )
        self.assertEqual(exit_code, 0)
        self.assertEqual(json.loads(stdout.getvalue()), result)

    def test_build_review_pipeline_context_reports_empty_queue(self) -> None:
        result = pipeline_skill_context.build_review_pipeline_context(
            repo="CodingThrust/problem-reductions",
            pr_number=None,
            state_file=Path("/tmp/problemreductions-review-state.json"),
            review_candidate_fetcher=lambda repo: [],
        )

        self.assertEqual(
            result,
            {
                "skill": "review-pipeline",
                "status": "empty",
            },
        )

    def test_build_review_pipeline_context_reports_manual_choice_for_ambiguous_card(self) -> None:
        result = pipeline_skill_context.build_review_pipeline_context(
            repo="CodingThrust/problem-reductions",
            pr_number=None,
            state_file=Path("/tmp/problemreductions-review-state.json"),
            review_candidate_fetcher=lambda repo: [
                {
                    "item_id": "PVTI_10",
                    "issue_number": 108,
                    "pr_number": 173,
                    "status": "Review pool",
                    "title": "[Model] LongestCommonSubsequence",
                    "eligibility": "ambiguous-linked-prs",
                    "recommendation": 173,
                    "linked_repo_prs": [
                        {"number": 170, "state": "CLOSED", "title": "Superseded LCS model"},
                        {"number": 173, "state": "OPEN", "title": "Fix #109: Add LCS reduction"},
                    ],
                }
            ],
        )

        self.assertEqual(
            result,
            {
                "skill": "review-pipeline",
                "status": "needs-user-choice",
                "options": [
                    {"number": 170, "state": "CLOSED", "title": "Superseded LCS model"},
                    {"number": 173, "state": "OPEN", "title": "Fix #109: Add LCS reduction"},
                ],
                "recommendation": 173,
            },
        )

    def test_build_review_pipeline_context_disambiguates_explicit_pr_choice(self) -> None:
        moves: list[tuple[str, str]] = []

        result = pipeline_skill_context.build_review_pipeline_context(
            repo="CodingThrust/problem-reductions",
            pr_number=173,
            state_file=Path("/tmp/problemreductions-review-state.json"),
            review_candidate_fetcher=lambda repo: [
                {
                    "item_id": "PVTI_10",
                    "issue_number": 108,
                    "pr_number": 173,
                    "status": "Review pool",
                    "title": "[Model] LongestCommonSubsequence",
                    "eligibility": "ambiguous-linked-prs",
                    "recommendation": 173,
                    "linked_repo_prs": [
                        {"number": 170, "state": "CLOSED", "title": "Superseded LCS model"},
                        {"number": 173, "state": "OPEN", "title": "Fix #109: Add LCS reduction"},
                    ],
                }
            ],
            mover=lambda item_id, status: moves.append((item_id, status)),
            pr_context_builder=lambda repo, pr_number: {
                "number": pr_number,
                "title": "Fix #109: Add LCS reduction",
            },
            review_preparer=lambda repo, pr_number: {
                "ready": True,
                "checkout": {"worktree_dir": "/tmp/review-pr-173"},
            },
        )

        self.assertEqual(moves, [("PVTI_10", "Under review")])
        self.assertEqual(
            result,
            {
                "skill": "review-pipeline",
                "status": "ready",
                "selection": {
                    "item_id": "PVTI_10",
                    "number": 173,
                    "issue_number": 108,
                    "pr_number": 173,
                    "status": "Review pool",
                    "title": "[Model] LongestCommonSubsequence",
                    "claimed": True,
                    "claimed_status": "Under review",
                },
                "pr": {
                    "number": 173,
                    "title": "Fix #109: Add LCS reduction",
                },
                "prep": {
                    "ready": True,
                    "checkout": {"worktree_dir": "/tmp/review-pr-173"},
                },
            },
        )

    def test_build_review_pipeline_context_returns_ready_bundle_for_eligible_pr(self) -> None:
        result = pipeline_skill_context.build_review_pipeline_context(
            repo="CodingThrust/problem-reductions",
            pr_number=None,
            state_file=Path("/tmp/problemreductions-review-state.json"),
            review_candidate_fetcher=lambda repo: [
                {
                    "item_id": "PVTI_11",
                    "issue_number": 117,
                    "pr_number": 570,
                    "status": "Review pool",
                    "title": "[Model] GraphPartitioning",
                    "eligibility": "eligible",
                    "reason": "copilot reviewed",
                }
            ],
            claim_entry=lambda **kwargs: {
                "item_id": "PVTI_11",
                "number": 570,
                "issue_number": 117,
                "pr_number": 570,
                "status": "Review pool",
                "title": "[Model] GraphPartitioning",
                "claimed": True,
                "claimed_status": "Under review",
            },
            pr_context_builder=lambda repo, pr_number: {
                "number": pr_number,
                "comments": {"counts": {"copilot_inline_comments": 1}},
            },
            review_preparer=lambda repo, pr_number: {
                "ready": True,
                "checkout": {"worktree_dir": "/tmp/review-pr-570"},
            },
        )

        self.assertEqual(
            result,
            {
                "skill": "review-pipeline",
                "status": "ready",
                "selection": {
                    "item_id": "PVTI_11",
                    "number": 570,
                    "issue_number": 117,
                    "pr_number": 570,
                    "status": "Review pool",
                    "title": "[Model] GraphPartitioning",
                    "claimed": True,
                    "claimed_status": "Under review",
                },
                "pr": {
                    "number": 570,
                    "comments": {"counts": {"copilot_inline_comments": 1}},
                },
                "prep": {
                    "ready": True,
                    "checkout": {"worktree_dir": "/tmp/review-pr-570"},
                },
            },
        )

    def test_build_review_pipeline_context_defaults_to_candidate_based_claim(self) -> None:
        with mock.patch.object(
            pipeline_skill_context.pipeline_board,
            "claim_entry_from_entries",
            return_value={
                "item_id": "PVTI_11",
                "number": 570,
                "issue_number": 117,
                "pr_number": 570,
                "status": "Review pool",
                "title": "[Model] GraphPartitioning",
                "claimed": True,
                "claimed_status": "Under review",
            },
        ) as claim_from_entries:
            result = pipeline_skill_context.build_review_pipeline_context(
                repo="CodingThrust/problem-reductions",
                pr_number=None,
                state_file=Path("/tmp/problemreductions-review-state.json"),
                review_candidate_fetcher=lambda repo: [
                    {
                        "item_id": "PVTI_11",
                        "issue_number": 117,
                        "pr_number": 570,
                        "status": "Review pool",
                        "title": "[Model] GraphPartitioning",
                        "eligibility": "eligible",
                        "reason": "copilot reviewed",
                    }
                ],
                pr_context_builder=lambda repo, pr_number: {"number": pr_number},
                review_preparer=lambda repo, pr_number: {"ready": True},
            )

        claim_from_entries.assert_called_once()
        self.assertEqual(result["status"], "ready")
        self.assertEqual(result["selection"]["pr_number"], 570)

    def test_build_review_pipeline_context_explicit_pr_defaults_to_candidate_based_claim(
        self,
    ) -> None:
        with mock.patch.object(
            pipeline_skill_context.pipeline_board,
            "claim_entry_from_entries",
            return_value={
                "item_id": "PVTI_11",
                "number": 570,
                "issue_number": 117,
                "pr_number": 570,
                "status": "Review pool",
                "title": "[Model] GraphPartitioning",
                "claimed": True,
                "claimed_status": "Under review",
            },
        ) as claim_from_entries:
            result = pipeline_skill_context.build_review_pipeline_context(
                repo="CodingThrust/problem-reductions",
                pr_number=570,
                state_file=Path("/tmp/problemreductions-review-state.json"),
                review_candidate_fetcher=lambda repo: [
                    {
                        "item_id": "PVTI_11",
                        "issue_number": 117,
                        "pr_number": 570,
                        "status": "Review pool",
                        "title": "[Model] GraphPartitioning",
                        "eligibility": "eligible",
                        "reason": "copilot reviewed",
                    }
                ],
                pr_context_builder=lambda repo, pr_number: {"number": pr_number},
                review_preparer=lambda repo, pr_number: {"ready": True},
            )

        claim_from_entries.assert_called_once_with(
            "review",
            {
                "PVTI_11": {
                    "number": 570,
                    "issue_number": 117,
                    "pr_number": 570,
                    "status": "Review pool",
                    "title": "[Model] GraphPartitioning",
                }
            },
            Path("/tmp/problemreductions-review-state.json"),
            target_number=570,
        )
        self.assertEqual(result["status"], "ready")
        self.assertEqual(result["selection"]["pr_number"], 570)

    def test_build_final_review_context_reports_empty_queue(self) -> None:
        result = pipeline_skill_context.build_final_review_context(
            repo="CodingThrust/problem-reductions",
            pr_number=None,
            state_file=Path("/tmp/problemreductions-final-review-state.json"),
            selection_fetcher=lambda **kwargs: None,
        )

        self.assertEqual(
            result,
            {
                "skill": "final-review",
                "status": "empty",
            },
        )

    def test_build_final_review_context_returns_ready_bundle_for_clean_prep(self) -> None:
        selection = {
            "item_id": "PVTI_22",
            "number": 615,
            "issue_number": 117,
            "pr_number": 615,
            "status": "Final review",
            "title": "[Model] GraphPartitioning",
        }
        prep = {
            "ready": True,
            "checkout": {
                "worktree_dir": "/tmp/final-pr-615",
                "base_sha": "abc123",
                "head_sha": "def456",
            },
            "merge": {"status": "clean", "conflicts": [], "likely_complex": False},
        }
        pr_context = {
            "number": 615,
            "title": "Fix #117: [Model] GraphPartitioning",
        }
        review_context = {
            "subject": {"kind": "model", "name": "GraphPartitioning"},
            "whitelist": {"ok": True},
            "completeness": {"ok": True},
        }

        result = pipeline_skill_context.build_final_review_context(
            repo="CodingThrust/problem-reductions",
            pr_number=None,
            state_file=Path("/tmp/problemreductions-final-review-state.json"),
            selection_fetcher=lambda **kwargs: selection,
            pr_context_builder=lambda repo, pr_number: pr_context,
            review_preparer=lambda repo, pr_number: prep,
            review_context_builder=lambda *, prep, pr_context: review_context,
        )

        self.assertEqual(
            result,
            {
                "skill": "final-review",
                "status": "ready",
                "selection": selection,
                "pr": pr_context,
                "prep": prep,
                "review_context": review_context,
            },
        )

    def test_build_final_review_context_keeps_review_context_on_conflicted_prep(self) -> None:
        selection = {
            "item_id": "PVTI_23",
            "number": 620,
            "issue_number": 118,
            "pr_number": 620,
            "status": "Final review",
            "title": "[Rule] BinPacking to ILP",
        }
        prep = {
            "ready": False,
            "checkout": {
                "worktree_dir": "/tmp/final-pr-620",
                "base_sha": "abc123",
                "head_sha": "def456",
            },
            "merge": {
                "status": "conflicted",
                "conflicts": ["src/rules/binpacking_ilp.rs"],
                "likely_complex": False,
            },
        }
        review_context = {
            "subject": {"kind": "rule", "name": "binpacking_ilp"},
            "whitelist": {"ok": True},
            "completeness": {"ok": True},
        }

        result = pipeline_skill_context.build_final_review_context(
            repo="CodingThrust/problem-reductions",
            pr_number=620,
            state_file=Path("/tmp/problemreductions-final-review-state.json"),
            selection_fetcher=lambda **kwargs: selection,
            pr_context_builder=lambda repo, pr_number: {"number": pr_number},
            review_preparer=lambda repo, pr_number: prep,
            review_context_builder=lambda *, prep, pr_context: review_context,
        )

        self.assertEqual(result["status"], "ready")
        self.assertEqual(result["prep"]["merge"]["status"], "conflicted")
        self.assertEqual(result["review_context"], review_context)

    def test_build_final_review_context_returns_warning_state_on_prep_failure(self) -> None:
        selection = {
            "item_id": "PVTI_24",
            "number": 621,
            "issue_number": 119,
            "pr_number": 621,
            "status": "Final review",
            "title": "[Model] FlowShopScheduling",
        }

        def fail_prepare(repo: str, pr_number: int) -> dict:
            raise RuntimeError("checkout failed")

        result = pipeline_skill_context.build_final_review_context(
            repo="CodingThrust/problem-reductions",
            pr_number=621,
            state_file=Path("/tmp/problemreductions-final-review-state.json"),
            selection_fetcher=lambda **kwargs: selection,
            pr_context_builder=lambda repo, pr_number: {"number": pr_number},
            review_preparer=fail_prepare,
        )

        self.assertEqual(
            result,
            {
                "skill": "final-review",
                "status": "ready-with-warnings",
                "selection": selection,
                "pr": {"number": 621},
                "prep": {"ready": False, "error": "checkout failed"},
                "review_context": None,
                "warnings": [
                    "failed to prepare final-review worktree: checkout failed",
                ],
            },
        )

    def test_build_review_implementation_context_without_current_pr(self) -> None:
        result = pipeline_skill_context.build_review_implementation_context(
            repo_root=Path("/tmp/repo"),
            kind=None,
            name=None,
            source=None,
            target=None,
            merge_base_getter=lambda repo_root: "abc123",
            head_sha_getter=lambda repo_root: "def456",
            diff_stat_getter=lambda repo_root, base_sha, head_sha: "2 files changed",
            changed_files_getter=lambda repo_root, base_sha, head_sha: [
                "src/lib.rs",
                "src/unit_tests/lib.rs",
            ],
            added_files_getter=lambda repo_root, base_sha, head_sha: [],
            current_pr_fetcher=lambda: None,
            review_context_builder=lambda repo_root, **kwargs: {
                "scope": {"review_type": "generic", "models": [], "rules": [], "changed_files": kwargs["changed_files"]},
                "subject": {"kind": "generic"},
                "changed_files": kwargs["changed_files"],
                "diff_stat": kwargs["diff_stat"],
                "whitelist": {"ok": True, "skipped": True},
                "completeness": {"ok": True, "skipped": True, "missing": []},
            },
        )

        self.assertEqual(result["skill"], "review-implementation")
        self.assertEqual(result["status"], "ready")
        self.assertEqual(result["git"]["base_sha"], "abc123")
        self.assertEqual(result["current_pr"], None)
        self.assertEqual(result["review_context"]["subject"]["kind"], "generic")

    def test_build_project_pipeline_context_reports_requested_blocked_issue(self) -> None:
        board_data = {
            "items": [
                {
                    "id": "PVTI_2",
                    "status": "Ready",
                    "content": {
                        "type": "Issue",
                        "number": 130,
                        "title": "[Rule] MultivariateQuadratic to ILP",
                    },
                }
            ]
        }
        issue_data = {
            130: {
                "number": 130,
                "title": "[Rule] MultivariateQuadratic to ILP",
                "body": "Linearize quadratic constraints.",
                "comments": [],
                "labels": [],
                "url": "https://github.com/CodingThrust/problem-reductions/issues/130",
            }
        }

        result = pipeline_skill_context.build_project_pipeline_context(
            repo="CodingThrust/problem-reductions",
            issue_number=130,
            repo_root=Path("/tmp/repo"),
            board_fetcher=lambda repo: board_data,
            issue_fetcher=lambda repo, issue_number: issue_data[issue_number],
            existing_problem_finder=lambda repo_root: {"ILP"},
        )

        self.assertEqual(result["skill"], "project-pipeline")
        self.assertEqual(result["status"], "requested-blocked")
        self.assertEqual(result["requested_issue"]["issue_number"], 130)
        self.assertEqual(
            result["requested_issue"]["blocking_reason"],
            'model "MultivariateQuadratic" not yet implemented on main',
        )


if __name__ == "__main__":
    unittest.main()

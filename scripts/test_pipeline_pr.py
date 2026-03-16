#!/usr/bin/env python3
import io
import unittest
from contextlib import redirect_stdout
from unittest import mock

from pipeline_pr import (
    build_current_pr_context,
    build_context_result,
    build_pr_context,
    build_linked_issue_result,
    build_snapshot,
    create_pr,
    emit_result,
    edit_pr_body,
    extract_codecov_summary,
    extract_linked_issue_number,
    format_issue_context,
    post_pr_comment,
    parse_args,
    summarize_check_runs,
    summarize_comments,
    wait_for_ci,
)


def make_inline_comment(
    login: str,
    *,
    body: str = "comment",
    path: str = "src/lib.rs",
    line: int = 7,
) -> dict:
    return {
        "user": {"login": login},
        "body": body,
        "path": path,
        "line": line,
        "original_line": line,
    }


def make_review(
    login: str,
    *,
    body: str = "review body",
    state: str = "COMMENTED",
) -> dict:
    return {
        "user": {"login": login},
        "body": body,
        "state": state,
    }


def make_issue_comment(login: str, *, body: str = "discussion") -> dict:
    return {
        "user": {"login": login},
        "body": body,
    }


def make_check_run(
    name: str,
    *,
    status: str = "completed",
    conclusion: str | None = "success",
) -> dict:
    return {
        "name": name,
        "status": status,
        "conclusion": conclusion,
    }


class PipelinePrHelpersTests(unittest.TestCase):
    def test_extract_linked_issue_number_prefers_body_over_title(self) -> None:
        linked_issue = extract_linked_issue_number(
            "Fix #117: Add GraphPartitioning model",
            "This supersedes older work and closes #42.",
        )
        self.assertEqual(linked_issue, 42)

    def test_extract_linked_issue_number_falls_back_to_title(self) -> None:
        linked_issue = extract_linked_issue_number(
            "Fix #117: Add GraphPartitioning model",
            "No explicit closing keyword here.",
        )
        self.assertEqual(linked_issue, 117)

    def test_summarize_comments_splits_human_copilot_and_codecov_sources(self) -> None:
        summary = summarize_comments(
            inline_comments=[
                make_inline_comment("copilot-pull-request-reviewer[bot]"),
                make_inline_comment("alice"),
            ],
            reviews=[
                make_review("bob", body="Please add tests"),
                make_review("copilot-pull-request-reviewer[bot]", body="bot review"),
                make_review("carol", body=""),
            ],
            issue_comments=[
                make_issue_comment("dave", body="Please update docs"),
                make_issue_comment("codecov[bot]", body="## [Codecov] Patch coverage is 82%"),
            ],
            linked_issue_comments=[
                make_issue_comment("erin", body="The literature citation is important"),
                make_issue_comment("deploy-bot[bot]", body="automated deployment note"),
            ],
        )

        self.assertEqual(summary["counts"]["inline_comments"], 2)
        self.assertEqual(summary["counts"]["copilot_inline_comments"], 1)
        self.assertEqual(summary["counts"]["human_inline_comments"], 1)
        self.assertEqual(summary["counts"]["human_reviews"], 1)
        self.assertEqual(summary["counts"]["human_issue_comments"], 1)
        self.assertEqual(summary["counts"]["human_linked_issue_comments"], 1)
        self.assertEqual(summary["counts"]["codecov_comments"], 1)

    def test_extract_codecov_summary_parses_latest_comment_and_filepaths(self) -> None:
        summary = extract_codecov_summary(
            [
                make_issue_comment("alice", body="looks fine"),
                make_issue_comment(
                    "codecov[bot]",
                    body=(
                        "## [Codecov]\n"
                        "Patch coverage: `84.21%`\n"
                        "Project coverage is `91.30%`\n"
                        "https://codecov.io/gh/CodingThrust/problem-reductions?"
                        "filepath=src%2Fmodels%2Fgraph%2Ffoo.rs&line=17\n"
                        "https://codecov.io/gh/CodingThrust/problem-reductions?"
                        "filepath=src%2Fmodels%2Fgraph%2Ffoo.rs&line=21\n"
                        "https://codecov.io/gh/CodingThrust/problem-reductions?"
                        "filepath=src%2Frules%2Ffoo_bar.rs&line=8\n"
                    ),
                ),
            ]
        )

        self.assertTrue(summary["found"])
        self.assertEqual(summary["patch_coverage"], 84.21)
        self.assertEqual(summary["project_coverage"], 91.30)
        self.assertEqual(
            summary["filepaths"],
            [
                "src/models/graph/foo.rs",
                "src/rules/foo_bar.rs",
            ],
        )

    def test_summarize_check_runs_reports_overall_state(self) -> None:
        self.assertEqual(
            summarize_check_runs({"check_runs": [make_check_run("test", status="queued", conclusion=None)]})["state"],
            "pending",
        )
        self.assertEqual(
            summarize_check_runs({"check_runs": [make_check_run("test", conclusion="failure")]})["state"],
            "failure",
        )
        self.assertEqual(
            summarize_check_runs(
                {
                    "check_runs": [
                        make_check_run("fmt"),
                        make_check_run("coverage", conclusion="neutral"),
                    ]
                }
            )["state"],
            "success",
        )

    def test_build_snapshot_includes_linked_issue_ci_and_codecov(self) -> None:
        snapshot = build_snapshot(
            {
                "number": 570,
                "title": "Fix #117: Add GraphPartitioning model",
                "body": "Closes #117",
                "state": "OPEN",
                "url": "https://github.com/CodingThrust/problem-reductions/pull/570",
                "headRefName": "feature/graph-partitioning",
                "baseRefName": "main",
                "mergeable": "MERGEABLE",
                "headRefOid": "abc123",
                "labels": [{"name": "model"}],
                "files": [{"path": "src/models/graph/graph_partitioning.rs"}],
                "commits": [{"oid": "abc123"}, {"oid": "def456"}],
                "additions": 120,
                "deletions": 7,
            },
            linked_issue_number=117,
            linked_issue={
                "number": 117,
                "title": "[Model] GraphPartitioning",
                "state": "OPEN",
            },
            ci_summary={"state": "success", "total": 3, "failing": 0, "pending": 0},
            codecov_summary={"found": True, "patch_coverage": 84.21, "filepaths": ["src/models/graph/graph_partitioning.rs"]},
        )

        self.assertEqual(snapshot["number"], 570)
        self.assertEqual(snapshot["linked_issue_number"], 117)
        self.assertEqual(snapshot["linked_issue"]["title"], "[Model] GraphPartitioning")
        self.assertEqual(snapshot["ci"]["state"], "success")
        self.assertEqual(snapshot["codecov"]["patch_coverage"], 84.21)
        self.assertEqual(snapshot["counts"]["files"], 1)
        self.assertEqual(snapshot["counts"]["commits"], 2)

    def test_build_current_pr_context_includes_repo_and_pr_fields(self) -> None:
        current = build_current_pr_context(
            "CodingThrust/problem-reductions",
            {
                "number": 570,
                "title": "Fix #117: Add GraphPartitioning model",
                "headRefName": "feature/graph-partitioning",
                "url": "https://github.com/CodingThrust/problem-reductions/pull/570",
            },
        )

        self.assertEqual(current["repo"], "CodingThrust/problem-reductions")
        self.assertEqual(current["pr_number"], 570)
        self.assertEqual(current["title"], "Fix #117: Add GraphPartitioning model")
        self.assertEqual(current["head_ref_name"], "feature/graph-partitioning")

    def test_format_issue_context_includes_title_body_and_comments(self) -> None:
        issue_context = format_issue_context(
            {
                "number": 117,
                "title": "[Model] GraphPartitioning",
                "body": "Implement the model.",
                "state": "OPEN",
            },
            [
                {
                    "author": "maintainer",
                    "body": "Use the paper notation.",
                    "created_at": "2026-03-15T09:00:00Z",
                    "is_bot": False,
                }
            ],
        )

        self.assertIn("# [Model] GraphPartitioning", issue_context)
        self.assertIn("Implement the model.", issue_context)
        self.assertIn("## Comments", issue_context)
        self.assertIn("**maintainer** (2026-03-15T09:00:00Z):", issue_context)

    def test_build_linked_issue_result_includes_normalized_comments_and_context(self) -> None:
        result = build_linked_issue_result(
            pr_number=570,
            linked_issue_number=117,
            linked_issue={
                "number": 117,
                "title": "[Model] GraphPartitioning",
                "body": "Implement the model.",
                "state": "OPEN",
                "url": "https://github.com/CodingThrust/problem-reductions/issues/117",
            },
            linked_issue_comments=[
                {
                    "user": {"login": "maintainer"},
                    "body": "Use the paper notation.",
                    "created_at": "2026-03-15T09:00:00Z",
                },
                {
                    "user": {"login": "deploy-bot[bot]"},
                    "body": "Automated message.",
                    "created_at": "2026-03-15T09:05:00Z",
                },
            ],
        )

        self.assertEqual(result["pr_number"], 570)
        self.assertEqual(result["linked_issue_number"], 117)
        self.assertEqual(len(result["linked_issue_comments"]), 2)
        self.assertEqual(result["human_linked_issue_comments"][0]["author"], "maintainer")
        self.assertIn("# [Model] GraphPartitioning", result["issue_context_text"])
        self.assertIn("**maintainer** (2026-03-15T09:00:00Z):", result["issue_context_text"])

    def test_build_context_result_merges_snapshot_comments_and_issue_context(self) -> None:
        snapshot = {
            "number": 570,
            "title": "Fix #117: Add GraphPartitioning model",
            "body": "Closes #117",
            "state": "OPEN",
            "url": "https://github.com/CodingThrust/problem-reductions/pull/570",
            "mergeable": "MERGEABLE",
            "head_ref_name": "feature/graph-partitioning",
            "base_ref_name": "main",
            "head_sha": "abc123",
            "linked_issue_number": 117,
            "linked_issue": {"number": 117, "title": "[Model] GraphPartitioning"},
            "files": ["src/models/graph/graph_partitioning.rs"],
            "commits": ["abc123", "def456"],
            "ci": {"state": "success"},
            "codecov": {"found": True, "patch_coverage": 92.0},
            "counts": {"files": 1, "commits": 2},
        }
        comments = {
            "inline_comments": [{"user": "alice", "body": "nit"}],
            "reviews": [{"user": "bob", "body": "looks good", "state": "COMMENTED"}],
            "human_issue_comments": [{"user": "carol", "body": "please update docs"}],
            "counts": {"inline_comments": 1},
        }
        linked_issue_result = {
            "linked_issue_number": 117,
            "linked_issue": {"number": 117, "title": "[Model] GraphPartitioning"},
            "linked_issue_comments": [{"author": "maintainer", "body": "Use paper notation"}],
            "human_linked_issue_comments": [{"author": "maintainer", "body": "Use paper notation"}],
            "issue_context_text": "# [Model] GraphPartitioning\n\nImplement the model.",
        }

        context = build_context_result(
            "CodingThrust/problem-reductions",
            snapshot,
            comments,
            linked_issue_result,
        )

        self.assertEqual(context["repo"], "CodingThrust/problem-reductions")
        self.assertEqual(context["pr_number"], 570)
        self.assertEqual(context["title"], "Fix #117: Add GraphPartitioning model")
        self.assertEqual(context["comments"]["inline_comments"][0]["user"], "alice")
        self.assertEqual(context["ci"]["state"], "success")
        self.assertEqual(context["codecov"]["patch_coverage"], 92.0)
        self.assertEqual(context["linked_issue_number"], 117)
        self.assertIn("GraphPartitioning", context["issue_context_text"])

    def test_emit_result_prints_context_text_report(self) -> None:
        context = {
            "repo": "CodingThrust/problem-reductions",
            "pr_number": 570,
            "title": "Fix #117: Add GraphPartitioning model",
            "url": "https://github.com/CodingThrust/problem-reductions/pull/570",
            "head_sha": "abc123",
            "comments": {
                "counts": {
                    "copilot_inline_comments": 2,
                    "human_inline_comments": 1,
                    "human_issue_comments": 1,
                    "human_linked_issue_comments": 1,
                    "human_reviews": 1,
                }
            },
            "ci": {"state": "failure", "failing": 1, "pending": 0},
            "codecov": {
                "found": True,
                "patch_coverage": 84.21,
                "project_coverage": 91.3,
                "filepaths": ["src/models/graph/graph_partitioning.rs"],
            },
            "linked_issue_number": 117,
            "issue_context_text": "# [Model] GraphPartitioning\n\nImplement the model.",
        }

        stdout = io.StringIO()
        with redirect_stdout(stdout):
            emit_result(context, "text")

        rendered = stdout.getvalue()
        self.assertIn("# PR Context Packet", rendered)
        self.assertIn("- PR: #570", rendered)
        self.assertIn("- Repo: CodingThrust/problem-reductions", rendered)
        self.assertIn("- Head SHA: `abc123`", rendered)
        self.assertIn("## Comment Summary", rendered)
        self.assertIn("- Copilot inline comments: 2", rendered)
        self.assertIn("## CI Summary", rendered)
        self.assertIn("- State: failure", rendered)
        self.assertIn("## Codecov", rendered)
        self.assertIn("- Patch coverage: 84.21%", rendered)
        self.assertIn("## Linked Issue Context", rendered)

    @mock.patch("pipeline_pr.build_linked_issue_context")
    @mock.patch("pipeline_pr.build_comments_summary")
    @mock.patch("pipeline_pr.build_pr_snapshot")
    def test_build_pr_context_assembles_existing_helper_results(
        self,
        build_pr_snapshot: mock.Mock,
        build_comments_summary: mock.Mock,
        build_linked_issue_context: mock.Mock,
    ) -> None:
        build_pr_snapshot.return_value = {
            "number": 570,
            "title": "Fix #117: Add GraphPartitioning model",
            "body": "Closes #117",
            "state": "OPEN",
            "url": "https://github.com/CodingThrust/problem-reductions/pull/570",
            "mergeable": "MERGEABLE",
            "head_ref_name": "feature/graph-partitioning",
            "base_ref_name": "main",
            "head_sha": "abc123",
            "linked_issue_number": 117,
            "linked_issue": {"number": 117, "title": "[Model] GraphPartitioning"},
            "files": ["src/models/graph/graph_partitioning.rs"],
            "commits": ["abc123"],
            "ci": {"state": "success"},
            "codecov": {"found": True},
            "counts": {"files": 1, "commits": 1},
        }
        build_comments_summary.return_value = {"inline_comments": [], "counts": {"inline_comments": 0}}
        build_linked_issue_context.return_value = {
            "linked_issue_number": 117,
            "linked_issue": {"number": 117, "title": "[Model] GraphPartitioning"},
            "linked_issue_comments": [],
            "human_linked_issue_comments": [],
            "issue_context_text": "# [Model] GraphPartitioning",
        }

        context = build_pr_context("CodingThrust/problem-reductions", 570)

        build_pr_snapshot.assert_called_once_with("CodingThrust/problem-reductions", 570)
        build_comments_summary.assert_called_once_with("CodingThrust/problem-reductions", 570)
        build_linked_issue_context.assert_called_once()
        self.assertEqual(context["pr_number"], 570)
        self.assertEqual(context["issue_context_text"], "# [Model] GraphPartitioning")

    def test_wait_for_ci_polls_until_terminal_state(self) -> None:
        summaries = [
            {"state": "pending", "total": 2, "pending": 1, "failing": 0},
            {"state": "pending", "total": 2, "pending": 1, "failing": 0},
            {"state": "success", "total": 2, "pending": 0, "failing": 0},
        ]
        sleeps: list[float] = []
        now = [0.0]

        def fake_fetcher() -> dict:
            return summaries.pop(0)

        def fake_monotonic() -> float:
            return now[0]

        def fake_sleep(seconds: float) -> None:
            sleeps.append(seconds)
            now[0] += seconds

        result = wait_for_ci(
            fake_fetcher,
            timeout_seconds=30,
            interval_seconds=5,
            monotonic_fn=fake_monotonic,
            sleep_fn=fake_sleep,
        )

        self.assertEqual(result["state"], "success")
        self.assertEqual(sleeps, [5, 5])

    @mock.patch("pipeline_pr.subprocess.check_call")
    def test_post_pr_comment_uses_gh_pr_comment_with_body_file(self, check_call: mock.Mock) -> None:
        post_pr_comment(
            "CodingThrust/problem-reductions",
            570,
            "/tmp/comment.md",
        )

        check_call.assert_called_once_with(
            [
                "gh",
                "pr",
                "comment",
                "570",
                "--repo",
                "CodingThrust/problem-reductions",
                "--body-file",
                "/tmp/comment.md",
            ]
        )

    @mock.patch("pipeline_pr.subprocess.check_call")
    def test_edit_pr_body_uses_gh_pr_edit_with_body_file(self, check_call: mock.Mock) -> None:
        edit_pr_body(
            "CodingThrust/problem-reductions",
            570,
            "/tmp/body.md",
        )

        check_call.assert_called_once_with(
            [
                "gh",
                "pr",
                "edit",
                "570",
                "--repo",
                "CodingThrust/problem-reductions",
                "--body-file",
                "/tmp/body.md",
            ]
        )

    @mock.patch("pipeline_pr.fetch_current_pr_data_for_repo")
    @mock.patch("pipeline_pr.run_gh_checked")
    def test_create_pr_uses_gh_pr_create_and_returns_current_context(
        self,
        run_gh_checked: mock.Mock,
        fetch_current_pr_data_for_repo: mock.Mock,
    ) -> None:
        fetch_current_pr_data_for_repo.return_value = {
            "number": 570,
            "title": "Fix #117: Add GraphPartitioning model",
            "headRefName": "issue-117-graph-partitioning",
            "url": "https://github.com/CodingThrust/problem-reductions/pull/570",
        }

        result = create_pr(
            "CodingThrust/problem-reductions",
            "Fix #117: Add GraphPartitioning model",
            "/tmp/pr-body.md",
            base="main",
            head="issue-117-graph-partitioning",
        )

        run_gh_checked.assert_called_once_with(
            "pr",
            "create",
            "--repo",
            "CodingThrust/problem-reductions",
            "--title",
            "Fix #117: Add GraphPartitioning model",
            "--body-file",
            "/tmp/pr-body.md",
            "--base",
            "main",
            "--head",
            "issue-117-graph-partitioning",
        )
        fetch_current_pr_data_for_repo.assert_called_once_with("CodingThrust/problem-reductions")
        self.assertEqual(result["pr_number"], 570)
        self.assertEqual(result["repo"], "CodingThrust/problem-reductions")

    def test_parse_args_accepts_comment_and_edit_body_commands(self) -> None:
        comment_args = parse_args(
            [
                "comment",
                "--repo",
                "CodingThrust/problem-reductions",
                "--pr",
                "570",
                "--body-file",
                "/tmp/comment.md",
            ]
        )
        self.assertEqual(comment_args.command, "comment")
        self.assertEqual(comment_args.body_file, "/tmp/comment.md")

        edit_args = parse_args(
            [
                "edit-body",
                "--repo",
                "CodingThrust/problem-reductions",
                "--pr",
                "570",
                "--body-file",
                "/tmp/body.md",
            ]
        )
        self.assertEqual(edit_args.command, "edit-body")
        self.assertEqual(edit_args.body_file, "/tmp/body.md")

        current_args = parse_args(["current", "--format", "json"])
        self.assertEqual(current_args.command, "current")

        linked_issue_args = parse_args(
            [
                "linked-issue",
                "--repo",
                "CodingThrust/problem-reductions",
                "--pr",
                "570",
                "--format",
                "json",
            ]
        )
        self.assertEqual(linked_issue_args.command, "linked-issue")

        create_args = parse_args(
            [
                "create",
                "--repo",
                "CodingThrust/problem-reductions",
                "--title",
                "Fix #117: Add GraphPartitioning model",
                "--body-file",
                "/tmp/pr-body.md",
                "--base",
                "main",
                "--head",
                "issue-117-graph-partitioning",
                "--format",
                "json",
            ]
        )
        self.assertEqual(create_args.command, "create")
        self.assertEqual(create_args.body_file, "/tmp/pr-body.md")

        context_args = parse_args(
            [
                "context",
                "--repo",
                "CodingThrust/problem-reductions",
                "--pr",
                "570",
                "--format",
                "json",
            ]
        )
        self.assertEqual(context_args.command, "context")
        self.assertEqual(context_args.pr, 570)

        current_context_args = parse_args(["context", "--current", "--format", "json"])
        self.assertEqual(current_context_args.command, "context")
        self.assertTrue(current_context_args.current)


if __name__ == "__main__":
    unittest.main()

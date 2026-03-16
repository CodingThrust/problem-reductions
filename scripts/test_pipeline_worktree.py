#!/usr/bin/env python3
import unittest
from unittest import mock

import pipeline_worktree
from pipeline_worktree import (
    prepare_issue_branch,
    plan_issue_worktree,
    plan_pr_worktree,
    summarize_merge,
)


class PipelineWorktreeTests(unittest.TestCase):
    def test_plan_issue_worktree_sanitizes_slug_and_uses_worktrees_dir(self) -> None:
        plan = plan_issue_worktree(
            "/tmp/problemreductions",
            issue_number=117,
            slug="Graph Partitioning / Exact",
            base_ref="origin/main",
        )

        self.assertEqual(plan["branch"], "issue-117-graph-partitioning-exact")
        self.assertEqual(
            plan["worktree_dir"],
            "/tmp/problemreductions/.worktrees/issue-117-graph-partitioning-exact",
        )
        self.assertEqual(plan["base_ref"], "origin/main")

    def test_plan_pr_worktree_uses_pull_ref_and_sanitized_local_branch(self) -> None:
        plan = plan_pr_worktree(
            "/tmp/problemreductions",
            pr_number=570,
            head_ref_name="feature/lcs cleanup",
            base_sha="base123",
            head_sha="head456",
        )

        self.assertEqual(plan["local_branch"], "review-pr-570-feature-lcs-cleanup")
        self.assertEqual(
            plan["worktree_dir"],
            "/tmp/problemreductions/.worktrees/review-pr-570-feature-lcs-cleanup",
        )
        self.assertEqual(
            plan["fetch_ref"],
            "pull/570/head:review-pr-570-feature-lcs-cleanup",
        )
        self.assertEqual(plan["base_sha"], "base123")
        self.assertEqual(plan["head_sha"], "head456")

    def test_summarize_merge_clean_result(self) -> None:
        summary = summarize_merge(
            worktree="/tmp/problemreductions/.worktrees/review-pr-570",
            exit_code=0,
            conflicts=[],
        )

        self.assertEqual(summary["status"], "clean")
        self.assertFalse(summary["likely_complex"])
        self.assertEqual(summary["conflicts"], [])

    def test_summarize_merge_conflicted_result_marks_complex_skill_conflicts(self) -> None:
        summary = summarize_merge(
            worktree="/tmp/problemreductions/.worktrees/review-pr-570",
            exit_code=1,
            conflicts=[
                ".claude/skills/add-model/SKILL.md",
                "src/models/graph/graph_partitioning.rs",
            ],
        )

        self.assertEqual(summary["status"], "conflicted")
        self.assertTrue(summary["likely_complex"])
        self.assertEqual(
            summary["conflicts"],
            [
                ".claude/skills/add-model/SKILL.md",
                "src/models/graph/graph_partitioning.rs",
            ],
        )

    def test_summarize_merge_without_conflicts_is_aborted(self) -> None:
        summary = summarize_merge(
            worktree="/tmp/problemreductions/.worktrees/review-pr-570",
            exit_code=128,
            conflicts=[],
        )

        self.assertEqual(summary["status"], "aborted")
        self.assertFalse(summary["likely_complex"])

    @mock.patch("pipeline_worktree.merge_main")
    @mock.patch("pipeline_worktree.checkout_pr_worktree")
    def test_prepare_review_bundles_checkout_and_clean_merge(
        self,
        checkout_pr_worktree: mock.Mock,
        merge_main: mock.Mock,
    ) -> None:
        prepare_review = getattr(pipeline_worktree, "prepare_review", None)
        self.assertIsNotNone(prepare_review)

        checkout_payload = {
            "pr_number": 570,
            "head_ref_name": "feature/lcs cleanup",
            "local_branch": "review-pr-570-feature-lcs-cleanup",
            "worktree_dir": "/tmp/problemreductions/.worktrees/review-pr-570-feature-lcs-cleanup",
            "fetch_ref": "pull/570/head:review-pr-570-feature-lcs-cleanup",
            "base_sha": "base123",
            "head_sha": "head456",
        }
        merge_payload = {
            "worktree": "/tmp/problemreductions/.worktrees/review-pr-570-feature-lcs-cleanup",
            "status": "clean",
            "conflicts": [],
            "likely_complex": False,
            "stdout": "Already up to date.\n",
            "stderr": "",
        }
        checkout_pr_worktree.return_value = checkout_payload
        merge_main.return_value = merge_payload

        result = prepare_review(
            repo="CodingThrust/problem-reductions",
            pr_number=570,
            repo_root="/tmp/problemreductions",
        )

        self.assertEqual(result["checkout"], checkout_payload)
        self.assertEqual(result["merge"], merge_payload)
        self.assertTrue(result["ready"])
        checkout_pr_worktree.assert_called_once_with(
            repo="CodingThrust/problem-reductions",
            pr_number=570,
            repo_root="/tmp/problemreductions",
        )
        merge_main.assert_called_once_with(
            worktree="/tmp/problemreductions/.worktrees/review-pr-570-feature-lcs-cleanup"
        )

    @mock.patch("pipeline_worktree.merge_main")
    @mock.patch("pipeline_worktree.checkout_pr_worktree")
    def test_prepare_review_marks_conflicted_merge_not_ready(
        self,
        checkout_pr_worktree: mock.Mock,
        merge_main: mock.Mock,
    ) -> None:
        prepare_review = getattr(pipeline_worktree, "prepare_review", None)
        self.assertIsNotNone(prepare_review)

        checkout_pr_worktree.return_value = {
            "pr_number": 570,
            "head_ref_name": "feature/lcs cleanup",
            "local_branch": "review-pr-570-feature-lcs-cleanup",
            "worktree_dir": "/tmp/problemreductions/.worktrees/review-pr-570-feature-lcs-cleanup",
            "fetch_ref": "pull/570/head:review-pr-570-feature-lcs-cleanup",
            "base_sha": "base123",
            "head_sha": "head456",
        }
        merge_main.return_value = {
            "worktree": "/tmp/problemreductions/.worktrees/review-pr-570-feature-lcs-cleanup",
            "status": "conflicted",
            "conflicts": [
                ".claude/skills/add-model/SKILL.md",
                "src/models/graph/graph_partitioning.rs",
            ],
            "likely_complex": True,
            "stdout": "Auto-merging ...\n",
            "stderr": "CONFLICT (content): Merge conflict in .claude/skills/add-model/SKILL.md\n",
        }

        result = prepare_review(
            repo="CodingThrust/problem-reductions",
            pr_number=570,
            repo_root="/tmp/problemreductions",
        )

        self.assertFalse(result["ready"])
        self.assertEqual(
            result["merge"]["conflicts"],
            [
                ".claude/skills/add-model/SKILL.md",
                "src/models/graph/graph_partitioning.rs",
            ],
        )
        self.assertTrue(result["merge"]["likely_complex"])

    @mock.patch("pipeline_worktree.run_git_checked")
    @mock.patch("pipeline_worktree.run_git")
    def test_prepare_issue_branch_creates_new_branch_when_missing(
        self,
        run_git: mock.Mock,
        run_git_checked: mock.Mock,
    ) -> None:
        run_git.side_effect = [
            "",  # git status --porcelain
            "abc123\n",  # rev-parse main
            "def456\n",  # rev-parse HEAD
        ]

        with mock.patch("pipeline_worktree.branch_exists", return_value=False):
            result = prepare_issue_branch(
                issue_number=117,
                slug="Graph Partitioning",
                base_ref="main",
                repo_root="/tmp/problemreductions",
            )

        self.assertEqual(result["branch"], "issue-117-graph-partitioning")
        self.assertEqual(result["action"], "create-branch")
        self.assertFalse(result["existing_branch"])
        tails = [call.args[1:] for call in run_git_checked.call_args_list]
        self.assertIn(("checkout", "main"), tails)
        self.assertIn(("checkout", "-b", "issue-117-graph-partitioning"), tails)

    @mock.patch("pipeline_worktree.run_git_checked")
    @mock.patch("pipeline_worktree.run_git")
    def test_prepare_issue_branch_reuses_existing_branch(
        self,
        run_git: mock.Mock,
        run_git_checked: mock.Mock,
    ) -> None:
        run_git.side_effect = [
            "",  # git status --porcelain
            "abc123\n",  # rev-parse main
            "def456\n",  # rev-parse HEAD
        ]

        with mock.patch("pipeline_worktree.branch_exists", return_value=True):
            result = prepare_issue_branch(
                issue_number=117,
                slug="Graph Partitioning",
                base_ref="main",
                repo_root="/tmp/problemreductions",
            )

        self.assertEqual(result["branch"], "issue-117-graph-partitioning")
        self.assertEqual(result["action"], "checkout-existing")
        self.assertTrue(result["existing_branch"])
        tails = [call.args[1:] for call in run_git_checked.call_args_list]
        self.assertIn(("checkout", "main"), tails)
        self.assertIn(("checkout", "issue-117-graph-partitioning"), tails)


if __name__ == "__main__":
    unittest.main()

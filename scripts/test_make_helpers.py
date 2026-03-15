#!/usr/bin/env python3
import shutil
import subprocess
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]


class MakeHelpersTests(unittest.TestCase):
    def test_helper_sources_under_dash(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            ["dash", "-c", ". scripts/make_helpers.sh"],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)

    def test_run_agent_enables_multi_agent_for_codex(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "codex() { printf '%s\\n' \"$@\"; }; "
                    "RUNNER=codex CODEX_MODEL=test-model "
                    "run_agent /dev/null 'test prompt'"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(
            proc.stdout.splitlines(),
            [
                "exec",
                "--enable",
                "multi_agent",
                "-m",
                "test-model",
                "-s",
                "danger-full-access",
                "test prompt",
            ],
        )

    def test_poll_project_items_uses_pipeline_board_cli(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "poll_project_items ready /tmp/state.json"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(
            proc.stdout.splitlines(),
            [
                "scripts/pipeline_board.py",
                "next",
                "ready",
                "/tmp/state.json",
            ],
        )

    def test_move_board_item_uses_pipeline_board_cli(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "move_board_item PVTI_demo final-review"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(
            proc.stdout.splitlines(),
            [
                "scripts/pipeline_board.py",
                "move",
                "PVTI_demo",
                "final-review",
            ],
        )

    def test_make_board_next_final_review_passes_repo(self) -> None:
        proc = subprocess.run(
            [
                "make",
                "-n",
                "board-next",
                "MODE=final-review",
                "REPO=CodingThrust/problem-reductions",
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn(
            'poll_project_items "final-review" "$state_file" "$repo"',
            proc.stdout,
        )

    def test_pr_snapshot_uses_pipeline_pr_cli(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "pr_snapshot CodingThrust/problem-reductions 570"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(
            proc.stdout.splitlines(),
            [
                "scripts/pipeline_pr.py",
                "snapshot",
                "--repo",
                "CodingThrust/problem-reductions",
                "--pr",
                "570",
                "--format",
                "json",
            ],
        )

    def test_pr_wait_ci_uses_pipeline_pr_cli(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "pr_wait_ci CodingThrust/problem-reductions 570 1200 15"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(
            proc.stdout.splitlines(),
            [
                "scripts/pipeline_pr.py",
                "wait-ci",
                "--repo",
                "CodingThrust/problem-reductions",
                "--pr",
                "570",
                "--timeout",
                "1200",
                "--interval",
                "15",
                "--format",
                "json",
            ],
        )

    def test_create_issue_worktree_uses_pipeline_worktree_cli(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "create_issue_worktree 117 graph-partitioning origin/main"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(
            proc.stdout.splitlines(),
            [
                "scripts/pipeline_worktree.py",
                "create-issue",
                "--issue",
                "117",
                "--slug",
                "graph-partitioning",
                "--base",
                "origin/main",
                "--format",
                "json",
            ],
        )

    def test_checkout_pr_worktree_uses_pipeline_worktree_cli(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "checkout_pr_worktree CodingThrust/problem-reductions 570"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(
            proc.stdout.splitlines(),
            [
                "scripts/pipeline_worktree.py",
                "checkout-pr",
                "--repo",
                "CodingThrust/problem-reductions",
                "--pr",
                "570",
                "--format",
                "json",
            ],
        )


if __name__ == "__main__":
    unittest.main()

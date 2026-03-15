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

    def test_skill_prompt_with_context_appends_json_for_codex(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "RUNNER=codex "
                    "skill_prompt_with_context review-pipeline '/review-pipeline 570' "
                    "'process PR #570' 'Selected queue item' '{\"pr_number\":570}'"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("Use the repo-local skill", proc.stdout)
        self.assertIn("Selected queue item", proc.stdout)
        self.assertIn('{"pr_number":570}', proc.stdout)

    def test_skill_prompt_with_context_keeps_claude_slash_command_clean(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "RUNNER=claude "
                    "skill_prompt_with_context review-pipeline '/review-pipeline 570' "
                    "'process PR #570' 'Selected queue item' '{\"pr_number\":570}'"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(proc.stdout.strip(), "/review-pipeline 570")

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
                "--format",
                "text",
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

    def test_claim_project_items_uses_pipeline_board_cli(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "claim_project_items ready /tmp/state.json"
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
                "claim-next",
                "ready",
                "/tmp/state.json",
                "--format",
                "json",
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

    def test_make_board_next_review_forwards_number_and_format(self) -> None:
        proc = subprocess.run(
            [
                "make",
                "-n",
                "board-next",
                "MODE=review",
                "REPO=CodingThrust/problem-reductions",
                "NUMBER=570",
                "FORMAT=json",
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn(
            'poll_project_items "review" "$state_file" "$repo" "570" "json"',
            proc.stdout,
        )

    def test_make_board_claim_review_forwards_repo_number_and_format(self) -> None:
        proc = subprocess.run(
            [
                "make",
                "-n",
                "board-claim",
                "MODE=review",
                "REPO=CodingThrust/problem-reductions",
                "NUMBER=570",
                "FORMAT=json",
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn(
            'claim_project_items "review" "$state_file" "$repo" "570" "json"',
            proc.stdout,
        )

    def test_board_next_json_uses_scripted_json_poll(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "board_next_json review CodingThrust/problem-reductions 570 /tmp/review.json"
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
                "review",
                "/tmp/review.json",
                "--format",
                "json",
                "--repo",
                "CodingThrust/problem-reductions",
                "--number",
                "570",
            ],
        )

    def test_board_claim_json_uses_scripted_json_claim(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "board_claim_json review CodingThrust/problem-reductions 570 /tmp/review.json"
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
                "claim-next",
                "review",
                "/tmp/review.json",
                "--format",
                "json",
                "--repo",
                "CodingThrust/problem-reductions",
                "--number",
                "570",
            ],
        )

    def test_make_run_review_uses_scripted_board_selection(self) -> None:
        proc = subprocess.run(
            ["make", "-n", "run-review"],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn('board_next_json review "$repo"', proc.stdout)
        self.assertIn('skill_prompt_with_context review-pipeline', proc.stdout)

    def test_make_run_pipeline_uses_scripted_board_selection(self) -> None:
        proc = subprocess.run(
            ["make", "-n", "run-pipeline"],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn('board_next_json ready "" "" "$state_file"', proc.stdout)
        self.assertIn('skill_prompt_with_context project-pipeline', proc.stdout)

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

    def test_issue_guards_uses_pipeline_checks_cli(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "issue_guards CodingThrust/problem-reductions 117 /tmp/repo"
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
                "scripts/pipeline_checks.py",
                "issue-guards",
                "--repo",
                "CodingThrust/problem-reductions",
                "--issue",
                "117",
                "--repo-root",
                "/tmp/repo",
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

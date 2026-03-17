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
                "--repo-root",
                ".",
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
                "--repo-root",
                ".",
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

    def test_review_pipeline_context_uses_skill_bundle_cli(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "review_pipeline_context CodingThrust/problem-reductions 570 /tmp/review.json"
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
                "scripts/pipeline_skill_context.py",
                "review-pipeline",
                "--repo",
                "CodingThrust/problem-reductions",
                "--state-file",
                "/tmp/review.json",
                "--format",
                "json",
                "--pr",
                "570",
            ],
        )

    def test_make_run_review_uses_skill_bundle_context(self) -> None:
        proc = subprocess.run(
            ["make", "-n", "run-review"],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn('review_pipeline_context "$repo"', proc.stdout)
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

    def test_watch_and_dispatch_uses_persistent_default_state_file(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "poll_project_items() { printf 'state:%s\\n' \"$2\" >&2; return 2; }; "
                    "watch_and_dispatch ready run-pipeline 'Ready issues'"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 2, proc.stderr)
        self.assertIn(
            "state:/tmp/problemreductions-ready-forever-state.json",
            proc.stderr,
        )

    def test_watch_and_dispatch_sleeps_after_successful_dispatch(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "flag=/tmp/test-watch-and-dispatch-$$; "
                    "rm -f \"$flag\"; "
                    "date() { printf '2026-03-16 00:00:00'; }; "
                    "poll_project_items() { "
                    "  if [ ! -f \"$flag\" ]; then : > \"$flag\"; printf 'PVTI_1\\t42\\n'; return 0; fi; "
                    "  return 2; "
                    "}; "
                    "make() { printf 'make:%s %s\\n' \"$1\" \"$2\"; return 0; }; "
                    "ack_polled_item() { printf 'ack:%s\\n' \"$2\"; }; "
                    "sleep() { printf 'sleep:%s\\n' \"$1\"; return 0; }; "
                    "MAKE=make POLL_INTERVAL=600 "
                    "watch_and_dispatch ready run-pipeline 'Ready issues'"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 2, proc.stderr)
        self.assertIn("make:run-pipeline N=42", proc.stdout)
        self.assertIn("ack:PVTI_1", proc.stdout)
        self.assertIn("sleep:600", proc.stdout)

    def test_watch_and_dispatch_uses_long_cache_when_pending_above_threshold(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "state_tmp=$(mktemp); "
                    'printf \'{"pending":["a","b","c","d","e"],"visible":{}}\' > "$state_tmp"; '
                    "poll_project_items() { printf 'max_age:%s\\n' \"$7\" >&2; return 2; }; "
                    "STATE_FILE=\"$state_tmp\" CACHE_THRESHOLD=5 "
                    "watch_and_dispatch ready run-pipeline 'Ready issues'"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 2, proc.stderr)
        # Above threshold: cache is reused but max_age is still the poll interval
        self.assertIn("max_age:600", proc.stderr)

    def test_watch_and_dispatch_invalidates_cache_when_pending_below_threshold(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "state_tmp=$(mktemp); "
                    'printf \'{"pending":["a","b"],"visible":{}}\' > "$state_tmp"; '
                    "poll_project_items() { printf 'max_age:%s\\n' \"$7\" >&2; return 2; }; "
                    "STATE_FILE=\"$state_tmp\" CACHE_THRESHOLD=5 "
                    "watch_and_dispatch ready run-pipeline 'Ready issues'"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 2, proc.stderr)
        # Below threshold: cache is invalidated but max_age is still the poll interval
        self.assertIn("max_age:600", proc.stderr)

    def test_watch_and_dispatch_requests_copilot_reviews_in_review_mode(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "state_tmp=$(mktemp); "
                    'printf \'{"pending":[],"visible":{}}\' > "$state_tmp"; '
                    "request_copilot_reviews() { printf 'copilot_reviews:%s\\n' \"$1\" >&2; }; "
                    "poll_project_items() { return 2; }; "
                    "STATE_FILE=\"$state_tmp\" CACHE_THRESHOLD=5 "
                    "watch_and_dispatch review run-review 'Copilot-reviewed PRs' myorg/myrepo"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 2, proc.stderr)
        self.assertIn("copilot_reviews:myorg/myrepo", proc.stderr)

    def test_watch_and_dispatch_skips_copilot_reviews_when_pending_above_threshold(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "state_tmp=$(mktemp); "
                    'printf \'{"pending":["a","b","c","d","e"],"visible":{}}\' > "$state_tmp"; '
                    "request_copilot_reviews() { printf 'copilot_reviews_called\\n' >&2; }; "
                    "poll_project_items() { return 2; }; "
                    "STATE_FILE=\"$state_tmp\" CACHE_THRESHOLD=5 "
                    "watch_and_dispatch review run-review 'Copilot-reviewed PRs' myorg/myrepo"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 2, proc.stderr)
        self.assertNotIn("copilot_reviews_called", proc.stderr)

    def test_watch_and_dispatch_pending_count_handles_missing_state_file(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "poll_project_items() { printf 'max_age:%s\\n' \"$7\" >&2; return 2; }; "
                    "STATE_FILE=/tmp/nonexistent-state-$$.json CACHE_THRESHOLD=5 "
                    "watch_and_dispatch ready run-pipeline 'Ready issues'"
                ),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 2, proc.stderr)
        # pending_count=0 < threshold=5, so cache should be invalidated; max_age = poll interval
        self.assertIn("max_age:600", proc.stderr)

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

    def test_issue_context_uses_pipeline_checks_cli(self) -> None:
        if shutil.which("dash") is None:
            self.skipTest("dash is not installed")

        proc = subprocess.run(
            [
                "dash",
                "-c",
                (
                    ". scripts/make_helpers.sh; "
                    "python3() { printf '%s\\n' \"$@\"; }; "
                    "issue_context CodingThrust/problem-reductions 117 /tmp/repo"
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
                "issue-context",
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

    def test_make_issue_context_uses_shared_helper(self) -> None:
        proc = subprocess.run(
            [
                "make",
                "-n",
                "issue-context",
                "ISSUE=117",
                "REPO=CodingThrust/problem-reductions",
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn(
            'issue_context "$repo" "117"',
            proc.stdout,
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

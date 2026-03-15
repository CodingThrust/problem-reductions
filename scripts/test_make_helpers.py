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


if __name__ == "__main__":
    unittest.main()

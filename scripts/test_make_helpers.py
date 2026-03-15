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


if __name__ == "__main__":
    unittest.main()

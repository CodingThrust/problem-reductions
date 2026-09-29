#!/usr/bin/env python3
import io
import json
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock

import pipeline_skill_context


class PipelineSkillContextTests(unittest.TestCase):
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

    def test_emit_result_prints_sorted_json_for_all_formats(self) -> None:
        expected_output = '{\n  "a": 2,\n  "b": 1\n}\n'

        for fmt in ["json"]:
            with self.subTest(fmt=fmt):
                stdout = io.StringIO()
                with redirect_stdout(stdout):
                    pipeline_skill_context.emit_result({"b": 1, "a": 2}, fmt)
                self.assertEqual(stdout.getvalue(), expected_output)

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
        }

        stdout = io.StringIO()
        with redirect_stdout(stdout):
            pipeline_skill_context.emit_result(result, "text")

        rendered = stdout.getvalue()
        self.assertIn("# Review Implementation Packet", rendered)
        self.assertIn("- Base SHA: `abc123`", rendered)
        self.assertIn("- Review type: model", rendered)
        self.assertIn("- Name: GraphPartitioning", rendered)
        self.assertNotIn("## Current PR", rendered)
        self.assertIn("## Deterministic Checks", rendered)
        self.assertIn("- Completeness: fail", rendered)

    def test_main_review_implementation_emits_ready_bundle_shape(self) -> None:
        result = {
            "skill": "review-implementation",
            "status": "ready",
            "git": {"base_sha": "abc123", "head_sha": "def456"},
            "review_context": {"subject": {"kind": "generic"}},
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
        self.assertNotIn("current_pr", result)
        self.assertEqual(result["review_context"]["subject"]["kind"], "generic")

if __name__ == "__main__":
    unittest.main()

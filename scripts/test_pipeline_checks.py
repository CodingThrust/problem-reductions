#!/usr/bin/env python3
import subprocess
import tempfile
import unittest
from unittest import mock
from pathlib import Path

import pipeline_checks
from pipeline_checks import (
    build_review_context,
    completeness_check,
    detect_scope_from_paths,
    file_whitelist_check,
    infer_review_subject,
    issue_guard_check,
    parse_args,
)


class PipelineChecksTests(unittest.TestCase):
    @mock.patch("pipeline_checks.run_gh_json")
    def test_fetch_existing_prs_falls_back_to_rest_search_on_pr_list_failure(
        self,
        run_gh_json: mock.Mock,
    ) -> None:
        run_gh_json.side_effect = [
            subprocess.CalledProcessError(1, ["gh", "pr", "list"]),
            {
                "items": [
                    {
                        "number": 223,
                        "pull_request": {
                            "url": "https://api.github.com/repos/CodingThrust/problem-reductions/pulls/223"
                        },
                    }
                ]
            },
            {
                "number": 223,
                "head": {"ref": "issue-212-multiprocessor-scheduling"},
                "html_url": "https://example.test/pull/223",
            },
        ]

        prs = pipeline_checks.fetch_existing_prs(
            "CodingThrust/problem-reductions",
            212,
        )

        self.assertEqual(
            prs,
            [
                {
                    "number": 223,
                    "headRefName": "issue-212-multiprocessor-scheduling",
                    "url": "https://example.test/pull/223",
                }
            ],
        )

    def test_detect_scope_reports_model_review_for_new_model_file(self) -> None:
        scope = detect_scope_from_paths(
            added_files=["src/models/graph/graph_partitioning.rs"],
            changed_files=[
                "src/models/graph/graph_partitioning.rs",
                "src/unit_tests/models/graph/graph_partitioning.rs",
            ],
        )

        self.assertEqual(scope["review_type"], "model")
        self.assertEqual(scope["models"][0]["category"], "graph")
        self.assertEqual(scope["models"][0]["file_stem"], "graph_partitioning")
        self.assertEqual(scope["models"][0]["problem_name"], "GraphPartitioning")

    def test_detect_scope_reports_rule_review_for_new_rule_file(self) -> None:
        scope = detect_scope_from_paths(
            added_files=["src/rules/binpacking_ilp.rs"],
            changed_files=["src/rules/binpacking_ilp.rs"],
        )

        self.assertEqual(scope["review_type"], "rule")
        self.assertEqual(scope["rules"][0]["rule_stem"], "binpacking_ilp")

    def test_detect_scope_reports_generic_when_no_new_model_or_rule_files(self) -> None:
        scope = detect_scope_from_paths(
            added_files=[],
            changed_files=["src/lib.rs", "docs/paper/reductions.typ"],
        )

        self.assertEqual(scope["review_type"], "generic")
        self.assertEqual(scope["models"], [])
        self.assertEqual(scope["rules"], [])

    def test_infer_review_subject_prefers_explicit_rule_metadata(self) -> None:
        scope = {
            "review_type": "rule",
            "models": [],
            "rules": [{"rule_stem": "binpacking_ilp"}],
            "changed_files": ["src/rules/binpacking_ilp.rs"],
        }

        subject = infer_review_subject(
            scope,
            kind="rule",
            name="binpacking_ilp",
            source="BinPacking",
            target="ILP",
        )

        self.assertEqual(subject["kind"], "rule")
        self.assertEqual(subject["name"], "binpacking_ilp")
        self.assertEqual(subject["source"], "BinPacking")
        self.assertEqual(subject["target"], "ILP")
        self.assertFalse(subject["inferred"])

    def test_infer_review_subject_infers_model_from_scope(self) -> None:
        scope = {
            "review_type": "model",
            "models": [{"problem_name": "GraphPartitioning"}],
            "rules": [],
            "changed_files": ["src/models/graph/graph_partitioning.rs"],
        }

        subject = infer_review_subject(scope)

        self.assertEqual(subject["kind"], "model")
        self.assertEqual(subject["name"], "GraphPartitioning")
        self.assertTrue(subject["inferred"])

    def test_file_whitelist_accepts_expected_model_files(self) -> None:
        report = file_whitelist_check(
            "model",
            [
                "src/models/graph/graph_partitioning.rs",
                "src/unit_tests/models/graph/graph_partitioning.rs",
                "src/example_db/model_builders.rs",
                "docs/paper/reductions.typ",
            ],
        )

        self.assertTrue(report["ok"])
        self.assertEqual(report["violations"], [])

    def test_file_whitelist_flags_unexpected_model_files(self) -> None:
        report = file_whitelist_check(
            "model",
            [
                "src/models/graph/graph_partitioning.rs",
                "Cargo.toml",
            ],
        )

        self.assertFalse(report["ok"])
        self.assertEqual(report["violations"][0]["path"], "Cargo.toml")

    def test_file_whitelist_accepts_expected_rule_files(self) -> None:
        report = file_whitelist_check(
            "rule",
            [
                "src/rules/binpacking_ilp.rs",
                "src/rules/mod.rs",
                "src/unit_tests/rules/binpacking_ilp.rs",
                "src/example_db/rule_builders.rs",
                "src/models/graph/bin_packing.rs",
                "docs/paper/reductions.typ",
            ],
        )

        self.assertTrue(report["ok"])
        self.assertEqual(report["violations"], [])

    def test_model_completeness_reports_all_required_components(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            repo = Path(tmpdir)
            self._write(
                repo / "src/models/graph/graph_partitioning.rs",
                """
                inventory::submit! { ProblemSchemaEntry { name: "GraphPartitioning" } }
                impl OptimizationProblem for GraphPartitioning<SimpleGraph> {}
                crate::declare_variants! { opt GraphPartitioning<SimpleGraph> => "1.2^n" }
                pub(crate) fn canonical_model_example_specs() -> Vec<ModelExampleSpec> { vec![] }
                """,
            )
            self._write(
                repo / "src/unit_tests/models/graph/graph_partitioning.rs",
                "#[test]\nfn test_graph_partitioning_basic() {}\n",
            )
            self._write(repo / "src/example_db/model_builders.rs", "pub fn build_model_examples() {}\n")
            self._write(
                repo / "docs/paper/reductions.typ",
                """
                #let display-name = (
                  "GraphPartitioning": [Graph Partitioning],
                )
                #problem-def("GraphPartitioning")[body][proof]
                """,
            )

            report = completeness_check("model", repo, name="GraphPartitioning")

            self.assertTrue(report["ok"])
            self.assertEqual(report["missing"], [])
            self.assertEqual(report["checks"]["paper_display_name"]["status"], "pass")

    def test_model_completeness_flags_missing_paper_entries(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            repo = Path(tmpdir)
            self._write(
                repo / "src/models/graph/graph_partitioning.rs",
                """
                inventory::submit! { ProblemSchemaEntry { name: "GraphPartitioning" } }
                impl OptimizationProblem for GraphPartitioning<SimpleGraph> {}
                crate::declare_variants! { opt GraphPartitioning<SimpleGraph> => "1.2^n" }
                pub(crate) fn canonical_model_example_specs() -> Vec<ModelExampleSpec> { vec![] }
                """,
            )
            self._write(
                repo / "src/unit_tests/models/graph/graph_partitioning.rs",
                "#[test]\nfn test_graph_partitioning_basic() {}\n",
            )
            self._write(repo / "src/example_db/model_builders.rs", "pub fn build_model_examples() {}\n")
            self._write(repo / "docs/paper/reductions.typ", "#let display-name = ()\n")

            report = completeness_check("model", repo, name="GraphPartitioning")

            self.assertFalse(report["ok"])
            self.assertIn("paper_definition", report["missing"])
            self.assertIn("paper_display_name", report["missing"])

    def test_rule_completeness_reports_all_required_components(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            repo = Path(tmpdir)
            self._write(
                repo / "src/rules/binpacking_ilp.rs",
                """
                #[reduction(overhead = { num_vars = "num_items" })]
                impl ReduceTo<ILP> for BinPacking {}
                pub(crate) fn canonical_rule_example_specs() -> Vec<RuleExampleSpec> { vec![] }
                """,
            )
            self._write(repo / "src/rules/mod.rs", "mod binpacking_ilp;\n")
            self._write(
                repo / "src/unit_tests/rules/binpacking_ilp.rs",
                "#[test]\nfn test_binpacking_to_ilp_closed_loop() {}\n",
            )
            self._write(repo / "src/example_db/rule_builders.rs", "pub fn build_rule_examples() {}\n")
            self._write(
                repo / "docs/paper/reductions.typ",
                '#reduction-rule("BinPacking", "ILP")[rule][proof]\n',
            )

            report = completeness_check(
                "rule",
                repo,
                name="binpacking_ilp",
                source="BinPacking",
                target="ILP",
            )

            self.assertTrue(report["ok"])
            self.assertEqual(report["checks"]["module_registration"]["status"], "pass")
            self.assertEqual(report["checks"]["paper_rule"]["status"], "pass")

    def test_rule_completeness_flags_missing_overhead_and_paper(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            repo = Path(tmpdir)
            self._write(
                repo / "src/rules/binpacking_ilp.rs",
                """
                #[reduction]
                impl ReduceTo<ILP> for BinPacking {}
                """,
            )
            self._write(repo / "src/rules/mod.rs", "")
            self._write(
                repo / "src/unit_tests/rules/binpacking_ilp.rs",
                "#[test]\nfn test_binpacking_to_ilp_closed_loop() {}\n",
            )
            self._write(repo / "src/example_db/rule_builders.rs", "pub fn build_rule_examples() {}\n")
            self._write(repo / "docs/paper/reductions.typ", "")

            report = completeness_check(
                "rule",
                repo,
                name="binpacking_ilp",
                source="BinPacking",
                target="ILP",
            )

            self.assertFalse(report["ok"])
            self.assertIn("overhead_form", report["missing"])
            self.assertIn("paper_rule", report["missing"])
            self.assertIn("module_registration", report["missing"])

    def test_issue_guards_pass_for_good_model_issue_without_existing_pr(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            repo = Path(tmpdir)
            report = issue_guard_check(
                repo,
                issue={
                    "number": 117,
                    "title": "[Model] GraphPartitioning",
                    "body": "Implement the model.",
                    "state": "OPEN",
                    "url": "https://example.test/issues/117",
                    "labels": [{"name": "Good"}],
                    "comments": [
                        {
                            "author": {"login": "maintainer"},
                            "body": "Use the paper notation.",
                        }
                    ],
                },
                existing_prs=[],
            )

            self.assertTrue(report["ok"])
            self.assertEqual(report["kind"], "model")
            self.assertEqual(report["checks"]["good_label"]["status"], "pass")
            self.assertEqual(report["checks"]["source_model"]["status"], "skip")
            self.assertEqual(report["comments"][0]["author"], "maintainer")
            self.assertEqual(report["action"], "create-pr")

    def test_issue_guards_fail_when_good_label_missing(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            repo = Path(tmpdir)
            report = issue_guard_check(
                repo,
                issue={
                    "number": 118,
                    "title": "[Model] GraphPartitioning",
                    "body": "Implement the model.",
                    "state": "OPEN",
                    "url": "https://example.test/issues/118",
                    "labels": [{"name": "NeedsCheck"}],
                    "comments": [],
                },
                existing_prs=[],
            )

            self.assertFalse(report["ok"])
            self.assertIn("good_label", report["missing"])
            self.assertEqual(report["checks"]["good_label"]["status"], "fail")

    def test_issue_guards_fail_rule_issue_when_target_model_missing(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            repo = Path(tmpdir)
            self._write(
                repo / "src/models/misc/bin_packing.rs",
                "pub struct BinPacking;\n",
            )

            report = issue_guard_check(
                repo,
                issue={
                    "number": 119,
                    "title": "[Rule] BinPacking to ILP",
                    "body": "Implement the reduction.",
                    "state": "OPEN",
                    "url": "https://example.test/issues/119",
                    "labels": [{"name": "Good"}],
                    "comments": [],
                },
                existing_prs=[],
            )

            self.assertFalse(report["ok"])
            self.assertEqual(report["kind"], "rule")
            self.assertEqual(report["source_problem"], "BinPacking")
            self.assertEqual(report["target_problem"], "ILP")
            self.assertEqual(report["checks"]["source_model"]["status"], "pass")
            self.assertEqual(report["checks"]["target_model"]["status"], "fail")
            self.assertIn("target_model", report["missing"])

    def test_issue_guards_report_existing_open_pr_for_resume(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            repo = Path(tmpdir)
            report = issue_guard_check(
                repo,
                issue={
                    "number": 120,
                    "title": "[Model] GraphPartitioning",
                    "body": "Implement the model.",
                    "state": "OPEN",
                    "url": "https://example.test/issues/120",
                    "labels": [{"name": "Good"}],
                    "comments": [],
                },
                existing_prs=[
                    {
                        "number": 650,
                        "headRefName": "issue-120-graph-partitioning",
                        "url": "https://example.test/pull/650",
                    }
                ],
            )

            self.assertTrue(report["ok"])
            self.assertEqual(report["action"], "resume-pr")
            self.assertEqual(report["resume_pr"]["number"], 650)
            self.assertEqual(report["resume_pr"]["head_ref_name"], "issue-120-graph-partitioning")

    def test_issue_context_alias_matches_issue_guard_contract(self) -> None:
        issue_context_check = getattr(pipeline_checks, "issue_context_check", None)
        self.assertIsNotNone(issue_context_check)

        with tempfile.TemporaryDirectory() as tmpdir:
            repo = Path(tmpdir)
            report = issue_context_check(
                repo,
                issue={
                    "number": 121,
                    "title": "[Model] ExactCoverBy3Sets",
                    "body": "Implement the model.",
                    "state": "OPEN",
                    "url": "https://example.test/issues/121",
                    "labels": [{"name": "Good"}],
                    "comments": [],
                },
                existing_prs=[],
            )

            self.assertTrue(report["ok"])
            self.assertEqual(report["issue_number"], 121)
            self.assertEqual(report["kind"], "model")
            self.assertEqual(report["action"], "create-pr")

    def test_build_review_context_reports_model_bundle(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            repo = Path(tmpdir)
            self._write(
                repo / "src/models/graph/graph_partitioning.rs",
                """
                inventory::submit! { ProblemSchemaEntry { name: "GraphPartitioning" } }
                impl OptimizationProblem for GraphPartitioning<SimpleGraph> {}
                crate::declare_variants! { opt GraphPartitioning<SimpleGraph> => "1.2^n" }
                pub(crate) fn canonical_model_example_specs() -> Vec<ModelExampleSpec> { vec![] }
                """,
            )
            self._write(
                repo / "src/unit_tests/models/graph/graph_partitioning.rs",
                "#[test]\nfn test_graph_partitioning_basic() {}\n",
            )
            self._write(
                repo / "docs/paper/reductions.typ",
                """
                #let display-name = (
                  "GraphPartitioning": [Graph Partitioning],
                )
                #problem-def("GraphPartitioning")[body][proof]
                """,
            )
            scope = detect_scope_from_paths(
                added_files=["src/models/graph/graph_partitioning.rs"],
                changed_files=[
                    "src/models/graph/graph_partitioning.rs",
                    "src/unit_tests/models/graph/graph_partitioning.rs",
                    "docs/paper/reductions.typ",
                ],
            )

            context = build_review_context(
                repo,
                diff_stat=" 3 files changed, 20 insertions(+)\n",
                scope=scope,
                subject=infer_review_subject(scope),
            )

            self.assertEqual(context["subject"]["kind"], "model")
            self.assertFalse(context["whitelist"]["skipped"])
            self.assertTrue(context["whitelist"]["ok"])
            self.assertFalse(context["completeness"]["skipped"])
            self.assertTrue(context["completeness"]["ok"])
            self.assertIn("GraphPartitioning", context["subject"]["name"])

    def test_build_review_context_skips_checks_for_generic_scope(self) -> None:
        scope = detect_scope_from_paths(
            added_files=[],
            changed_files=["src/lib.rs", "README.md"],
        )

        context = build_review_context(
            ".",
            diff_stat=" 2 files changed, 3 insertions(+)\n",
            scope=scope,
            subject=infer_review_subject(scope),
        )

        self.assertEqual(context["subject"]["kind"], "generic")
        self.assertTrue(context["whitelist"]["skipped"])
        self.assertTrue(context["completeness"]["skipped"])

    def test_parse_args_accepts_review_context(self) -> None:
        args = parse_args(
            [
                "review-context",
                "--repo-root",
                ".",
                "--base",
                "abc123",
                "--head",
                "def456",
                "--format",
                "json",
            ]
        )

        self.assertEqual(args.command, "review-context")
        self.assertEqual(args.base, "abc123")
        self.assertEqual(args.head, "def456")

    def test_parse_args_accepts_issue_context(self) -> None:
        args = parse_args(
            [
                "issue-context",
                "--repo",
                "CodingThrust/problem-reductions",
                "--issue",
                "117",
                "--format",
                "json",
            ]
        )

        self.assertEqual(args.command, "issue-context")
        self.assertEqual(args.repo, "CodingThrust/problem-reductions")
        self.assertEqual(args.issue, 117)

    def _write(self, path: Path, content: str) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content.strip() + "\n")


if __name__ == "__main__":
    unittest.main()

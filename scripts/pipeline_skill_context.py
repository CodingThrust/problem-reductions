#!/usr/bin/env python3
"""Skill-scoped context bundle CLI (review-implementation packet)."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Callable

# Ensure sibling modules are importable regardless of how this file is invoked
# (as a script, as ``python3 scripts/pipeline_skill_context.py``, or via
# ``from scripts.pipeline_skill_context import ...``).
sys.path.insert(0, str(Path(__file__).resolve().parent))

import pipeline_checks  # noqa: E402


def report_check_status(check: dict | None) -> str:
    if not check:
        return "unknown"
    if check.get("skipped"):
        return "skipped"
    return "pass" if check.get("ok") else "fail"


def render_review_implementation_text(result: dict) -> str:
    git = result.get("git") or {}
    review_context = result.get("review_context") or {}
    scope = review_context.get("scope") or {}
    subject = review_context.get("subject") or {}
    lines = [
        "# Review Implementation Packet",
        "",
        "## Review Range",
        f"- Base SHA: `{git.get('base_sha', '')}`",
        f"- Head SHA: `{git.get('head_sha', '')}`",
        f"- Repo root: `{git.get('repo_root', '')}`",
        "",
        "## Scope",
        f"- Review type: {scope.get('review_type', 'unknown')}",
        f"- Subject kind: {subject.get('kind', 'unknown')}",
    ]
    if subject.get("name"):
        lines.append(f"- Name: {subject['name']}")
    if subject.get("source"):
        lines.append(f"- Source: {subject['source']}")
    if subject.get("target"):
        lines.append(f"- Target: {subject['target']}")

    models = scope.get("models") or []
    if models:
        lines.append("- Added models:")
        lines.extend(
            f"  - {model.get('problem_name')} (`{model.get('path')}`)"
            for model in models
        )
    rules = scope.get("rules") or []
    if rules:
        lines.append("- Added rules:")
        lines.extend(
            f"  - {rule.get('rule_stem')} (`{rule.get('path')}`)"
            for rule in rules
        )

    lines.extend(
        [
            "",
            "## Deterministic Checks",
            f"- Whitelist: {report_check_status(review_context.get('whitelist'))}",
            f"- Completeness: {report_check_status(review_context.get('completeness'))}",
        ]
    )
    missing = (review_context.get("completeness") or {}).get("missing") or []
    if missing:
        lines.append("- Missing items:")
        lines.extend(f"  - `{item}`" for item in missing)

    changed_files = review_context.get("changed_files") or []
    lines.extend(["", "## Changed Files"])
    if changed_files:
        lines.extend(f"- `{path}`" for path in changed_files)
    else:
        lines.append("- None captured")

    diff_stat = review_context.get("diff_stat")
    if diff_stat:
        lines.extend(["", "## Diff Stat", "```text", diff_stat, "```"])

    return "\n".join(lines) + "\n"


def render_text(result: dict) -> str:
    if result.get("skill") == "review-implementation":
        return render_review_implementation_text(result)
    return json.dumps(result, indent=2, sort_keys=True) + "\n"


def emit_result(result: dict, fmt: str) -> None:
    if fmt == "text":
        print(render_text(result), end="")
        return
    print(json.dumps(result, indent=2, sort_keys=True))


def git_output_in(repo_root: str | Path, *args: str) -> list[str]:
    output = subprocess.check_output(
        ["git", "-C", str(repo_root), *args],
        text=True,
    )
    return [line for line in output.splitlines() if line]


def git_text_in(repo_root: str | Path, *args: str) -> str:
    return subprocess.check_output(
        ["git", "-C", str(repo_root), *args],
        text=True,
    )


def default_review_implementation_context_builder(
    repo_root: str | Path,
    *,
    diff_stat: str,
    changed_files: list[str],
    added_files: list[str],
    kind: str | None,
    name: str | None,
    source: str | None,
    target: str | None,
) -> dict:
    scope = pipeline_checks.detect_scope_from_paths(
        added_files=added_files,
        changed_files=changed_files,
    )
    subject = pipeline_checks.infer_review_subject(
        scope,
        kind=kind,
        name=name,
        source=source,
        target=target,
    )
    return pipeline_checks.build_review_context(
        repo_root,
        diff_stat=diff_stat,
        scope=scope,
        subject=subject,
    )



def build_review_implementation_context(
    *,
    repo_root: Path,
    kind: str | None,
    name: str | None,
    source: str | None,
    target: str | None,
    merge_base_getter: Callable[[Path], str] | None = None,
    head_sha_getter: Callable[[Path], str] | None = None,
    diff_stat_getter: Callable[[Path, str, str], str] | None = None,
    changed_files_getter: Callable[[Path, str, str], list[str]] | None = None,
    added_files_getter: Callable[[Path, str, str], list[str]] | None = None,
    review_context_builder: Callable[..., dict] | None = None,
) -> dict:
    merge_base_getter = merge_base_getter or (
        lambda repo_root: git_text_in(repo_root, "merge-base", "origin/main", "HEAD").strip()
    )
    head_sha_getter = head_sha_getter or (
        lambda repo_root: git_text_in(repo_root, "rev-parse", "HEAD").strip()
    )
    diff_stat_getter = diff_stat_getter or (
        lambda repo_root, base_sha, head_sha: git_text_in(
            repo_root,
            "diff",
            "--stat",
            f"{base_sha}..{head_sha}",
        )
    )
    changed_files_getter = changed_files_getter or (
        lambda repo_root, base_sha, head_sha: git_output_in(
            repo_root,
            "diff",
            "--name-only",
            f"{base_sha}..{head_sha}",
        )
    )
    added_files_getter = added_files_getter or (
        lambda repo_root, base_sha, head_sha: git_output_in(
            repo_root,
            "diff",
            "--name-only",
            "--diff-filter=A",
            f"{base_sha}..{head_sha}",
        )
    )
    review_context_builder = review_context_builder or default_review_implementation_context_builder

    base_sha = merge_base_getter(repo_root)
    head_sha = head_sha_getter(repo_root)
    diff_stat = diff_stat_getter(repo_root, base_sha, head_sha)
    changed_files = changed_files_getter(repo_root, base_sha, head_sha)
    added_files = added_files_getter(repo_root, base_sha, head_sha)
    review_context = review_context_builder(
        repo_root,
        diff_stat=diff_stat,
        changed_files=changed_files,
        added_files=added_files,
        kind=kind,
        name=name,
        source=source,
        target=target,
    )

    return {
        "skill": "review-implementation",
        "status": "ready",
        "git": {
            "repo_root": str(repo_root),
            "base_sha": base_sha,
            "head_sha": head_sha,
        },
        "review_context": review_context,
    }


def add_review_implementation_parser(subparsers) -> None:
    parser = subparsers.add_parser("review-implementation")
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--kind", choices=["model", "rule", "generic"])
    parser.add_argument("--name")
    parser.add_argument("--source")
    parser.add_argument("--target")
    parser.add_argument("--format", choices=["json", "text"], default="json")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Skill-scoped context bundles.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    add_review_implementation_parser(subparsers)

    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])

    if args.command == "review-implementation":
        emit_result(
            build_review_implementation_context(
                repo_root=args.repo_root,
                kind=args.kind,
                name=getattr(args, "name", None),
                source=getattr(args, "source", None),
                target=getattr(args, "target", None),
            ),
            args.format,
        )
        return 0

    raise AssertionError(f"Unhandled command: {args.command}")


if __name__ == "__main__":
    raise SystemExit(main())

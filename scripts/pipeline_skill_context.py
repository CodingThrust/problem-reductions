#!/usr/bin/env python3
"""Skill-scoped context bundle CLI skeleton."""

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

import pipeline_board  # noqa: E402
import pipeline_checks  # noqa: E402
import pipeline_pr  # noqa: E402
import pipeline_worktree  # noqa: E402


PROJECT_BOARD_NUMBER = 8
PROJECT_BOARD_LIMIT = 500
DEFAULT_REPO = "CodingThrust/problem-reductions"


def build_status_result(skill: str, *, status: str, **fields: object) -> dict:
    result = {
        "skill": skill,
        "status": status,
    }
    for key, value in fields.items():
        if value is not None:
            result[key] = value
    return result


def report_check_status(check: dict | None) -> str:
    if not check:
        return "unknown"
    if check.get("skipped"):
        return "skipped"
    return "pass" if check.get("ok") else "fail"


def first_paragraph(text: str | None) -> str:
    if not text:
        return ""
    paragraphs = [chunk.strip() for chunk in text.split("\n\n") if chunk.strip()]
    if not paragraphs:
        return ""
    return " ".join(paragraphs[0].split())


def scan_existing_problems(repo_root: str | Path) -> set[str]:
    problem_names: set[str] = set()
    models_root = Path(repo_root) / "src/models"
    if not models_root.exists():
        return problem_names

    for path in sorted(models_root.rglob("*.rs")):
        text = path.read_text()
        for match in pipeline_checks.re.finditer(
            r"\bpub\s+(?:struct|enum)\s+([A-Z][A-Za-z0-9_]*)\b",
            text,
        ):
            problem_names.add(match.group(1))
    return problem_names


def review_pipeline_suggested_mode(result: dict) -> str:
    status = result.get("status")
    if status == "empty":
        return "empty"
    if status == "needs-user-choice":
        return "needs-user-choice"

    merge_status = ((result.get("prep") or {}).get("merge") or {}).get("status")
    if merge_status == "conflicted":
        return "conflicted-fix"
    if merge_status == "aborted":
        return "manual-followup"

    ci_state = ((result.get("pr") or {}).get("ci") or {}).get("state")
    if ci_state == "failure":
        return "fix-ci"
    return "normal-fix"


def review_pipeline_seed_items(result: dict) -> list[str]:
    blockers: list[str] = []
    prep = result.get("prep") or {}
    merge_status = (prep.get("merge") or {}).get("status")
    if merge_status == "conflicted":
        blockers.append("merge conflicts with main")
    elif merge_status == "aborted":
        blockers.append("merge prep aborted")

    pr = result.get("pr") or {}
    ci_state = (pr.get("ci") or {}).get("state")
    if ci_state == "failure":
        blockers.append("CI is failing")

    comment_counts = (pr.get("comments") or {}).get("counts") or {}
    human_count = sum(
        int(comment_counts.get(key, 0))
        for key in [
            "human_inline_comments",
            "human_issue_comments",
            "human_linked_issue_comments",
            "human_reviews",
        ]
    )
    if human_count:
        blockers.append(f"{human_count} human review items to audit")

    deduped: list[str] = []
    for blocker in blockers:
        if blocker not in deduped:
            deduped.append(blocker)
    return deduped


def render_review_pipeline_text(result: dict) -> str:
    lines = [
        "# Review Pipeline Packet",
        "",
        "## Selection",
        f"- Bundle status: {result.get('status')}",
    ]

    if result.get("status") == "empty":
        lines.append("- No eligible review-pipeline item is currently available.")
        return "\n".join(lines) + "\n"

    if result.get("status") == "needs-user-choice":
        lines.extend(
            [
                "",
                "## Ambiguous PR Options",
            ]
        )
        for option in result.get("options") or []:
            lines.append(
                f"- PR #{option.get('number')} [{option.get('state', 'UNKNOWN')}] {option.get('title') or ''}".rstrip()
            )
        if result.get("recommendation") is not None:
            lines.append(f"- Recommended PR: #{result['recommendation']}")
        return "\n".join(lines) + "\n"

    selection = result.get("selection") or {}
    pr = result.get("pr") or {}
    prep = result.get("prep") or {}
    comments = pr.get("comments") or {}
    counts = comments.get("counts") or {}
    ci = pr.get("ci") or {}
    codecov = pr.get("codecov") or {}
    checkout = prep.get("checkout") or {}
    merge = prep.get("merge") or {}

    if selection.get("pr_number") is not None:
        lines.append(f"- PR: #{selection['pr_number']}")
    if selection.get("item_id"):
        lines.append(f"- Board item: `{selection['item_id']}`")
    if selection.get("issue_number") is not None:
        lines.append(f"- Linked issue: #{selection['issue_number']}")
    if pr.get("title") or selection.get("title"):
        lines.append(f"- Title: {pr.get('title') or selection.get('title')}")
    if pr.get("url"):
        lines.append(f"- URL: {pr['url']}")

    lines.extend(
        [
            "",
            "## Recommendation Seed",
            f"- Suggested mode: {review_pipeline_suggested_mode(result)}",
        ]
    )
    seed_items = review_pipeline_seed_items(result)
    if seed_items:
        lines.append("- Attention points:")
        lines.extend(f"  - {item}" for item in seed_items)
    else:
        lines.append("- Attention points: none from deterministic checks")

    lines.extend(
        [
            "",
            "## Comment Summary",
            f"- Human inline comments: {counts.get('human_inline_comments', 0)}",
            f"- Human PR issue comments: {counts.get('human_issue_comments', 0)}",
            f"- Human linked-issue comments: {counts.get('human_linked_issue_comments', 0)}",
            f"- Human review bodies: {counts.get('human_reviews', 0)}",
        ]
    )

    lines.extend(
        [
            "",
            "## CI / Coverage",
            f"- CI state: {ci.get('state', 'unknown')}",
        ]
    )
    if ci:
        lines.append(f"- Failing checks: {ci.get('failing', 0)}")
        lines.append(f"- Pending checks: {ci.get('pending', 0)}")
    if codecov.get("found"):
        lines.append(f"- Patch coverage: {codecov.get('patch_coverage')}%")
        if codecov.get("project_coverage") is not None:
            lines.append(f"- Project coverage: {codecov.get('project_coverage')}%")

    lines.extend(
        [
            "",
            "## Merge Prep",
            f"- Ready: {str(prep.get('ready')).lower()}",
            f"- Merge status: {merge.get('status', 'unknown')}",
        ]
    )
    if checkout.get("worktree_dir"):
        lines.append(f"- Worktree: `{checkout['worktree_dir']}`")
    if checkout.get("head_ref_name"):
        lines.append(f"- PR head branch: `{checkout['head_ref_name']}`")
    conflicts = merge.get("conflicts") or []
    if conflicts:
        lines.append("- Conflicts:")
        lines.extend(f"  - `{conflict}`" for conflict in conflicts)

    if pr.get("issue_context_text"):
        lines.extend(
            [
                "",
                "## Linked Issue Context",
                pr["issue_context_text"],
            ]
        )

    return "\n".join(lines) + "\n"


def final_review_suggested_mode(result: dict) -> str:
    status = result.get("status")
    if status == "empty":
        return "empty"
    if status == "ready-with-warnings":
        return "warning-fallback"

    merge_status = ((result.get("prep") or {}).get("merge") or {}).get("status")
    if merge_status == "conflicted":
        return "conflicted-review"
    if merge_status == "aborted":
        return "warning-fallback"
    return "normal-review"


def final_review_seed_items(result: dict) -> list[str]:
    review_context = result.get("review_context") or {}
    prep = result.get("prep") or {}
    warnings = list(result.get("warnings") or [])
    blockers = list(warnings)

    merge_status = (prep.get("merge") or {}).get("status")
    if merge_status == "conflicted":
        blockers.append("merge conflicts with main")
    elif merge_status == "aborted":
        blockers.append("merge prep aborted")

    whitelist = review_context.get("whitelist") or {}
    if whitelist and not whitelist.get("ok"):
        blockers.append("files outside expected whitelist")

    completeness = review_context.get("completeness") or {}
    for missing in completeness.get("missing", []):
        blockers.append(f"missing completeness item: {missing}")

    comment_counts = ((result.get("pr") or {}).get("comments") or {}).get("counts") or {}
    manual_comment_count = sum(
        int(comment_counts.get(key, 0))
        for key in [
            "human_inline_comments",
            "human_issue_comments",
            "human_linked_issue_comments",
            "human_reviews",
        ]
    )
    if manual_comment_count:
        blockers.append(
            f"manual comment audit required for {manual_comment_count} human review items"
        )

    deduped: list[str] = []
    for blocker in blockers:
        if blocker not in deduped:
            deduped.append(blocker)
    return deduped


def render_final_review_text(result: dict) -> str:
    selection = result.get("selection") or {}
    pr = result.get("pr") or {}
    prep = result.get("prep") or {}
    review_context = result.get("review_context") or {}
    subject = review_context.get("subject") or {}
    comments = pr.get("comments") or {}
    counts = comments.get("counts") or {}
    checkout = prep.get("checkout") or {}
    merge = prep.get("merge") or {}

    lines = [
        "# Final Review Packet",
        "",
        "## Selection",
        f"- Bundle status: {result.get('status')}",
    ]
    if selection.get("pr_number") is not None:
        lines.append(f"- PR: #{selection['pr_number']}")
    if selection.get("item_id"):
        lines.append(f"- Board item: `{selection['item_id']}`")
    if selection.get("issue_number") is not None:
        lines.append(f"- Linked issue: #{selection['issue_number']}")
    if pr.get("title") or selection.get("title"):
        lines.append(f"- Title: {pr.get('title') or selection.get('title')}")
    if pr.get("url"):
        lines.append(f"- URL: {pr['url']}")

    lines.extend(
        [
            "",
            "## Recommendation Seed",
            f"- Suggested mode: {final_review_suggested_mode(result)}",
        ]
    )
    seed_items = final_review_seed_items(result)
    if seed_items:
        lines.append("- Review blockers / attention points:")
        lines.extend(f"  - {item}" for item in seed_items)
    else:
        lines.append("- Review blockers / attention points: none from deterministic checks")

    lines.extend(
        [
            "",
            "## Subject",
            f"- Kind: {subject.get('kind', 'unknown')}",
        ]
    )
    if subject.get("name"):
        lines.append(f"- Name: {subject['name']}")
    if subject.get("source"):
        lines.append(f"- Source: {subject['source']}")
    if subject.get("target"):
        lines.append(f"- Target: {subject['target']}")

    lines.extend(
        [
            "",
            "## Comment Summary",
            f"- Human reviews: {counts.get('human_reviews', 0)}",
            f"- Human inline comments: {counts.get('human_inline_comments', 0)}",
            f"- Human PR issue comments: {counts.get('human_issue_comments', 0)}",
            f"- Human linked-issue comments: {counts.get('human_linked_issue_comments', 0)}",
        ]
    )
    if pr.get("issue_context_text"):
        lines.extend(
            [
                "",
                "### Linked Issue Context",
                pr["issue_context_text"],
            ]
        )

    lines.extend(
        [
            "",
            "## Merge Prep",
            f"- Ready: {str(prep.get('ready')).lower()}",
            f"- Merge status: {merge.get('status', 'unknown')}",
        ]
    )
    if checkout.get("worktree_dir"):
        lines.append(f"- Worktree: `{checkout['worktree_dir']}`")
    conflicts = merge.get("conflicts") or []
    if conflicts:
        lines.append("- Conflicts:")
        lines.extend(f"  - `{conflict}`" for conflict in conflicts)
    warnings = result.get("warnings") or []
    if warnings:
        lines.append("- Warnings:")
        lines.extend(f"  - {warning}" for warning in warnings)

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

    full_diff = review_context.get("full_diff")
    if full_diff:
        lines.extend(["", "## Full Diff", "```diff", full_diff, "```"])

    pred_list = review_context.get("pred_list")
    if pred_list:
        lines.extend(["", "## Problem Catalog (`pred list`)", "```text", pred_list, "```"])

    return "\n".join(lines) + "\n"


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


def render_project_pipeline_text(result: dict) -> str:
    ready_issues = result.get("ready_issues") or []
    eligible = [issue for issue in ready_issues if issue.get("eligible")]
    blocked = [issue for issue in ready_issues if not issue.get("eligible")]
    in_progress = result.get("in_progress_issues") or []
    requested = result.get("requested_issue")

    lines = [
        "# Project Pipeline Packet",
        "",
        "## Queue Summary",
        f"- Bundle status: {result.get('status')}",
        f"- Ready issues: {len(ready_issues)}",
        f"- Eligible ready issues: {len(eligible)}",
        f"- Blocked ready issues: {len(blocked)}",
        f"- In progress issues: {len(in_progress)}",
        f"- Existing problems on main: {len(result.get('existing_problems') or [])}",
    ]

    if requested is not None:
        lines.extend(
            [
                "",
                "## Requested Issue",
                f"- Issue: #{requested.get('issue_number')}",
                f"- Title: {requested.get('title') or 'unknown'}",
                f"- Eligible: {str(bool(requested.get('eligible'))).lower()}",
            ]
        )
        if requested.get("blocking_reason"):
            lines.append(f"- Blocking reason: {requested['blocking_reason']}")

    lines.extend(["", "## Eligible Ready Issues"])
    if eligible:
        for issue in eligible:
            lines.append(f"- #{issue.get('issue_number')} {issue.get('title')}")
            lines.append(f"  - Kind: {issue.get('kind', 'unknown')}")
            lines.append(
                f"  - Pending rules unblocked: {issue.get('pending_rule_count', 0)}"
            )
            if issue.get("summary"):
                lines.append(f"  - Summary: {issue['summary']}")
    else:
        lines.append("- None")

    lines.extend(["", "## Blocked Ready Issues"])
    if blocked:
        for issue in blocked:
            lines.append(f"- #{issue.get('issue_number')} {issue.get('title')}")
            lines.append(f"  - Blocking reason: {issue.get('blocking_reason')}")
            if issue.get("summary"):
                lines.append(f"  - Summary: {issue['summary']}")
    else:
        lines.append("- None")

    if in_progress:
        lines.extend(["", "## In Progress Issues"])
        for issue in in_progress:
            lines.append(f"- #{issue.get('issue_number')} {issue.get('title')}")

    return "\n".join(lines) + "\n"


def render_text(result: dict) -> str:
    if result.get("skill") == "review-pipeline":
        return render_review_pipeline_text(result)
    if result.get("skill") == "final-review":
        return render_final_review_text(result)
    if result.get("skill") == "review-implementation":
        return render_review_implementation_text(result)
    if result.get("skill") == "project-pipeline":
        return render_project_pipeline_text(result)
    return json.dumps(result, indent=2, sort_keys=True) + "\n"


def emit_result(result: dict, fmt: str) -> None:
    if fmt == "text":
        print(render_text(result), end="")
        return
    print(json.dumps(result, indent=2, sort_keys=True))


def fetch_review_candidates(repo: str) -> list[dict]:
    owner = repo.split("/", 1)[0]
    board_data = pipeline_board.fetch_board_items(
        owner,
        PROJECT_BOARD_NUMBER,
        PROJECT_BOARD_LIMIT,
    )
    return pipeline_board.review_candidates(
        board_data,
        repo,
        pipeline_board.resolve_issue_pr,
        pipeline_board.fetch_pr_info,
        batch_pr_fetcher=pipeline_board.batch_fetch_prs_with_reviews,
    )


def build_ready_result(*, skill: str, selection: dict, pr: dict, prep: dict) -> dict:
    return build_status_result(
        skill,
        status="ready",
        selection=selection,
        pr=pr,
        prep=prep,
    )


def build_ambiguous_selection(candidate: dict, *, pr_number: int) -> dict:
    return {
        "item_id": candidate["item_id"],
        "number": pr_number,
        "issue_number": candidate.get("issue_number"),
        "pr_number": pr_number,
        "status": candidate.get("status"),
        "title": candidate.get("title"),
    }


def select_final_review_entry(
    *,
    repo: str,
    pr_number: int | None,
) -> dict | None:
    """Find a Final-review board entry for the given PR (or the first available one)."""
    owner = repo.split("/", 1)[0]
    board_data = pipeline_board.fetch_board_items(
        owner,
        PROJECT_BOARD_NUMBER,
        PROJECT_BOARD_LIMIT,
    )
    items = [
        item
        for item in board_data.get("items", [])
        if item.get("status") == pipeline_board.STATUS_FINAL_REVIEW
    ]
    for item in items:
        content = item.get("content") or {}
        number = content.get("number")
        if number is None:
            continue
        item_type = content.get("type", "")
        if item_type == "PullRequest":
            item_pr = int(number)
        elif item_type == "Issue":
            item_pr = pipeline_board.resolve_issue_pr(repo, int(number))
            if item_pr is None:
                continue
        else:
            continue
        entry = {
            "item_id": pipeline_board.item_identity(item),
            "number": item_pr,
            "pr_number": item_pr,
            "status": item.get("status"),
            "title": content.get("title"),
        }
        if item_type == "Issue":
            entry["issue_number"] = int(number)
        if pr_number is not None:
            if item_pr == pr_number:
                return entry
        else:
            state = pipeline_board.fetch_pr_state(repo, item_pr)
            if state == "OPEN":
                return entry
    return None


def _get_current_gh_user() -> str:
    """Return the GitHub login of the currently authenticated user."""
    try:
        output = subprocess.check_output(
            ["gh", "api", "user", "--jq", ".login"],
            text=True,
            stderr=subprocess.DEVNULL,
        )
        return output.strip()
    except Exception:
        return ""


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


def infer_final_review_subject(scope: dict, pr_context: dict) -> dict:
    subject = pipeline_checks.infer_review_subject(scope)
    linked_issue = pr_context.get("linked_issue") or {}
    linked_title = (linked_issue.get("title") or "").strip()

    rule_match = pipeline_checks.RULE_TITLE_RE.match(linked_title)
    if rule_match:
        subject["kind"] = "rule"
        subject["source"] = rule_match.group("source")
        subject["target"] = rule_match.group("target")
        return subject

    model_match = pipeline_checks.MODEL_TITLE_RE.match(linked_title)
    if model_match:
        subject["kind"] = "model"
        subject["name"] = subject.get("name") or model_match.group("name")
        subject["source"] = None
        subject["target"] = None

    return subject


def build_final_review_checks(*, prep: dict, pr_context: dict) -> dict:
    checkout = prep.get("checkout") or {}
    worktree_dir = checkout.get("worktree_dir")
    base_sha = checkout.get("base_sha")
    head_sha = checkout.get("head_sha")
    if not worktree_dir or not base_sha or not head_sha:
        raise ValueError("prepare-review output missing checkout diff range")

    diff_range = f"{base_sha}..{head_sha}"
    changed_files = git_output_in(worktree_dir, "diff", "--name-only", diff_range)
    added_files = git_output_in(
        worktree_dir,
        "diff",
        "--name-only",
        "--diff-filter=A",
        diff_range,
    )
    scope = pipeline_checks.detect_scope_from_paths(
        added_files=added_files,
        changed_files=changed_files,
    )
    subject = infer_final_review_subject(scope, pr_context)
    review_context = pipeline_checks.build_review_context(
        worktree_dir,
        diff_stat=git_text_in(worktree_dir, "diff", "--stat", diff_range),
        scope=scope,
        subject=subject,
    )
    review_context["full_diff"] = git_text_in(worktree_dir, "diff", diff_range)
    review_context["pred_list"] = _run_pred_list(worktree_dir)
    return review_context


def _run_pred_list(worktree_dir: str | Path) -> str | None:
    """Run ``pred list`` in *worktree_dir*, building the CLI first if needed."""
    pred_cmd = ["cargo", "run", "-p", "problemreductions-cli", "--bin", "pred", "--", "list"]
    try:
        return subprocess.check_output(
            pred_cmd, cwd=str(worktree_dir), text=True, stderr=subprocess.DEVNULL,
        )
    except (subprocess.CalledProcessError, FileNotFoundError):
        pass
    # Binary may not exist yet — build it and retry.
    try:
        subprocess.check_call(
            ["make", "cli"], cwd=str(worktree_dir),
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        )
        return subprocess.check_output(
            pred_cmd, cwd=str(worktree_dir), text=True, stderr=subprocess.DEVNULL,
        )
    except Exception:
        return None


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


def classify_project_issue(
    entry: dict,
    *,
    issue: dict,
    existing_problems: set[str],
    pending_rule_counts: dict[str, int],
) -> dict:
    kind, source_problem, target_problem = pipeline_checks.issue_kind_from_title(
        entry.get("title")
    )
    blocking_reason = None
    eligible = True
    if kind == "rule":
        missing = [
            problem
            for problem in [source_problem, target_problem]
            if problem and problem not in existing_problems
        ]
        if missing:
            eligible = False
            blocking_reason = f'model "{missing[0]}" not yet implemented on main'

    issue_number = int(entry["issue_number"])
    return {
        "item_id": entry.get("item_id"),
        "issue_number": issue_number,
        "title": entry.get("title"),
        "kind": kind,
        "source_problem": source_problem,
        "target_problem": target_problem,
        "eligible": eligible,
        "blocking_reason": blocking_reason,
        "pending_rule_count": pending_rule_counts.get(entry.get("title", ""), 0)
        if kind == "rule"
        else pending_rule_counts.get(
            pipeline_checks.MODEL_TITLE_RE.match(entry.get("title", "")).group("name")
            if pipeline_checks.MODEL_TITLE_RE.match(entry.get("title", ""))
            else "",
            0,
        ),
        "summary": first_paragraph(issue.get("body")),
        "issue": issue,
    }


def build_pending_rule_counts(
    ready_entries: list[dict],
    in_progress_entries: list[dict],
) -> dict[str, int]:
    counts: dict[str, int] = {}
    for entry in [*ready_entries, *in_progress_entries]:
        kind, source_problem, target_problem = pipeline_checks.issue_kind_from_title(
            entry.get("title")
        )
        if kind != "rule":
            continue
        for problem in [source_problem, target_problem]:
            if not problem:
                continue
            counts[problem] = counts.get(problem, 0) + 1
    return counts


def fetch_project_board_data(repo: str) -> dict:
    owner = repo.split("/", 1)[0]
    return pipeline_board.fetch_board_items(
        owner,
        PROJECT_BOARD_NUMBER,
        PROJECT_BOARD_LIMIT,
    )


def build_project_pipeline_context(
    *,
    repo: str,
    issue_number: int | None,
    repo_root: Path,
    board_fetcher: Callable[[str], dict] | None = None,
    issue_fetcher: Callable[[str, int], dict] | None = None,
    batch_issue_fetcher: Callable[[str, list[int]], dict[int, dict]] | None = None,
    existing_problem_finder: Callable[[Path], set[str]] | None = None,
) -> dict:
    board_fetcher = board_fetcher or fetch_project_board_data
    _custom_issue_fetcher = issue_fetcher is not None
    issue_fetcher = issue_fetcher or pipeline_checks.fetch_issue
    # Only use batch fetcher when no custom per-item fetcher was injected (e.g. tests)
    if batch_issue_fetcher is None and not _custom_issue_fetcher:
        batch_issue_fetcher = pipeline_board.batch_fetch_issues
    existing_problem_finder = existing_problem_finder or scan_existing_problems

    board_data = board_fetcher(repo)
    ready_entries = sorted(
        pipeline_board.ready_entries(board_data).values(),
        key=lambda entry: entry["issue_number"],
    )
    in_progress_entries = pipeline_board.status_items(
        board_data,
        pipeline_board.STATUS_IN_PROGRESS,
    )
    existing_problems = existing_problem_finder(repo_root)
    pending_rule_counts = build_pending_rule_counts(ready_entries, in_progress_entries)

    ready_entries_items = sorted(
        pipeline_board.ready_entries(board_data).items(),
        key=lambda pair: pair[1]["issue_number"],
    )

    # Batch-fetch all issue data in one API call when batch fetcher is available
    if batch_issue_fetcher is not None:
        all_issue_numbers = [int(entry["issue_number"]) for _, entry in ready_entries_items]
        issues_cache = batch_issue_fetcher(repo, all_issue_numbers)

        def _fetch_one(repo: str, n: int) -> dict:
            if n in issues_cache:
                return issues_cache[n]
            return issue_fetcher(repo, n)
    else:
        _fetch_one = issue_fetcher

    ready_issues = [
        classify_project_issue(
            dict(entry, item_id=item_id),
            issue=_fetch_one(repo, int(entry["issue_number"])),
            existing_problems=existing_problems,
            pending_rule_counts=pending_rule_counts,
        )
        for item_id, entry in ready_entries_items
    ]

    requested_issue = None
    if issue_number is not None:
        requested_issue = next(
            (
                issue
                for issue in ready_issues
                if int(issue["issue_number"]) == issue_number
            ),
            None,
        )

    eligible_ready_issues = [issue for issue in ready_issues if issue.get("eligible")]

    if not ready_issues:
        status = "empty"
    elif issue_number is not None and requested_issue is None:
        status = "requested-missing"
    elif requested_issue is not None and not requested_issue.get("eligible"):
        status = "requested-blocked"
    elif not eligible_ready_issues:
        status = "no-eligible-issues"
    else:
        status = "ready"

    return build_status_result(
        "project-pipeline",
        status=status,
        repo=repo,
        existing_problems=sorted(existing_problems),
        ready_issues=ready_issues,
        in_progress_issues=in_progress_entries,
        requested_issue=requested_issue,
    )


def _select_candidate(
    candidates: list[dict],
    pr_number: int | None,
) -> dict | None:
    """Pick a candidate from the review-candidates list (no state file, no claiming)."""
    if pr_number is not None:
        return next(
            (
                c
                for c in candidates
                if int(c.get("pr_number") or c.get("number") or -1) == pr_number
            ),
            None,
        )
    eligible = [c for c in candidates if c.get("eligibility") == "eligible"]
    return eligible[0] if eligible else None


def _selection_from_candidate(candidate: dict) -> dict:
    """Build a selection dict from a candidate (read-only, no board move)."""
    item_id = str(candidate["item_id"])
    return {
        "item_id": item_id,
        "number": int(candidate.get("pr_number") or candidate["number"]),
        "issue_number": candidate.get("issue_number"),
        "pr_number": int(candidate.get("pr_number") or candidate["number"]),
        "status": candidate.get("status"),
        "title": candidate.get("title"),
    }


def build_review_pipeline_context(
    *,
    repo: str,
    pr_number: int | None,
    review_candidate_fetcher: Callable[[str], list[dict]] | None = None,
    pr_context_builder: Callable[[str, int], dict] | None = None,
    review_preparer: Callable[[str, int], dict] | None = None,
) -> dict:
    """Build review-pipeline context (read-only, no board move).

    The agent is responsible for claiming the item (moving to Under review)
    after it has verified the PR is review-ready and is about to start work.
    """
    review_candidate_fetcher = review_candidate_fetcher or fetch_review_candidates
    pr_context_builder = pr_context_builder or pipeline_pr.build_pr_context
    review_preparer = review_preparer or (
        lambda repo, pr_number: pipeline_worktree.prepare_review_from_cwd(
            repo=repo,
            pr_number=pr_number,
        )
    )

    candidates = review_candidate_fetcher(repo)
    if not candidates:
        return build_status_result("review-pipeline", status="empty")

    # Check for ambiguous cards first
    if pr_number is None:
        ambiguous = next(
            (c for c in candidates if c.get("eligibility") == "ambiguous-linked-prs"),
            None,
        )
        if ambiguous is not None:
            return build_status_result(
                "review-pipeline",
                status="needs-user-choice",
                options=ambiguous.get("linked_repo_prs", []),
                recommendation=ambiguous.get("recommendation"),
            )
    else:
        matching_ambiguous = next(
            (
                c
                for c in candidates
                if c.get("eligibility") == "ambiguous-linked-prs"
                and any(
                    int(option["number"]) == pr_number
                    for option in c.get("linked_repo_prs", [])
                )
            ),
            None,
        )
        if matching_ambiguous is not None:
            selection = build_ambiguous_selection(matching_ambiguous, pr_number=pr_number)
            return build_ready_result(
                skill="review-pipeline",
                selection=selection,
                pr=pr_context_builder(repo, pr_number),
                prep=review_preparer(repo, pr_number),
            )

    candidate = _select_candidate(candidates, pr_number)
    if candidate is None:
        return build_status_result("review-pipeline", status="empty")

    if candidate.get("eligibility") != "eligible":
        return build_status_result("review-pipeline", status="empty")

    selection = _selection_from_candidate(candidate)
    selected_pr_number = int(selection["pr_number"])
    return build_ready_result(
        skill="review-pipeline",
        selection=selection,
        pr=pr_context_builder(repo, selected_pr_number),
        prep=review_preparer(repo, selected_pr_number),
    )


def build_final_review_context(
    *,
    repo: str,
    pr_number: int | None,
    selection_fetcher: Callable[..., dict | None] | None = None,
    pr_context_builder: Callable[[str, int], dict] | None = None,
    review_preparer: Callable[[str, int], dict] | None = None,
    review_context_builder: Callable[..., dict] | None = None,
) -> dict:
    selection_fetcher = selection_fetcher or select_final_review_entry
    pr_context_builder = pr_context_builder or pipeline_pr.build_pr_context
    review_preparer = review_preparer or (
        lambda repo, pr_number: pipeline_worktree.prepare_review_from_cwd(
            repo=repo,
            pr_number=pr_number,
        )
    )
    review_context_builder = review_context_builder or build_final_review_checks

    selection = selection_fetcher(
        repo=repo,
        pr_number=pr_number,
    )
    if selection is None:
        return build_status_result("final-review", status="empty")

    selected_pr_number = int(selection.get("pr_number") or selection["number"])
    pr_context = pr_context_builder(repo, selected_pr_number)

    # Self-review warning: flag if reviewer is the PR author (unless repo owner).
    pr_author = (pr_context.get("author") or "").lower()
    current_user = _get_current_gh_user().lower()
    repo_owner = repo.split("/", 1)[0].lower() if "/" in repo else ""
    self_review_warning = None
    if pr_author and current_user and pr_author == current_user and current_user != repo_owner:
        self_review_warning = f"Self-review: PR author '{pr_author}' is the current reviewer"

    prep: dict
    try:
        prep = review_preparer(repo, selected_pr_number)
    except Exception as exc:
        return {
            "skill": "final-review",
            "status": "ready-with-warnings",
            "selection": selection,
            "pr": pr_context,
            "prep": {
                "ready": False,
                "error": str(exc),
            },
            "review_context": None,
            "warnings": [
                f"failed to prepare final-review worktree: {exc}",
            ],
        }

    try:
        review_context = review_context_builder(
            prep=prep,
            pr_context=pr_context,
        )
    except Exception as exc:
        return {
            "skill": "final-review",
            "status": "ready-with-warnings",
            "selection": selection,
            "pr": pr_context,
            "prep": prep,
            "review_context": None,
            "warnings": [
                f"failed to derive final-review review context: {exc}",
            ],
        }

    warnings = [self_review_warning] if self_review_warning else []
    return build_status_result(
        "final-review",
        status="ready",
        selection=selection,
        pr=pr_context,
        prep=prep,
        review_context=review_context,
        warnings=warnings or None,
    )


def add_bundle_parser(
    subparsers,
    command: str,
) -> None:
    parser = subparsers.add_parser(command)
    parser.add_argument("--repo", required=True)
    parser.add_argument("--pr", type=int)
    parser.add_argument("--format", choices=["json", "text"], default="json")


def add_review_implementation_parser(subparsers) -> None:
    parser = subparsers.add_parser("review-implementation")
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--kind", choices=["model", "rule", "generic"])
    parser.add_argument("--name")
    parser.add_argument("--source")
    parser.add_argument("--target")
    parser.add_argument("--format", choices=["json", "text"], default="json")


def add_project_pipeline_parser(subparsers) -> None:
    parser = subparsers.add_parser("project-pipeline")
    parser.add_argument("--repo", default=DEFAULT_REPO)
    parser.add_argument("--issue", type=int)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--format", choices=["json", "text"], default="json")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Skill-scoped pipeline context bundles.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    add_bundle_parser(subparsers, "review-pipeline")
    add_bundle_parser(subparsers, "final-review")
    add_review_implementation_parser(subparsers)
    add_project_pipeline_parser(subparsers)

    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])

    if args.command == "review-pipeline":
        emit_result(
            build_review_pipeline_context(
                repo=args.repo,
                pr_number=args.pr,
            ),
            args.format,
        )
        return 0

    if args.command == "final-review":
        emit_result(
            build_final_review_context(
                repo=args.repo,
                pr_number=args.pr,
            ),
            args.format,
        )
        return 0

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

    if args.command == "project-pipeline":
        emit_result(
            build_project_pipeline_context(
                repo=args.repo,
                issue_number=args.issue,
                repo_root=args.repo_root,
            ),
            args.format,
        )
        return 0

    raise AssertionError(f"Unhandled command: {args.command}")


if __name__ == "__main__":
    raise SystemExit(main())

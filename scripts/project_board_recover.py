#!/usr/bin/env python3
"""Compatibility wrapper for project-board status recovery."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

import pipeline_board

PROJECT_ID = "PVT_kwDOBrtarc4BRNVy"
STATUS_FIELD_ID = "PVTSSF_lADOBrtarc4BRNVyzg_GmQc"

STATUS_BACKLOG = pipeline_board.STATUS_BACKLOG
STATUS_READY = pipeline_board.STATUS_READY
STATUS_REVIEW_POOL = pipeline_board.STATUS_REVIEW_POOL
STATUS_FINAL_REVIEW = pipeline_board.STATUS_FINAL_REVIEW
STATUS_DONE = pipeline_board.STATUS_DONE
STATUS_OPTION_IDS = pipeline_board.STATUS_OPTION_IDS
FAILURE_LABELS = pipeline_board.FAILURE_LABELS
label_names = pipeline_board.label_names
linked_pr_numbers = pipeline_board.linked_pr_numbers
is_tracked_issue_title = pipeline_board.is_tracked_issue_title
all_checks_green = pipeline_board.all_checks_green
infer_issue_status = pipeline_board.infer_issue_status
build_recovery_plan = pipeline_board.build_recovery_plan
apply_plan = pipeline_board.apply_plan
save_backup = pipeline_board.save_backup
print_summary = pipeline_board.print_summary
print_examples = pipeline_board.print_examples


def run_gh(*args: str) -> str:
    return subprocess.check_output(["gh", *args], text=True)


def fetch_board_items(owner: str, project_number: int, limit: int) -> dict:
    return pipeline_board.json.loads(
        run_gh(
            "project",
            "item-list",
            str(project_number),
            "--owner",
            owner,
            "--format",
            "json",
            "--limit",
            str(limit),
        )
    )


def fetch_issues(repo: str, limit: int) -> list[dict]:
    return pipeline_board.json.loads(
        run_gh(
            "issue",
            "list",
            "-R",
            repo,
            "--state",
            "all",
            "--limit",
            str(limit),
            "--json",
            "number,state,closedAt,title,labels",
        )
    )


def fetch_prs(repo: str, limit: int) -> list[dict]:
    return pipeline_board.json.loads(
        run_gh(
            "pr",
            "list",
            "-R",
            repo,
            "--state",
            "all",
            "--limit",
            str(limit),
            "--json",
            "number,state,isDraft,mergedAt,title,url,reviewDecision,statusCheckRollup,closingIssuesReferences",
        )
    )


def default_backup_path(project_number: int) -> Path:
    return pipeline_board.default_backup_path(project_number)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Recover project board item statuses after Status-field recreation."
    )
    parser.add_argument("--owner", default="CodingThrust")
    parser.add_argument("--repo", default="CodingThrust/problem-reductions")
    parser.add_argument("--project-number", type=int, default=8)
    parser.add_argument("--project-id", default=PROJECT_ID)
    parser.add_argument("--field-id", default=STATUS_FIELD_ID)
    parser.add_argument("--limit", type=int, default=500)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--backup-file", type=Path)
    parser.add_argument("--plan-file", type=Path)
    parser.add_argument("--no-examples", action="store_true")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])

    board_data = fetch_board_items(args.owner, args.project_number, args.limit)
    issues = fetch_issues(args.repo, args.limit)
    prs = fetch_prs(args.repo, args.limit)

    plan = build_recovery_plan(board_data, issues, prs)
    if args.plan_file is not None:
        args.plan_file.parent.mkdir(parents=True, exist_ok=True)
        args.plan_file.write_text(
            pipeline_board.json.dumps(plan, indent=2, sort_keys=True) + "\n"
        )

    print_summary(plan)
    if not args.no_examples:
        print_examples(plan)

    if not args.apply:
        return 0

    backup_file = args.backup_file or default_backup_path(args.project_number)
    save_backup(
        backup_file,
        board_data=board_data,
        issues=issues,
        prs=prs,
        plan=plan,
    )
    changed = apply_plan(plan, project_id=args.project_id, field_id=args.field_id)
    print("")
    print(f"Applied {changed} status updates.")
    print(f"Backup written to {backup_file}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

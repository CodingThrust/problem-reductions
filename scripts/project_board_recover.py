#!/usr/bin/env python3
"""Recover GitHub Project board statuses after the Status field was recreated."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

PROJECT_ID = "PVT_kwDOBrtarc4BRNVy"
STATUS_FIELD_ID = "PVTSSF_lADOBrtarc4BRNVyzg_GmQc"

STATUS_BACKLOG = "Backlog"
STATUS_READY = "Ready"
STATUS_REVIEW_POOL = "Review pool"
STATUS_FINAL_REVIEW = "Final review"
STATUS_DONE = "Done"

STATUS_OPTION_IDS = {
    STATUS_BACKLOG: "ab337660",
    STATUS_READY: "f37d0d80",
    STATUS_REVIEW_POOL: "7082ed60",
    STATUS_FINAL_REVIEW: "51a3d8bb",
    STATUS_DONE: "6aca54fa",
}

FAILURE_LABELS = {"PoorWritten", "Wrong", "Trivial", "Useless"}
COPILOT_REVIEWERS = {
    "copilot-pull-request-reviewer",
    "copilot-pull-request-reviewer[bot]",
}


def run_gh(*args: str) -> str:
    return subprocess.check_output(["gh", *args], text=True)


def fetch_board_items(owner: str, project_number: int, limit: int) -> dict:
    return json.loads(
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
    return json.loads(
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
    return json.loads(
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


def fetch_pr_reviews(repo: str, pr_number: int) -> list[dict]:
    data = json.loads(
        run_gh("pr", "view", str(pr_number), "-R", repo, "--json", "reviews")
    )
    return data.get("reviews", [])


def label_names(issue: dict) -> set[str]:
    return {label["name"] for label in issue.get("labels", [])}


def linked_pr_numbers(item: dict) -> list[int]:
    urls = item.get("linked pull requests") or []
    numbers = []
    for url in urls:
        try:
            numbers.append(int(url.rstrip("/").split("/")[-1]))
        except ValueError:
            continue
    return numbers


def is_tracked_issue_title(title: str | None) -> bool:
    if not title:
        return False
    return title.startswith("[Model]") or title.startswith("[Rule]")


def has_copilot_review(reviews: list[dict]) -> bool:
    for review in reviews:
        author = review.get("author") or review.get("user") or {}
        if author.get("login") in COPILOT_REVIEWERS:
            return True
    return False


def all_checks_green(pr: dict) -> bool:
    statuses = pr.get("statusCheckRollup") or []
    if not statuses:
        return False

    for status in statuses:
        typename = status.get("__typename")
        if typename == "CheckRun":
            if status.get("status") != "COMPLETED":
                return False
            if status.get("conclusion") not in {"SUCCESS", "SKIPPED", "NEUTRAL"}:
                return False
        elif typename == "StatusContext":
            if status.get("state") != "SUCCESS":
                return False
    return True


def infer_issue_status(
    issue: dict,
    linked_prs: list[dict],
    pr_reviews: dict[int, list[dict]],
) -> tuple[str, str]:
    labels = label_names(issue)
    merged_prs = [pr for pr in linked_prs if pr.get("mergedAt")]
    open_prs = [pr for pr in linked_prs if pr.get("state") == "OPEN"]

    if merged_prs:
        pr_numbers = ", ".join(f"#{pr['number']}" for pr in merged_prs)
        return STATUS_DONE, f"linked merged PR {pr_numbers}"

    if issue.get("state") == "CLOSED":
        return STATUS_DONE, "issue itself is closed"

    if open_prs:
        waiting_for_copilot = [
            pr
            for pr in open_prs
            if not has_copilot_review(pr_reviews.get(int(pr["number"]), []))
        ]
        if waiting_for_copilot:
            pr_numbers = ", ".join(f"#{pr['number']}" for pr in waiting_for_copilot)
            return STATUS_REVIEW_POOL, f"open PR {pr_numbers} waiting for Copilot review"

        green_prs = [pr for pr in open_prs if all_checks_green(pr)]
        if len(green_prs) == len(open_prs):
            pr_numbers = ", ".join(f"#{pr['number']}" for pr in open_prs)
            return STATUS_FINAL_REVIEW, f"Copilot reviewed green open PR {pr_numbers}"

        pr_numbers = ", ".join(f"#{pr['number']}" for pr in open_prs)
        return STATUS_REVIEW_POOL, f"open PR {pr_numbers} still implementing or fixing review"

    if "Good" in labels:
        return STATUS_READY, 'label "Good" present and no linked PR'

    if labels & FAILURE_LABELS:
        bad = ", ".join(sorted(labels & FAILURE_LABELS))
        return STATUS_BACKLOG, f"failure labels present: {bad}"

    return STATUS_BACKLOG, "default backlog: no linked PR and no Ready signal"


def build_recovery_plan(
    board_data: dict,
    issues: list[dict],
    prs: list[dict],
    pr_reviews: dict[int, list[dict]],
) -> list[dict]:
    issues_by_number = {issue["number"]: issue for issue in issues}
    prs_by_number = {pr["number"]: pr for pr in prs}

    plan = []
    for item in board_data.get("items", []):
        content = item.get("content") or {}
        issue_number = content.get("number")
        if issue_number is None:
            continue

        issue = issues_by_number.get(issue_number)
        if issue is None:
            continue

        title = content.get("title") or issue.get("title")
        if not is_tracked_issue_title(title):
            continue

        linked_prs = [
            prs_by_number[pr_number]
            for pr_number in linked_pr_numbers(item)
            if pr_number in prs_by_number
        ]
        status_name, reason = infer_issue_status(issue, linked_prs, pr_reviews)
        plan.append(
            {
                "item_id": item["id"],
                "issue_number": issue_number,
                "title": title,
                "current_status": item.get("status"),
                "proposed_status": status_name,
                "option_id": STATUS_OPTION_IDS[status_name],
                "reason": reason,
            }
        )

    return sorted(plan, key=lambda entry: entry["issue_number"])


def save_backup(
    backup_file: Path,
    *,
    board_data: dict,
    issues: list[dict],
    prs: list[dict],
    pr_reviews: dict[int, list[dict]],
    plan: list[dict],
) -> None:
    backup_file.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "board_data": board_data,
        "issues": issues,
        "prs": prs,
        "pr_reviews": pr_reviews,
        "plan": plan,
    }
    backup_file.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")


def apply_plan(
    plan: list[dict],
    *,
    project_id: str,
    field_id: str,
) -> int:
    changed = 0
    for entry in plan:
        if entry["current_status"] == entry["proposed_status"]:
            continue
        subprocess.check_call(
            [
                "gh",
                "project",
                "item-edit",
                "--project-id",
                project_id,
                "--id",
                entry["item_id"],
                "--field-id",
                field_id,
                "--single-select-option-id",
                entry["option_id"],
            ]
        )
        changed += 1
    return changed


def default_backup_path(project_number: int) -> Path:
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    return Path("/tmp") / f"project-{project_number}-status-recovery-{stamp}.json"


def print_summary(plan: list[dict]) -> None:
    counts = Counter(entry["proposed_status"] for entry in plan)
    print("Proposed status counts:")
    for status_name in [
        STATUS_BACKLOG,
        STATUS_READY,
        STATUS_REVIEW_POOL,
        STATUS_FINAL_REVIEW,
        STATUS_DONE,
    ]:
        print(f"  {status_name}: {counts.get(status_name, 0)}")


def print_examples(plan: list[dict], limit: int = 20) -> None:
    print("")
    print(f"First {min(limit, len(plan))} assignments:")
    for entry in plan[:limit]:
        print(
            f"  #{entry['issue_number']:<4} {entry['proposed_status']:<13} "
            f"{entry['reason']} | {entry['title']}"
        )


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

    prs_by_number = {pr["number"]: pr for pr in prs}
    open_linked_pr_numbers = sorted(
        {
            pr_number
            for item in board_data.get("items", [])
            for pr_number in linked_pr_numbers(item)
            if prs_by_number.get(pr_number, {}).get("state") == "OPEN"
        }
    )
    pr_reviews = {
        pr_number: fetch_pr_reviews(args.repo, pr_number)
        for pr_number in open_linked_pr_numbers
    }

    plan = build_recovery_plan(board_data, issues, prs, pr_reviews)
    if args.plan_file is not None:
        args.plan_file.parent.mkdir(parents=True, exist_ok=True)
        args.plan_file.write_text(json.dumps(plan, indent=2, sort_keys=True) + "\n")

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
        pr_reviews=pr_reviews,
        plan=plan,
    )
    changed = apply_plan(plan, project_id=args.project_id, field_id=args.field_id)
    print("")
    print(f"Applied {changed} status updates.")
    print(f"Backup written to {backup_file}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Shared project-board logic for polling, recovery, and board CLI helpers."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path
from typing import Callable

PROJECT_ID = "PVT_kwDOBrtarc4BRNVy"
STATUS_FIELD_ID = "PVTSSF_lADOBrtarc4BRNVyzg_GmQc"

COPILOT_REVIEWER = "copilot-pull-request-reviewer[bot]"
COPILOT_REVIEWERS = {
    "copilot-pull-request-reviewer",
    COPILOT_REVIEWER,
}

STATUS_BACKLOG = "Backlog"
STATUS_READY = "Ready"
STATUS_IN_PROGRESS = "In progress"
STATUS_REVIEW_POOL = "Review pool"
STATUS_UNDER_REVIEW = "Under review"
STATUS_FINAL_REVIEW = "Final review"
STATUS_ON_HOLD = "OnHold"
STATUS_DONE = "Done"

STATUS_OPTION_IDS = {
    STATUS_BACKLOG: "ab337660",
    STATUS_READY: "f37d0d80",
    STATUS_IN_PROGRESS: "a12cfc9c",
    STATUS_REVIEW_POOL: "7082ed60",
    STATUS_UNDER_REVIEW: "f04790ca",
    STATUS_FINAL_REVIEW: "51a3d8bb",
    STATUS_ON_HOLD: "48dfe446",
    STATUS_DONE: "6aca54fa",
}

STATUS_ALIASES = {
    "backlog": STATUS_BACKLOG,
    "ready": STATUS_READY,
    "in-progress": STATUS_IN_PROGRESS,
    "in_progress": STATUS_IN_PROGRESS,
    "in progress": STATUS_IN_PROGRESS,
    "review-pool": STATUS_REVIEW_POOL,
    "review_pool": STATUS_REVIEW_POOL,
    "review pool": STATUS_REVIEW_POOL,
    "under-review": STATUS_UNDER_REVIEW,
    "under_review": STATUS_UNDER_REVIEW,
    "under review": STATUS_UNDER_REVIEW,
    "final-review": STATUS_FINAL_REVIEW,
    "final_review": STATUS_FINAL_REVIEW,
    "final review": STATUS_FINAL_REVIEW,
    "on-hold": STATUS_ON_HOLD,
    "on_hold": STATUS_ON_HOLD,
    "on hold": STATUS_ON_HOLD,
    "onhold": STATUS_ON_HOLD,
    "done": STATUS_DONE,
}

FAILURE_LABELS = {"PoorWritten", "Wrong", "Trivial", "Useless"}


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


def fetch_pr_reviews(repo: str, pr_number: int) -> list[dict]:
    data = json.loads(run_gh("api", f"repos/{repo}/pulls/{pr_number}/reviews"))
    if not isinstance(data, list):
        raise ValueError(f"Unexpected PR review payload for #{pr_number}: {data!r}")
    return data


def fetch_pr_state(repo: str, pr_number: int) -> str:
    return run_gh(
        "pr",
        "view",
        str(pr_number),
        "--repo",
        repo,
        "--json",
        "state",
        "--jq",
        ".state",
    ).strip()


def resolve_issue_pr(repo: str, issue_number: int) -> int | None:
    data = json.loads(
        run_gh(
            "pr",
            "list",
            "-R",
            repo,
            "--search",
            f"Fix #{issue_number} in:title state:open",
            "--json",
            "number",
            "--limit",
            "1",
        )
    )
    if not data:
        return None
    return int(data[0]["number"])


def item_identity(item: dict) -> str:
    item_id = item.get("id")
    if item_id is not None:
        return str(item_id)

    content = item.get("content") or {}
    number = content.get("number")
    item_type = content.get("type", "item")
    if number is not None:
        return f"{item_type}:{number}"

    title = item.get("title")
    if title:
        return str(title)

    raise ValueError(f"Board item has no stable identity: {item!r}")


def load_state(state_file: Path) -> dict:
    if not state_file.exists():
        return {"visible": {}, "pending": []}

    raw = state_file.read_text().strip()
    if not raw:
        return {"visible": {}, "pending": []}

    data = json.loads(raw)
    if not isinstance(data, dict):
        raise ValueError(f"State file must contain a JSON object: {state_file}")

    visible = data.get("visible", {})
    pending = data.get("pending", [])
    if not isinstance(visible, dict) or not isinstance(pending, list):
        raise ValueError(f"Invalid poll state format: {state_file}")

    return {"visible": visible, "pending": [str(item_id) for item_id in pending]}


def save_state(state_file: Path, state: dict) -> None:
    state_file.parent.mkdir(parents=True, exist_ok=True)
    state_file.write_text(json.dumps(state, indent=2, sort_keys=True) + "\n")


def has_copilot_review(reviews: list[dict]) -> bool:
    return any(
        (review.get("author") or review.get("user") or {}).get("login")
        in COPILOT_REVIEWERS
        for review in reviews
    )


def linked_pr_numbers(item: dict, repo: str | None = None) -> list[int]:
    urls = item.get("linked pull requests") or []
    numbers: list[int] = []

    if repo is not None:
        prefix = f"https://github.com/{repo}/pull/"
        for url in urls:
            if not isinstance(url, str) or not url.startswith(prefix):
                continue
            suffix = url.removeprefix(prefix)
            if suffix.isdigit():
                numbers.append(int(suffix))
        return numbers

    for url in urls:
        if not isinstance(url, str):
            continue
        try:
            numbers.append(int(url.rstrip("/").split("/")[-1]))
        except ValueError:
            continue
    return numbers


def linked_repo_pr_numbers(item: dict, repo: str) -> list[int]:
    return linked_pr_numbers(item, repo)


def ready_entries(board_data: dict) -> dict[str, dict]:
    entries = {}
    for item in board_data.get("items", []):
        if item.get("status") != STATUS_READY:
            continue

        content = item.get("content") or {}
        number = content.get("number")
        if number is None:
            continue

        entries[item_identity(item)] = {"number": int(number)}
    return entries


def review_entries(
    board_data: dict,
    repo: str,
    review_fetcher: Callable[[str, int], list[dict]],
    pr_resolver: Callable[[str, int], int | None] | None,
    pr_state_fetcher: Callable[[str, int], str],
) -> dict[str, dict]:
    entries = {}
    for item in board_data.get("items", []):
        if item.get("status") != STATUS_REVIEW_POOL:
            continue

        content = item.get("content") or {}
        item_type = content.get("type")
        number = content.get("number")
        if number is None:
            continue

        pr_number: int | None
        if item_type == "PullRequest":
            pr_number = int(number)
            if pr_state_fetcher(repo, pr_number) != "OPEN":
                continue
        elif item_type == "Issue":
            linked_numbers = linked_pr_numbers(item, repo)
            if len(linked_numbers) > 1:
                continue
            if len(linked_numbers) == 1:
                pr_number = linked_numbers[0]
                if pr_state_fetcher(repo, pr_number) != "OPEN":
                    continue
            else:
                if pr_resolver is None:
                    raise ValueError("review mode requires pr_resolver for issue cards without linked PRs")
                pr_number = pr_resolver(repo, int(number))
                if pr_number is None:
                    continue
                if pr_state_fetcher(repo, pr_number) != "OPEN":
                    continue
        else:
            pr_number = None

        if pr_number is None:
            continue

        reviews = review_fetcher(repo, pr_number)
        if has_copilot_review(reviews):
            entries[item_identity(item)] = {"number": pr_number}
    return entries


def final_review_entries(
    board_data: dict,
    repo: str,
    pr_resolver: Callable[[str, int], int | None] | None,
    pr_state_fetcher: Callable[[str, int], str],
) -> dict[str, dict]:
    entries = {}
    for item in board_data.get("items", []):
        if item.get("status") != STATUS_FINAL_REVIEW:
            continue

        content = item.get("content") or {}
        item_type = content.get("type")
        number = content.get("number")
        if number is None:
            continue

        pr_number: int | None
        if item_type == "PullRequest":
            pr_number = int(number)
            if pr_state_fetcher(repo, pr_number) != "OPEN":
                continue
        elif item_type == "Issue":
            linked_numbers = linked_pr_numbers(item, repo)
            if len(linked_numbers) > 1:
                continue
            if len(linked_numbers) == 1:
                pr_number = linked_numbers[0]
                if pr_state_fetcher(repo, pr_number) != "OPEN":
                    continue
            else:
                if pr_resolver is None:
                    raise ValueError(
                        "final-review mode requires pr_resolver for issue cards without linked PRs"
                    )
                pr_number = pr_resolver(repo, int(number))
                if pr_number is None:
                    continue
                if pr_state_fetcher(repo, pr_number) != "OPEN":
                    continue
        else:
            pr_number = None

        if pr_number is None:
            continue

        entries[item_identity(item)] = {"number": pr_number}
    return entries


def current_entries(
    mode: str,
    board_data: dict,
    repo: str | None = None,
    review_fetcher: Callable[[str, int], list[dict]] | None = None,
    pr_resolver: Callable[[str, int], int | None] | None = None,
    pr_state_fetcher: Callable[[str, int], str] | None = None,
) -> dict[str, dict]:
    if mode == "ready":
        return ready_entries(board_data)
    if mode == "review":
        if repo is None:
            raise ValueError("repo is required in review mode")
        if review_fetcher is None or pr_state_fetcher is None:
            raise ValueError("review mode requires review_fetcher and pr_state_fetcher")
        return review_entries(
            board_data,
            repo,
            review_fetcher,
            pr_resolver,
            pr_state_fetcher,
        )
    if mode == "final-review":
        if repo is None:
            raise ValueError("repo is required in final-review mode")
        if pr_state_fetcher is None:
            raise ValueError("final-review mode requires pr_state_fetcher")
        return final_review_entries(
            board_data,
            repo,
            pr_resolver,
            pr_state_fetcher,
        )
    raise ValueError(f"Unsupported mode: {mode}")


def process_snapshot(
    mode: str,
    board_data: dict,
    state_file: Path,
    repo: str | None = None,
    review_fetcher: Callable[[str, int], list[dict]] | None = None,
    pr_resolver: Callable[[str, int], int | None] | None = None,
    pr_state_fetcher: Callable[[str, int], str] | None = None,
) -> tuple[str, int] | None:
    state = load_state(state_file)
    previous_visible = state["visible"]
    current_visible = current_entries(
        mode,
        board_data,
        repo,
        review_fetcher,
        pr_resolver,
        pr_state_fetcher,
    )

    pending = [item_id for item_id in state["pending"] if item_id in current_visible]
    entered = sorted(
        (item_id for item_id in current_visible if item_id not in previous_visible),
        key=lambda item_id: (current_visible[item_id]["number"], item_id),
    )
    for item_id in entered:
        if item_id not in pending:
            pending.append(item_id)

    state["visible"] = current_visible
    state["pending"] = pending
    save_state(state_file, state)

    if not pending:
        return None

    item_id = pending[0]
    return item_id, int(current_visible[item_id]["number"])


def ack_item(state_file: Path, item_id: str) -> None:
    state = load_state(state_file)
    state["pending"] = [
        pending_id for pending_id in state["pending"] if pending_id != item_id
    ]
    save_state(state_file, state)


def label_names(issue: dict) -> set[str]:
    return {label["name"] for label in issue.get("labels", [])}


def is_tracked_issue_title(title: str | None) -> bool:
    if not title:
        return False
    return title.startswith("[Model]") or title.startswith("[Rule]")


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


def normalize_status_name(status: str) -> str:
    normalized = status.strip()
    if normalized in STATUS_OPTION_IDS:
        return normalized

    alias = STATUS_ALIASES.get(normalized.lower())
    if alias is None:
        choices = ", ".join(sorted(STATUS_OPTION_IDS))
        raise ValueError(f"Unsupported status {status!r}. Expected one of: {choices}")
    return alias


def move_item(
    item_id: str,
    status: str,
    *,
    project_id: str = PROJECT_ID,
    field_id: str = STATUS_FIELD_ID,
) -> None:
    status_name = normalize_status_name(status)
    subprocess.check_call(
        [
            "gh",
            "project",
            "item-edit",
            "--project-id",
            project_id,
            "--id",
            item_id,
            "--field-id",
            field_id,
            "--single-select-option-id",
            STATUS_OPTION_IDS[status_name],
        ]
    )


def apply_plan(
    plan: list[dict],
    *,
    project_id: str = PROJECT_ID,
    field_id: str = STATUS_FIELD_ID,
) -> int:
    changed = 0
    for entry in plan:
        if entry["current_status"] == entry["proposed_status"]:
            continue
        move_item(
            entry["item_id"],
            entry["proposed_status"],
            project_id=project_id,
            field_id=field_id,
        )
        changed += 1
    return changed


def print_next_item(
    next_item: tuple[str, int] | None,
    *,
    mode: str,
    fmt: str = "text",
) -> int:
    if next_item is None:
        return 1

    item_id, number = next_item
    if fmt == "json":
        print(
            json.dumps(
                {
                    "mode": mode,
                    "item_id": item_id,
                    "number": number,
                }
            )
        )
    else:
        print(f"{item_id}\t{number}")
    return 0


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Project board automation helpers.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    next_parser = subparsers.add_parser("next")
    next_parser.add_argument("mode", choices=["ready", "review", "final-review"])
    next_parser.add_argument("state_file", type=Path)
    next_parser.add_argument("--repo")
    next_parser.add_argument("--owner", default="CodingThrust")
    next_parser.add_argument("--project-number", type=int, default=8)
    next_parser.add_argument("--limit", type=int, default=500)
    next_parser.add_argument("--format", choices=["text", "json"], default="text")

    ack_parser = subparsers.add_parser("ack")
    ack_parser.add_argument("state_file", type=Path)
    ack_parser.add_argument("item_id")

    move_parser = subparsers.add_parser("move")
    move_parser.add_argument("item_id")
    move_parser.add_argument("status")
    move_parser.add_argument("--project-id", default=PROJECT_ID)
    move_parser.add_argument("--field-id", default=STATUS_FIELD_ID)

    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])

    if args.command == "ack":
        ack_item(args.state_file, args.item_id)
        return 0

    if args.command == "move":
        move_item(
            args.item_id,
            args.status,
            project_id=args.project_id,
            field_id=args.field_id,
        )
        return 0

    if args.mode in {"review", "final-review"} and not args.repo:
        raise SystemExit(f"--repo is required in {args.mode} mode")

    board_data = fetch_board_items(args.owner, args.project_number, args.limit)
    next_item = process_snapshot(
        args.mode,
        board_data,
        args.state_file,
        repo=args.repo,
        review_fetcher=fetch_pr_reviews,
        pr_resolver=resolve_issue_pr,
        pr_state_fetcher=fetch_pr_state,
    )
    return print_next_item(next_item, mode=args.mode, fmt=args.format)


if __name__ == "__main__":
    raise SystemExit(main())

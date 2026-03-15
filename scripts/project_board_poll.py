#!/usr/bin/env python3
"""Track eligible project-board items and expose a retryable pending queue."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Callable

COPILOT_REVIEWER = "copilot-pull-request-reviewer[bot]"


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


def fetch_pr_reviews(repo: str, pr_number: int) -> list[dict]:
    output = subprocess.check_output(
        ["gh", "api", f"repos/{repo}/pulls/{pr_number}/reviews"],
        text=True,
    )
    data = json.loads(output)
    if not isinstance(data, list):
        raise ValueError(f"Unexpected PR review payload for #{pr_number}: {data!r}")
    return data


def resolve_issue_pr(repo: str, issue_number: int) -> int | None:
    output = subprocess.check_output(
        [
            "gh",
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
        ],
        text=True,
    )
    data = json.loads(output)
    if not data:
        return None
    return int(data[0]["number"])


def has_copilot_review(reviews: list[dict]) -> bool:
    return any(
        review.get("user", {}).get("login") == COPILOT_REVIEWER for review in reviews
    )


def ready_entries(board_data: dict) -> dict[str, dict]:
    entries = {}
    for item in board_data.get("items", []):
        if item.get("status") != "Ready":
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
    review_fetcher: Callable[[str, int], list[dict]] = fetch_pr_reviews,
    pr_resolver: Callable[[str, int], int | None] = resolve_issue_pr,
) -> dict[str, dict]:
    entries = {}
    for item in board_data.get("items", []):
        if item.get("status") != "Review pool":
            continue

        content = item.get("content") or {}
        item_type = content.get("type")
        number = content.get("number")
        if number is None:
            continue

        pr_number: int | None
        if item_type == "PullRequest":
            pr_number = int(number)
        elif item_type == "Issue":
            pr_number = pr_resolver(repo, int(number))
        else:
            pr_number = None

        if pr_number is None:
            continue

        reviews = review_fetcher(repo, pr_number)
        if has_copilot_review(reviews):
            entries[item_identity(item)] = {"number": pr_number}
    return entries


def current_entries(
    mode: str,
    board_data: dict,
    repo: str | None = None,
    review_fetcher: Callable[[str, int], list[dict]] = fetch_pr_reviews,
    pr_resolver: Callable[[str, int], int | None] = resolve_issue_pr,
) -> dict[str, dict]:
    if mode == "ready":
        return ready_entries(board_data)
    if mode == "review":
        if repo is None:
            raise ValueError("repo is required in review mode")
        return review_entries(board_data, repo, review_fetcher, pr_resolver)
    raise ValueError(f"Unsupported mode: {mode}")


def process_snapshot(
    mode: str,
    board_data: dict,
    state_file: Path,
    repo: str | None = None,
    review_fetcher: Callable[[str, int], list[dict]] = fetch_pr_reviews,
    pr_resolver: Callable[[str, int], int | None] = resolve_issue_pr,
) -> tuple[str, int] | None:
    state = load_state(state_file)
    previous_visible = state["visible"]
    current_visible = current_entries(mode, board_data, repo, review_fetcher, pr_resolver)

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


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Track newly eligible board items for forever pollers."
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    poll = subparsers.add_parser("poll")
    poll.add_argument("mode", choices=["ready", "review"])
    poll.add_argument("state_file", type=Path)
    poll.add_argument("--repo")

    ack = subparsers.add_parser("ack")
    ack.add_argument("state_file", type=Path)
    ack.add_argument("item_id")

    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])

    if args.command == "ack":
        ack_item(args.state_file, args.item_id)
        return 0

    if args.mode == "review" and not args.repo:
        raise SystemExit("--repo is required in review mode")

    board_data = json.load(sys.stdin)
    next_item = process_snapshot(
        args.mode,
        board_data,
        args.state_file,
        repo=args.repo,
    )
    if next_item is None:
        return 1

    item_id, number = next_item
    print(f"{item_id}\t{number}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

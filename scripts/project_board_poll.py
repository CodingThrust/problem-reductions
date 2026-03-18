#!/usr/bin/env python3
"""Compatibility wrapper for the board poller CLI."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

import pipeline_board

item_identity = pipeline_board.item_identity
load_state = pipeline_board.load_state
save_state = pipeline_board.save_state
ready_entries = pipeline_board.ready_entries
ack_item = pipeline_board.ack_item


def fetch_pr_reviews(repo: str, pr_number: int) -> list[dict]:
    output = subprocess.check_output(
        ["gh", "api", f"repos/{repo}/pulls/{pr_number}/reviews"],
        text=True,
    )
    data = pipeline_board.json.loads(output)
    if not isinstance(data, list):
        raise ValueError(f"Unexpected PR review payload for #{pr_number}: {data!r}")
    return data


def fetch_pr_state(repo: str, pr_number: int) -> str:
    return subprocess.check_output(
        [
            "gh",
            "pr",
            "view",
            str(pr_number),
            "--repo",
            repo,
            "--json",
            "state",
            "--jq",
            ".state",
        ],
        text=True,
    ).strip()


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
    data = pipeline_board.json.loads(output)
    if not data:
        return None
    return int(data[0]["number"])


def linked_repo_pr_numbers(item: dict, repo: str) -> list[int]:
    return pipeline_board.linked_repo_pr_numbers(item, repo)


def review_entries(
    board_data: dict,
    repo: str,
    pr_resolver=resolve_issue_pr,
    pr_state_fetcher=fetch_pr_state,
) -> dict[str, dict]:
    return pipeline_board.review_entries(
        board_data,
        repo,
        pr_resolver,
        pr_state_fetcher,
    )


def current_entries(
    mode: str,
    board_data: dict,
    repo: str | None = None,
    pr_resolver=resolve_issue_pr,
    pr_state_fetcher=fetch_pr_state,
) -> dict[str, dict]:
    return pipeline_board.current_entries(
        mode,
        board_data,
        repo,
        pr_resolver,
        pr_state_fetcher,
    )


def process_snapshot(
    mode: str,
    board_data: dict,
    state_file: Path,
    repo: str | None = None,
    pr_resolver=resolve_issue_pr,
    pr_state_fetcher=fetch_pr_state,
) -> tuple[str, int] | None:
    return pipeline_board.process_snapshot(
        mode,
        board_data,
        state_file,
        repo,
        pr_resolver,
        pr_state_fetcher,
    )


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

    board_data = pipeline_board.json.load(sys.stdin)
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

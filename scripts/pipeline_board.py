#!/usr/bin/env python3
"""Shared project-board logic for polling, recovery, and board CLI helpers."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time
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


def _graphql_board_query(project_id: str, page_size: int, cursor: str | None) -> str:
    """Build a lightweight GraphQL query for project board items.

    Fetches only id, status, and content (type/number/title) — no other field
    values.  This costs ~1 GraphQL point per page vs ~20-50 for
    ``gh project item-list`` which pulls all field values.
    """
    after = f', after: "{cursor}"' if cursor else ""
    return (
        "query {"
        f'  node(id: "{project_id}") {{'
        "    ... on ProjectV2 {"
        f"      items(first: {page_size}{after}) {{"
        "        pageInfo { hasNextPage endCursor }"
        "        nodes {"
        "          id"
        '          fieldValueByName(name: "Status") {'
        "            ... on ProjectV2ItemFieldSingleSelectValue { name }"
        "          }"
        "          content {"
        "            __typename"
        "            ... on Issue { number title }"
        "            ... on PullRequest { number title }"
        "          }"
        "        }"
        "      }"
        "    }"
        "  }"
        "}"
    )


def _parse_graphql_board_items(raw: dict) -> tuple[list[dict], bool, str | None]:
    """Parse a GraphQL board response into (items, has_next_page, cursor)."""
    node = raw.get("data", {}).get("node") or {}
    project = node  # already a ProjectV2 via inline fragment
    connection = project.get("items") or {}
    page_info = connection.get("pageInfo") or {}
    has_next = page_info.get("hasNextPage", False)
    cursor = page_info.get("endCursor")

    items: list[dict] = []
    for gql_node in connection.get("nodes") or []:
        status_field = gql_node.get("fieldValueByName") or {}
        status = status_field.get("name")

        content_raw = gql_node.get("content") or {}
        typename = content_raw.get("__typename")
        if typename not in ("Issue", "PullRequest"):
            continue

        content = {
            "type": typename,
            "number": content_raw.get("number"),
            "title": content_raw.get("title"),
        }
        items.append({
            "id": gql_node.get("id"),
            "status": status,
            "content": content,
            "title": content_raw.get("title"),
        })

    return items, has_next, cursor


def fetch_board_items_graphql(
    project_id: str,
    limit: int,
    *,
    page_size: int = 100,
    cache_file: Path | None = None,
    cache_max_age: float = 120,
) -> dict:
    """Fetch board items using a lightweight custom GraphQL query.

    Returns ``{"items": [...]}`` in the same shape as ``gh project item-list``
    (minus ``linked pull requests`` which callers resolve via ``pr_resolver``).
    Costs ~1 GraphQL point per 100-item page vs ~20-50 for the full CLI fetch.
    """
    if cache_file is not None:
        try:
            age = time.time() - cache_file.stat().st_mtime
            if age < cache_max_age:
                return json.loads(cache_file.read_text())
        except (FileNotFoundError, json.JSONDecodeError):
            pass

    all_items: list[dict] = []
    cursor: str | None = None
    while len(all_items) < limit:
        batch = min(page_size, limit - len(all_items))
        query = _graphql_board_query(project_id, batch, cursor)
        raw = json.loads(run_gh("api", "graphql", "-f", f"query={query}"))
        items, has_next, cursor = _parse_graphql_board_items(raw)
        all_items.extend(items)
        if not has_next:
            break

    data = {"items": all_items}
    if cache_file is not None:
        cache_file.parent.mkdir(parents=True, exist_ok=True)
        cache_file.write_text(json.dumps(data))

    return data


DEFAULT_OWNER = "CodingThrust"
DEFAULT_PROJECT_NUMBER = 8


def fetch_board_items(
    owner: str,
    project_number: int,
    limit: int,
    *,
    cache_file: Path | None = None,
    cache_max_age: float = 120,
    lite: bool = False,
) -> dict:
    """Fetch project board items, optionally using a file cache.

    When *lite* is True **and** the target is the default project, uses a
    lightweight custom GraphQL query (~1 point per 100 items) that omits
    ``linked pull requests``.  Callers that need linked PRs (review,
    final-review) should pass ``lite=False`` (the default).

    For non-default owner/project_number, always falls back to the full
    ``gh project item-list`` fetch regardless of *lite*.
    """
    if cache_file is not None:
        try:
            age = time.time() - cache_file.stat().st_mtime
            if age < cache_max_age:
                return json.loads(cache_file.read_text())
        except (FileNotFoundError, json.JSONDecodeError):
            pass

    if lite and owner == DEFAULT_OWNER and project_number == DEFAULT_PROJECT_NUMBER:
        data = fetch_board_items_graphql(
            PROJECT_ID,
            limit,
        )
    else:
        data = json.loads(
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

    if cache_file is not None:
        cache_file.parent.mkdir(parents=True, exist_ok=True)
        cache_file.write_text(json.dumps(data))

    return data


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


def fetch_pr_info(repo: str, pr_number: int) -> dict:
    data = json.loads(
        run_gh(
            "pr",
            "view",
            str(pr_number),
            "--repo",
            repo,
            "--json",
            "number,state,title,url",
        )
    )
    if not isinstance(data, dict):
        raise ValueError(f"Unexpected PR payload for #{pr_number}: {data!r}")
    return data


def batch_fetch_prs_with_reviews(repo: str, pr_numbers: list[int]) -> dict[int, dict]:
    """Fetch PR info + reviews for multiple PRs in a single GraphQL call.

    Returns dict keyed by PR number with keys: number, state, title, url, reviews.
    """
    if not pr_numbers:
        return {}

    owner, name = repo.split("/", 1)

    fragments = []
    for n in pr_numbers:
        fragments.append(
            f"pr_{n}: pullRequest(number: {n}) {{"
            f"  number state title url"
            f"  reviews(last: 50) {{ nodes {{ author {{ login }} state }} }}"
            f"}}"
        )

    query = (
        f'query {{ repository(owner: "{owner}", name: "{name}") {{'
        + " ".join(fragments)
        + "}}"
    )

    raw = json.loads(run_gh("api", "graphql", "-f", f"query={query}"))
    repo_data = raw.get("data", {}).get("repository", {})

    result: dict[int, dict] = {}
    for n in pr_numbers:
        pr_data = repo_data.get(f"pr_{n}")
        if pr_data is None:
            continue
        reviews_nodes = pr_data.get("reviews", {}).get("nodes", [])
        result[n] = {
            "number": pr_data["number"],
            "state": pr_data["state"],
            "title": pr_data.get("title", ""),
            "url": pr_data.get("url", ""),
            "reviews": reviews_nodes,
        }
    return result


def batch_fetch_issues(repo: str, issue_numbers: list[int]) -> dict[int, dict]:
    """Fetch issue metadata for multiple issues in a single GraphQL call.

    Returns dict keyed by issue number with keys: number, title, body, state, url, labels, comments.
    """
    if not issue_numbers:
        return {}

    owner, name = repo.split("/", 1)

    fragments = []
    for n in issue_numbers:
        fragments.append(
            f"issue_{n}: issue(number: {n}) {{"
            f"  number title body state url"
            f"  labels(first: 20) {{ nodes {{ name }} }}"
            f"  comments(first: 50) {{ nodes {{ body }} }}"
            f"}}"
        )

    query = (
        f'query {{ repository(owner: "{owner}", name: "{name}") {{'
        + " ".join(fragments)
        + "}}"
    )

    raw = json.loads(run_gh("api", "graphql", "-f", f"query={query}"))
    repo_data = raw.get("data", {}).get("repository", {})

    result: dict[int, dict] = {}
    for n in issue_numbers:
        issue_data = repo_data.get(f"issue_{n}")
        if issue_data is None:
            continue
        result[n] = {
            "number": issue_data["number"],
            "title": issue_data.get("title", ""),
            "body": issue_data.get("body", ""),
            "state": issue_data.get("state", ""),
            "url": issue_data.get("url", ""),
            "labels": issue_data.get("labels", {}).get("nodes", []),
            "comments": issue_data.get("comments", {}).get("nodes", []),
        }
    return result


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


def entry_title(item: dict) -> str | None:
    content = item.get("content") or {}
    return content.get("title") or item.get("title")


def build_entry(
    item: dict,
    *,
    number: int,
    issue_number: int | None = None,
    pr_number: int | None = None,
) -> dict:
    return {
        "number": number,
        "issue_number": issue_number,
        "pr_number": pr_number,
        "status": item.get("status"),
        "title": entry_title(item),
    }


def _scan_existing_problems(repo_root: str | Path) -> set[str]:
    """Return public struct/enum names under src/models/ (lightweight model scan)."""
    import re as _re

    models_root = Path(repo_root) / "src" / "models"
    if not models_root.exists():
        return set()
    names: set[str] = set()
    for path in models_root.rglob("*.rs"):
        for m in _re.finditer(r"\bpub\s+(?:struct|enum)\s+([A-Z][A-Za-z0-9_]*)\b", path.read_text()):
            names.add(m.group(1))
    return names


def _is_rule_blocked(title: str | None, existing_problems: set[str]) -> bool:
    """Return True if *title* is a [Rule] whose source or target model is missing."""
    import re as _re

    if not title:
        return False
    m = _re.match(r"^\[Rule\]\s+(?P<source>.+?)\s+to\s+(?P<target>.+?)\s*$", title)
    if not m:
        return False
    return m.group("source") not in existing_problems or m.group("target") not in existing_problems


def ready_entries(board_data: dict, *, repo_root: str | Path | None = None) -> dict[str, dict]:
    existing: set[str] | None = None
    if repo_root is not None:
        existing = _scan_existing_problems(repo_root)

    entries = {}
    for item in board_data.get("items", []):
        if item.get("status") != STATUS_READY:
            continue

        content = item.get("content") or {}
        number = content.get("number")
        if number is None:
            continue

        if existing is not None:
            title = content.get("title") or item.get("title")
            if _is_rule_blocked(title, existing):
                continue

        issue_number = int(number)
        entries[item_identity(item)] = build_entry(
            item,
            number=issue_number,
            issue_number=issue_number,
        )
    return entries


def status_items(
    board_data: dict,
    status_name: str,
    *,
    content_types: set[str] | None = None,
) -> list[dict]:
    if content_types is None:
        content_types = {"Issue"}
    items = []
    for item in board_data.get("items", []):
        if item.get("status") != status_name:
            continue

        content = item.get("content") or {}
        item_type = content.get("type")
        if item_type not in content_types:
            continue

        number = content.get("number")
        if number is None:
            continue

        issue_number = int(number) if item_type == "Issue" else None
        pr_number = int(number) if item_type == "PullRequest" else None
        entry = build_entry(
            item,
            number=int(number),
            issue_number=issue_number,
            pr_number=pr_number,
        )
        entry["item_id"] = item_identity(item)
        items.append(entry)

    return sorted(items, key=lambda entry: (entry["number"], entry["item_id"]))


def review_entries(
    board_data: dict,
    repo: str,
    review_fetcher: Callable[[str, int], list[dict]],
    pr_resolver: Callable[[str, int], int | None] | None,
    pr_state_fetcher: Callable[[str, int], str],
    *,
    batch_pr_fetcher: Callable[[str, list[int]], dict[int, dict]] | None = None,
) -> dict[str, dict]:
    pr_cache: dict[int, dict] = {}
    if batch_pr_fetcher is not None:
        all_pr_numbers: list[int] = []
        for item in board_data.get("items", []):
            if item.get("status") != STATUS_REVIEW_POOL:
                continue
            content = item.get("content") or {}
            number = content.get("number")
            if number is None:
                continue
            if content.get("type") == "PullRequest":
                all_pr_numbers.append(int(number))
            else:
                all_pr_numbers.extend(linked_pr_numbers(item, repo))
        if all_pr_numbers:
            pr_cache = batch_pr_fetcher(repo, all_pr_numbers)

    def _get_pr_state(pr_num: int) -> str:
        if pr_num in pr_cache:
            return str(pr_cache[pr_num].get("state", ""))
        return pr_state_fetcher(repo, pr_num)

    def _get_reviews(pr_num: int) -> list[dict]:
        if pr_num in pr_cache:
            return pr_cache[pr_num].get("reviews", [])
        return review_fetcher(repo, pr_num)

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
            if _get_pr_state(pr_number) != "OPEN":
                continue
        elif item_type == "Issue":
            linked_numbers = linked_pr_numbers(item, repo)
            if len(linked_numbers) > 1:
                continue
            if len(linked_numbers) == 1:
                pr_number = linked_numbers[0]
                if _get_pr_state(pr_number) != "OPEN":
                    continue
            else:
                if pr_resolver is None:
                    raise ValueError("review mode requires pr_resolver for issue cards without linked PRs")
                pr_number = pr_resolver(repo, int(number))
                if pr_number is None:
                    continue
                if _get_pr_state(pr_number) != "OPEN":
                    continue
        else:
            pr_number = None

        if pr_number is None:
            continue

        reviews = _get_reviews(pr_number)
        if has_copilot_review(reviews):
            issue_number = int(number) if item_type == "Issue" else None
            entries[item_identity(item)] = build_entry(
                item,
                number=pr_number,
                issue_number=issue_number,
                pr_number=pr_number,
            )
    return entries


def review_candidates(
    board_data: dict,
    repo: str,
    review_fetcher: Callable[[str, int], list[dict]],
    pr_resolver: Callable[[str, int], int | None] | None,
    pr_info_fetcher: Callable[[str, int], dict],
    *,
    batch_pr_fetcher: Callable[[str, list[int]], dict[int, dict]] | None = None,
) -> list[dict]:
    # Pre-fetch all known PR numbers in one batch if batch fetcher is available
    pr_cache: dict[int, dict] = {}
    if batch_pr_fetcher is not None:
        all_pr_numbers: list[int] = []
        for item in board_data.get("items", []):
            if item.get("status") != STATUS_REVIEW_POOL:
                continue
            content = item.get("content") or {}
            number = content.get("number")
            if number is None:
                continue
            if content.get("type") == "PullRequest":
                all_pr_numbers.append(int(number))
            else:
                all_pr_numbers.extend(linked_pr_numbers(item, repo))
        if all_pr_numbers:
            pr_cache = batch_pr_fetcher(repo, all_pr_numbers)

    def _get_pr_info(pr_num: int) -> dict:
        if pr_num in pr_cache:
            return pr_cache[pr_num]
        return pr_info_fetcher(repo, pr_num)

    def _get_reviews(pr_num: int) -> list[dict]:
        if pr_num in pr_cache:
            return pr_cache[pr_num].get("reviews", [])
        return review_fetcher(repo, pr_num)

    candidates = []
    for item in board_data.get("items", []):
        if item.get("status") != STATUS_REVIEW_POOL:
            continue

        content = item.get("content") or {}
        item_type = content.get("type")
        number = content.get("number")
        if number is None:
            continue

        base_entry = build_entry(item, number=0)
        base_entry["item_id"] = item_identity(item)
        issue_number = int(number) if item_type == "Issue" else None

        if item_type == "PullRequest":
            pr_number = int(number)
            pr_info = _get_pr_info(pr_number)
            state = pr_info.get("state")
            base_entry.update({"number": pr_number, "pr_number": pr_number})
            if state != "OPEN":
                base_entry.update(
                    {
                        "eligibility": "stale-closed-pr",
                        "reason": f"linked PR #{pr_number} is {state}",
                    }
                )
                candidates.append(base_entry)
                continue

            reviews = _get_reviews(pr_number)
            if has_copilot_review(reviews):
                base_entry.update({"eligibility": "eligible", "reason": "copilot reviewed"})
            else:
                base_entry.update(
                    {
                        "eligibility": "waiting-for-copilot",
                        "reason": f"open PR #{pr_number} waiting for Copilot review",
                    }
                )
            candidates.append(base_entry)
            continue

        if item_type != "Issue":
            continue

        base_entry["issue_number"] = issue_number
        linked_numbers = linked_pr_numbers(item, repo)
        if len(linked_numbers) > 1:
            linked_infos = [_get_pr_info(pr_number) for pr_number in linked_numbers]
            open_numbers = [
                int(info["number"])
                for info in linked_infos
                if str(info.get("state")).upper() == "OPEN"
            ]
            recommendation = open_numbers[0] if len(open_numbers) == 1 else None
            base_entry.update(
                {
                    "number": recommendation or int(linked_infos[0]["number"]),
                    "pr_number": recommendation,
                    "eligibility": "ambiguous-linked-prs",
                    "reason": "multiple linked repo PRs require confirmation",
                    "recommendation": recommendation,
                    "linked_repo_prs": [
                        {
                            "number": int(info["number"]),
                            "state": str(info.get("state")),
                            "title": info.get("title"),
                        }
                        for info in linked_infos
                    ],
                }
            )
            candidates.append(base_entry)
            continue

        if len(linked_numbers) == 1:
            pr_number = linked_numbers[0]
            pr_info = _get_pr_info(pr_number)
            state = pr_info.get("state")
            base_entry.update({"number": pr_number, "pr_number": pr_number})
            if state != "OPEN":
                base_entry.update(
                    {
                        "eligibility": "stale-closed-pr",
                        "reason": f"linked PR #{pr_number} is {state}",
                    }
                )
                candidates.append(base_entry)
                continue
        else:
            if pr_resolver is None:
                raise ValueError("review candidate listing requires pr_resolver for issue cards without linked PRs")
            pr_number = pr_resolver(repo, issue_number)
            if pr_number is None:
                base_entry.update(
                    {
                        "number": issue_number,
                        "pr_number": None,
                        "eligibility": "no-open-pr",
                        "reason": f"issue #{issue_number} has no open PR",
                    }
                )
                candidates.append(base_entry)
                continue
            base_entry.update({"number": pr_number, "pr_number": pr_number})

        reviews = _get_reviews(pr_number)
        if has_copilot_review(reviews):
            base_entry.update({"eligibility": "eligible", "reason": "copilot reviewed"})
        else:
            base_entry.update(
                {
                    "eligibility": "waiting-for-copilot",
                    "reason": f"open PR #{pr_number} waiting for Copilot review",
                }
            )
        candidates.append(base_entry)

    return sorted(
        candidates,
        key=lambda entry: (
            entry["pr_number"] is None,
            entry["number"],
            entry["item_id"],
        ),
    )


def final_review_entries(
    board_data: dict,
    repo: str,
    pr_resolver: Callable[[str, int], int | None] | None,
    pr_state_fetcher: Callable[[str, int], str],
    *,
    batch_pr_fetcher: Callable[[str, list[int]], dict[int, dict]] | None = None,
) -> dict[str, dict]:
    pr_cache: dict[int, dict] = {}
    if batch_pr_fetcher is not None:
        all_pr_numbers: list[int] = []
        for item in board_data.get("items", []):
            if item.get("status") != STATUS_FINAL_REVIEW:
                continue
            content = item.get("content") or {}
            number = content.get("number")
            if number is None:
                continue
            if content.get("type") == "PullRequest":
                all_pr_numbers.append(int(number))
            else:
                all_pr_numbers.extend(linked_pr_numbers(item, repo))
        if all_pr_numbers:
            pr_cache = batch_pr_fetcher(repo, all_pr_numbers)

    def _get_pr_state(pr_num: int) -> str:
        if pr_num in pr_cache:
            return str(pr_cache[pr_num].get("state", ""))
        return pr_state_fetcher(repo, pr_num)

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
            if _get_pr_state(pr_number) != "OPEN":
                continue
        elif item_type == "Issue":
            linked_numbers = linked_pr_numbers(item, repo)
            if len(linked_numbers) > 1:
                continue
            if len(linked_numbers) == 1:
                pr_number = linked_numbers[0]
                if _get_pr_state(pr_number) != "OPEN":
                    continue
            else:
                if pr_resolver is None:
                    raise ValueError(
                        "final-review mode requires pr_resolver for issue cards without linked PRs"
                    )
                pr_number = pr_resolver(repo, int(number))
                if pr_number is None:
                    continue
                if _get_pr_state(pr_number) != "OPEN":
                    continue
        else:
            pr_number = None

        if pr_number is None:
            continue

        issue_number = int(number) if item_type == "Issue" else None
        entries[item_identity(item)] = build_entry(
            item,
            number=pr_number,
            issue_number=issue_number,
            pr_number=pr_number,
        )
    return entries


def current_entries(
    mode: str,
    board_data: dict,
    repo: str | None = None,
    review_fetcher: Callable[[str, int], list[dict]] | None = None,
    pr_resolver: Callable[[str, int], int | None] | None = None,
    pr_state_fetcher: Callable[[str, int], str] | None = None,
    *,
    batch_pr_fetcher: Callable[[str, list[int]], dict[int, dict]] | None = None,
    repo_root: str | Path | None = None,
) -> dict[str, dict]:
    if mode == "ready":
        return ready_entries(board_data, repo_root=repo_root)
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
            batch_pr_fetcher=batch_pr_fetcher,
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
            batch_pr_fetcher=batch_pr_fetcher,
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
    target_number: int | None = None,
    *,
    batch_pr_fetcher: Callable[[str, list[int]], dict[int, dict]] | None = None,
    repo_root: str | Path | None = None,
) -> tuple[str, int] | None:
    next_entry = select_next_entry(
        mode,
        board_data,
        state_file,
        repo,
        review_fetcher,
        pr_resolver,
        pr_state_fetcher,
        target_number,
        batch_pr_fetcher=batch_pr_fetcher,
        repo_root=repo_root,
    )
    if next_entry is None:
        return None
    return str(next_entry["item_id"]), int(next_entry["number"])


def select_entry_from_entries(
    current_visible: dict[str, dict],
    state_file: Path,
    target_number: int | None = None,
) -> dict | None:
    state = load_state(state_file)
    previous_visible = state["visible"]

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

    if target_number is not None:
        matching_item_id = next(
            (
                item_id
                for item_id, entry in current_visible.items()
                if int(entry["number"]) == target_number
            ),
            None,
        )
        if matching_item_id is None:
            return None
        entry = dict(current_visible[matching_item_id])
        entry["item_id"] = matching_item_id
        return entry

    if not pending:
        return None

    item_id = pending[0]
    entry = dict(current_visible[item_id])
    entry["item_id"] = item_id
    return entry


def select_next_entry(
    mode: str,
    board_data: dict,
    state_file: Path,
    repo: str | None = None,
    review_fetcher: Callable[[str, int], list[dict]] | None = None,
    pr_resolver: Callable[[str, int], int | None] | None = None,
    pr_state_fetcher: Callable[[str, int], str] | None = None,
    target_number: int | None = None,
    *,
    batch_pr_fetcher: Callable[[str, list[int]], dict[int, dict]] | None = None,
    repo_root: str | Path | None = None,
) -> dict | None:
    current_visible = current_entries(
        mode,
        board_data,
        repo,
        review_fetcher,
        pr_resolver,
        pr_state_fetcher,
        batch_pr_fetcher=batch_pr_fetcher,
        repo_root=repo_root,
    )
    return select_entry_from_entries(
        current_visible,
        state_file,
        target_number=target_number,
    )


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


def claimed_status_for_mode(mode: str) -> str:
    if mode == "ready":
        return STATUS_IN_PROGRESS
    if mode == "review":
        return STATUS_UNDER_REVIEW
    raise ValueError(f"Unsupported claim-next mode: {mode}")


def claim_entry_from_entries(
    mode: str,
    current_visible: dict[str, dict],
    state_file: Path,
    target_number: int | None = None,
    mover: Callable[[str, str], None] | None = None,
) -> dict | None:
    next_entry = select_entry_from_entries(
        current_visible,
        state_file,
        target_number=target_number,
    )
    if next_entry is None:
        return None

    claimed_status = claimed_status_for_mode(mode)
    move = mover or move_item
    move(str(next_entry["item_id"]), claimed_status)
    return {
        **next_entry,
        "claimed": True,
        "claimed_status": claimed_status,
    }


def claim_next_entry(
    mode: str,
    board_data: dict,
    state_file: Path,
    repo: str | None = None,
    review_fetcher: Callable[[str, int], list[dict]] | None = None,
    pr_resolver: Callable[[str, int], int | None] | None = None,
    pr_state_fetcher: Callable[[str, int], str] | None = None,
    target_number: int | None = None,
    mover: Callable[[str, str], None] | None = None,
    *,
    batch_pr_fetcher: Callable[[str, list[int]], dict[int, dict]] | None = None,
    repo_root: str | Path | None = None,
) -> dict | None:
    current_visible = current_entries(
        mode,
        board_data,
        repo=repo,
        review_fetcher=review_fetcher,
        pr_resolver=pr_resolver,
        pr_state_fetcher=pr_state_fetcher,
        batch_pr_fetcher=batch_pr_fetcher,
        repo_root=repo_root,
    )
    return claim_entry_from_entries(
        mode,
        current_visible,
        state_file,
        target_number=target_number,
        mover=mover,
    )


def eligible_review_candidate_entries(candidates: list[dict]) -> dict[str, dict]:
    entries: dict[str, dict] = {}
    for candidate in candidates:
        if candidate.get("eligibility") != "eligible":
            continue

        item_id = candidate.get("item_id")
        number = candidate.get("pr_number") or candidate.get("number")
        if item_id is None or number is None:
            continue

        issue_number = candidate.get("issue_number")
        pr_number = candidate.get("pr_number")
        entries[str(item_id)] = {
            "number": int(number),
            "issue_number": int(issue_number) if issue_number is not None else None,
            "pr_number": int(pr_number) if pr_number is not None else int(number),
            "status": candidate.get("status"),
            "title": candidate.get("title"),
        }
    return entries


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
    next_item: dict | None,
    *,
    mode: str,
    fmt: str = "text",
) -> int:
    if next_item is None:
        return 1

    if fmt == "json":
        payload = {"mode": mode, **next_item}
        print(json.dumps(payload))
    else:
        print(f"{next_item['item_id']}\t{next_item['number']}")
    return 0


def print_claim_result(
    claim_result: dict | None,
    *,
    mode: str,
    fmt: str = "json",
) -> int:
    if claim_result is None:
        return 1

    if fmt == "json":
        payload = {"mode": mode, **claim_result}
        print(json.dumps(payload))
    else:
        print(
            f"{claim_result['item_id']}\t"
            f"{claim_result['number']}\t"
            f"{claim_result['claimed_status']}"
        )
    return 0


def print_candidate_list(
    mode: str,
    items: list[dict],
    *,
    fmt: str = "text",
) -> int:
    if fmt == "json":
        print(json.dumps({"mode": mode, "items": items}))
        return 0

    for item in items:
        number = item.get("pr_number") or item.get("issue_number") or item["number"]
        title = item.get("title") or ""
        eligibility = item.get("eligibility") or ""
        print(f"{item['item_id']}\t{number}\t{eligibility}\t{title}")
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
    next_parser.add_argument("--number", type=int)
    next_parser.add_argument("--format", choices=["text", "json"], default="text")
    next_parser.add_argument("--board-cache", type=Path, default=None)
    next_parser.add_argument("--board-cache-max-age", type=float, default=120)
    next_parser.add_argument("--repo-root", type=Path, default=None,
                             help="Repo root for filtering blocked [Rule] issues in ready mode")

    claim_parser = subparsers.add_parser("claim-next")
    claim_parser.add_argument("mode", choices=["ready", "review"])
    claim_parser.add_argument("state_file", type=Path)
    claim_parser.add_argument("--repo")
    claim_parser.add_argument("--owner", default="CodingThrust")
    claim_parser.add_argument("--project-number", type=int, default=8)
    claim_parser.add_argument("--limit", type=int, default=500)
    claim_parser.add_argument("--number", type=int)
    claim_parser.add_argument("--format", choices=["text", "json"], default="json")
    claim_parser.add_argument("--project-id", default=PROJECT_ID)
    claim_parser.add_argument("--field-id", default=STATUS_FIELD_ID)
    claim_parser.add_argument("--board-cache", type=Path, default=None)
    claim_parser.add_argument("--board-cache-max-age", type=float, default=120)
    claim_parser.add_argument("--repo-root", type=Path, default=None,
                              help="Repo root for filtering blocked [Rule] issues in ready mode")

    ack_parser = subparsers.add_parser("ack")
    ack_parser.add_argument("state_file", type=Path)
    ack_parser.add_argument("item_id")

    list_parser = subparsers.add_parser("list")
    list_parser.add_argument(
        "mode",
        choices=["ready", "in-progress", "review-pool", "review", "final-review"],
    )
    list_parser.add_argument("--repo")
    list_parser.add_argument("--owner", default="CodingThrust")
    list_parser.add_argument("--project-number", type=int, default=8)
    list_parser.add_argument("--limit", type=int, default=500)
    list_parser.add_argument("--format", choices=["text", "json"], default="text")
    list_parser.add_argument("--board-cache", type=Path, default=None)
    list_parser.add_argument("--board-cache-max-age", type=float, default=120)

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

    if args.command == "claim-next":
        if args.mode == "review" and not args.repo:
            raise SystemExit("--repo is required in claim-next review mode")
        lite = args.mode not in {"review", "final-review"}
        board_data = fetch_board_items(args.owner, args.project_number, args.limit, cache_file=args.board_cache, cache_max_age=args.board_cache_max_age, lite=lite)
        if args.mode == "review":
            review_entries_map = eligible_review_candidate_entries(
                review_candidates(
                    board_data,
                    args.repo,
                    fetch_pr_reviews,
                    resolve_issue_pr,
                    fetch_pr_info,
                    batch_pr_fetcher=batch_fetch_prs_with_reviews,
                )
            )
            claim_result = claim_entry_from_entries(
                args.mode,
                review_entries_map,
                args.state_file,
                target_number=args.number,
                mover=lambda item_id, status: move_item(
                    item_id,
                    status,
                    project_id=args.project_id,
                    field_id=args.field_id,
                ),
            )
        else:
            claim_result = claim_next_entry(
                args.mode,
                board_data,
                args.state_file,
                repo=args.repo,
                review_fetcher=fetch_pr_reviews,
                pr_resolver=resolve_issue_pr,
                pr_state_fetcher=fetch_pr_state,
                target_number=args.number,
                mover=lambda item_id, status: move_item(
                    item_id,
                    status,
                    project_id=args.project_id,
                    field_id=args.field_id,
                ),
                repo_root=getattr(args, "repo_root", None),
            )
        return print_claim_result(claim_result, mode=args.mode, fmt=args.format)

    if args.command == "list":
        if args.mode == "review" and not args.repo:
            raise SystemExit("--repo is required in list review mode")
        lite = args.mode not in {"review", "final-review"}
        board_data = fetch_board_items(args.owner, args.project_number, args.limit, cache_file=args.board_cache, cache_max_age=args.board_cache_max_age, lite=lite)
        if args.mode == "ready":
            items = status_items(board_data, STATUS_READY)
            return print_candidate_list(args.mode, items, fmt=args.format)
        if args.mode == "in-progress":
            items = status_items(board_data, STATUS_IN_PROGRESS)
            return print_candidate_list(args.mode, items, fmt=args.format)
        if args.mode == "review-pool":
            items = status_items(
                board_data,
                STATUS_REVIEW_POOL,
                content_types={"Issue", "PullRequest"},
            )
            return print_candidate_list(args.mode, items, fmt=args.format)
        if args.mode == "review":
            items = review_candidates(
                board_data,
                args.repo,
                fetch_pr_reviews,
                resolve_issue_pr,
                fetch_pr_info,
                batch_pr_fetcher=batch_fetch_prs_with_reviews,
            )
            return print_candidate_list(args.mode, items, fmt=args.format)
        if args.mode == "final-review":
            items = status_items(
                board_data,
                STATUS_FINAL_REVIEW,
                content_types={"Issue", "PullRequest"},
            )
            return print_candidate_list(args.mode, items, fmt=args.format)
        raise SystemExit(f"Unsupported list mode: {args.mode}")

    if args.mode in {"review", "final-review"} and not args.repo:
        raise SystemExit(f"--repo is required in {args.mode} mode")

    lite = args.mode not in {"review", "final-review"}
    board_data = fetch_board_items(args.owner, args.project_number, args.limit, cache_file=args.board_cache, cache_max_age=args.board_cache_max_age, lite=lite)
    if args.mode == "review":
        review_entries_map = eligible_review_candidate_entries(
            review_candidates(
                board_data,
                args.repo,
                fetch_pr_reviews,
                resolve_issue_pr,
                fetch_pr_info,
                batch_pr_fetcher=batch_fetch_prs_with_reviews,
            )
        )
        next_item = select_entry_from_entries(
            review_entries_map,
            args.state_file,
            target_number=args.number,
        )
    else:
        next_item = select_next_entry(
            args.mode,
            board_data,
            args.state_file,
            repo=args.repo,
            review_fetcher=fetch_pr_reviews,
            pr_resolver=resolve_issue_pr,
            pr_state_fetcher=fetch_pr_state,
            target_number=args.number,
            batch_pr_fetcher=batch_fetch_prs_with_reviews,
            repo_root=getattr(args, "repo_root", None),
        )
    return print_next_item(next_item, mode=args.mode, fmt=args.format)


if __name__ == "__main__":
    raise SystemExit(main())

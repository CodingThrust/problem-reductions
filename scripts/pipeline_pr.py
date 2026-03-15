#!/usr/bin/env python3
"""Shared PR metadata, comments, CI, and codecov helpers."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import time
from typing import Callable
from urllib.parse import unquote

COPILOT_REVIEWERS = {
    "copilot-pull-request-reviewer",
    "copilot-pull-request-reviewer[bot]",
}
CODECOV_REVIEWER = "codecov[bot]"

_CLOSING_ISSUE_RE = re.compile(
    r"(?i)\b(?:fix(?:e[sd])?|close[sd]?|resolve[sd]?)\s+"
    r"(?:(?:[-.\w]+/[-.\w]+)#)?(\d+)\b"
)
_GENERIC_ISSUE_RE = re.compile(r"(?<![A-Za-z0-9_])#(\d+)\b")
_PATCH_COVERAGE_RE = re.compile(
    r"(?i)patch coverage(?:\s+is|:)?\s*`?(\d+(?:\.\d+)?)%`?"
)
_PROJECT_COVERAGE_RE = re.compile(
    r"(?i)project coverage(?:\s+is|:)?\s*`?(\d+(?:\.\d+)?)%`?"
)
_FILEPATH_RE = re.compile(r"filepath=([^&\"\s)]+)")


def run_gh(*args: str) -> str:
    return subprocess.check_output(["gh", *args], text=True)


def run_gh_json(*args: str):
    return json.loads(run_gh(*args))


def run_gh_checked(*args: str) -> None:
    subprocess.check_call(["gh", *args])


def login_for(entry: dict) -> str:
    return (entry.get("user") or entry.get("author") or {}).get("login", "")


def is_bot_login(login: str) -> bool:
    return login.endswith("[bot]") or login in COPILOT_REVIEWERS


def extract_linked_issue_number(title: str | None, body: str | None) -> int | None:
    for text in [body or "", title or ""]:
        match = _CLOSING_ISSUE_RE.search(text)
        if match:
            return int(match.group(1))

    for text in [body or "", title or ""]:
        match = _GENERIC_ISSUE_RE.search(text)
        if match:
            return int(match.group(1))

    return None


def normalize_issue_thread_comment(comment: dict) -> dict:
    login = login_for(comment)
    created_at = comment.get("createdAt") or comment.get("created_at")
    return {
        "author": login,
        "body": comment.get("body", ""),
        "created_at": created_at,
        "is_bot": is_bot_login(login),
    }


def format_issue_context(issue: dict | None, comments: list[dict] | None = None) -> str:
    if not issue:
        return "No linked issue found."

    title = issue.get("title") or f"Issue #{issue.get('number')}"
    body = issue.get("body") or ""
    lines = [f"# {title}", ""]
    if body:
        lines.extend([body, ""])

    human_comments = [
        comment for comment in (comments or []) if not comment.get("is_bot")
    ]
    if human_comments:
        lines.extend(["## Comments", ""])
        for comment in human_comments:
            author = comment.get("author") or "unknown"
            created_at = comment.get("created_at") or "unknown-time"
            lines.append(f"**{author}** ({created_at}):")
            lines.append(comment.get("body", ""))
            lines.append("")

    return "\n".join(lines).strip()


def summarize_comments(
    inline_comments: list[dict],
    reviews: list[dict],
    issue_comments: list[dict],
    linked_issue_comments: list[dict] | None = None,
) -> dict:
    linked_issue_comments = linked_issue_comments or []

    normalized_inline = []
    for comment in inline_comments:
        login = login_for(comment)
        normalized_inline.append(
            {
                "user": login,
                "body": comment.get("body", ""),
                "path": comment.get("path"),
                "line": comment.get("line") or comment.get("original_line"),
                "is_bot": is_bot_login(login),
                "is_copilot": login in COPILOT_REVIEWERS,
            }
        )

    normalized_reviews = []
    for review in reviews:
        login = login_for(review)
        normalized_reviews.append(
            {
                "user": login,
                "body": review.get("body", ""),
                "state": review.get("state"),
                "is_bot": is_bot_login(login),
                "is_copilot": login in COPILOT_REVIEWERS,
            }
        )

    normalized_issue_comments = []
    for comment in issue_comments:
        login = login_for(comment)
        normalized_issue_comments.append(
            {
                "user": login,
                "body": comment.get("body", ""),
                "is_bot": is_bot_login(login),
                "is_codecov": login == CODECOV_REVIEWER,
            }
        )

    normalized_linked_issue_comments = []
    for comment in linked_issue_comments:
        login = login_for(comment)
        normalized_linked_issue_comments.append(
            {
                "user": login,
                "body": comment.get("body", ""),
                "is_bot": is_bot_login(login),
            }
        )

    human_reviews = [
        review
        for review in normalized_reviews
        if not review["is_bot"] and review["body"].strip()
    ]
    codecov_comments = [
        comment for comment in normalized_issue_comments if comment["is_codecov"]
    ]

    return {
        "inline_comments": normalized_inline,
        "reviews": normalized_reviews,
        "issue_comments": normalized_issue_comments,
        "linked_issue_comments": normalized_linked_issue_comments,
        "human_inline_comments": [
            comment for comment in normalized_inline if not comment["is_bot"]
        ],
        "copilot_inline_comments": [
            comment for comment in normalized_inline if comment["is_copilot"]
        ],
        "human_reviews": human_reviews,
        "human_issue_comments": [
            comment
            for comment in normalized_issue_comments
            if not comment["is_bot"] and not comment["is_codecov"]
        ],
        "human_linked_issue_comments": [
            comment
            for comment in normalized_linked_issue_comments
            if not comment["is_bot"]
        ],
        "codecov_comments": codecov_comments,
        "counts": {
            "inline_comments": len(normalized_inline),
            "copilot_inline_comments": sum(
                1 for comment in normalized_inline if comment["is_copilot"]
            ),
            "human_inline_comments": sum(
                1 for comment in normalized_inline if not comment["is_bot"]
            ),
            "reviews": len(normalized_reviews),
            "human_reviews": len(human_reviews),
            "issue_comments": len(normalized_issue_comments),
            "human_issue_comments": sum(
                1
                for comment in normalized_issue_comments
                if not comment["is_bot"] and not comment["is_codecov"]
            ),
            "linked_issue_comments": len(normalized_linked_issue_comments),
            "human_linked_issue_comments": sum(
                1 for comment in normalized_linked_issue_comments if not comment["is_bot"]
            ),
            "codecov_comments": len(codecov_comments),
        },
    }


def extract_codecov_summary(issue_comments: list[dict]) -> dict:
    codecov_comments = [
        comment for comment in issue_comments if login_for(comment) == CODECOV_REVIEWER
    ]
    if not codecov_comments:
        return {
            "found": False,
            "body": None,
            "patch_coverage": None,
            "project_coverage": None,
            "filepaths": [],
        }

    body = codecov_comments[-1].get("body", "")
    patch_match = _PATCH_COVERAGE_RE.search(body)
    project_match = _PROJECT_COVERAGE_RE.search(body)

    filepaths: list[str] = []
    seen: set[str] = set()
    for encoded in _FILEPATH_RE.findall(body):
        path = unquote(encoded)
        if path not in seen:
            seen.add(path)
            filepaths.append(path)

    return {
        "found": True,
        "body": body,
        "patch_coverage": float(patch_match.group(1)) if patch_match else None,
        "project_coverage": float(project_match.group(1)) if project_match else None,
        "filepaths": filepaths,
    }


def summarize_check_runs(payload: dict) -> dict:
    runs = payload.get("check_runs") or []
    normalized_runs = []
    pending = 0
    failing = 0
    succeeding = 0

    for run in runs:
        status = (run.get("status") or "").lower()
        conclusion = run.get("conclusion")
        normalized_conclusion = conclusion.lower() if isinstance(conclusion, str) else None
        normalized_runs.append(
            {
                "name": run.get("name"),
                "status": status,
                "conclusion": normalized_conclusion,
                "details_url": run.get("details_url"),
            }
        )

        if status != "completed" or normalized_conclusion is None:
            pending += 1
        elif normalized_conclusion in {"success", "skipped", "neutral"}:
            succeeding += 1
        else:
            failing += 1

    if failing:
        state = "failure"
    elif pending or not normalized_runs:
        state = "pending"
    else:
        state = "success"

    return {
        "state": state,
        "total": len(normalized_runs),
        "pending": pending,
        "failing": failing,
        "succeeding": succeeding,
        "runs": normalized_runs,
    }


def build_snapshot(
    pr_data: dict,
    *,
    linked_issue_number: int | None = None,
    linked_issue: dict | None = None,
    ci_summary: dict | None = None,
    codecov_summary: dict | None = None,
) -> dict:
    if linked_issue_number is None:
        linked_issue_number = extract_linked_issue_number(
            pr_data.get("title"),
            pr_data.get("body"),
        )

    labels = [label.get("name") for label in pr_data.get("labels", []) if label.get("name")]
    files = [
        file_info.get("path") or file_info.get("filename")
        for file_info in pr_data.get("files", [])
        if file_info.get("path") or file_info.get("filename")
    ]
    commits = [
        commit.get("oid") or commit.get("commit", {}).get("oid")
        for commit in pr_data.get("commits", [])
    ]

    return {
        "number": pr_data.get("number"),
        "title": pr_data.get("title"),
        "body": pr_data.get("body"),
        "state": pr_data.get("state"),
        "url": pr_data.get("url"),
        "mergeable": pr_data.get("mergeable"),
        "head_ref_name": pr_data.get("headRefName"),
        "base_ref_name": pr_data.get("baseRefName"),
        "head_sha": pr_data.get("headRefOid"),
        "linked_issue_number": linked_issue_number,
        "linked_issue": linked_issue,
        "labels": labels,
        "files": files,
        "commits": commits,
        "additions": pr_data.get("additions", 0),
        "deletions": pr_data.get("deletions", 0),
        "ci": ci_summary,
        "codecov": codecov_summary,
        "counts": {
            "labels": len(labels),
            "files": len(files),
            "commits": len(commits),
        },
    }


def build_current_pr_context(repo: str, pr_data: dict) -> dict:
    return {
        "repo": repo,
        "pr_number": pr_data.get("number"),
        "title": pr_data.get("title"),
        "head_ref_name": pr_data.get("headRefName"),
        "url": pr_data.get("url"),
    }


def build_linked_issue_result(
    *,
    pr_number: int,
    linked_issue_number: int | None,
    linked_issue: dict | None,
    linked_issue_comments: list[dict] | None = None,
) -> dict:
    normalized_comments = [
        normalize_issue_thread_comment(comment)
        for comment in (linked_issue_comments or [])
    ]
    human_comments = [
        comment for comment in normalized_comments if not comment["is_bot"]
    ]
    return {
        "pr_number": pr_number,
        "linked_issue_number": linked_issue_number,
        "linked_issue": linked_issue,
        "linked_issue_comments": normalized_comments,
        "human_linked_issue_comments": human_comments,
        "issue_context_text": format_issue_context(linked_issue, normalized_comments),
    }


def wait_for_ci(
    fetcher: Callable[[], dict],
    *,
    timeout_seconds: float,
    interval_seconds: float,
    monotonic_fn: Callable[[], float] = time.monotonic,
    sleep_fn: Callable[[float], None] = time.sleep,
) -> dict:
    start = monotonic_fn()
    attempts = 0

    while True:
        attempts += 1
        summary = dict(fetcher())
        summary.setdefault("state", "pending")
        summary["attempts"] = attempts
        summary["elapsed_seconds"] = round(monotonic_fn() - start, 3)

        if summary["state"] != "pending":
            summary["timed_out"] = False
            return summary

        if monotonic_fn() + interval_seconds > start + timeout_seconds:
            summary["state"] = "timeout"
            summary["timed_out"] = True
            return summary

        sleep_fn(interval_seconds)


def fetch_pr_data(repo: str, pr_number: int) -> dict:
    return run_gh_json(
        "pr",
        "view",
        str(pr_number),
        "--repo",
        repo,
        "--json",
        (
            "number,title,body,labels,files,additions,deletions,commits,"
            "headRefName,baseRefName,headRefOid,url,state,mergeable"
        ),
    )


def fetch_current_repo() -> str:
    data = run_gh_json("repo", "view", "--json", "nameWithOwner")
    repo = data.get("nameWithOwner")
    if not repo:
        raise ValueError(f"Unexpected repo payload: {data!r}")
    return repo


def fetch_current_pr_data() -> dict:
    return run_gh_json("pr", "view", "--json", "number,title,headRefName,url")


def fetch_current_pr_data_for_repo(repo: str) -> dict:
    return run_gh_json(
        "pr",
        "view",
        "--repo",
        repo,
        "--json",
        "number,title,headRefName,url",
    )


def fetch_issue_data(repo: str, issue_number: int) -> dict:
    return run_gh_json(
        "issue",
        "view",
        str(issue_number),
        "--repo",
        repo,
        "--json",
        "number,title,body,labels,state,url",
    )


def fetch_issue_comments(repo: str, issue_number: int) -> list[dict]:
    data = run_gh_json("api", f"repos/{repo}/issues/{issue_number}/comments")
    if not isinstance(data, list):
        raise ValueError(f"Unexpected issue comments payload for #{issue_number}: {data!r}")
    return data


def fetch_inline_comments(repo: str, pr_number: int) -> list[dict]:
    data = run_gh_json("api", f"repos/{repo}/pulls/{pr_number}/comments")
    if not isinstance(data, list):
        raise ValueError(f"Unexpected inline comments payload for PR #{pr_number}: {data!r}")
    return data


def fetch_reviews(repo: str, pr_number: int) -> list[dict]:
    data = run_gh_json("api", f"repos/{repo}/pulls/{pr_number}/reviews")
    if not isinstance(data, list):
        raise ValueError(f"Unexpected reviews payload for PR #{pr_number}: {data!r}")
    return data


def fetch_check_runs(repo: str, head_sha: str) -> dict:
    return run_gh_json("api", f"repos/{repo}/commits/{head_sha}/check-runs")


def fetch_linked_issue_bundle(repo: str, pr_data: dict) -> tuple[int | None, dict | None]:
    issue_number = extract_linked_issue_number(pr_data.get("title"), pr_data.get("body"))
    if issue_number is None:
        return None, None
    return issue_number, fetch_issue_data(repo, issue_number)


def fetch_ci_summary(repo: str, pr_number: int, pr_data: dict | None = None) -> dict:
    pr_data = pr_data or fetch_pr_data(repo, pr_number)
    head_sha = pr_data.get("headRefOid")
    if not head_sha:
        raise ValueError(f"PR #{pr_number} is missing headRefOid")

    summary = summarize_check_runs(fetch_check_runs(repo, head_sha))
    summary["pr_number"] = pr_number
    summary["head_sha"] = head_sha
    return summary


def build_comments_summary(repo: str, pr_number: int, pr_data: dict | None = None) -> dict:
    pr_data = pr_data or fetch_pr_data(repo, pr_number)
    linked_issue_number = extract_linked_issue_number(
        pr_data.get("title"),
        pr_data.get("body"),
    )

    summary = summarize_comments(
        inline_comments=fetch_inline_comments(repo, pr_number),
        reviews=fetch_reviews(repo, pr_number),
        issue_comments=fetch_issue_comments(repo, pr_number),
        linked_issue_comments=(
            fetch_issue_comments(repo, linked_issue_number)
            if linked_issue_number is not None
            else []
        ),
    )
    summary["pr_number"] = pr_number
    summary["linked_issue_number"] = linked_issue_number
    return summary


def build_codecov_summary(repo: str, pr_number: int) -> dict:
    summary = extract_codecov_summary(fetch_issue_comments(repo, pr_number))
    summary["pr_number"] = pr_number
    return summary


def build_pr_snapshot(repo: str, pr_number: int) -> dict:
    pr_data = fetch_pr_data(repo, pr_number)
    linked_issue_number, linked_issue = fetch_linked_issue_bundle(repo, pr_data)
    return build_snapshot(
        pr_data,
        linked_issue_number=linked_issue_number,
        linked_issue=linked_issue,
        ci_summary=fetch_ci_summary(repo, pr_number, pr_data),
        codecov_summary=build_codecov_summary(repo, pr_number),
    )


def create_pr(
    repo: str,
    title: str,
    body_file: str,
    *,
    base: str | None = None,
    head: str | None = None,
) -> dict:
    args = [
        "pr",
        "create",
        "--repo",
        repo,
        "--title",
        title,
        "--body-file",
        body_file,
    ]
    if base:
        args.extend(["--base", base])
    if head:
        args.extend(["--head", head])
    run_gh_checked(*args)
    return build_current_pr_context(repo, fetch_current_pr_data_for_repo(repo))


def post_pr_comment(repo: str, pr_number: int, body_file: str) -> None:
    run_gh_checked(
        "pr",
        "comment",
        str(pr_number),
        "--repo",
        repo,
        "--body-file",
        body_file,
    )


def edit_pr_body(repo: str, pr_number: int, body_file: str) -> None:
    run_gh_checked(
        "pr",
        "edit",
        str(pr_number),
        "--repo",
        repo,
        "--body-file",
        body_file,
    )


def emit_result(result: dict, fmt: str) -> None:
    if fmt == "json":
        print(json.dumps(result, indent=2, sort_keys=True))
        return

    if "number" in result:
        print(f"PR #{result['number']}: {result.get('title', '')}")
        return

    print(json.dumps(result, indent=2, sort_keys=True))


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="PR automation helpers.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    for name in [
        "current",
        "snapshot",
        "comments",
        "ci",
        "wait-ci",
        "codecov",
        "linked-issue",
        "create",
        "comment",
        "edit-body",
    ]:
        command = subparsers.add_parser(name)
        if name == "current":
            command.add_argument("--format", choices=["json", "text"], default="json")
        else:
            command.add_argument("--repo", required=True)
            if name != "create":
                command.add_argument("--pr", required=True, type=int)
        if name == "wait-ci":
            command.add_argument("--timeout", type=float, default=900)
            command.add_argument("--interval", type=float, default=30)
        elif name == "create":
            command.add_argument("--title", required=True)
            command.add_argument("--body-file", required=True)
            command.add_argument("--base")
            command.add_argument("--head")
            command.add_argument("--format", choices=["json", "text"], default="json")
        elif name in {"comment", "edit-body"}:
            command.add_argument("--body-file", required=True)
        elif name != "current":
            command.add_argument("--format", choices=["json", "text"], default="json")

    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])

    if args.command == "current":
        emit_result(
            build_current_pr_context(fetch_current_repo(), fetch_current_pr_data()),
            args.format,
        )
        return 0

    if args.command == "snapshot":
        emit_result(build_pr_snapshot(args.repo, args.pr), args.format)
        return 0

    if args.command == "comments":
        emit_result(build_comments_summary(args.repo, args.pr), args.format)
        return 0

    if args.command == "ci":
        emit_result(fetch_ci_summary(args.repo, args.pr), args.format)
        return 0

    if args.command == "wait-ci":
        result = wait_for_ci(
            lambda: fetch_ci_summary(args.repo, args.pr),
            timeout_seconds=args.timeout,
            interval_seconds=args.interval,
        )
        emit_result(result, args.format)
        return 0

    if args.command == "codecov":
        emit_result(build_codecov_summary(args.repo, args.pr), args.format)
        return 0

    if args.command == "linked-issue":
        pr_data = fetch_pr_data(args.repo, args.pr)
        issue_number, issue = fetch_linked_issue_bundle(args.repo, pr_data)
        issue_comments = (
            fetch_issue_comments(args.repo, issue_number)
            if issue_number is not None
            else []
        )
        emit_result(
            build_linked_issue_result(
                pr_number=args.pr,
                linked_issue_number=issue_number,
                linked_issue=issue,
                linked_issue_comments=issue_comments,
            ),
            args.format,
        )
        return 0

    if args.command == "create":
        emit_result(
            create_pr(
                args.repo,
                args.title,
                args.body_file,
                base=args.base,
                head=args.head,
            ),
            args.format,
        )
        return 0

    if args.command == "comment":
        post_pr_comment(args.repo, args.pr, args.body_file)
        return 0

    if args.command == "edit-body":
        edit_pr_body(args.repo, args.pr, args.body_file)
        return 0

    raise AssertionError(f"Unhandled command: {args.command}")


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Shared worktree helpers for issue and PR pipeline flows."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path


def sanitize_component(text: str) -> str:
    normalized = re.sub(r"[^A-Za-z0-9]+", "-", text.strip().lower()).strip("-")
    return normalized or "work"


def plan_issue_worktree(
    repo_root: str | Path,
    *,
    issue_number: int,
    slug: str,
    base_ref: str = "origin/main",
) -> dict:
    repo_root = str(Path(repo_root))
    branch = f"issue-{issue_number}-{sanitize_component(slug)}"
    worktree_dir = str(Path(repo_root) / ".worktrees" / branch)
    return {
        "issue_number": issue_number,
        "slug": slug,
        "branch": branch,
        "worktree_dir": worktree_dir,
        "base_ref": base_ref,
    }


def plan_pr_worktree(
    repo_root: str | Path,
    *,
    pr_number: int,
    head_ref_name: str,
    base_sha: str,
    head_sha: str,
) -> dict:
    repo_root = str(Path(repo_root))
    local_branch = f"review-pr-{pr_number}-{sanitize_component(head_ref_name)}"
    worktree_dir = str(Path(repo_root) / ".worktrees" / local_branch)
    return {
        "pr_number": pr_number,
        "head_ref_name": head_ref_name,
        "local_branch": local_branch,
        "worktree_dir": worktree_dir,
        "fetch_ref": f"pull/{pr_number}/head:{local_branch}",
        "base_sha": base_sha,
        "head_sha": head_sha,
    }


def summarize_merge(
    *,
    worktree: str | Path,
    exit_code: int,
    conflicts: list[str],
) -> dict:
    conflicts = sorted(conflicts)
    if exit_code == 0:
        status = "clean"
    elif conflicts:
        status = "conflicted"
    else:
        status = "aborted"

    likely_complex = len(conflicts) > 1 or any(
        path.startswith(".claude/skills/add-model/")
        or path.startswith(".claude/skills/add-rule/")
        for path in conflicts
    )

    return {
        "worktree": str(worktree),
        "status": status,
        "conflicts": conflicts,
        "likely_complex": likely_complex,
    }


def run_git(repo_root: str | Path, *args: str) -> str:
    return subprocess.check_output(["git", "-C", str(repo_root), *args], text=True)


def run_git_checked(repo_root: str | Path, *args: str) -> None:
    subprocess.check_output(["git", "-C", str(repo_root), *args], stderr=subprocess.STDOUT)


def run_gh_json(*args: str):
    return json.loads(subprocess.check_output(["gh", *args], text=True))


def repo_root_from(path: str | Path) -> Path:
    return Path(run_git(path, "rev-parse", "--show-toplevel").strip())


def branch_exists(repo_root: str | Path, branch: str) -> bool:
    proc = subprocess.run(
        ["git", "-C", str(repo_root), "rev-parse", "--verify", branch],
        capture_output=True,
        text=True,
    )
    return proc.returncode == 0


def prepare_issue_branch(
    *,
    issue_number: int,
    slug: str,
    base_ref: str = "main",
    repo_root: str | Path | None = None,
) -> dict:
    repo_root = Path(repo_root or repo_root_from(Path.cwd())).resolve()
    plan = plan_issue_worktree(
        repo_root,
        issue_number=issue_number,
        slug=slug,
        base_ref=base_ref,
    )

    status_output = run_git(repo_root, "status", "--porcelain").strip()
    if status_output:
        raise RuntimeError("working tree is dirty; stash or commit changes before branching")

    run_git_checked(repo_root, "checkout", base_ref)
    existing_branch = branch_exists(repo_root, plan["branch"])
    if existing_branch:
        run_git_checked(repo_root, "checkout", plan["branch"])
        action = "checkout-existing"
    else:
        run_git_checked(repo_root, "checkout", "-b", plan["branch"])
        action = "create-branch"

    base_sha = run_git(repo_root, "rev-parse", base_ref).strip()
    head_sha = run_git(repo_root, "rev-parse", "HEAD").strip()
    return {
        **plan,
        "existing_branch": existing_branch,
        "action": action,
        "base_sha": base_sha,
        "head_sha": head_sha,
    }


def create_issue_worktree(
    *,
    issue_number: int,
    slug: str,
    base_ref: str = "origin/main",
    repo_root: str | Path | None = None,
) -> dict:
    repo_root = Path(repo_root or repo_root_from(Path.cwd())).resolve()
    plan = plan_issue_worktree(
        repo_root,
        issue_number=issue_number,
        slug=slug,
        base_ref=base_ref,
    )

    Path(plan["worktree_dir"]).parent.mkdir(parents=True, exist_ok=True)
    remote, _, branch_name = base_ref.partition("/")
    if remote and branch_name:
        run_git_checked(repo_root, "fetch", remote, branch_name)
    run_git_checked(
        repo_root,
        "worktree",
        "add",
        plan["worktree_dir"],
        "-b",
        plan["branch"],
        base_ref,
    )

    base_sha = run_git(repo_root, "rev-parse", base_ref).strip()
    head_sha = run_git(plan["worktree_dir"], "rev-parse", "HEAD").strip()
    return {
        **plan,
        "base_sha": base_sha,
        "head_sha": head_sha,
    }


def checkout_pr_worktree(
    *,
    repo: str,
    pr_number: int,
    repo_root: str | Path | None = None,
) -> dict:
    repo_root = Path(repo_root or repo_root_from(Path.cwd())).resolve()
    pr_data = run_gh_json(
        "pr",
        "view",
        str(pr_number),
        "--repo",
        repo,
        "--json",
        "headRefName,headRefOid,baseRefName",
    )

    # baseRefOid is not available via gh pr view --json; resolve it locally
    run_git_checked(repo_root, "fetch", "origin", pr_data["baseRefName"])
    base_sha = run_git(repo_root, "rev-parse", f"origin/{pr_data['baseRefName']}").strip()

    plan = plan_pr_worktree(
        repo_root,
        pr_number=pr_number,
        head_ref_name=pr_data["headRefName"],
        base_sha=base_sha,
        head_sha=pr_data["headRefOid"],
    )

    worktree_dir = Path(plan["worktree_dir"])

    # If the worktree already exists from a previous run, remove it first
    if worktree_dir.exists():
        run_git_checked(repo_root, "worktree", "remove", "--force", str(worktree_dir))
    # Also clean up the local branch if it exists (may be left over after worktree removal)
    branch_check = subprocess.run(
        ["git", "-C", str(repo_root), "rev-parse", "--verify", plan["local_branch"]],
        capture_output=True,
    )
    if branch_check.returncode == 0:
        subprocess.run(
            ["git", "-C", str(repo_root), "branch", "-D", plan["local_branch"]],
            capture_output=True,
        )

    worktree_dir.parent.mkdir(parents=True, exist_ok=True)
    run_git_checked(repo_root, "fetch", "origin", plan["fetch_ref"])
    run_git_checked(repo_root, "worktree", "add", plan["worktree_dir"], plan["local_branch"])
    return plan


def merge_main(
    *,
    worktree: str | Path,
) -> dict:
    worktree = Path(worktree).resolve()
    run_git_checked(worktree, "fetch", "origin", "main")
    proc = subprocess.run(
        ["git", "-C", str(worktree), "merge", "origin/main", "--no-edit"],
        text=True,
        capture_output=True,
    )

    conflict_output = run_git(worktree, "diff", "--name-only", "--diff-filter=U").strip()
    conflicts = [line for line in conflict_output.splitlines() if line]
    summary = summarize_merge(worktree=worktree, exit_code=proc.returncode, conflicts=conflicts)
    summary["stdout"] = proc.stdout
    summary["stderr"] = proc.stderr
    return summary


def prepare_review(
    *,
    repo: str,
    pr_number: int,
    repo_root: str | Path | None = None,
) -> dict:
    checkout = checkout_pr_worktree(
        repo=repo,
        pr_number=pr_number,
        repo_root=repo_root,
    )
    merge = merge_main(worktree=checkout["worktree_dir"])
    return {
        "repo": repo,
        "pr_number": pr_number,
        "ready": merge["status"] == "clean",
        "checkout": checkout,
        "merge": merge,
    }


def enter(
    *,
    name: str,
    base_ref: str = "origin/main",
    repo_root: str | Path | None = None,
) -> dict:
    """Create a named worktree from base_ref. Idempotent — removes stale worktree/branch if they exist."""
    repo_root = Path(repo_root or repo_root_from(Path.cwd())).resolve()
    branch = sanitize_component(name)
    worktree_dir = str(repo_root / ".worktrees" / branch)

    # Remove stale worktree if it exists from a previous run
    if Path(worktree_dir).exists():
        run_git_checked(repo_root, "worktree", "remove", "--force", worktree_dir)
    # Clean up stale branch if it exists
    if branch_exists(repo_root, branch):
        result = subprocess.run(
            ["git", "-C", str(repo_root), "branch", "-D", branch],
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            raise RuntimeError(
                f"Failed to delete stale branch '{branch}': {result.stderr.strip()}"
            )

    # Fetch the base ref
    remote, _, branch_name = base_ref.partition("/")
    if remote and branch_name:
        run_git_checked(repo_root, "fetch", remote, branch_name)

    Path(worktree_dir).parent.mkdir(parents=True, exist_ok=True)
    run_git_checked(repo_root, "worktree", "add", worktree_dir, "-b", branch, base_ref)

    return {
        "worktree_dir": worktree_dir,
        "branch": branch,
        "base_ref": base_ref,
    }


def prepare_review_from_cwd(*, repo: str, pr_number: int) -> dict:
    """Build prep data from CWD (assumes PR branch already checked out via EnterWorktree + gh pr checkout)."""
    cwd = str(Path.cwd())
    run_git_checked(cwd, "fetch", "origin", "main")
    base_sha = run_git(cwd, "merge-base", "origin/main", "HEAD").strip()
    head_sha = run_git(cwd, "rev-parse", "HEAD").strip()
    return {
        "repo": repo,
        "pr_number": pr_number,
        "ready": True,
        "checkout": {
            "worktree_dir": cwd,
            "base_sha": base_sha,
            "head_sha": head_sha,
        },
        "merge": {"status": "skipped"},
    }


def cleanup_worktree(*, worktree: str | Path) -> dict:
    worktree = Path(worktree).resolve()
    # Determine repo root before cleanup — if the worktree is already gone,
    # fall back to the parent .worktrees directory's repo.
    if worktree.is_dir():
        try:
            repo_root = repo_root_from(worktree)
        except Exception:
            repo_root = worktree.parent.parent
    else:
        # Worktree dir already deleted; derive repo root from parent.
        repo_root = worktree.parent.parent  # .worktrees/<branch> -> repo root
    repo_root = Path(repo_root).resolve()

    # Validate that repo_root is actually a git repository.
    if not (repo_root / ".git").exists():
        return {
            "worktree": str(worktree),
            "removed": not worktree.exists(),
            "branch_still_exists": False,
            "error": f"derived repo root {repo_root} is not a git repository",
        }

    # Always prune first — this cleans up stale worktree entries for
    # directories that were already deleted outside of git.
    subprocess.run(
        ["git", "-C", str(repo_root), "worktree", "prune"],
        capture_output=True,
    )

    # Remove worktree if it still exists in git's tracking.
    subprocess.run(
        ["git", "-C", str(repo_root), "worktree", "remove", str(worktree), "--force"],
        capture_output=True,
        text=True,
    )

    branch_name = worktree.name
    try:
        branch_list = run_git(
            repo_root, "branch", "--list", "--format=%(refname:short)"
        ).splitlines()
    except subprocess.CalledProcessError:
        branch_list = []
    return {
        "worktree": str(worktree),
        "removed": not worktree.exists(),
        "branch_still_exists": branch_name in branch_list,
    }


def emit_result(result: dict, fmt: str) -> None:
    if fmt == "json":
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(json.dumps(result, indent=2, sort_keys=True))


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Pipeline worktree helpers.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    enter_parser = subparsers.add_parser("enter")
    enter_parser.add_argument("--name", required=True)
    enter_parser.add_argument("--base", default="origin/main")
    enter_parser.add_argument("--repo-root")
    enter_parser.add_argument("--format", choices=["json", "text"], default="json")

    create_issue = subparsers.add_parser("create-issue")
    create_issue.add_argument("--issue", required=True, type=int)
    create_issue.add_argument("--slug", required=True)
    create_issue.add_argument("--base", default="origin/main")
    create_issue.add_argument("--repo-root")
    create_issue.add_argument("--format", choices=["json", "text"], default="json")

    prepare_issue = subparsers.add_parser("prepare-issue-branch")
    prepare_issue.add_argument("--issue", required=True, type=int)
    prepare_issue.add_argument("--slug", required=True)
    prepare_issue.add_argument("--base", default="main")
    prepare_issue.add_argument("--repo-root")
    prepare_issue.add_argument("--format", choices=["json", "text"], default="json")

    checkout_pr = subparsers.add_parser("checkout-pr")
    checkout_pr.add_argument("--repo", required=True)
    checkout_pr.add_argument("--pr", required=True, type=int)
    checkout_pr.add_argument("--repo-root")
    checkout_pr.add_argument("--format", choices=["json", "text"], default="json")

    prepare_review = subparsers.add_parser("prepare-review")
    prepare_review.add_argument("--repo", required=True)
    prepare_review.add_argument("--pr", required=True, type=int)
    prepare_review.add_argument("--repo-root")
    prepare_review.add_argument("--format", choices=["json", "text"], default="json")

    merge_parser = subparsers.add_parser("merge-main")
    merge_parser.add_argument("--worktree", required=True)
    merge_parser.add_argument("--format", choices=["json", "text"], default="json")

    cleanup_parser = subparsers.add_parser("cleanup")
    cleanup_parser.add_argument("--worktree", required=True)
    cleanup_parser.add_argument("--format", choices=["json", "text"], default="json")

    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])

    if args.command == "enter":
        emit_result(
            enter(
                name=args.name,
                base_ref=args.base,
                repo_root=args.repo_root,
            ),
            args.format,
        )
        return 0

    if args.command == "create-issue":
        emit_result(
            create_issue_worktree(
                issue_number=args.issue,
                slug=args.slug,
                base_ref=args.base,
                repo_root=args.repo_root,
            ),
            args.format,
        )
        return 0

    if args.command == "prepare-issue-branch":
        emit_result(
            prepare_issue_branch(
                issue_number=args.issue,
                slug=args.slug,
                base_ref=args.base,
                repo_root=args.repo_root,
            ),
            args.format,
        )
        return 0

    if args.command == "checkout-pr":
        emit_result(
            checkout_pr_worktree(
                repo=args.repo,
                pr_number=args.pr,
                repo_root=args.repo_root,
            ),
            args.format,
        )
        return 0

    if args.command == "prepare-review":
        emit_result(
            prepare_review(
                repo=args.repo,
                pr_number=args.pr,
                repo_root=args.repo_root,
            ),
            args.format,
        )
        return 0

    if args.command == "merge-main":
        emit_result(merge_main(worktree=args.worktree), args.format)
        return 0

    if args.command == "cleanup":
        emit_result(cleanup_worktree(worktree=args.worktree), args.format)
        return 0

    raise AssertionError(f"Unhandled command: {args.command}")


if __name__ == "__main__":
    raise SystemExit(main())

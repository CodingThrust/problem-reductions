#!/usr/bin/env python3
"""Batch fix & implement pipeline for rule issues.

Triages, fixes, implements, and combines batches of issues into a single PR.
No GitHub comments are posted — only issue bodies are edited.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from collections import Counter
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime, timezone
from pathlib import Path

REPO = "CodingThrust/problem-reductions"

# Keywords in #770 diagnoses that indicate fundamentally broken issues
BROKEN_KEYWORDS = [
    "counterexample",
    "reverse direction fails",
    "fails on non-trivial",
    "all fail",
    "empty issue body",
    "empty body",
    "stub",
    "placeholder text",
    "no reduction algorithm",
    "degenerate",
    "fds broken",
    "fds don't allow",
    "fds only produce",
    "bcnf violation",
    "key closure fails",
    "self-contradictory",
    "not reproduced",
    "construction incomplete",
]

# Diagnosis from #770 for each known issue
ISSUE_DIAGNOSES: dict[int, str] = {
    238: "Backward direction flawed: spanning path != Hamiltonian circuit",
    381: "Stub -- no reduction algorithm in issue body",
    166: "Threshold formula inconsistent with described construction; needs rederivation",
    277: "Empty issue body",
    363: "Flow >= R is weaker than exact partition; counterexample A={1,3}",
    459: "FDs don't allow deriving non-selected vertex attrs; key closure fails",
    523: "Reverse direction fails; counterexample C5 with J=3",
    472: "Bound K=K_OLA+C undefined; no formula for C",
    385: "Containment direction wrong -- backwards from VC intersection requirement",
    423: "m=2 case degenerate (zero latency); should be 3-Partition source",
    460: "FDs broken -- universe attrs can't determine each other",
    462: "FDs only produce auxiliary attrs; no BCNF violation possible",
    250: "Vague -- admits intermediate chain; example self-contradictory",
    435: "Multiple constructions attempted, all fail on non-trivial examples",
    436: "Same problem as #435 -- correct Kou construction not reproduced",
    461: "Construction incomplete -- placeholder text, missing FDs",
}


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def create_log_dir() -> Path:
    ts = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S")
    log_dir = Path("logs") / f"batch-{ts}"
    log_dir.mkdir(parents=True, exist_ok=True)
    return log_dir


def write_stage_log(log_dir: Path, stage: str, results: list[dict]) -> Path:
    path = log_dir / f"{stage}.json"
    data = {
        "stage": stage,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "results": results,
    }
    path.write_text(json.dumps(data, indent=2) + "\n")
    return path


def write_summary(log_dir: Path, summary: dict) -> Path:
    path = log_dir / "summary.json"
    path.write_text(json.dumps(summary, indent=2) + "\n")
    return path


def run_cmd(
    cmd: list[str],
    *,
    check: bool = True,
    capture: bool = True,
    cwd: str | None = None,
) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, capture_output=capture, text=True, check=check, cwd=cwd)


def print_stage_summary(name: str, results: list[dict]) -> None:
    counts = Counter(r["status"] for r in results)
    parts = [f"{v} {k}" for k, v in sorted(counts.items())]
    print(f"[{name}] {', '.join(parts)}")


def build_summary(
    input_issues: list[int],
    triage: list[dict],
    fix: list[dict],
    impl: list[dict],
    fin: dict | None,
) -> dict:
    return {
        "input_issues": input_issues,
        "triage": dict(Counter(r["status"] for r in triage)),
        "fix": dict(Counter(r["status"] for r in fix)),
        "implement": dict(Counter(r["status"] for r in impl)),
        "finalize": {
            "pr_url": fin.get("pr_url") if fin else None,
            "pr_number": fin.get("pr_number") if fin else None,
            "issues_closed": fin.get("issues_closed", []) if fin else [],
        },
    }


# ---------------------------------------------------------------------------
# Stage 1: Triage
# ---------------------------------------------------------------------------

def stage_triage(issues: list[int]) -> list[dict]:
    results = []
    for issue_num in issues:
        print(f"  Triaging #{issue_num}...")
        result = triage_one(issue_num)
        results.append(result)
        print(f"    -> {result['status']}: {result.get('reason', '')}")
    return results


def triage_one(issue_num: int) -> dict:
    diagnosis = ISSUE_DIAGNOSES.get(issue_num, "")

    # Check if fundamentally broken
    diagnosis_lower = diagnosis.lower()
    for keyword in BROKEN_KEYWORDS:
        if keyword.lower() in diagnosis_lower:
            return {"issue": issue_num, "status": "broken", "reason": diagnosis}

    # Fetch issue context
    try:
        proc = run_cmd([
            "python3", "scripts/pipeline_checks.py", "issue-context",
            "--repo", REPO, "--issue", str(issue_num), "--format", "json",
        ])
        ctx = json.loads(proc.stdout)
    except (subprocess.CalledProcessError, json.JSONDecodeError) as exc:
        return {"issue": issue_num, "status": "error", "reason": f"Context fetch failed: {exc}"}

    checks = ctx.get("checks", {})

    # Model guards
    if checks.get("source_model") == "fail":
        return {"issue": issue_num, "status": "blocked", "reason": f"Source model missing"}
    if checks.get("target_model") == "fail":
        return {"issue": issue_num, "status": "blocked", "reason": f"Target model missing"}

    # Already good
    if checks.get("good_label") == "pass":
        return {"issue": issue_num, "status": "pass", "reason": "Good label present, models exist"}

    return {"issue": issue_num, "status": "fixable", "reason": diagnosis or "Needs fix-issue"}


# ---------------------------------------------------------------------------
# Stage 2: Fix
# ---------------------------------------------------------------------------

def stage_fix(triage_results: list[dict]) -> list[dict]:
    results = []
    for item in triage_results:
        if item["status"] != "fixable":
            results.append(item)
            continue
        print(f"  Fixing #{item['issue']}...")
        result = fix_one(item["issue"], item.get("reason", ""))
        results.append(result)
        print(f"    -> {result['status']}: {result.get('reason', '')}")
    return results


def fix_one(issue_num: int, diagnosis: str) -> dict:
    prompt = (
        f"Fix GitHub issue #{issue_num} in repo {REPO}.\n\n"
        f"Known problem from #770: {diagnosis}\n\n"
        "Your task:\n"
        f"1. Read the issue: `gh issue view {issue_num} --json body,comments,labels`\n"
        "2. Read both source and target model files in the codebase (grep for struct names)\n"
        "3. Fix the issue body to be correct and consistent with the codebase models\n"
        f"4. Update via `gh issue edit {issue_num} --body \"...\"` — do NOT post any comments\n"
        "5. Verify field names, overhead expressions, and algorithm match actual model getters\n"
        "6. Report what you fixed\n\n"
        "Do NOT post any GitHub comments. Only edit the issue body."
    )

    try:
        proc = run_cmd(
            ["claude", "--print", "--model", "sonnet", "-p", prompt],
            check=False,
        )
        if proc.returncode != 0:
            return {"issue": issue_num, "status": "failed-fix",
                    "reason": f"Agent exit {proc.returncode}"}
    except FileNotFoundError:
        return {"issue": issue_num, "status": "failed-fix",
                "reason": "claude CLI not found"}

    # Re-check
    try:
        proc2 = run_cmd(["gh", "issue", "view", str(issue_num), "--json", "body,labels"])
        data = json.loads(proc2.stdout)
        labels = [lab["name"] for lab in data.get("labels", [])]
        body = data.get("body", "")

        if "Good" in labels:
            return {"issue": issue_num, "status": "pass", "reason": "Good label present after fix"}

        has_algo = "algorithm" in body.lower() or "reduction" in body.lower()
        has_overhead = "overhead" in body.lower()
        if has_algo and has_overhead and len(body) > 500:
            return {"issue": issue_num, "status": "pass", "reason": "Body looks complete after fix"}

        return {"issue": issue_num, "status": "failed-fix",
                "reason": "Issue still incomplete after fix"}
    except Exception as exc:
        return {"issue": issue_num, "status": "failed-fix", "reason": f"Re-check failed: {exc}"}


# ---------------------------------------------------------------------------
# Stage 3: Implement (parallel worktree agents)
# ---------------------------------------------------------------------------

def stage_implement(fix_results: list[dict], *, max_jobs: int = 6) -> list[dict]:
    passing = [r for r in fix_results if r["status"] == "pass"]
    non_passing = [r for r in fix_results if r["status"] != "pass"]

    if not passing:
        print("  No issues to implement.")
        return non_passing

    print(f"  Implementing {len(passing)} issues with {max_jobs} parallel agents...")

    results = list(non_passing)
    with ThreadPoolExecutor(max_workers=max_jobs) as pool:
        futures = {
            pool.submit(implement_one, item["issue"]): item["issue"]
            for item in passing
        }
        for future in as_completed(futures):
            issue_num = futures[future]
            try:
                result = future.result()
            except Exception as exc:
                result = {"issue": issue_num, "status": "failed-impl", "reason": str(exc)}
            results.append(result)
            print(f"    #{issue_num} -> {result['status']}")

    return results


def implement_one(issue_num: int) -> dict:
    worktree_name = f"batch-{issue_num}"

    # Create worktree
    try:
        proc = run_cmd([
            "python3", "scripts/pipeline_worktree.py", "enter",
            "--name", worktree_name, "--format", "json",
        ])
        wt = json.loads(proc.stdout)
        worktree_dir = wt["worktree_dir"]
    except (subprocess.CalledProcessError, json.JSONDecodeError) as exc:
        return {"issue": issue_num, "status": "failed-impl",
                "reason": f"Worktree failed: {exc}"}

    prompt = (
        f"Implement the reduction rule for GitHub issue #{issue_num} in this repo.\n\n"
        "DO NOT create a PR or push. Just implement and commit locally.\n\n"
        "Steps:\n"
        f"1. Read the issue: `gh issue view {issue_num} --json body,comments`\n"
        "2. Read `.claude/skills/add-rule/SKILL.md` and follow its implementation steps\n"
        "3. Read reference implementations mentioned in CLAUDE.md before writing code\n"
        "4. Implement the reduction rule, tests, example_db entry, and paper entry\n"
        "5. Run `make check` to verify everything compiles and passes\n"
        '6. Commit all changes locally with a message starting with "feat: add "\n\n'
        "DO NOT push or create a PR. Just implement and commit locally."
    )

    try:
        proc = run_cmd(
            ["claude", "--print", "--model", "opus", "-p", prompt],
            check=False,
            cwd=worktree_dir,
        )
    except FileNotFoundError:
        return {"issue": issue_num, "status": "failed-impl",
                "reason": "claude CLI not found", "worktree": worktree_dir}

    if proc.returncode != 0:
        return {"issue": issue_num, "status": "failed-impl",
                "reason": f"Agent exit {proc.returncode}", "worktree": worktree_dir}

    # Get commit SHA
    try:
        sha_proc = run_cmd(["git", "-C", worktree_dir, "log", "--oneline", "-1"])
        commit_line = sha_proc.stdout.strip()
        commit_sha = commit_line.split()[0] if commit_line else ""
    except Exception:
        commit_sha = ""

    if not commit_sha:
        return {"issue": issue_num, "status": "failed-impl",
                "reason": "No commit in worktree", "worktree": worktree_dir}

    # Check for uncommitted changes
    try:
        status_proc = run_cmd(["git", "-C", worktree_dir, "status", "--porcelain"])
        if status_proc.stdout.strip():
            return {"issue": issue_num, "status": "failed-impl",
                    "reason": "Uncommitted changes", "worktree": worktree_dir,
                    "commit_sha": commit_sha}
    except Exception:
        pass

    return {
        "issue": issue_num,
        "status": "implemented",
        "commit_sha": commit_sha,
        "worktree": worktree_dir,
    }


# ---------------------------------------------------------------------------
# Stage 4: Finalize (cherry-pick, PR, cleanup)
# ---------------------------------------------------------------------------

def stage_finalize(impl_results: list[dict]) -> dict | None:
    implemented = [r for r in impl_results if r["status"] == "implemented"]
    if not implemented:
        return {"status": "skipped", "reason": "No issues implemented"}

    branch = "feat/770-warn-batch"
    print(f"  Creating branch {branch} from origin/main...")

    try:
        run_cmd(["git", "fetch", "origin", "main"])
        run_cmd(["git", "branch", "-D", branch], check=False)
        run_cmd(["git", "checkout", "-b", branch, "origin/main"])
    except subprocess.CalledProcessError as exc:
        return {"status": "error", "reason": f"Branch creation failed: {exc}"}

    # Cherry-pick
    picked: list[int] = []
    skipped: list[dict] = []
    for item in implemented:
        issue_num = item["issue"]
        sha = item["commit_sha"]
        print(f"  Cherry-picking #{issue_num} ({sha[:8]})...")

        proc = run_cmd(["git", "cherry-pick", sha], check=False)
        if proc.returncode != 0:
            if resolve_append_conflicts():
                run_cmd(["git", "cherry-pick", "--continue", "--no-edit"], check=False)
                picked.append(issue_num)
                print(f"    -> resolved conflict")
            else:
                run_cmd(["git", "cherry-pick", "--abort"], check=False)
                skipped.append({"issue": issue_num, "reason": "Cherry-pick conflict"})
                print(f"    -> skipped (conflict)")
        else:
            picked.append(issue_num)

    if not picked:
        return {"status": "error", "reason": "All cherry-picks failed"}

    # make check
    print("  Running make check...")
    check_proc = run_cmd(["make", "check"], check=False)
    if check_proc.returncode != 0:
        return {"status": "error", "reason": "make check failed",
                "picked": picked,
                "stderr": (check_proc.stderr or "")[-500:]}

    # Create PR
    print("  Creating PR...")
    fixes_lines = "\n".join(f"Fixes #{n}" for n in picked)
    issue_bullets = "\n".join(f"- **#{n}**" for n in picked)
    pr_body = (
        f"## Summary\n\n"
        f"Add {len(picked)} corrected Tier 1a/1b reduction rules from #770.\n\n"
        f"### Implemented\n{issue_bullets}\n\n{fixes_lines}\n"
    )

    try:
        run_cmd(["git", "push", "-u", "origin", branch])
        pr_proc = run_cmd([
            "gh", "pr", "create",
            "--title", f"feat: add {len(picked)} Tier 1a/1b corrected reduction rules (#770)",
            "--body", pr_body,
            "--base", "main",
            "--head", branch,
        ])
        pr_url = pr_proc.stdout.strip()
        pr_number = int(pr_url.rstrip("/").split("/")[-1]) if pr_url else None
    except subprocess.CalledProcessError as exc:
        return {"status": "error", "reason": f"PR failed: {exc}", "picked": picked}

    # Cleanup worktrees
    print("  Cleaning up worktrees...")
    for item in impl_results:
        wt = item.get("worktree")
        if wt:
            run_cmd(
                ["python3", "scripts/pipeline_worktree.py", "cleanup", "--worktree", wt],
                check=False,
            )

    return {
        "status": "success",
        "pr_url": pr_url,
        "pr_number": pr_number,
        "issues_closed": picked,
        "issues_skipped": skipped,
    }


def resolve_append_conflicts() -> bool:
    """Resolve cherry-pick conflicts in append-only files (mod.rs, reductions.typ)."""
    proc = run_cmd(["git", "diff", "--name-only", "--diff-filter=U"], check=False)
    conflicted = [f.strip() for f in proc.stdout.strip().split("\n") if f.strip()]
    if not conflicted:
        return False

    allowed_suffixes = ("mod.rs", "reductions.typ", "analysis.rs")
    for filepath in conflicted:
        if not any(filepath.endswith(s) for s in allowed_suffixes):
            return False

        content = Path(filepath).read_text()
        resolved = re.sub(
            r"<<<<<<<[^\n]*\n(.*?)=======\n(.*?)>>>>>>>[^\n]*\n",
            r"\1\2",
            content,
            flags=re.DOTALL,
        )
        if "<<<<<<" in resolved:
            return False
        Path(filepath).write_text(resolved)
        run_cmd(["git", "add", filepath])

    return True


# ---------------------------------------------------------------------------
# Commands
# ---------------------------------------------------------------------------

def cmd_fix_and_implement(args) -> int:
    log_dir = create_log_dir()
    print(f"Logging to {log_dir}")

    # Stage 1
    triage_results = stage_triage(args.issues)
    write_stage_log(log_dir, "triage", triage_results)
    print_stage_summary("Triage", triage_results)

    # Stage 2
    fix_results = stage_fix(triage_results)
    write_stage_log(log_dir, "fix", fix_results)
    print_stage_summary("Fix", fix_results)

    if args.dry_run:
        passing = [r for r in fix_results if r["status"] == "pass"]
        print(f"\n[Dry run] {len(passing)} issues would proceed to implementation.")
        write_summary(log_dir, build_summary(args.issues, triage_results, fix_results, [], None))
        return 0

    # Stage 3
    impl_results = stage_implement(fix_results, max_jobs=args.jobs)
    write_stage_log(log_dir, "implement", impl_results)
    print_stage_summary("Implement", impl_results)

    # Stage 4
    fin_result = stage_finalize(impl_results)
    write_stage_log(log_dir, "finalize", [fin_result] if fin_result else [])

    summary = build_summary(args.issues, triage_results, fix_results, impl_results, fin_result)
    write_summary(log_dir, summary)

    if fin_result and fin_result.get("pr_url"):
        print(f"\nPR created: {fin_result['pr_url']}")
    else:
        print("\nNo PR created (all issues failed or were skipped).")
    return 0


def cmd_implement(args) -> int:
    report = json.loads(Path(args.from_report).read_text())
    log_dir = Path(args.from_report).parent
    impl_results = stage_implement(report["results"], max_jobs=args.jobs)
    write_stage_log(log_dir, "implement", impl_results)
    print_stage_summary("Implement", impl_results)
    return 0


def cmd_finalize(args) -> int:
    report = json.loads(Path(args.from_report).read_text())
    log_dir = Path(args.from_report).parent
    fin_result = stage_finalize(report["results"])
    write_stage_log(log_dir, "finalize", [fin_result] if fin_result else [])
    if fin_result and fin_result.get("pr_url"):
        print(f"PR created: {fin_result['pr_url']}")
    return 0


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main() -> int:
    parser = argparse.ArgumentParser(
        description="Batch fix & implement pipeline for rule issues.",
    )
    sub = parser.add_subparsers(dest="command")

    p_full = sub.add_parser("fix-and-implement", help="Full pipeline")
    p_full.add_argument("issues", nargs="+", type=int, help="Issue numbers")
    p_full.add_argument("--jobs", type=int, default=6, help="Parallel agents (default 6)")
    p_full.add_argument("--dry-run", action="store_true", help="Triage + fix only")

    p_impl = sub.add_parser("implement", help="Resume from fix report")
    p_impl.add_argument("--from-report", required=True)
    p_impl.add_argument("--jobs", type=int, default=6)

    p_fin = sub.add_parser("finalize", help="Resume from implement report")
    p_fin.add_argument("--from-report", required=True)

    args = parser.parse_args()
    if not args.command:
        parser.print_help()
        return 1

    handlers = {
        "fix-and-implement": cmd_fix_and_implement,
        "implement": cmd_implement,
        "finalize": cmd_finalize,
    }
    return handlers[args.command](args)


if __name__ == "__main__":
    raise SystemExit(main())

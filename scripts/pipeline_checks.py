#!/usr/bin/env python3
"""Deterministic review checks for scope detection and file whitelists."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path


MODEL_WHITELIST = [
    "src/models/",
    "src/unit_tests/models/",
    "src/example_db/model_builders.rs",
    "src/example_db/rule_builders.rs",
    "docs/paper/reductions.typ",
    "tests/suites/trait_consistency.rs",
]

RULE_WHITELIST = [
    "src/rules/",
    "src/unit_tests/rules/",
    "src/example_db/rule_builders.rs",
    "src/models/",
    "docs/paper/reductions.typ",
]

IGNORED_RULE_FILES = {
    "src/rules/mod.rs",
    "src/rules/traits.rs",
    "src/rules/cost.rs",
    "src/rules/graph.rs",
    "src/rules/registry.rs",
}


def snake_to_camel(name: str) -> str:
    return "".join(part.capitalize() for part in name.split("_") if part)


def camel_to_snake(name: str) -> str:
    chars: list[str] = []
    for index, char in enumerate(name):
        if char.isupper() and index > 0 and (
            not name[index - 1].isupper()
            or (index + 1 < len(name) and name[index + 1].islower())
        ):
            chars.append("_")
        chars.append(char.lower())
    return "".join(chars)


def is_new_model_file(path: str) -> bool:
    return (
        path.startswith("src/models/")
        and path.endswith(".rs")
        and not path.endswith("/mod.rs")
    )


def is_new_rule_file(path: str) -> bool:
    return (
        path.startswith("src/rules/")
        and path.endswith(".rs")
        and path not in IGNORED_RULE_FILES
        and not path.endswith("/mod.rs")
    )


def detect_scope_from_paths(*, added_files: list[str], changed_files: list[str]) -> dict:
    models = []
    rules = []

    for path in added_files:
        if is_new_model_file(path):
            parts = path.split("/")
            category = parts[2]
            file_stem = Path(path).stem
            models.append(
                {
                    "path": path,
                    "category": category,
                    "file_stem": file_stem,
                    "problem_name": snake_to_camel(file_stem),
                }
            )
        elif is_new_rule_file(path):
            rules.append(
                {
                    "path": path,
                    "rule_stem": Path(path).stem,
                }
            )

    if models and rules:
        review_type = "model+rule"
    elif models:
        review_type = "model"
    elif rules:
        review_type = "rule"
    else:
        review_type = "generic"

    return {
        "review_type": review_type,
        "models": models,
        "rules": rules,
        "added_files": list(added_files),
        "changed_files": list(changed_files),
    }


def path_allowed(kind: str, path: str) -> tuple[bool, str | None]:
    if kind == "model":
        allowed = any(
            path == prefix or path.startswith(prefix)
            for prefix in MODEL_WHITELIST
        )
        if not allowed:
            return False, "not in whitelist for model PR"
        return True, None

    if kind == "rule":
        allowed = any(
            path == prefix or path.startswith(prefix)
            for prefix in RULE_WHITELIST
        )
        if not allowed:
            return False, "not in whitelist for rule PR"
        return True, None

    raise ValueError(f"Unsupported whitelist kind: {kind}")


def file_whitelist_check(kind: str, files: list[str]) -> dict:
    violations = []
    for path in files:
        ok, reason = path_allowed(kind, path)
        if not ok:
            violations.append({"path": path, "reason": reason})

    return {
        "kind": kind,
        "ok": not violations,
        "files": list(files),
        "violations": violations,
    }


def read_text(path: Path) -> str:
    return path.read_text() if path.exists() else ""


def find_model_file(repo_root: Path, file_stem: str) -> Path | None:
    matches = sorted((repo_root / "src/models").glob(f"*/{file_stem}.rs"))
    return matches[0] if matches else None


def find_problem_declaration(repo_root: Path, problem_name: str) -> Path | None:
    pattern = re.compile(rf"\b(?:pub\s+)?(?:struct|enum)\s+{re.escape(problem_name)}\b")
    model_root = repo_root / "src/models"
    if not model_root.exists():
        return None

    for path in sorted(model_root.rglob("*.rs")):
        if pattern.search(path.read_text()):
            return path
    return None


def check_entry(
    *,
    status: str,
    path: str | None = None,
    detail: str | None = None,
) -> dict:
    return {
        "status": status,
        "path": path,
        "detail": detail,
    }


def model_completeness(repo_root: Path, name: str) -> dict:
    file_stem = camel_to_snake(name)
    model_file = find_model_file(repo_root, file_stem)
    test_file = None
    if model_file is not None:
        category = model_file.parent.name
        test_file = repo_root / "src/unit_tests/models" / category / f"{file_stem}.rs"

    model_text = read_text(model_file) if model_file is not None else ""
    trait_text = read_text(repo_root / "src/unit_tests/trait_consistency.rs")
    paper_text = read_text(repo_root / "docs/paper/reductions.typ")

    is_optimization = f"impl OptimizationProblem for {name}" in model_text

    checks = {
        "model_file": (
            check_entry(status="pass", path=str(model_file.relative_to(repo_root)))
            if model_file is not None
            else check_entry(status="fail", detail="missing model implementation file")
        ),
        "problem_schema": (
            check_entry(status="pass", path=str(model_file.relative_to(repo_root)))
            if model_file is not None and f'name: "{name}"' in model_text
            else check_entry(status="fail", detail="missing ProblemSchemaEntry for model")
        ),
        "declare_variants": (
            check_entry(status="pass", path=str(model_file.relative_to(repo_root)))
            if model_file is not None
            and "crate::declare_variants!" in model_text
            and re.search(r"\b(?:default\s+)?(?:opt|sat)\b", model_text)
            else check_entry(status="fail", detail="missing declare_variants! with opt/sat entries")
        ),
        "canonical_example": (
            check_entry(status="pass", path=str(model_file.relative_to(repo_root)))
            if model_file is not None and "canonical_model_example_specs" in model_text
            else check_entry(status="fail", detail="missing canonical_model_example_specs")
        ),
        "unit_tests": (
            check_entry(status="pass", path=str(test_file.relative_to(repo_root)))
            if test_file is not None and test_file.exists()
            else check_entry(status="fail", detail="missing model unit tests")
        ),
        "paper_definition": (
            check_entry(status="pass", path="docs/paper/reductions.typ")
            if f'#problem-def("{name}")' in paper_text
            else check_entry(status="fail", detail="missing problem-def entry in paper")
        ),
        "paper_display_name": (
            check_entry(status="pass", path="docs/paper/reductions.typ")
            if f'"{name}":' in paper_text
            else check_entry(status="fail", detail="missing display-name entry in paper")
        ),
        "trait_consistency": (
            check_entry(status="pass", path="src/unit_tests/trait_consistency.rs")
            if name in trait_text and "test_all_problems_implement_trait_correctly" in trait_text
            else check_entry(status="fail", detail="missing trait consistency entry")
        ),
        "trait_direction": (
            check_entry(status="pass", path="src/unit_tests/trait_consistency.rs")
            if not is_optimization
            else (
                check_entry(status="pass", path="src/unit_tests/trait_consistency.rs")
                if name in trait_text.split("fn test_direction()", 1)[-1]
                else check_entry(status="fail", detail="missing optimization direction check")
            )
        ),
    }

    missing = [check_id for check_id, entry in checks.items() if entry["status"] == "fail"]
    return {
        "kind": "model",
        "name": name,
        "ok": not missing,
        "checks": checks,
        "missing": missing,
    }


def rule_completeness(
    repo_root: Path,
    name: str,
    *,
    source: str | None = None,
    target: str | None = None,
) -> dict:
    rule_file = repo_root / "src/rules" / f"{name}.rs"
    test_file = repo_root / "src/unit_tests/rules" / f"{name}.rs"
    mod_file = repo_root / "src/rules/mod.rs"
    paper_file = repo_root / "docs/paper/reductions.typ"

    rule_text = read_text(rule_file)
    mod_text = read_text(mod_file)
    paper_text = read_text(paper_file)

    paper_pattern = None
    if source and target:
        paper_pattern = f'#reduction-rule("{source}", "{target}"'

    checks = {
        "rule_file": (
            check_entry(status="pass", path=str(rule_file.relative_to(repo_root)))
            if rule_file.exists()
            else check_entry(status="fail", detail="missing rule implementation file")
        ),
        "module_registration": (
            check_entry(status="pass", path=str(mod_file.relative_to(repo_root)))
            if rule_file.exists() and name in mod_text
            else check_entry(status="fail", detail="missing src/rules/mod.rs registration")
        ),
        "unit_tests": (
            check_entry(status="pass", path=str(test_file.relative_to(repo_root)))
            if test_file.exists()
            else check_entry(status="fail", detail="missing rule unit tests")
        ),
        "overhead_form": (
            check_entry(status="pass", path=str(rule_file.relative_to(repo_root)))
            if rule_file.exists() and "#[reduction(overhead = {" in rule_text
            else check_entry(status="fail", detail="missing #[reduction(overhead = {...})] form")
        ),
        "canonical_example": (
            check_entry(status="pass", path=str(rule_file.relative_to(repo_root)))
            if rule_file.exists() and "canonical_rule_example_specs" in rule_text
            else check_entry(status="fail", detail="missing canonical_rule_example_specs")
        ),
        "paper_rule": (
            check_entry(status="pass", path=str(paper_file.relative_to(repo_root)))
            if paper_pattern is not None and paper_pattern in paper_text
            else check_entry(
                status="fail",
                detail=(
                    "missing reduction-rule entry in paper"
                    if paper_pattern is not None
                    else "source/target required to check paper reduction-rule entry"
                ),
            )
        ),
    }

    missing = [check_id for check_id, entry in checks.items() if entry["status"] == "fail"]
    return {
        "kind": "rule",
        "name": name,
        "source": source,
        "target": target,
        "ok": not missing,
        "checks": checks,
        "missing": missing,
    }


def completeness_check(
    kind: str,
    repo_root: str | Path,
    *,
    name: str,
    source: str | None = None,
    target: str | None = None,
) -> dict:
    repo_root = Path(repo_root)
    if kind == "model":
        return model_completeness(repo_root, name)
    if kind == "rule":
        return rule_completeness(repo_root, name, source=source, target=target)
    raise ValueError(f"Unsupported completeness kind: {kind}")


def infer_review_subject(
    scope: dict,
    *,
    kind: str | None = None,
    name: str | None = None,
    source: str | None = None,
    target: str | None = None,
) -> dict:
    if kind is not None:
        return {
            "kind": kind,
            "name": name,
            "source": source,
            "target": target,
            "inferred": False,
        }

    review_type = scope.get("review_type")
    if review_type == "model" and len(scope.get("models", [])) == 1:
        model = scope["models"][0]
        return {
            "kind": "model",
            "name": model.get("problem_name"),
            "source": None,
            "target": None,
            "inferred": True,
        }

    if review_type == "rule" and len(scope.get("rules", [])) == 1:
        rule = scope["rules"][0]
        return {
            "kind": "rule",
            "name": rule.get("rule_stem"),
            "source": source,
            "target": target,
            "inferred": True,
        }

    return {
        "kind": "generic",
        "name": None,
        "source": None,
        "target": None,
        "inferred": True,
    }


def skipped_check(reason: str) -> dict:
    return {
        "ok": True,
        "skipped": True,
        "reason": reason,
    }


def build_review_context(
    repo_root: str | Path,
    *,
    diff_stat: str,
    scope: dict,
    subject: dict,
) -> dict:
    changed_files = list(scope.get("changed_files", []))
    kind = subject.get("kind")

    if kind in {"model", "rule"}:
        whitelist = file_whitelist_check(kind, changed_files)
        whitelist["skipped"] = False
        whitelist["reason"] = None
    else:
        whitelist = skipped_check("no model/rule subject available")

    if kind == "model" and subject.get("name"):
        completeness = completeness_check(
            "model",
            repo_root,
            name=subject["name"],
        )
        completeness["skipped"] = False
        completeness["reason"] = None
    elif kind == "rule" and subject.get("name") and subject.get("source") and subject.get("target"):
        completeness = completeness_check(
            "rule",
            repo_root,
            name=subject["name"],
            source=subject["source"],
            target=subject["target"],
        )
        completeness["skipped"] = False
        completeness["reason"] = None
    elif kind == "rule":
        completeness = skipped_check("rule completeness requires source and target")
    else:
        completeness = skipped_check("no model/rule subject available")

    return {
        "diff_stat": diff_stat,
        "changed_files": changed_files,
        "scope": scope,
        "subject": subject,
        "whitelist": whitelist,
        "completeness": completeness,
    }


RULE_TITLE_RE = re.compile(r"^\[Rule\]\s+(?P<source>.+?)\s+to\s+(?P<target>.+?)\s*$")
MODEL_TITLE_RE = re.compile(r"^\[Model\]\s+(?P<name>.+?)\s*$")


def issue_kind_from_title(title: str | None) -> tuple[str, str | None, str | None]:
    title = (title or "").strip()

    rule_match = RULE_TITLE_RE.match(title)
    if rule_match:
        return "rule", rule_match.group("source"), rule_match.group("target")

    if MODEL_TITLE_RE.match(title):
        return "model", None, None

    return "other", None, None


def normalize_issue_comment(comment: dict) -> dict:
    author = comment.get("author") or comment.get("user") or {}
    return {
        "author": author.get("login") or author.get("name") or "",
        "body": comment.get("body", ""),
    }


def normalize_existing_pr(pr: dict) -> dict:
    return {
        "number": pr.get("number"),
        "head_ref_name": pr.get("headRefName"),
        "url": pr.get("url"),
    }


def issue_guard_check(
    repo_root: str | Path,
    *,
    issue: dict,
    existing_prs: list[dict],
) -> dict:
    repo_root = Path(repo_root)
    title = issue.get("title", "")
    labels = [label.get("name") for label in issue.get("labels", []) if label.get("name")]
    comments = [normalize_issue_comment(comment) for comment in issue.get("comments", [])]
    kind, source_problem, target_problem = issue_kind_from_title(title)

    checks = {
        "good_label": (
            check_entry(status="pass", detail='label "Good" present')
            if "Good" in labels
            else check_entry(status="fail", detail='missing required "Good" label')
        ),
        "source_model": check_entry(status="skip", detail="not a rule issue"),
        "target_model": check_entry(status="skip", detail="not a rule issue"),
    }

    if kind == "rule":
        source_path = find_problem_declaration(repo_root, source_problem or "")
        target_path = find_problem_declaration(repo_root, target_problem or "")
        checks["source_model"] = (
            check_entry(status="pass", path=str(source_path.relative_to(repo_root)))
            if source_path is not None
            else check_entry(
                status="fail",
                detail=f"model {source_problem!r} not found under src/models/",
            )
        )
        checks["target_model"] = (
            check_entry(status="pass", path=str(target_path.relative_to(repo_root)))
            if target_path is not None
            else check_entry(
                status="fail",
                detail=f"model {target_problem!r} not found under src/models/",
            )
        )

    missing = [name for name, entry in checks.items() if entry["status"] == "fail"]
    normalized_existing_prs = [normalize_existing_pr(pr) for pr in existing_prs]
    resume_pr = normalized_existing_prs[0] if normalized_existing_prs else None

    return {
        "issue_number": issue.get("number"),
        "title": title,
        "body": issue.get("body"),
        "state": issue.get("state"),
        "url": issue.get("url"),
        "labels": labels,
        "comments": comments,
        "kind": kind,
        "source_problem": source_problem,
        "target_problem": target_problem,
        "ok": not missing,
        "checks": checks,
        "missing": missing,
        "existing_prs": normalized_existing_prs,
        "resume_pr": resume_pr,
        "action": "resume-pr" if resume_pr is not None else "create-pr",
    }


def issue_context_check(
    repo_root: str | Path,
    *,
    issue: dict,
    existing_prs: list[dict],
) -> dict:
    return issue_guard_check(
        repo_root,
        issue=issue,
        existing_prs=existing_prs,
    )


def run_gh_json(*args: str):
    return json.loads(subprocess.check_output(["gh", *args], text=True))


def fetch_issue(repo: str, issue_number: int) -> dict:
    return run_gh_json(
        "issue",
        "view",
        str(issue_number),
        "--repo",
        repo,
        "--json",
        "number,title,body,state,url,labels,comments",
    )


def fetch_existing_prs_via_search(repo: str, issue_number: int) -> list[dict]:
    query = f'repo:{repo} is:pr is:open "Fix #{issue_number}"'
    data = run_gh_json("api", "search/issues", "-f", f"q={query}")
    items = data.get("items", [])
    prs: list[dict] = []
    for item in items:
        number = item.get("number")
        if number is None or "pull_request" not in item:
            continue
        pr = run_gh_json("api", f"repos/{repo}/pulls/{number}")
        prs.append(
            {
                "number": pr.get("number"),
                "headRefName": (pr.get("head") or {}).get("ref"),
                "url": pr.get("html_url"),
            }
        )
    return prs


def fetch_existing_prs(repo: str, issue_number: int) -> list[dict]:
    try:
        data = run_gh_json(
            "pr",
            "list",
            "--repo",
            repo,
            "--state",
            "open",
            "--search",
            f"Fix #{issue_number}",
            "--json",
            "number,headRefName,url",
        )
    except subprocess.CalledProcessError:
        return fetch_existing_prs_via_search(repo, issue_number)
    if not isinstance(data, list):
        raise ValueError(f"Unexpected PR list payload for issue #{issue_number}: {data!r}")
    return data


def git_output(*args: str) -> list[str]:
    output = subprocess.check_output(["git", *args], text=True)
    return [line for line in output.splitlines() if line]


def git_text(*args: str) -> str:
    return subprocess.check_output(["git", *args], text=True)


def load_file_list(path: str | Path) -> list[str]:
    lines = Path(path).read_text().splitlines()
    return [line.strip() for line in lines if line.strip()]


def emit_result(result: dict, fmt: str) -> None:
    print(json.dumps(result, indent=2, sort_keys=True))


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Pipeline review checks.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    detect = subparsers.add_parser("detect-scope")
    detect.add_argument("--base", required=True)
    detect.add_argument("--head", required=True)
    detect.add_argument("--format", choices=["json", "text"], default="json")

    whitelist = subparsers.add_parser("file-whitelist")
    whitelist.add_argument("--kind", choices=["model", "rule"], required=True)
    whitelist.add_argument("--files-file", required=True)
    whitelist.add_argument("--format", choices=["json", "text"], default="json")

    completeness = subparsers.add_parser("completeness")
    completeness.add_argument("--kind", choices=["model", "rule"], required=True)
    completeness.add_argument("--name", required=True)
    completeness.add_argument("--source")
    completeness.add_argument("--target")
    completeness.add_argument("--repo-root", default=".")
    completeness.add_argument("--format", choices=["json", "text"], default="json")

    review_context = subparsers.add_parser("review-context")
    review_context.add_argument("--repo-root", default=".")
    review_context.add_argument("--base", required=True)
    review_context.add_argument("--head", required=True)
    review_context.add_argument("--kind", choices=["model", "rule", "generic"])
    review_context.add_argument("--name")
    review_context.add_argument("--source")
    review_context.add_argument("--target")
    review_context.add_argument("--format", choices=["json", "text"], default="json")

    issue_guards = subparsers.add_parser("issue-guards")
    issue_guards.add_argument("--repo", required=True)
    issue_guards.add_argument("--issue", required=True, type=int)
    issue_guards.add_argument("--repo-root", default=".")
    issue_guards.add_argument("--format", choices=["json", "text"], default="json")

    issue_context = subparsers.add_parser("issue-context")
    issue_context.add_argument("--repo", required=True)
    issue_context.add_argument("--issue", required=True, type=int)
    issue_context.add_argument("--repo-root", default=".")
    issue_context.add_argument("--format", choices=["json", "text"], default="json")

    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])

    if args.command == "detect-scope":
        changed_files = git_output("diff", "--name-only", f"{args.base}..{args.head}")
        added_files = git_output(
            "diff",
            "--name-only",
            "--diff-filter=A",
            f"{args.base}..{args.head}",
        )
        emit_result(
            detect_scope_from_paths(
                added_files=added_files,
                changed_files=changed_files,
            ),
            args.format,
        )
        return 0

    if args.command == "file-whitelist":
        emit_result(
            file_whitelist_check(args.kind, load_file_list(args.files_file)),
            args.format,
        )
        return 0

    if args.command == "completeness":
        emit_result(
            completeness_check(
                args.kind,
                args.repo_root,
                name=args.name,
                source=args.source,
                target=args.target,
            ),
            args.format,
        )
        return 0

    if args.command == "review-context":
        changed_files = git_output("diff", "--name-only", f"{args.base}..{args.head}")
        added_files = git_output(
            "diff",
            "--name-only",
            "--diff-filter=A",
            f"{args.base}..{args.head}",
        )
        scope = detect_scope_from_paths(
            added_files=added_files,
            changed_files=changed_files,
        )
        subject = infer_review_subject(
            scope,
            kind=args.kind,
            name=args.name,
            source=args.source,
            target=args.target,
        )
        emit_result(
            build_review_context(
                args.repo_root,
                diff_stat=git_text("diff", "--stat", f"{args.base}..{args.head}"),
                scope=scope,
                subject=subject,
            ),
            args.format,
        )
        return 0

    if args.command in {"issue-guards", "issue-context"}:
        emit_result(
            issue_context_check(
                args.repo_root,
                issue=fetch_issue(args.repo, args.issue),
                existing_prs=fetch_existing_prs(args.repo, args.issue),
            ),
            args.format,
        )
        return 0

    raise AssertionError(f"Unhandled command: {args.command}")


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Mine merged PRs from CodingThrust/problem-reductions to understand project evolution.

Extracts all merged PRs, classifies them by type ([Rule]/[Model]), author type
(agent vs human), and project phase (manual / basic skills / full pipeline).

Phase boundaries:
  - Phase 1 (manual):       before 2026-02-22 (no add-model/add-rule skills)
  - Phase 2 (basic skills): 2026-02-22 to 2026-02-28 (add-model/add-rule exist)
  - Phase 3 (full pipeline): 2026-03-01 onwards (meta-power batch resolution)

Output: JSON to stdout with summary, per-phase breakdown, and full PR list.
"""

import json
import re
import subprocess
import sys
from datetime import datetime, timezone

REPO = "CodingThrust/problem-reductions"

# Phase boundary dates (UTC).  Determined from:
#   git show 3ddc415 --format="%ai"  => 2026-02-22  (add-model / add-rule skills)
#   git show 2cfb1b7 --format="%ai"  => 2026-03-01  (meta-power skill)
PHASE_BOUNDARIES = [
    datetime(2026, 2, 22, tzinfo=timezone.utc),   # Phase 1 -> Phase 2
    datetime(2026, 3, 1, tzinfo=timezone.utc),     # Phase 2 -> Phase 3
]

PHASE_LABELS = ["manual", "basic-skills", "full-pipeline"]

AGENT_LOGINS = {"github-actions"}


def is_agent(author: dict) -> bool:
    """Classify author as agent (bot) or human."""
    login = author.get("login", "")
    if "[bot]" in login:
        return True
    if login in AGENT_LOGINS:
        return True
    return author.get("is_bot", False)


def classify_phase(merged_at: str) -> int:
    """Return 1-based phase number from the merged-at timestamp."""
    dt = datetime.fromisoformat(merged_at.replace("Z", "+00:00"))
    for i, boundary in enumerate(PHASE_BOUNDARIES):
        if dt < boundary:
            return i + 1
    return len(PHASE_BOUNDARIES) + 1


def classify_type(title: str) -> str | None:
    """Return 'Rule', 'Model', or None based on PR title heuristics."""
    if "[Rule]" in title:
        return "Rule"
    if "[Model]" in title:
        return "Model"
    # Heuristic: detect issue-linked PRs whose branch or title imply a model/rule
    # e.g. "Fix #52: TravelingSalesman to ILP reduction" => Rule
    # e.g. "Fix #47: Add HamiltonianCycle model" => Model
    title_lower = title.lower()
    if re.search(r"\breduction\b", title_lower) and re.search(r"\bto\b", title_lower):
        return "Rule"
    if re.search(r"\badd\b.*\bmodel\b", title_lower):
        return "Model"
    return None


def fetch_prs() -> list[dict]:
    """Fetch all merged PRs from GitHub."""
    cmd = [
        "gh", "pr", "list",
        "--repo", REPO,
        "--state", "merged",
        "--limit", "999",
        "--json", "number,title,author,createdAt,mergedAt,labels,headRefName",
    ]
    result = subprocess.run(cmd, capture_output=True, text=True, check=True)
    return json.loads(result.stdout)


def main():
    prs_raw = fetch_prs()

    prs = []
    for pr in sorted(prs_raw, key=lambda x: x["number"]):
        pr_type = classify_type(pr["title"])
        agent = is_agent(pr["author"])
        phase = classify_phase(pr["mergedAt"])

        prs.append({
            "number": pr["number"],
            "title": pr["title"],
            "author": pr["author"]["login"],
            "created_at": pr["createdAt"],
            "merged_at": pr["mergedAt"],
            "branch": pr["headRefName"],
            "is_agent": agent,
            "phase": phase,
            "type": pr_type,
        })

    # Summary
    rule_prs = [p for p in prs if p["type"] == "Rule"]
    model_prs = [p for p in prs if p["type"] == "Model"]
    agent_prs = [p for p in prs if p["is_agent"]]
    human_prs = [p for p in prs if not p["is_agent"]]

    summary = {
        "total_prs": len(prs),
        "rule_prs": len(rule_prs),
        "model_prs": len(model_prs),
        "other_prs": len(prs) - len(rule_prs) - len(model_prs),
        "agent_authored": len(agent_prs),
        "human_authored": len(human_prs),
    }

    # Per-phase breakdown
    by_phase = []
    for phase_num, label in enumerate(PHASE_LABELS, start=1):
        phase_prs = [p for p in prs if p["phase"] == phase_num]
        by_phase.append({
            "phase": phase_num,
            "label": label,
            "count": len(phase_prs),
            "rule_count": len([p for p in phase_prs if p["type"] == "Rule"]),
            "model_count": len([p for p in phase_prs if p["type"] == "Model"]),
            "agent_count": len([p for p in phase_prs if p["is_agent"]]),
            "human_count": len([p for p in phase_prs if not p["is_agent"]]),
        })

    output = {
        "summary": summary,
        "by_phase": by_phase,
        "phase_boundaries": {
            "phase_1_end": PHASE_BOUNDARIES[0].isoformat(),
            "phase_2_end": PHASE_BOUNDARIES[1].isoformat(),
        },
        "prs": prs,
    }

    json.dump(output, sys.stdout, indent=2)
    print()  # trailing newline


if __name__ == "__main__":
    main()

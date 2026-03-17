#!/usr/bin/env python3
"""Submit enriched issue files to GitHub sequentially with validation."""

import os
import re
import json
import subprocess
import sys
import time

RULES_DIR = "references/issues/rules"
MODELS_DIR = "references/issues/models"
REPO = "CodingThrust/problem-reductions"
MILESTONE = "Garey & Johnson"


def parse_frontmatter(content: str) -> tuple[dict, str]:
    """Parse YAML frontmatter and return (metadata, body)."""
    if not content.startswith("---"):
        return {}, content
    parts = content.split("---", 2)
    if len(parts) < 3:
        return {}, content
    # Simple YAML parsing (avoid dependency)
    meta = {}
    for line in parts[1].strip().split("\n"):
        line = line.strip()
        if ":" in line:
            key, _, val = line.partition(":")
            val = val.strip().strip("'\"")
            meta[key.strip()] = val
    body = parts[2].strip()
    return meta, body


def check_duplicate(label: str, title: str) -> int | None:
    """Check for existing open issue with same title. Returns issue number or None."""
    # Extract key words from title for search
    keywords = title.replace("[Rule]", "").replace("[Model]", "").strip()
    result = subprocess.run(
        ["gh", "issue", "list", "--repo", REPO, "--label", label,
         "--search", f'in:title {keywords}',
         "--json", "number,title", "--limit", "5"],
        capture_output=True, text=True
    )
    if result.returncode != 0:
        return None
    issues = json.loads(result.stdout)
    for issue in issues:
        if issue["title"].strip().lower() == title.strip().lower():
            return issue["number"]
    return None


def create_issue(title: str, labels: str, body: str) -> str | None:
    """Create GitHub issue. Returns URL or None on failure."""
    result = subprocess.run(
        ["gh", "issue", "create", "--repo", REPO,
         "--title", title,
         "--label", labels,
         "--milestone", MILESTONE,
         "--body", body],
        capture_output=True, text=True
    )
    if result.returncode != 0:
        print(f"    gh error: {result.stderr.strip()}")
        return None
    url = result.stdout.strip()
    return url


def verify_issue(number: int, title: str, body_len: int) -> bool:
    """Read back issue and verify title/body match."""
    result = subprocess.run(
        ["gh", "issue", "view", str(number), "--repo", REPO,
         "--json", "title,body"],
        capture_output=True, text=True
    )
    if result.returncode != 0:
        return False
    data = json.loads(result.stdout)
    if data["title"] != title:
        print(f"    VERIFY FAIL: title mismatch: {data['title']!r} vs {title!r}")
        return False
    remote_len = len(data.get("body", ""))
    if abs(remote_len - body_len) > body_len * 0.1:
        print(f"    VERIFY FAIL: body length {remote_len} vs submitted {body_len}")
        return False
    return True


def close_issue(number: int, reason: str):
    """Close a bad issue."""
    subprocess.run(
        ["gh", "issue", "close", str(number), "--repo", REPO,
         "--reason", "not planned",
         "--comment", f"Auto-closed: {reason}"],
        capture_output=True, text=True
    )


def process_file(filepath: str) -> str:
    """Process a single file. Returns log line."""
    filename = os.path.basename(filepath)

    # Step 1: Read file fresh
    with open(filepath, "r") as f:
        content = f.read()

    # Step 2: Parse
    meta, body = parse_frontmatter(content)
    title = meta.get("title", "")
    labels = meta.get("labels", "")

    # Step 3: Validation
    if not title:
        return f"❌ {filename} -> BLOCKED: no title found in frontmatter"

    if len(body) < 100:
        return f"❌ {filename} -> BLOCKED: body too short ({len(body)} chars)"

    # Title-body consistency for rules
    if "[Rule]" in title:
        source_match = re.search(r'\*\*Source:\*\*\s*(.*)', body)
        target_match = re.search(r'\*\*Target:\*\*\s*(.*)', body)
        if not source_match or not target_match:
            return f"❌ {filename} -> BLOCKED: missing Source/Target in body"

    # Title-body consistency for models
    if "[Model]" in title:
        model_name = title.replace("[Model]", "").strip()
        # Split CamelCase into words for matching
        camel_words = re.findall(r'[A-Z][a-z]+|[A-Z]+(?=[A-Z][a-z])|[a-z]+|\d+', model_name)
        # Also try the raw name and space-split words
        all_words = set(w.lower() for w in camel_words if len(w) > 2)
        all_words.update(w.lower() for w in model_name.split() if len(w) > 3)
        body_lower = body[:2000].lower()
        if not any(w in body_lower for w in all_words):
            return f"❌ {filename} -> BLOCKED: body doesn't reference model name ({all_words})"

    # TBD check (warning only)
    tbd_warning = ""
    # Check if (TBD) is the sole content of a section
    if re.search(r'\n\(TBD\)\s*\n', body):
        tbd_warning = " [WARNING: has residual (TBD)]"

    # Duplicate check
    dup = check_duplicate(labels, title)
    if dup:
        return f"⏭️  {filename} -> SKIP: duplicate #{dup}"

    # Step 4: Create issue
    url = create_issue(title, labels, body)
    if not url:
        return f"❌ {filename} -> BLOCKED: gh issue create failed"

    # Extract issue number from URL
    num_match = re.search(r'/(\d+)$', url)
    if not num_match:
        return f"❌ {filename} -> BLOCKED: could not parse issue number from {url}"
    issue_num = int(num_match.group(1))

    # Step 5: Verify
    if not verify_issue(issue_num, title, len(body)):
        close_issue(issue_num, "post-creation verification failed")
        return f"❌ {filename} -> BLOCKED: verification failed, closed #{issue_num}"

    return f"✅ {filename} -> #{issue_num} (title OK, body {len(body)} chars){tbd_warning}"


def main():
    # Build file list
    files = []

    # Rules R131-R151
    for f in sorted(os.listdir(RULES_DIR)):
        m = re.match(r'R(\d+)_', f)
        if m and 131 <= int(m.group(1)) <= 151:
            files.append(os.path.join(RULES_DIR, f))

    # Models
    model_ids = ['P1', 'P53', 'P129', 'P142', 'P143',
                 'P185', 'P186', 'P187', 'P188', 'P189', 'P190', 'P191',
                 'P193', 'P194', 'P195', 'P196', 'P197', 'P198',
                 'P199', 'P200', 'P201', 'P202', 'P203', 'P204', 'P205', 'P206',
                 'P293']
    for mid in model_ids:
        candidates = [f for f in os.listdir(MODELS_DIR) if f.startswith(mid + '_')]
        if candidates:
            files.append(os.path.join(MODELS_DIR, sorted(candidates)[0]))

    print(f"Submitting {len(files)} issues sequentially...\n")

    results = []
    for i, filepath in enumerate(files):
        print(f"[{i+1}/{len(files)}] {os.path.basename(filepath)}")
        result = process_file(filepath)
        results.append(result)
        print(f"  {result}")
        # Rate limiting
        if i < len(files) - 1:
            time.sleep(1.5)

    # Summary
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)
    created = sum(1 for r in results if r.startswith("✅"))
    skipped = sum(1 for r in results if r.startswith("⏭️"))
    blocked = sum(1 for r in results if r.startswith("❌"))
    print(f"Created: {created}, Skipped: {skipped}, Blocked: {blocked}")
    print()
    for r in results:
        print(r)


if __name__ == "__main__":
    main()

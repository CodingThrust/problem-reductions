#!/usr/bin/env python3
"""Submit enriched markdown files as GitHub issues."""
import subprocess
import sys
import time
import json
import re
import os

REPO = "CodingThrust/problem-reductions"
MILESTONE = "Garey & Johnson"
BASE = "/Users/xiweipan/Codes/problem-reductions/references/issues"

# Files to submit: models first, then rules
FILES = [
    # Models
    ("models/P20_partial_feedback_edge_set.md", "model"),
    ("models/P168_string-to-string_correction.md", "model"),
    ("models/P169_grouping_by_swapping.md", "model"),
    ("models/P170_external_macro_data_compression.md", "model"),
    ("models/P171_internal_macro_data_compression.md", "model"),
    ("models/P173_rectilinear_picture_compression.md", "model"),
    ("models/P174_minimum_cardinality_key.md", "model"),
    ("models/P175_additional_key.md", "model"),
    ("models/P176_prime_attribute_name.md", "model"),
    ("models/P177_boyce-codd_normal_form_violation.md", "model"),
    ("models/P178_conjunctive_query_foldability.md", "model"),
    ("models/P179_conjunctive_boolean_query.md", "model"),
    ("models/P180_tableau_equivalence.md", "model"),
    ("models/P182_safety_of_database_transaction_systems.md", "model"),
    ("models/P183_consistency_of_database_frequency_tables.md", "model"),
    # Rules
    ("rules/R114_sc_stringtostringcorrection.md", "rule"),
    ("rules/R115_fes_groupingbyswapping.md", "rule"),
    ("rules/R116_vc_externalmacrodatacompression.md", "rule"),
    ("rules/R117_vc_internalmacrodatacompression.md", "rule"),
    ("rules/R118_x3c_regularexpressionsubstitution.md", "rule"),
    ("rules/R119_3sat_rectilinearpicturecompression.md", "rule"),
    ("rules/R120_vc_minimumcardinalitykey.md", "rule"),
    ("rules/R121_hs_additionalkey.md", "rule"),
    ("rules/R122_mck_primeattributename.md", "rule"),
    ("rules/R123_hs_boycecnfv.md", "rule"),
    ("rules/R124_3col_conjunctivequeryfoldability.md", "rule"),
    ("rules/R125_clique_conjunctivebooleanquery.md", "rule"),
    ("rules/R126_3sat_tableauequivalence.md", "rule"),
    ("rules/R127_m3sat_serializability.md", "rule"),
    ("rules/R128_hs_safetydbts.md", "rule"),
    ("rules/R129_3sat_consistencyfreqtables.md", "rule"),
]

def parse_frontmatter(content):
    """Parse YAML frontmatter and return (title, labels, body)."""
    lines = content.split('\n')
    if lines[0].strip() != '---':
        return None, None, content

    end_idx = None
    for i in range(1, len(lines)):
        if lines[i].strip() == '---':
            end_idx = i
            break

    if end_idx is None:
        return None, None, content

    frontmatter = '\n'.join(lines[1:end_idx])
    body = '\n'.join(lines[end_idx+1:]).strip()

    title = None
    labels = None
    for line in frontmatter.split('\n'):
        if line.startswith('title:'):
            title = line.split(':', 1)[1].strip().strip('"').strip("'")
        elif line.startswith('labels:'):
            labels = line.split(':', 1)[1].strip().strip('"').strip("'")

    return title, labels, body

def validate(filename, title, labels, body):
    """Validate issue before submission. Returns (ok, reason)."""
    if not title:
        return False, "no title found in frontmatter"
    if not body or len(body) < 100:
        return False, f"body is empty or too short ({len(body) if body else 0} chars)"

    if "[Rule]" in title:
        if "**Source:**" not in body or "**Target:**" not in body:
            return False, "body missing **Source:** or **Target:** lines"
    elif "[Model]" in title:
        if "## Definition" not in body and "## Motivation" not in body:
            return False, "body missing ## Definition or ## Motivation section"

    return True, "OK"

def create_issue(title, labels, body):
    """Create GitHub issue and return (success, issue_number_or_error)."""
    # Write body to temp file to avoid shell escaping issues
    tmp = "/tmp/gh_issue_body.md"
    with open(tmp, 'w') as f:
        f.write(body)

    cmd = [
        "gh", "issue", "create",
        "--repo", REPO,
        "--title", title,
        "--label", labels,
        "--milestone", MILESTONE,
        "--body-file", tmp,
    ]

    result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)

    if result.returncode != 0:
        return False, f"gh error: {result.stderr.strip()}"

    url = result.stdout.strip()
    match = re.search(r'/issues/(\d+)', url)
    if not match:
        return False, f"could not parse issue number from: {url}"

    return True, int(match.group(1))

def verify_issue(number, expected_title):
    """Verify issue was created correctly. Returns (ok, reason)."""
    cmd = [
        "gh", "issue", "view", str(number),
        "--repo", REPO,
        "--json", "title,body",
    ]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=15)
    if result.returncode != 0:
        return False, f"verify failed: {result.stderr.strip()}"

    data = json.loads(result.stdout)
    if data["title"] != expected_title:
        return False, f"title mismatch: got '{data['title']}'"

    return True, "OK"

def close_issue(number, reason="not planned"):
    """Close a bad issue."""
    subprocess.run([
        "gh", "issue", "close", str(number),
        "--repo", REPO,
        "--reason", reason,
        "--comment", "Auto-closed: post-creation verification failed",
    ], capture_output=True, text=True, timeout=15)

def main():
    results = []

    for relpath, default_label in FILES:
        filename = os.path.basename(relpath)
        filepath = os.path.join(BASE, relpath)

        # Step 1: Read file
        try:
            with open(filepath, 'r') as f:
                content = f.read()
        except FileNotFoundError:
            print(f"❌ {filename} -> BLOCKED: file not found")
            results.append((filename, "❌", "BLOCKED: file not found", "-", 0))
            continue

        # Step 2: Parse
        title, labels, body = parse_frontmatter(content)
        if not labels:
            labels = default_label

        # Step 3: Validate
        ok, reason = validate(filename, title, labels, body)
        if not ok:
            print(f"❌ {filename} -> BLOCKED: {reason}")
            results.append((filename, "❌", f"BLOCKED: {reason}", "-", 0))
            continue

        body_len = len(body)

        # Step 4: Create issue
        ok, num_or_err = create_issue(title, labels, body)
        if not ok:
            print(f"❌ {filename} -> FAILED: {num_or_err}")
            results.append((filename, "❌", f"FAILED: {num_or_err}", "-", body_len))
            continue

        issue_num = num_or_err

        # Step 5: Verify
        ok, reason = verify_issue(issue_num, title)
        if not ok:
            print(f"❌ {filename} -> #{issue_num} VERIFY FAILED: {reason}")
            close_issue(issue_num)
            results.append((filename, "❌", f"VERIFY FAILED: {reason}", f"#{issue_num} (closed)", body_len))
            continue

        print(f"✅ {filename} -> #{issue_num} (title OK, body {body_len} chars)")
        results.append((filename, "✅", "OK", f"#{issue_num}", body_len))

        # Step 7: Rate limit
        time.sleep(1)

    # Final summary
    print("\n## Final Results\n")
    print(f"| # | File | Status | Issue | Body chars |")
    print(f"|---|------|--------|-------|------------|")
    for i, (fname, status, detail, issue, blen) in enumerate(results, 1):
        print(f"| {i} | {fname} | {status} {detail} | {issue} | {blen} |")

    success = sum(1 for r in results if r[1] == "✅")
    failed = sum(1 for r in results if r[1] == "❌")
    print(f"\n**Total: {success} created, {failed} failed/blocked**")

if __name__ == "__main__":
    main()

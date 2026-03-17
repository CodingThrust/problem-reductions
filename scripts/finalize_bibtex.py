#!/usr/bin/env python3
"""
Deduplicate and sort bibliography.bib entries by cite key.
Usage: uv run scripts/finalize_bibtex.py
"""
import re
from pathlib import Path

BIB = Path("references/Garey&Johnson/bibliography.bib")
text = BIB.read_text()
entries = re.split(r'\n\n(?=@)', text.strip())
seen = {}
for e in entries:
    # Keys may contain commas, e.g. {Aho, 1977a,
    # Capture everything between { and ,\n (comma followed by newline)
    m = re.match(r'@\w+\{(.+?),\s*\n', e)
    if m:
        key = m.group(1).strip()
        if key not in seen:
            seen[key] = e

sorted_entries = sorted(seen.values(), key=lambda e: re.search(r'\{(.+?),\s*\n', e).group(1).lower())
BIB.write_text("\n\n".join(sorted_entries) + "\n")
print(f"Final: {len(sorted_entries)} unique entries")

#!/usr/bin/env python3
"""
Enrich bibliography.bib entries with DOI and URL via Semantic Scholar API.
Usage: uv run scripts/enrich_bibtex.py
Requires: httpx (uv add httpx)
"""
import json, re, time, httpx
from pathlib import Path

BIB_PATH = Path("references/Garey&Johnson/bibliography.bib")
SS_SEARCH = "https://api.semanticscholar.org/graph/v1/paper/search"
FIELDS = "title,authors,year,externalIds,openAccessPdf"
RATE_SLEEP = 3.0  # seconds between requests (be conservative to avoid 429s)

def parse_entries(text):
    """Return list of (key, full_entry_str) from bib text."""
    entries = re.split(r'\n\n(?=@)', text.strip())
    result = []
    for e in entries:
        # Keys may contain commas, e.g. {Aho, 1977a,
        m = re.match(r'@\w+\{(.+?),\s*\n', e)
        if m:
            result.append((m.group(1).strip(), e))
    return result

def get_title(entry_str):
    m = re.search(r'title\s*=\s*\{([^}]+)\}', entry_str, re.IGNORECASE)
    return m.group(1).strip() if m else None

def get_author(entry_str):
    m = re.search(r'author\s*=\s*\{([^}]+)\}', entry_str, re.IGNORECASE)
    return m.group(1).split(' and ')[0].strip() if m else None

def get_year(entry_str):
    m = re.search(r'year\s*=\s*\{(\d{4})\}', entry_str, re.IGNORECASE)
    return m.group(1) if m else None

def query_semantic_scholar(title, author, year):
    # Strip LaTeX braces/commands from title before querying
    clean_title = re.sub(r'[{}\\]', '', title)
    query = f"{clean_title} {author}"
    for attempt in range(4):
        try:
            r = httpx.get(SS_SEARCH, params={"query": query, "fields": FIELDS, "limit": 3}, timeout=10)
            if r.status_code == 429:
                wait = 10 * (2 ** attempt)
                print(f"  429 rate limit, waiting {wait}s...")
                time.sleep(wait)
                continue
            r.raise_for_status()
            data = r.json()
            break
        except Exception as e:
            print(f"  SS error: {e}")
            return None, None
    else:
        return None, None
    for paper in data.get("data", []):
        py = str(paper.get("year", ""))
        if year and py != year:
            continue
        doi = paper.get("externalIds", {}).get("DOI")
        pdf = (paper.get("openAccessPdf") or {}).get("url")
        if doi or pdf:
            return doi, pdf
    return None, None

def inject_fields(entry_str, doi, url):
    """Add doi/url fields before the closing }."""
    additions = []
    if doi and "doi" not in entry_str.lower():
        additions.append(f"  doi     = {{{doi}}},")
    if url and "url" not in entry_str.lower():
        additions.append(f"  url     = {{{url}}},")
    if not additions:
        return entry_str
    # Insert before last closing brace
    return entry_str.rstrip().rstrip("}") + "\n" + "\n".join(additions) + "\n}"

def main():
    text = BIB_PATH.read_text()
    entries = parse_entries(text)
    enriched = []
    hits = 0
    for i, (key, entry) in enumerate(entries):
        # Skip non-paper entries
        if "@misc" in entry[:10] and "private communication" in entry:
            enriched.append(entry)
            continue
        title = get_title(entry)
        author = get_author(entry)
        year = get_year(entry)
        if not title:
            enriched.append(entry)
            continue
        print(f"[{i+1}/{len(entries)}] {key}: {title[:60]}...")
        doi, url = query_semantic_scholar(title, author or "", year or "")
        if doi or url:
            entry = inject_fields(entry, doi, url)
            hits += 1
            print(f"  → doi={doi}  url={'yes' if url else 'no'}")
        else:
            print(f"  → not found")
        enriched.append(entry)
        time.sleep(RATE_SLEEP)

    BIB_PATH.write_text("\n\n".join(enriched) + "\n")
    print(f"\nEnriched {hits}/{len(entries)} entries.")

if __name__ == "__main__":
    main()

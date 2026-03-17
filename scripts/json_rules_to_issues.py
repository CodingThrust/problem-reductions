#!/usr/bin/env python3
"""Convert Garey & Johnson reduction JSONs to GitHub rule issue markdown.

Source: references/Garey&Johnson/reductions/*.json
Target: references/issues/rules/*.md

Each JSON has: id, from_problem, to_problem, source_location, text.
"""

import json
import re
import unicodedata
from pathlib import Path


# ---------------------------------------------------------------------------
# BibTeX helpers
# ---------------------------------------------------------------------------

def _extract_bib_fields(block: str) -> dict[str, str]:
    """Extract field=value pairs from a bib entry block, handling nested braces."""
    fields = {}
    for fm in re.finditer(r"(\w+)\s*=\s*\{", block):
        name = fm.group(1).lower()
        start = fm.end()  # position after the opening {
        depth = 1
        i = start
        while i < len(block) and depth > 0:
            if block[i] == "{":
                depth += 1
            elif block[i] == "}":
                depth -= 1
            i += 1
        fields[name] = block[start : i - 1].strip()
    return fields


def parse_bib(bib_path: Path) -> dict[str, dict]:
    """Parse a .bib file into {key: {field: value}} dict."""
    text = bib_path.read_text()
    entries = {}
    for block in re.split(r"\n\n(?=@)", text.strip()):
        m = re.match(r"@(\w+)\{([^,\n]+),", block)
        if not m:
            continue
        entry_type = m.group(1).lower()
        key = m.group(2).strip()
        fields = {"_type": entry_type, **_extract_bib_fields(block)}
        entries[key] = fields
    return entries


def parse_citation(citation: str) -> tuple[list[str], str] | None:
    """Parse '[Even, Itai, and Shamir, 1976]' into (['Even','Itai','Shamir'], '1976').

    Returns None if not a standard author-year citation.
    """
    raw = citation.strip("[]").strip()
    m = re.search(r",\s*(\d{4}[a-z]?|——)\s*$", raw)
    if not m:
        return None
    year = m.group(1)
    author_str = raw[: m.start()].strip()
    author_str = re.sub(r",\s*and\s+", " and ", author_str)
    parts = [p.strip() for p in re.split(r",\s*", author_str) if p.strip()]
    authors = []
    for part in parts:
        authors.extend(a.strip() for a in part.split(" and ") if a.strip())
    return authors, year


def citation_to_constructed_key(authors: list[str], year: str) -> str:
    """Build the multi-author bib key: 'Even and Itai and Shamir1976'."""
    return " and ".join(authors) + year


_SUFFIXES = re.compile(r",?\s+(?:jr\.?|sr\.?|i{2,3}v?|2nd|3rd)\s*$", re.I)
# LaTeX accents in two forms:
# Form 1: {\"o} or {\v{C}} — braces-wrapped
_LATEX_ACCENT1 = re.compile(r"\{\\[^a-zA-Z{}]*\{?([a-zA-Z]+)\}?\}")
# Form 2: \'{a} or \'a — bare backslash command
_LATEX_ACCENT2 = re.compile(r"\\[^a-zA-Z{}\s](?:\{([a-zA-Z]+)\}|([a-zA-Z]))")
_LATEX_MISC = re.compile(r"[{}\\]")


def _normalize_name(s: str) -> str:
    """Strip LaTeX markup, fold unicode diacritics to ASCII, lowercase."""
    s = _LATEX_ACCENT1.sub(r"\1", s)
    s = _LATEX_ACCENT2.sub(lambda m: m.group(1) or m.group(2), s)
    s = _LATEX_MISC.sub("", s)
    s = unicodedata.normalize("NFKD", s).encode("ascii", "ignore").decode()
    return s.lower().strip()


def bib_author_lastnames(author_field: str) -> list[str]:
    """Extract last names from a bib author field.

    Handles:
    - Suffixes: 'E. G. Coffman, Jr' → 'Coffman'
    - Compound surnames: 'Larry van Sickle' → 'van Sickle'
      (everything after the last initial 'X.' sequence)
    - LaTeX accents stripped for comparison.
    """
    names = []
    for person in author_field.split(" and "):
        person = _SUFFIXES.sub("", person.strip())
        parts = person.split()
        # Skip leading initials (like 'M.', 'R. L.') to get the surname
        # An initial is a 1-2 char token ending with '.' or a single letter
        i = 0
        while i < len(parts) - 1 and re.match(r"^[A-Z]\.?$", parts[i]):
            i += 1
        surname = " ".join(parts[i:])
        names.append(surname)
    return names


def citation_matches_bib_authors(cite_authors: list[str], bib_author_field: str) -> bool:
    """Return True if the citation author set matches the bib author field.

    Uses suffix matching so 'Van Sickle' matches 'Larry van Sickle',
    and ASCII-folds LaTeX/Unicode (so 'Erdos' matches 'Erd{\\"{o}}s').
    """
    bib_surnames = bib_author_lastnames(bib_author_field)
    if len(cite_authors) != len(bib_surnames):
        return False
    remaining = list(bib_surnames)
    for ca in cite_authors:
        ca_norm = _normalize_name(ca)
        for i, bs in enumerate(remaining):
            bs_norm = _normalize_name(bs)
            # Check if citation name matches end of bib surname (handles 'Van Sickle'→'Sickle')
            if bs_norm == ca_norm or bs_norm.endswith(ca_norm):
                remaining.pop(i)
                break
        else:
            return False
    return True


def find_citations(text: str) -> list[str]:
    """Return all unique '[Author(s), year]' citations found in text, in order."""
    seen: dict[str, bool] = {}
    for c in re.findall(r"\[[^\]]+,\s*(?:\d{4}[a-z]?|——)[^\]]*\]", text):
        if c not in seen:
            seen[c] = True
    return list(seen)


def lookup_bib(citation: str, bib: dict[str, dict]) -> tuple[str, dict] | tuple[None, None]:
    """Resolve a citation string to (bib_key, entry).

    Resolution order:
    1. Exact constructed key (e.g. 'Karp1972', 'Stockmeyer and Meyer1973').
    2. Constructed key + letter suffix (e.g. 'Itai1977' → 'Itai1977a').
    3. Author+year matching against actual bib author fields:
       compare author last-name sets and bare year.
       Return the match only if it is unique (prevents wrong guesses).
    """
    parsed = parse_citation(citation)
    if not parsed:
        return None, None
    cite_authors, cite_year = parsed
    cite_year_bare = re.sub(r"[a-z]$", "", cite_year)

    # 1. Exact constructed key
    key = citation_to_constructed_key(cite_authors, cite_year)
    if key in bib:
        return key, bib[key]

    # 2. Constructed key + letter suffix (for bare year without suffix)
    if cite_year == cite_year_bare:
        for suffix in "abcdefgh":
            candidate = key + suffix
            if candidate in bib:
                return candidate, bib[candidate]

    # 2.5. If citation year has a suffix (e.g. 1976c), try first_author + year_with_suffix
    #      and verify the author set matches — disambiguates 'Garey and Johnson, 1976c' → Garey1976c
    if cite_year != cite_year_bare and cite_authors:
        candidate = cite_authors[0] + cite_year
        if candidate in bib:
            entry = bib[candidate]
            if citation_matches_bib_authors(cite_authors, entry.get("author", "")):
                return candidate, entry

    # 3. Author+year matching: search all bib entries
    matches = []
    for bkey, entry in bib.items():
        if entry.get("year", "") != cite_year_bare:
            continue
        bib_author = entry.get("author", "")
        if not bib_author:
            continue
        if citation_matches_bib_authors(cite_authors, bib_author):
            matches.append((bkey, entry))

    if len(matches) == 1:
        return matches[0]

    # Deduplicate: the same paper may appear under multiple first-author keys,
    # possibly with authors listed in different order or with abbreviated vs full
    # first names. Use a frozenset of normalized last-words as the canonical identity.
    if len(matches) > 1:
        def canonical_authors(entry):
            people = [p.strip() for p in entry.get("author", "").split(" and ") if p.strip()]
            return frozenset(
                _normalize_name(p.split()[-1]) for p in people if p.split()
            )
        unique = {}
        for bkey, entry in matches:
            ck = canonical_authors(entry)
            if ck not in unique:
                unique[ck] = (bkey, entry)
        if len(unique) == 1:
            return next(iter(unique.values()))

    return None, None


def format_bib_entry(key: str, entry: dict) -> str:
    """Format a bib entry as a short human-readable citation string with its key."""
    def f(name):
        return entry.get(name, "")

    author = f("author")
    year = f("year") or "n.d."
    title = f("title")
    etype = entry.get("_type", "")

    parts = [f"[`{key}`] {author} ({year}). \"{title}\"."]

    if etype == "article":
        journal = f("journal")
        vol = f("volume")
        num = f("number")
        pages = f("pages").replace("--", "–")
        loc = f"*{journal}*"
        if vol:
            loc += f" {vol}"
        if num:
            loc += f"({num})"
        if pages:
            loc += f", pp. {pages}"
        parts.append(loc + ".")
    elif etype == "inproceedings":
        book = f("booktitle")
        pages = f("pages").replace("--", "–")
        pub = f("publisher")
        loc = f"In: *{book}*"
        if pages:
            loc += f", pp. {pages}"
        if pub:
            loc += f". {pub}"
        parts.append(loc + ".")
    elif etype == "book":
        pub = f("publisher")
        addr = f("address")
        loc = pub
        if addr:
            loc += f", {addr}"
        if loc:
            parts.append(loc + ".")
    elif etype in ("techreport", "misc", "phdthesis", "mastersthesis", "incollection"):
        institution = f("institution") or f("school") or f("publisher")
        booktitle = f("booktitle")
        if booktitle:
            parts.append(f"In: *{booktitle}*.")
        if institution:
            parts.append(institution + ".")

    return " ".join(parts)


def build_references_section(text: str, bib: dict[str, dict]) -> list[str]:
    """Find all citations in text, look up in bib, return markdown lines."""
    citations = find_citations(text)
    if not citations:
        return []

    lines = ["", "## References", ""]
    for cite in citations:
        key, entry = lookup_bib(cite, bib)
        if entry:
            lines.append(f"- **{cite}**: {format_bib_entry(key, entry)}")
        else:
            lines.append(f"- **{cite}**: *(not found in bibliography)*")
    return lines


# ---------------------------------------------------------------------------
# Problem name helpers
# ---------------------------------------------------------------------------

def to_rust_name(problem_name: str) -> str:
    """Convert 'VERTEX COVER' -> 'VertexCover', '3-SAT (3SAT)' -> '3Sat'.

    Steps:
    1. Strip parenthetical abbreviations like (3SAT) or (SAT).
    2. Split on whitespace and hyphens.
    3. Capitalize each token and join.
    """
    name = re.sub(r"\s*\(.*?\)", "", problem_name).strip()
    return "".join(w.capitalize() for w in re.split(r"[\s\-]+", name) if w)


def extract_reference_line(text: str) -> str:
    """Extract the 'Reference: ...' line from text, or return empty string."""
    for line in text.split("\n"):
        if re.match(r"\s*Reference:", line):
            return line.strip()
    return ""


def is_appendix_entry(text: str) -> bool:
    """Return True if text is a GJ appendix entry (starts with [CODE])."""
    return bool(re.match(r"^\[", text.strip()))


def blockquote(text: str) -> str:
    """Prefix every line with '> ' for Markdown blockquote."""
    return "\n".join("> " + line if line.strip() else ">" for line in text.splitlines())


# ---------------------------------------------------------------------------
# Issue generation
# ---------------------------------------------------------------------------

def rule_json_to_md(data: dict, bib: dict[str, dict]) -> str:
    from_problem = data.get("from_problem", "")
    to_problem = data.get("to_problem", "")
    source_loc = data.get("source_location", "")
    text = data.get("text", "")

    # Build reference field
    reference = f"Garey & Johnson, *Computers and Intractability*, {source_loc}"

    header = [
        "---",
        "name: Rule",
        "about: Propose a new reduction rule",
        f'title: "[Rule] {from_problem} to {to_problem}"',
        "labels: rule",
        "assignees: ''",
        "---",
        "",
        f"**Source:** {from_problem}",
        f"**Target:** {to_problem}",
        "**Motivation:** (TBD)",
        f"**Reference:** {reference}",
        "",
    ]

    footer = [
        "",
        "## Size Overhead",
        "",
        "| Target metric (code name) | Polynomial (using symbols above) |",
        "|----------------------------|----------------------------------|",
        "| (TBD) | (TBD) |",
        "",
        "## Validation Method",
        "",
        "(TBD)",
        "",
        "## Example",
        "",
        "(TBD)",
        "",
    ]

    if is_appendix_entry(text):
        # Appendix entry: problem definition + comments, no reduction algorithm.
        # Quote the full source text faithfully (Reference line stays in blockquote).
        body = [
            "## GJ Source Entry",
            "",
            blockquote(text),
            "",
            "## Reduction Algorithm",
            "",
            "(TBD)",
        ]
    else:
        # Theorem/proof entry: the text IS the reduction construction.
        body = [
            "## Reduction Algorithm",
            "",
            blockquote(text),
        ]

    refs = build_references_section(text, bib)

    return "\n".join(header + body + footer + refs)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    base = Path(__file__).parent.parent / "references"
    src_dir = base / "Garey&Johnson" / "reductions"
    bib_path = base / "Garey&Johnson" / "bibliography.bib"
    out_dir = base / "issues" / "rules"
    out_dir.mkdir(parents=True, exist_ok=True)

    bib = parse_bib(bib_path)
    print(f"Loaded {len(bib)} bib entries from {bib_path.name}")

    count = 0
    for f in sorted(src_dir.glob("*.json")):
        with open(f) as fp:
            data = json.load(fp)
        md = rule_json_to_md(data, bib)
        out_file = out_dir / f"{f.stem}.md"
        out_file.write_text(md)
        count += 1

    print(f"Converted {count} rule JSONs → {out_dir}")


if __name__ == "__main__":
    main()

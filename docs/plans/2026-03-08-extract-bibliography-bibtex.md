# Extract Garey & Johnson Bibliography as BibTeX

> **For Claude:** Use `superpowers:dispatching-parallel-agents` for the extraction phase. Use `superpowers:executing-plans` for enrichment and finalization.

**Goal:** Extract the full "Reference and Author Index" from the Garey & Johnson PDF into `references/Garey&Johnson/bibliography.bib`, then enrich every entry with `doi` and `url` fields via Semantic Scholar API + web search fallback.

**Status:** Fresh start — `bibliography.bib` does not exist yet. `scripts/enrich_bibtex.py` and `scripts/finalize_bibtex.py` already exist.

---

## Source Facts

- **PDF path:** `Computers and intractability  a guide to the theory of NP-completeness.pdf`
- **Bibliography location:** PDF pages **302–336** (book pages 291–325)
- **Page offset:** PDF page = book page + 11
- **First entry:** ABDEL-WAHAB, H. M. [1976] (PDF p302)
- **Last entry:** ZALCSTEIN, Y. *See* LIPTON., R. J. (PDF p336, book p325)
- **Estimated entries:** 400–500
- **"See also" lines:** skip — these are cross-references, not citable entries

---

## BibTeX Key Convention

Use `LastNameYear[letter]` — e.g. `Gavril1977b`, `Karp1972`.
Multi-author: first author only — `GareyJohnson1979`.
No spaces or commas in keys.

## Entry Type Mapping

| Book format | BibTeX type |
|-------------|-------------|
| `Journal Vol, pages` | `@article` |
| `Proc. / Ann. Symp.` | `@inproceedings` |
| `Doctoral Thesis` | `@phdthesis` |
| `Technical Report` / `Report No.` | `@techreport` |
| `unpublished manuscript` / `private communication` | `@misc` |
| Standalone book title (publisher, city) | `@book` |
| Chapter `in X (ed.)` | `@incollection` |

## Field Population Rules

- `author`: "Firstname Lastname" format, `and`-joined
- `year`: strip the letter suffix from `[1977b]`
- `title`: the quoted title string (in `{curly braces}` to preserve capitalization)
- `journal` / `booktitle` / `publisher` / `school` / `institution`: as appropriate
- `volume`, `pages`, `number`: from the citation text
- `pages`: always `180--187` (double dash)
- `doi`: added by enrichment step
- `url`: open-access PDF or publisher page, added by enrichment step
- `note`: for `unpublished`, `private communication`, `to appear`, or errata info
- Conference booktitles: expand abbreviations — `Proc. 8th Ann. ACM Symp. on Theory of Computing` → `booktitle = {Proceedings of the 8th Annual ACM Symposium on Theory of Computing}`

## Hard Cases

- **"See also X"** — skip entirely (cross-reference, not a publication)
- **private communication** — `@misc{Author, Year, author = {...}, year = {YYYY}, note = {private communication}}`
- **unpublished manuscript** — `@misc` with `note = {unpublished manuscript}`
- **"to appear"** — include journal name, add `note = {to appear}`
- **Errata** — fold into `note` field
- **No title** (e.g., "See JONES, N. D.") — skip entirely

---

## Phase 1: Parallel Extraction (6 subagents)

Split the 35-page bibliography into 6 chunks. Each subagent reads its pages from the PDF, extracts all BibTeX entries, and writes to a separate temp file.

| Agent | PDF pages | Book pages | Approx. range | Output file |
|-------|-----------|------------|---------------|-------------|
| 1 | 302–307 | 291–296 | ABDEL-WAHAB → COOK | `/tmp/gj_bib_01.bib` |
| 2 | 308–313 | 297–302 | COOK → GAREY [1974] | `/tmp/gj_bib_02.bib` |
| 3 | 314–319 | 303–308 | GAREY [1975] → IBARRA | `/tmp/gj_bib_03.bib` |
| 4 | 320–325 | 309–314 | IBARAKI → LYNCH | `/tmp/gj_bib_04.bib` |
| 5 | 326–331 | 315–320 | MAHESHWARI → SAHNI | `/tmp/gj_bib_05.bib` |
| 6 | 332–336 | 321–325 | SAHNI → ZALCSTEIN | `/tmp/gj_bib_06.bib` |

**Each subagent instruction template:**

```
Read PDF pages {START}–{END} from "Computers and intractability  a guide to the theory of NP-completeness.pdf".
Extract every bibliographic entry as BibTeX. Skip "See also" cross-reference lines.
Follow these conventions: [paste BibTeX Key Convention + Entry Type Mapping + Field Population Rules from plan].
Write all entries to {OUTPUT_FILE}. Do NOT include entries that start on a prior page chunk.
If an entry spans from the previous page chunk, include it in full.
```

**After all 6 complete:** Concatenate to `references/Garey&Johnson/bibliography.bib`:

```bash
cat /tmp/gj_bib_01.bib /tmp/gj_bib_02.bib /tmp/gj_bib_03.bib \
    /tmp/gj_bib_04.bib /tmp/gj_bib_05.bib /tmp/gj_bib_06.bib \
    > references/Garey&Johnson/bibliography.bib
```

**Commit:**
```bash
git add references/Garey&Johnson/bibliography.bib
git commit -m "chore: extract Garey & Johnson bibliography (~450 entries)"
```

---

## Phase 2: Enrich with Semantic Scholar API

**Step 1:** Run the existing enrichment script:
```bash
cd /Users/xiweipan/Codes/problem-reductions && uv run scripts/enrich_bibtex.py
```

This queries `https://api.semanticscholar.org/graph/v1/paper/search` for each entry (1 req/s), injecting `doi` and `url` fields where found. Expected runtime: ~8–10 minutes for ~450 entries.

**Step 2:** Review misses — web search fallback for high-importance entries:
- `{Karp, 1972}` — Reducibility among combinatorial problems
- `{Cook, 1971a}` — The complexity of theorem proving procedures
- `{Levin, 1973}` — Universal sequential search problems
- Any entry referenced by existing JSON models in `references/Garey&Johnson/`

**Step 3:** Commit:
```bash
git add references/Garey&Johnson/bibliography.bib
git commit -m "chore: enrich bibliography with DOI/URL via Semantic Scholar"
```

---

## Phase 3: Deduplicate, Sort, Validate

**Step 1:** Run finalization script:
```bash
uv run scripts/finalize_bibtex.py
```

**Step 2:** Verify:
```bash
grep -c '^@' references/Garey&Johnson/bibliography.bib   # expect 400–500
grep -c 'doi\s*=' references/Garey&Johnson/bibliography.bib  # aim >60%
```

**Step 3:** Spot-check key entries:
```bash
grep -A8 '{Karp, 1972' references/Garey&Johnson/bibliography.bib
grep -A8 '{Cook, 1971' references/Garey&Johnson/bibliography.bib
```

**Step 4:** Final commit:
```bash
git add references/Garey&Johnson/bibliography.bib
git commit -m "feat: complete Garey & Johnson bibliography with DOI/URL enrichment"
```

---
name: update-papers
description: Use when the research paper collection in docs/research/ needs refreshing — fetches PDFs for new references.bib entries, retries missing ones, regenerates the index, and syncs PDFs to the shared rclone remote
---

# Update Papers

PDFs for `docs/paper/references.bib` live in `docs/research/raw/`, tracked by
`docs/research/manifest.json`; `docs/research/index.md` cross-references them against
`docs/paper/reductions.typ`. Scripts: `scripts/fetch_papers.py`, `scripts/gen_paper_index.py`.

Requires `rclone` with a `gdrive` remote and `PAPERS_REMOTE` set (e.g.
`export PAPERS_REMOTE=gdrive:problemreductions-papers`) for the push step.

## Flow

```bash
make papers          # lookup (arXiv/OA via Semantic Scholar) -> download -> scihub -> status
make papers-index    # regenerate docs/research/index.md
make papers-push     # upload new/changed PDFs + manifest to $PAPERS_REMOTE
```

All steps are incremental and idempotent; OA 403s are normal and fall through to Sci-Hub.

## Leftovers

After `make papers`, check `make papers-status` for missing entries. For each one, search
`"<title>" <first author> pdf`: author homepages, arXiv by title, and open-access fallbacks
(LIPIcs/Dagstuhl, HAL, ECCC, IACR ePrint). Download with
`curl -L -o docs/research/raw/<bibkey>.pdf '<url>'` and confirm with `file` that it is a PDF,
not an HTML paywall page. Rerun `make papers-index` and `make papers-push` afterwards.

Skip textbooks; they have no single PDF: garey1979, sipser2012, cormen2022, conway1967.

## Troubleshooting

- All Sci-Hub mirrors fail: retry later or update `SCIHUB_DOMAINS` in `scripts/fetch_papers.py`.
- rclone auth expired: `rclone config reconnect gdrive:`.
- Stale manifest: delete `docs/research/manifest.json` and rerun `make papers`; PDFs on disk are
  kept.
- New bib entry ignored: it must parse as `@type{key, ...}` with `title` and ideally `doi`.

Report: newly downloaded, still missing (and why), and whether the push succeeded.

---
name: how-to-ship
description: Use when turning a GitHub issue into a pull request, opening or updating a PR, fixing CI failures, review comments or codecov gaps on a PR, resolving merge conflicts with main, or preparing a PR for a human to merge.
---

# How to Ship

Carry one item from issue to a merge-ready PR. Implementation itself follows how-to-code,
how-to-verify, and how-to-write-manual; this guide covers the gates and GitHub mechanics around it.
A human merges — never run `gh pr merge`. Never force push (`--force`, `-f`, `--force-with-lease`).

```bash
REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner)
```

## Issue → PR

1. Preflight once and reuse the JSON:
   `python3 scripts/pipeline_checks.py issue-context --repo "$REPO" --issue N --format json`.
   It returns title/body/labels/comments, `kind`, `source_problem`/`target_problem`,
   `checks.{good_label,source_model,target_model}`, `existing_prs`, `resume_pr`, `action`.
2. Refuse to start when:
   - `checks.good_label` fails — the issue has not passed triage; use how-to-triage-issue.
   - a `[Rule]`'s source or target model is missing on main — post
     `gh issue comment N --body "Blocked: model <Name> does not exist on main yet. Implement it first (or file a [Model] issue)."`
     and stop. Never implement the missing model inside the rule PR.
3. Read every issue comment: maintainer and contributor comments override the body.
4. Branch: if `action == "resume-pr"` and `resume_pr.head_ref_name` contains `issue-N`, check out
   that branch and continue the existing PR. Otherwise branch from fresh `origin/main` with a name
   containing `issue-N` (e.g. `issue-N-<slug>`).
5. Plan in an uncommitted working note (scratch dir); never commit plan files or add `docs/plans/`.
6. Implement. Scope is one item per PR. Only exception: a `[Model]` issue that explicitly claims
   direct ILP solvability ships the model and its `<Model> -> ILP` rule together, with the rule held
   to the full production bar (exact transform, closed-loop/infeasible tests, example, paper entry).
7. Gate locally: `make check && make paper`. `make paper` regenerates ignored exports — check
   `git status --short` and never stage `docs/src/reductions/*.json` or `docs/paper/data/`.
8. Push and open the PR titled `Fix #N: <issue title>` with `Fixes #N` in the body (the body ends
   with the attribution line from the session instructions):
   `python3 scripts/pipeline_pr.py create --repo "$REPO" --title "Fix #N: ..." --body-file body.md --base main --head <branch>`.
9. Mandatory review: dispatch a fresh-context subagent told to follow how-to-review for this PR; it
   posts `## Agentic Review Report`. Fix every Critical and Important finding, then continue below.

## Fixing feedback

Start from one packet: `python3 scripts/pipeline_pr.py context --repo "$REPO" --pr "$PR" --format text`
(`--format json` for raw arrays). Triage in this order:

1. **CI failures** — `pipeline_pr.py ci`; reproduce with `make clippy` / `make test` / `make fmt-check`.
2. **Review comments** — check all five sources, listing each explicitly:
   `inline_comments` (user inline comments are the most often missed), `reviews`,
   `human_issue_comments` (PR conversation), `human_linked_issue_comments`, `codecov_comments`.
   Evaluate each on its merits: apply correct suggestions, apply the spirit of partial ones, and
   explain in the reply why a wrong one was not applied.
3. **Coverage gaps** — `python3 scripts/pipeline_pr.py codecov --repo "$REPO" --pr "$PR"` lists
   patch coverage and files; read the uncovered paths and add tests for them. For PR gaps, trust
   this report over rerunning `make coverage`.

Then `make check` (plus `make paper` if the paper or examples changed), commit, push, and reply on
each addressed inline thread saying what changed and in which commit:
`gh api repos/$REPO/pulls/$PR/comments/<comment_id>/replies -f body="..."`
(ids from `gh api repos/$REPO/pulls/$PR/comments`). Answer non-inline feedback with one PR comment
via `pipeline_pr.py comment`.

## Merging main

`git fetch origin && git merge origin/main` (merge, not rebase — no history rewrites). Conflicts are
almost always both sides appending to ordered lists in `mod.rs`, `lib.rs`, `create.rs`,
`dispatch.rs`, or `reductions.typ`: keep both entries in order. After a `.bib` conflict, check for
duplicate keys:

```bash
grep '^@' docs/paper/references.bib | sed 's/@[a-z]*{//; s/,$//' | sort | uniq -d
```

After merging main, remove any manual dispatch arm the registry now makes redundant, and re-run
`make check && make paper` before pushing.

## Merge-ready

1. Local gate passes and the branch is pushed.
2. CI: `python3 scripts/pipeline_pr.py wait-ci --repo "$REPO" --pr "$PR"` (defaults: 900 s timeout,
   30 s interval). Check once or twice at most; if still pending, report "CI pending, local checks
   pass" instead of babysitting. Fix and push if it fails.
3. No open Critical/Important findings and no unanswered reviewer comments.
4. Approve: `gh pr review "$PR" --approve` (fails harmlessly when you authored the PR).
5. Post the community-call checklist on the linked issue (on the PR if there is none), with real
   problem names and PR number substituted:

   ```markdown
   Please kindly check the following items (PR #<PR>):
   - [ ] **Paper** ([PDF](https://github.com/CodingThrust/problem-reductions/blob/main/docs/paper/reductions.pdf)): check definition, proof sketch, example figure, and reproducible `pred` commands
   - [ ] **Implementation (Optional)**: spot-check the source files changed in this PR for correctness

   Join the discussion on [Zulip](https://julialang.zulipchat.com/#narrow/channel/365542-problem-reductions) — feel free to ask questions or leave feedback there.
   ```

6. Hand the PR URL to the human to merge. Record any deferred follow-ups as a PR comment.

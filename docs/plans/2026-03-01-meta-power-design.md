# Design: meta-power skill

## Purpose

Batch-resolve open `[Model]` and `[Rule]` GitHub issues end-to-end with full autonomy: plan, implement, review, fix, merge.

## Architecture

**Outer orchestrator** pattern: meta-power runs in the main Claude session and shells out to `make run-plan` for each issue's implementation. This keeps the orchestrator's context clean while delegating heavy work to subprocess sessions.

## Pipeline per Issue

```
Phase 1: Plan         /issue-to-pr <number>  → branch + PR with plan
Phase 2: Execute      make run-plan          → subprocess implements the plan
Phase 3: Review       push, make copilot-review
Phase 4: Fix loop     (up to 3 retries)
                        sleep 5m → /fix-pr → push → sleep 5m → check CI
                        if CI green → break
Phase 5: Merge        gh pr merge --squash
Phase 6: Sync         git checkout main && git pull
```

## Ordering

1. All `[Model]` issues first (ascending issue number)
2. All `[Rule]` issues second (ascending issue number)

No DAG — models-first is sufficient since rules depend on models.

## Error Handling

Every failure → log + skip to next issue. Never block the batch.

| Phase | Failure | Action |
|-------|---------|--------|
| Plan | Validation fails | Skip |
| Execute | Subprocess exits non-zero | Skip |
| Fix loop | 3 retries exhausted | Leave PR open, skip |
| Merge | Conflict | Leave PR open, skip |

## Parameters

- `MAX_RETRIES = 3`
- `CI_WAIT = 5 minutes`
- Auto-merge: yes (squash)
- Summary table printed at end

## Design Decisions

- **Why outer orchestrator?** Each `make run-plan` gets a fresh 500-turn context. The outer session just monitors and coordinates.
- **Why models-first only?** Rules rarely depend on each other. If a rule's source model is missing, `issue-to-pr` validation catches it and skips.
- **Why 3 retries?** Most fixable issues resolve in 1-2 rounds. More retries burn tokens on genuinely hard problems.
- **Why auto-merge?** Full CI + Copilot review provides sufficient quality gate. The point of the skill is batch autonomy.

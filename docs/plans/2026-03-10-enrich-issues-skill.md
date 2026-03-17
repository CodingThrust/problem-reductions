# Enrich-Issues Skill Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create a skill that fills TBD sections in `references/issues(fixed)/` rule and model files using web search + literature, writing completed versions to `references/issues/`.

**Architecture:** Single skill file (`enrich-issues/SKILL.md`) that orchestrates a 3-phase pipeline: scan & classify → parallel enrichment via subagents → update GitHub issue #183. Each invocation takes a rule range (e.g., "R01-R24").

**Tech Stack:** Claude Code skills, Agent tool (parallel subagents), WebSearch, GitHub CLI (`gh`)

---

### Task 1: Create the skill file

**Files:**
- Create: `.claude/skills/enrich-issues/SKILL.md`

**Step 1: Write the skill file**

See full content in the step below. The skill is a single markdown file with the complete workflow.

**Step 2: Register in CLAUDE.md**

Modify: `.claude/CLAUDE.md` — add to the Skills section:
```
- [enrich-issues](skills/enrich-issues/SKILL.md) -- Fill TBD sections in extracted G&J issue files using web search + literature. Takes a rule range, checks codebase for existing implementations, enriches missing content, and updates tracking issue #183.
```

**Step 3: Commit**

```bash
git add .claude/skills/enrich-issues/SKILL.md .claude/CLAUDE.md
git commit -m "feat: add enrich-issues skill for filling TBD sections in G&J issue files"
```

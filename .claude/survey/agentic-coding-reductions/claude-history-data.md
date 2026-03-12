# Claude History Data for Paper

Raw metrics extracted from `~/.claude` on 2026-03-13.

## Global Claude Code Stats (Jan 14 – Feb 25, 2026)

| Metric | Value |
|--------|-------|
| Days active | 40 |
| Total messages | 157,325 |
| Total sessions | 329 |
| Total tool calls | 36,553 |
| Avg messages/day | 3,933 |
| Avg sessions/day | 8.2 |
| Peak messages/day | 18,224 (Feb 16) |
| Peak sessions/day | 32 (Feb 13) |
| Peak tool calls/day | 5,234 (Jan 28) |

## Problemreductions Project Stats

### Session Data
| Metric | Value |
|--------|-------|
| Session transcript files | 283 |
| Total transcript data | 300 MB |
| Largest session | 12.5 MB |
| Median session | 457 KB |
| Sessions > 1 MB | 79 |
| Sessions > 5 MB | 12 |

### From Session Metadata (108 sessions with timing)
| Metric | Value |
|--------|-------|
| Total wall-clock time | 6,897 min (115 hours) |
| User messages | 630 |
| Assistant messages | 9,429 |
| Automation ratio (asst/user) | 15.0x |
| Avg user msgs/session | 5.8 |
| Git commits (from sessions) | 140 |
| Git pushes | 88 |
| Commits per hour | 1.2 |
| Tool calls per session | 51 |
| Input tokens | 255,954 |
| Output tokens | 732,996 |

### Tool Usage (across 108 measured sessions)
| Tool | Count |
|------|-------|
| Bash | 1,661 |
| Read | 1,284 |
| Grep | 629 |
| Edit | 595 |
| Task | 272 |
| TaskUpdate | 245 |
| TodoWrite | 161 |
| AskUserQuestion | 151 |
| TaskCreate | 133 |
| Glob | 100 |
| Skill | 97 |
| Write | 81 |
| WebFetch | 34 |
| WebSearch | 34 |

### Languages Touched
| Language | File operations |
|----------|----------------|
| Rust | 1,239 |
| Markdown | 431 |
| JavaScript | 55 |
| JSON | 37 |
| YAML | 10 |

## Git History

### Commits
| Metric | Value |
|--------|-------|
| Total commits (main) | 253 |
| Commits (all branches) | 1,089 |
| Co-Authored-By: Claude commits | 1,510 |
| Contributors | 4 (GiggleLiu, Jinguo Liu, Shiwen An, Xiwei Pan) |
| Merged PRs | 59 |
| Fix # PRs (issue-driven) | 10 |
| feat: PRs | 16 |
| Project start date | 2026-01-09 |

### Codebase Growth Timeline
| Date | Models | Rules | Test files | Examples | Rust files |
|------|--------|-------|------------|----------|------------|
| Jan 10 (initial) | 17 | 0 | 0 | 0 | 36 |
| Jan 26 (feature parity) | 20 | 22 | 0 | 1 | ~74 |
| Feb 1 | 20 | 24 | 0 | 1 | 74 |
| Feb 15 (arch redesign) | 21 | 44 | 101 | 35 | 204 |
| Mar 1 | 23 | 51 | 105 | 42 | 218 |
| Mar 13 (current) | 27 | 50 | 114 | 45 | 232 |

### Current Project Size
| Component | Count/Size |
|-----------|------------|
| Rust source (src/) | 54,599 LOC |
| Test files (src/unit_tests + tests/) | 28,343 LOC |
| Examples | 6,362 LOC |
| Skill files | 3,664 LOC |
| CLAUDE.md | 253 lines |
| Models | 27 |
| Rules | 50 |
| Examples | 45 |
| Skills | 14 |

### Peak Development Days
| Date | Commits | Sessions | Messages | Tool calls | Key activity |
|------|---------|----------|----------|------------|--------------|
| Jan 25 | 41 | 22 | 12,868 | 3,734 | Feature parity sprint (Julia port) |
| Jan 28 | ~30 | 3 | 18,055 | 5,234 | UnitDiskMapping gadgets |
| Feb 12 | 26 | 82 | 4,540 | 1,508 | Overhead expression system began |
| Feb 13 | 61 | 43 | 13,169 | 2,529 | Variant system, MIS redesign |
| Feb 14 | 67 | 16 | 10,454 | 1,885 | Circuit reductions |
| Feb 15 | 40 | 13 | 4,526 | 783 | Expression system migration |
| Feb 16 | 69 | 3 | 18,224 | 2,490 | problem_size trait, graph export |
| Mar 12 | 113 | 26 | N/A | N/A | Pipeline automation, 6 PRs merged |

## GitHub Issues

### Overall
| Metric | Value |
|--------|-------|
| Total issues | 500+ |
| Open | 350 |
| Closed | 150 |
| Rule issues | 271 |
| Model issues | 183 |

### Issue Authors
| Author | Issues |
|--------|--------|
| isPANN | 414 |
| GiggleLiu | 34 |
| zazabap | 28 |
| QingyunQian | 19 |
| hmyuuu | 4 |
| fliingelephant | 2 |
| exAClior | 1 |

### Peak Issue Creation Days
| Date | Issues |
|------|--------|
| Mar 11 | 251 |
| Mar 12 | 78 |
| Mar 10 | 38 |
| Mar 9 | 26 |

### Quality Gate Results (322 checked of isPANN's 414)
| Verdict | Count | Percentage |
|---------|-------|------------|
| Good | 81 | 25% |
| PoorWritten | 124 | 39% |
| Wrong | 64 | 20% |
| Trivial | 43 | 13% |
| Useless | 18 | 6% |
| **Rejection rate** | **241/322** | **75%** |

### All Issues Quality Check (all authors)
| Verdict | Count |
|---------|-------|
| Good | 105 |
| PoorWritten | 138 |
| Wrong | 64 |
| Trivial | 45 |
| Useless | 19 |
| Total checked | 371 |

## Skill Invocations (from history.jsonl)
| Skill | Count |
|-------|-------|
| /compact | 33 |
| /superpowers:brainstorm | 15 |
| /mcp | 7 |
| /fix-pr | 5 |
| /passes | 4 |
| /model | 4 |
| /superpowers:execute-plan | 3 |
| /test-feature | 3 |
| /check-rule-redundancy | 3 |
| /review-pipeline | 2 |
| /review-implementation | 2 |
| /writing-plans | 2 |

## Prompt Length Distribution (2,196 prompts)
| Category | Count | Percentage |
|----------|-------|------------|
| 1–3 words | 650 | 30% |
| 4–10 words | 1,038 | 47% |
| 11–30 words | 592 | 27% |
| 30+ words | 79 | 4% |

## User Prompt Evolution Examples

### Phase 1 (Jan 9, Manual)
```
"start implementing milestone 1"
"improve test coverage to >95 and start milestone 3"
"detect missing tests compared with Julia package."
"compare your implementation with UnitDiskMapping, do not skip any test"
"incorrect, it is King's subgraph!"
```

### Phase 2 (Jan 26 – Feb, Basic Skills)
```
"/superpowers:brainstorm check issue 10 and 11"
"implement Satisfiability -> Maximum Independent Set reduction"
"resolve pr comments, fix ci"
"commit this in a pr"
```

### Phase 3 (Mar, Full Pipeline)
```
"make run-pipeline"
"/review-pipeline"
"/check-rule-redundancy"
"make run-issue N=570"
```

## All Projects by Usage (top 10)
| Project | Prompts |
|---------|---------|
| problemreductions | 2,346 |
| cryochamber | 582 |
| sci-brainstorm | 329 |
| DSAA3071TheoryOfComputation | 226 |
| omeinsum-rs | 197 |
| BPDecoderPlus | 157 |
| private-note | 154 |
| agentic-tests | 153 |
| dev | 130 |
| yao-rs | 127 |

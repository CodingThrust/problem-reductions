---
name: dev-setup
description: Use when a new maintainer or contributor needs a working development environment for this repo — detects missing tools, installs them after one confirmation, authenticates gh, and verifies with make check
---

# Dev Setup

Get a machine to the point where `make check` passes and the contributor tools work.
The tool list lives in `.claude/skills/dev-setup/dependencies.md` (core, contributor workflow,
optional).

## Flow

1. **Detect.** `uname -s` selects the macOS or Linux install commands (on Linux also check the package
   manager; the table assumes apt). Run every check command in `dependencies.md` and collect
   what is missing.
2. **Confirm once.** Show one list of missing tools with the exact install command for each,
   grouped by tier, and mark which need `sudo`. Ask whether to install all, core only, or a
   subset. Never run `sudo` or a `curl | sh` installer before this confirmation.
3. **Install** the approved set without further prompts, then re-run the checks and report
   anything that still fails.
4. **Auth.** If `gh` is in scope: `gh auth status`, else `gh auth login`; then
   `gh repo view CodingThrust/problem-reductions --json name` to confirm access.
5. **Verify.** `make check` (fmt-check + clippy + test). If contributor tools were installed,
   also `pred list | head -3` and `pred-sym big-o "2^n"`.

## Non-obvious failures

| Symptom | Cause / fix |
|---------|-------------|
| `highs-sys` build fails: cmake not found, C++ errors | `highs` is an unconditional dependency built from source; needs cmake and a C/C++ toolchain |
| `highs-sys`: "Unable to find libclang" | bindgen needs libclang (`libclang-dev`; on macOS the Xcode CLT) |
| `make fmt-check` fails | `make fmt` |
| `make doc` fails at `npm ci` | node/npm missing or too old; CI uses Node 22 |
| `make coverage` missing llvm-profdata | `rustup component add llvm-tools-preview` |
| `pred` behaves differently from the docs | stale install; rerun `make cli` (reinstalls from the workspace) |

Finish with a short summary: installed, already present, skipped, and the `make check` result.

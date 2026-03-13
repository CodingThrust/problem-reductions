---
name: dev-setup
description: Interactive wizard to install and configure all development tools for new maintainers
---

# Dev Setup

Interactive wizard that helps new maintainers install and configure all tools needed for the problemreductions project.

## Step 1: Dependencies Checklist

Check if `skills/dev-setup/dependencies.md` exists and has content.

- **If it exists**, ask the user:
  > "Found existing dependencies checklist. Use it as-is, or rescan project files for changes?"
  - **Use existing** → read `dependencies.md` and proceed to Step 2
  - **Rescan** → scan project files (see Scan Targets below), overwrite `dependencies.md`, then proceed

- **If it does not exist**, scan project files and generate `dependencies.md` with the format shown in the existing file. Then proceed.

### Scan Targets

When scanning, read these files for tool references:

- `Makefile` — tool invocations (cargo, mdbook, typst, uv, julia, python3, jq, gh, claude)
- `.claude/skills/*/SKILL.md` — CLI references (gh, git, make, cargo, claude, pred)
- `.github/workflows/*.yml` — installed tools, rustup components, and actions
- `scripts/pyproject.toml` — Python tooling (uv)
- `scripts/jl/Project.toml` — Julia dependency
- `Cargo.toml` / `problemreductions-cli/Cargo.toml` — feature flags and build deps

Organize tools into three tiers in `dependencies.md`:
- **Core** — needed to build, test, and generate docs
- **Skill** — needed for the AI-assisted pipeline (gh, claude, pred, copilot-review)
- **Optional** — nice to have but not required (julia)

Each tool needs: name, check command, install command (macOS), install command (Linux), purpose.

## Step 2: Detect Platform

```bash
uname -s
```

- `Darwin` → use macOS install commands
- `Linux` → use Linux install commands

## Step 3: Install Core Tools

For each tool in the **Core Tools** table of `dependencies.md`:

1. Run the check command
2. **If found** → print `[tool] installed` and continue
3. **If missing** → print the install command for the detected platform, then execute it

After all core tools are done, ask:
> "Core tools are installed. Do you also want to set up the AI pipeline tools (gh, claude, pred, copilot-review)?"

- **Yes** → proceed to Step 4
- **No** → skip to Step 6

## Step 4: Install Skill Tools

For each tool in the **Skill Tools** table:

1. Run the check command
2. **If found** → print `[tool] installed` and continue
3. **If missing** → print the install command, then execute it

Note: `pred` is built from the local workspace. Use `cargo install --path problemreductions-cli`.

After skill tools, ask:
> "Want to install optional tools (julia)?"

- **Yes** → install optional tools using the same check/install pattern
- **No** → continue

## Step 5: Auth and Configuration

Skip this step if the user declined skill tools in Step 3.

### 5a: GitHub CLI auth

```bash
gh auth status
```

If not authenticated, run `gh auth login`.

### 5b: Repo access

```bash
gh repo view --json name
```

If this fails, the user needs repo access. Explain how to request it.

### 5c: Project board access

```bash
gh project list --owner <org-or-user>
```

If this fails with permission errors, run:
```bash
gh auth refresh -s read:project,project
```

Explain that the `project-pipeline` and `review-pipeline` skills require these OAuth scopes.

## Step 6: Verification

Run the full check:

```bash
make check
```

This runs `fmt-check + clippy + test`. Print a pass/fail summary for each stage.

### Troubleshooting Common Failures

| Failure | Fix |
|---------|-----|
| `fmt-check` fails | Run `make fmt` to auto-fix |
| Linker errors in clippy/test | Missing C/C++ toolchain for `ilp-highs` feature. Install Xcode CLT (`xcode-select --install` on macOS) or `build-essential` (`sudo apt install build-essential` on Linux) |
| "HiGHS not found" or cmake errors | Install cmake: `brew install cmake` (macOS) or `sudo apt install cmake` (Linux) |
| `cargo llvm-cov` fails with "missing llvm-profdata" | `rustup component add llvm-tools-preview` |

If `make check` passes and the user declined skill tools, print:
> "Setup complete! All core tools installed and verified. You're ready to contribute."

If it fails, walk through the troubleshooting table and offer to run the fix commands.

### Pipeline Verification (skill tier only)

If the user installed skill tools, also verify the autonomous pipeline works:

**6b: Test `make run-pipeline` prerequisites**

```bash
# Verify gh can access the project board
gh project item-list 8 --owner CodingThrust --format json --limit 1
```

If this fails, the user likely needs org-level project scopes:
```bash
gh auth refresh -s read:project,project
```

**6c: Test `make run-review` prerequisites**

```bash
# Verify gh-copilot-review works
gh copilot-review --help
```

**6d: Test claude CLI**

```bash
claude --version
```

If all pipeline checks pass, explain the project-based contribution pipeline:

> **Setup complete!** All tools installed and verified.
>
> ## How the Project Pipeline Works
>
> This project uses a [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1) to track issues through an automated pipeline. Issues flow through these columns:
>
> ```
> Ready → In Progress → review-agentic → In Review → Done
> ```
>
> Two `make` commands drive this pipeline:
>
> ### `make run-pipeline` (issue → PR)
> Picks the next **Ready** issue, moves it to **In Progress**, implements it (using `/issue-to-pr` → `/add-model` or `/add-rule`), creates a PR, then moves it to **review-agentic**.
>
> ### `make run-review` (PR → review)
> Picks the next **review-agentic** PR, waits for Copilot review, fixes comments and CI failures, runs agentic feature tests, then moves it to **In Review** for human approval.
>
> ### Targeting specific items
> - `make run-pipeline N=42` — process issue #42
> - `make run-review N=570` — process PR #570
>
> ### Available skills for manual work
> You can also invoke individual skills directly:
> - `/issue-to-pr 42` — convert a specific issue into a PR
> - `/add-model` — interactively add a new problem model
> - `/add-rule` — interactively add a new reduction rule
> - `/fix-pr` — fix review comments and CI on the current PR
> - `/release` — prepare a new crate release

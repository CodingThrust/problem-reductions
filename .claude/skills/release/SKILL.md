---
name: release
description: Use when preparing a new crate release, bumping versions, or tagging a release
---

# Release

Guide for creating a new release of problemreductions.

## Step 1: Determine Version Bump

Compare against the last release tag:

```bash
git tag -l 'v0.*' | sort -V          # find latest tag
git log <last-tag>..HEAD --oneline    # review commits
git diff <last-tag>..HEAD --stat      # review scope
```

Apply semver for 0.x (pre-1.0):
- **Patch** (0.x.Y) — bug fixes, docs, CI only
- **Minor** (0.X.0) — new features, new reductions, new public API
- **Major** — reserved for post-1.0

## Step 2: Verify Clean State

```bash
make test clippy
```

Both must pass with zero warnings before proceeding.

## Step 3: Release

```bash
make release V=x.y.z
```

This target bumps versions in `Cargo.toml`, `problemreductions-macros/Cargo.toml`, and `problemreductions-cli/Cargo.toml`, runs `cargo check`, commits, tags, and pushes. CI publishes all three crates to crates.io.

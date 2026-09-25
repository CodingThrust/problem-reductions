---
name: release
description: Use when cutting a new problemreductions crate release — checks the repo is releasable, proposes the version bump with a changelog summary, and runs make release after explicit confirmation
---

# Release

`make release V=x.y.z` bumps the version in `Cargo.toml`, `problemreductions-macros/Cargo.toml`,
and `problemreductions-cli/Cargo.toml` (plus the inter-crate dependency versions), runs
`cargo check`, commits `release: vX.Y.Z`, tags `vX.Y.Z`, and pushes `main` and tags. CI then
publishes all three crates to crates.io. A pushed tag cannot be taken back cleanly, so every
gate below is hard.

## Gates (refuse if any fails)

```bash
git branch --show-current                  # must be main
git status --porcelain                     # must be empty
git fetch origin main && git rev-parse HEAD origin/main   # must be equal
make check                                 # fmt-check + clippy + test must pass
```

`make release` enforces the same branch/clean/up-to-date guard and runs `make check` itself.

## Version and changelog

```bash
last=$(git tag -l 'v0.*' | sort -V | tail -1)
git log "$last"..HEAD --oneline
git diff "$last"..HEAD --stat
```

Pre-1.0 semver: **patch** (0.x.Y) for fixes, docs, CI; **minor** (0.X.0) for new models, rules,
CLI features, or any public API change, including breaking ones.

Show the user the proposed version, the reason, and a grouped changelog summary (new models,
new rules, CLI, fixes, breaking changes). Run `make release V=x.y.z` only after the user
explicitly confirms that exact version. Afterwards report the tag and point to the release CI
run (`gh run list --workflow release.yml --limit 1`).

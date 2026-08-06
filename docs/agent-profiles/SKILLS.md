# Skills

Example generation goes through the example catalog and generated paper data.
When a workflow needs a paper/example instance, prefer the catalog path over ad hoc `examples/reduction_*.rs` binaries:

- use `docs/paper/data/examples.json` directly for paper/example data
- run `cargo run --features "example-db" --example export_examples` when canonical examples change
- use `pred create --example <PROBLEM_SPEC>` to materialize a canonical model example as normal problem JSON
- use `pred create --example <SOURCE_SPEC> --to <TARGET_SPEC>` to materialize a canonical rule example as normal problem JSON
- when adding new example coverage, register a catalog entry instead of creating a new standalone reduction example file

Post-refactor extension points:

- new model load/serialize/brute-force dispatch comes from `declare_variants!` in the model file, with an optional `default`
- alias resolution lives in `problemreductions-cli/src/problem_name.rs`
- `pred create` UX lives in `problemreductions-cli/src/commands/create.rs`
- model examples live in `src/example_db/model_builders.rs`; rule examples live beside their rules and are collected by `src/rules/mod.rs`

- [issue-to-pr] — Convert a GitHub issue into a PR with an implementation plan
- [add-model] — Add a new problem model to the codebase
- [add-rule] — Add a new reduction rule to the codebase
- [review-implementation] — Review implementation completeness via parallel subagents
- [fix-pr] — Resolve PR review comments, CI failures, and coverage gaps
- [check-issue] — Quality gate for Rule and Model GitHub issues
- [topology-sanity-check] — Run sanity checks on the reduction graph: detect orphan problems and redundant rules
- [project-pipeline] — Pick the next ready issue, implement it, and move it through the project workflow
- [review-pipeline] — Process PRs in Review pool: fix comments, fix CI, run agentic review, move to Final review
- [propose] — Interactive brainstorming that turns a new model or rule idea into a GitHub issue
- [final-review] — Interactive maintainer review for PRs in the Final review column
- [dev-setup] — Install and configure the maintainer development environment
- [write-model-in-paper] — Write or improve a problem-def entry in the Typst paper
- [write-rule-in-paper] — Write or improve a reduction-rule entry in the Typst paper
- [release] — Create a new crate release with version bump

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
- model examples live beside each model in `canonical_model_example_specs()` (collected by `src/example_db/model_builders.rs`); rule examples live beside their rules and are collected by `src/rules/mod.rs`

Guides (auto-invoked while working):

- [how-to-code] — Implement or modify a problem model or reduction rule
- [how-to-verify] — Certify a reduction and check reduction-graph topology
- [how-to-write-manual] — Write or audit Typst manual entries and mdBook docs
- [how-to-review] — Fresh-context PR review with a `pred` feature test
- [how-to-triage-issue] — Quality-check and fix Model and Rule GitHub issues
- [how-to-ship] — Take an issue to a merge-ready pull request

Tools (invoked on request):

- [propose] — Turn a new model or rule idea into a GitHub issue
- [find-solver] — Match a real-world problem to a model, route, and solver
- [find-problem] — Find source problems a given solver handles
- [dev-setup] — Install and configure the development environment
- [release] — Guarded crate release with version bump
- [update-papers] — Refresh the research paper collection

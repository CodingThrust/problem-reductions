# Skills

Example generation now goes through the example catalog and dedicated exporter.
When a workflow needs a paper/example instance, prefer the catalog path over ad hoc `examples/reduction_*.rs` binaries:

- use `make examples` or `cargo run --features "ilp-highs example-db" --example export_examples`
- use `pred create --example <SOURCE_SPEC> --to <TARGET_SPEC>` to materialize a canonical rule example as normal problem JSON
- when adding new example coverage, register a catalog entry instead of creating a new standalone reduction example file

- [issue-to-pr] — Convert a GitHub issue into a PR with an implementation plan
- [add-model] — Add a new problem model to the codebase
- [add-rule] — Add a new reduction rule to the codebase
- [review-implementation] — Review implementation completeness via parallel subagents
- [fix-pr] — Resolve PR review comments, CI failures, and coverage gaps
- [check-issue] — Quality gate for Rule and Model GitHub issues
- [topology-sanity-check] — Run sanity checks on the reduction graph: detect orphan problems and redundant rules
- [write-model-in-paper] — Write or improve a problem-def entry in the Typst paper
- [write-rule-in-paper] — Write or improve a reduction-rule entry in the Typst paper
- [release] — Create a new crate release with version bump
- [meta-power] — Batch-resolve all open Model and Rule issues autonomously

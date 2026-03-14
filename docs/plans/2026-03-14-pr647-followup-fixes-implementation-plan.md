# PR647 Follow-Up Fixes Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix the current PR's concrete regressions and drift without starting the larger architecture refactor.

**Architecture:** Keep the current registry/example-db design, but close three gaps: make `One` payloads round-trip through registry-backed loading, resolve `pred create --example` from canonical example data instead of generic graph variants, and include `example-db` tests in normal verification paths. Prefer small local fixes over new abstractions.

**Tech Stack:** Rust, Cargo tests, CLI integration tests, GitHub Actions, Makefile

---

### Task 1: Lock In Regression Tests

**Files:**
- Modify: `src/unit_tests/types.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`

- [ ] **Step 1: Add failing serde tests for `One`**

Add tests proving `serde_json` serializes `One` as `1` and deserializes `1` back to `One`.

- [ ] **Step 2: Run the narrow serde test and verify RED**

Run: `cargo test test_one_json -- --exact`
Expected: FAIL until `One` gets custom serde behavior.

- [ ] **Step 3: Add failing CLI tests for shorthand canonical examples**

Add CLI tests for:
- `pred create --example MIS`
- `pred create --example MIS/i32`
- `pred create --example MVC/i32 --to MIS/i32`

- [ ] **Step 4: Run the new CLI tests and verify RED**

Run: `cargo test -p problemreductions-cli test_create_model_example_mis_shorthand -- --exact`
Run: `cargo test -p problemreductions-cli test_create_model_example_mis_weight_only -- --exact`
Run: `cargo test -p problemreductions-cli test_create_rule_example_mvc_to_mis_weight_only -- --exact`
Expected: FAIL with current ambiguity/lookup behavior.

### Task 2: Fix Round-Trip and Example Resolution

**Files:**
- Modify: `src/types.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Optional: `src/example_db/mod.rs`

- [ ] **Step 1: Implement custom serde for `One`**

Serialize `One` as integer `1`. Deserialize from integer `1` and reject other values.

- [ ] **Step 2: Make `create --example` resolve against canonical example refs**

Use the actual model/rule example DB keys instead of all reduction-graph variants, while keeping alias parsing and value-based matching.

- [ ] **Step 3: Run the focused regression tests and verify GREEN**

Run:
- `cargo test test_one_json -- --exact`
- `cargo test -p problemreductions-cli test_create_model_example_mis_shorthand -- --exact`
- `cargo test -p problemreductions-cli test_create_model_example_mis_weight_only -- --exact`
- `cargo test -p problemreductions-cli test_create_rule_example_mvc_to_mis_weight_only -- --exact`

Expected: PASS

### Task 3: Restore Verification Coverage

**Files:**
- Modify: `Makefile`
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1: Update normal test commands to include `example-db`**

Change repo verification commands so `example_db` tests run in regular `make test` and CI test jobs.

- [ ] **Step 2: Re-run the exact previously failing commands**

Run:
- `cargo test -p problemreductions-cli test_create_`
- `cargo test example_db:: --features 'ilp-highs example-db'`

Expected: PASS

- [ ] **Step 3: Run final verification**

Run:
- `cargo test -p problemreductions-cli test_create_`
- `cargo test example_db:: --features 'ilp-highs example-db'`

Expected: PASS with zero failures.

# Makefile for problemreductions

.PHONY: help build test mcp-test fmt clippy doc mdbook paper examples clean coverage rust-export compare qubo-testdata export-schemas release run-plan run-issue run-pipeline run-review diagrams jl-testdata cli cli-demo copilot-review

# Default target
help:
	@echo "Available targets:"
	@echo "  build        - Build the project"
	@echo "  test         - Run all tests"
	@echo "  mcp-test     - Run MCP server tests"
	@echo "  fmt          - Format code with rustfmt"
	@echo "  fmt-check    - Check code formatting"
	@echo "  clippy       - Run clippy lints"
	@echo "  doc          - Build mdBook documentation"
	@echo "  diagrams     - Generate SVG diagrams from Typst (light + dark)"
	@echo "  mdbook       - Build and serve mdBook (with live reload)"
	@echo "  paper        - Build Typst paper (requires typst)"
	@echo "  coverage     - Generate coverage report (requires cargo-llvm-cov)"
	@echo "  clean        - Clean build artifacts"
	@echo "  check        - Quick check (fmt + clippy + test)"
	@echo "  rust-export  - Generate Rust mapping JSON exports"
	@echo "  compare      - Generate and compare Rust mapping exports"
	@echo "  examples     - Generate example JSON for paper"
	@echo "  export-schemas - Export problem schemas to JSON"
	@echo "  qubo-testdata - Regenerate QUBO test data (requires uv)"
	@echo "  jl-testdata  - Regenerate Julia parity test data (requires julia)"
	@echo "  release V=x.y.z - Tag and push a new release (triggers CI publish)"
	@echo "  cli          - Build the pred CLI tool"
	@echo "  cli-demo     - Run closed-loop CLI demo (build + exercise all commands)"
	@echo "  run-plan   - Execute a plan with Claude autorun (latest plan in docs/plans/)"
	@echo "  run-issue N=<number> - Run issue-to-pr --execute for a GitHub issue"
	@echo "  run-pipeline [N=<number>] - Pick a Ready issue, implement, move to review-agentic"
	@echo "  run-review [N=<number>] - Pick PR from review-agentic, fix comments/CI, run agentic tests"
	@echo "  copilot-review - Request Copilot code review on current PR"

# Build the project
build:
	cargo build --features ilp-highs

# Run all tests (including ignored tests)
test:
	cargo test --features "ilp-highs example-db" -- --include-ignored

# Run MCP server tests
mcp-test:  ## Run MCP server tests
	cargo test --features mcp -p problemreductions-cli mcp

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Run clippy
clippy:
	cargo clippy --all-targets --features ilp-highs -- -D warnings

# Build mdBook documentation
doc:
	cargo run --example export_graph
	cargo run --example export_schemas
	mdbook build docs
	RUSTDOCFLAGS="--default-theme=dark" cargo doc --features ilp-highs --no-deps
	rm -rf docs/book/api
	cp -r target/doc docs/book/api

# Generate SVG diagrams from Typst sources (light + dark themes)
TYPST_DOC_DIAGRAMS := $(wildcard docs/src/static/*.typ)
TYPST_PAPER_DIAGRAMS := $(wildcard docs/paper/static/*.typ)
diagrams:
	@for src in $(TYPST_DOC_DIAGRAMS); do \
		base=$$(basename $$src .typ); \
		echo "Compiling $$base (doc)..."; \
		typst compile $$src --root=. --input dark=false docs/src/static/$$base.svg; \
		typst compile $$src --root=. --input dark=true docs/src/static/$$base-dark.svg; \
	done

# Build and serve mdBook with API docs
mdbook:
	@echo "Exporting graph..."
	@cargo run --example export_graph 2>&1 | tail -1
	@echo "Exporting schemas..."
	@cargo run --example export_schemas 2>&1 | tail -1
	@echo "Building API docs..."
	@RUSTDOCFLAGS="--default-theme=dark" cargo doc --features ilp-highs --no-deps 2>&1 | tail -1
	@echo "Building mdBook..."
	@mdbook build
	rm -rf book/api
	cp -r target/doc book/api
	@-lsof -ti:3001 | xargs kill 2>/dev/null || true
	@echo "Serving at http://localhost:3001"
	python3 -m http.server 3001 -d book &
	@sleep 1 && (command -v xdg-open >/dev/null && xdg-open http://localhost:3001 || open http://localhost:3001)

# Generate all example JSON files for the paper
examples:
	cargo run --features "ilp-highs example-db" --example export_examples
	cargo run --features ilp-highs --example export_petersen_mapping

# Export problem schemas to JSON
export-schemas:
	cargo run --example export_schemas

# Build Typst paper (generates examples first)
paper: examples
	cargo run --example export_graph
	cargo run --example export_schemas
	cd docs/paper && typst compile --root .. reductions.typ reductions.pdf

# Generate coverage report (requires: cargo install cargo-llvm-cov)
coverage:
	@command -v cargo-llvm-cov >/dev/null 2>&1 || { echo "Installing cargo-llvm-cov..."; cargo install cargo-llvm-cov; }
	cargo llvm-cov --features ilp-highs --workspace --html --open

# Clean build artifacts
clean:
	cargo clean

# Quick check before commit
check: fmt-check clippy test
	@echo "✅ All checks passed!"

# Regenerate QUBO test data from Python (requires uv)
qubo-testdata:
	cd scripts && uv run python generate_qubo_tests.py

jl-testdata:  ## Regenerate Julia parity test data
	cd scripts/jl && julia --project=. generate_testdata.jl

# Release a new version: make release V=0.2.0
release:
ifndef V
	$(error Usage: make release V=x.y.z)
endif
	@echo "Releasing v$(V)..."
	sed -i '' 's/^version = ".*"/version = "$(V)"/' Cargo.toml
	sed -i '' 's/^version = ".*"/version = "$(V)"/' problemreductions-macros/Cargo.toml
	sed -i '' 's/^version = ".*"/version = "$(V)"/' problemreductions-cli/Cargo.toml
	sed -i '' 's/problemreductions-macros = { version = "[^"]*"/problemreductions-macros = { version = "$(V)"/' Cargo.toml
	sed -i '' 's/problemreductions = { version = "[^"]*"/problemreductions = { version = "$(V)"/' problemreductions-cli/Cargo.toml
	cargo check
	git add Cargo.toml problemreductions-macros/Cargo.toml problemreductions-cli/Cargo.toml
	git commit -m "release: v$(V)"
	git tag -a "v$(V)" -m "Release v$(V)"
	git push origin main --tags
	@echo "v$(V) pushed — CI will publish to crates.io"

# Build and install the pred CLI tool
cli:
	cargo install --path problemreductions-cli

# Generate Rust mapping JSON exports for all graphs and modes
GRAPHS := diamond bull house petersen
MODES := unweighted weighted triangular
rust-export:
	@mkdir -p tests/julia
	@for graph in $(GRAPHS); do \
		for mode in $(MODES); do \
			echo "Exporting $$graph ($$mode)..."; \
			cargo run --example export_mapping_stages -- $$graph $$mode; \
		done; \
	done

# Generate Rust exports and show comparison
compare: rust-export
	@echo ""
	@echo "=== Julia vs Rust Comparison ==="
	@for graph in $(GRAPHS); do \
		echo ""; \
		echo "=== $$graph ==="; \
		echo "-- unweighted --"; \
		echo "Julia: $$(jq '{nodes: .num_grid_nodes, overhead: .mis_overhead, tape: .num_tape_entries}' tests/data/$${graph}_unweighted_trace.json)"; \
		echo "Rust:  $$(jq '{nodes: .stages[3].num_nodes, overhead: .total_overhead, tape: ((.crossing_tape | length) + (.simplifier_tape | length))}' tests/data/$${graph}_rust_unweighted.json)"; \
		echo "-- weighted --"; \
		echo "Julia: $$(jq '{nodes: .num_grid_nodes, overhead: .mis_overhead, tape: .num_tape_entries}' tests/data/$${graph}_weighted_trace.json)"; \
		echo "Rust:  $$(jq '{nodes: .stages[3].num_nodes, overhead: .total_overhead, tape: ((.crossing_tape | length) + (.simplifier_tape | length))}' tests/data/$${graph}_rust_weighted.json)"; \
		echo "-- triangular --"; \
		echo "Julia: $$(jq '{nodes: .num_grid_nodes, overhead: .mis_overhead, tape: .num_tape_entries}' tests/data/$${graph}_triangular_trace.json)"; \
		echo "Rust:  $$(jq '{nodes: .stages[3].num_nodes, overhead: .total_overhead, tape: ((.crossing_tape | length) + (.simplifier_tape | length))}' tests/data/$${graph}_rust_triangular.json)"; \
	done

# Run a plan with Claude
# Usage: make run-plan [INSTRUCTIONS="..."] [OUTPUT=output.log] [AGENT_TYPE=claude]
# PLAN_FILE defaults to the most recently modified file in docs/plans/
INSTRUCTIONS ?=
OUTPUT ?= claude-output.log
AGENT_TYPE ?= claude
PLAN_FILE ?= $(shell ls -t docs/plans/*.md 2>/dev/null | head -1)

run-plan:
	@NL=$$'\n'; \
	BRANCH=$$(git branch --show-current); \
	PLAN_FILE="$(PLAN_FILE)"; \
	if [ "$(AGENT_TYPE)" = "claude" ]; then \
		PROCESS="1. Read the plan file$${NL}2. Execute the plan — it specifies which skill(s) to use$${NL}3. Push: git push origin $$BRANCH$${NL}4. If a PR already exists for this branch, skip. Otherwise create one."; \
	else \
		PROCESS="1. Read the plan file$${NL}2. Execute the tasks step by step. For each task, implement and test before moving on.$${NL}3. Push: git push origin $$BRANCH$${NL}4. If a PR already exists for this branch, skip. Otherwise create one."; \
	fi; \
	PROMPT="Execute the plan in '$$PLAN_FILE'."; \
	if [ -n "$(INSTRUCTIONS)" ]; then \
		PROMPT="$${PROMPT}$${NL}$${NL}## Additional Instructions$${NL}$(INSTRUCTIONS)"; \
	fi; \
	PROMPT="$${PROMPT}$${NL}$${NL}## Process$${NL}$${PROCESS}$${NL}$${NL}## Rules$${NL}- Tests should be strong enough to catch regressions.$${NL}- Do not modify tests to make them pass.$${NL}- Test failure must be reported."; \
	echo "=== Prompt ===" && echo "$$PROMPT" && echo "===" ; \
	claude --dangerously-skip-permissions \
		--model opus \
		--verbose \
		--max-turns 500 \
		-p "$$PROMPT" 2>&1 | tee "$(OUTPUT)"

# Run issue-to-pr --execute for a GitHub issue
# Usage: make run-issue N=42
N ?=
run-issue:
	@if [ -z "$(N)" ]; then echo "Usage: make run-issue N=<issue-number>"; exit 1; fi
	claude --dangerously-skip-permissions \
		--model opus \
		--verbose \
		--max-turns 500 \
		-p "/issue-to-pr $(N) --execute" 2>&1 | tee "issue-$(N)-output.log"

# Closed-loop CLI demo: exercises all commands end-to-end
PRED := cargo run -p problemreductions-cli --release --
CLI_DEMO_DIR := /tmp/pred-cli-demo
cli-demo: cli
	@echo "=== pred CLI closed-loop demo ==="
	@rm -rf $(CLI_DEMO_DIR) && mkdir -p $(CLI_DEMO_DIR)
	@set -e; \
	PRED="./target/release/pred"; \
	\
	echo ""; \
	echo "--- 1. list: all registered problems ---"; \
	$$PRED list; \
	$$PRED list -o $(CLI_DEMO_DIR)/problems.json; \
	\
	echo ""; \
	echo "--- 2. show: inspect MIS (variants, fields, reductions) ---"; \
	$$PRED show MIS; \
	$$PRED show MIS -o $(CLI_DEMO_DIR)/mis_info.json; \
	\
	echo ""; \
	echo "--- 3. to: explore 2-hop outgoing neighborhood ---"; \
	$$PRED to MIS --hops 2; \
	$$PRED to MIS --hops 2 -o $(CLI_DEMO_DIR)/mis_hops.json; \
	\
	echo ""; \
	echo "--- 4. from: incoming neighbors ---"; \
	$$PRED from QUBO --hops 1; \
	\
	echo ""; \
	echo "--- 5. path: find reduction paths ---"; \
	$$PRED path MIS QUBO; \
	$$PRED path MIS QUBO -o $(CLI_DEMO_DIR)/path_mis_qubo.json; \
	$$PRED path Factoring SpinGlass; \
	$$PRED path MIS QUBO --cost minimize:num_variables; \
	\
	echo ""; \
	echo "--- 6. path --all: enumerate all paths ---"; \
	$$PRED path MIS QUBO --all; \
	$$PRED path MIS QUBO --all -o $(CLI_DEMO_DIR)/all_paths/; \
	\
	echo ""; \
	echo "--- 7. export-graph: full reduction graph ---"; \
	$$PRED export-graph -o $(CLI_DEMO_DIR)/graph.json; \
	\
	echo ""; \
	echo "--- 8. create: build problem instances ---"; \
	$$PRED create MIS --graph 0-1,1-2,2-3,3-4,4-0 -o $(CLI_DEMO_DIR)/mis.json; \
	$$PRED create MIS --graph 0-1,1-2,2-3 --weights 2,1,3,1 -o $(CLI_DEMO_DIR)/mis_weighted.json; \
	$$PRED create SAT --num-vars 3 --clauses "1,2;-1,3;2,-3" -o $(CLI_DEMO_DIR)/sat.json; \
	$$PRED create 3SAT --num-vars 4 --clauses "1,2,3;-1,2,-3;1,-2,3" -o $(CLI_DEMO_DIR)/3sat.json; \
	$$PRED create QUBO --matrix "1,-0.5;-0.5,2" -o $(CLI_DEMO_DIR)/qubo.json; \
	$$PRED create KColoring --k 3 --graph 0-1,1-2,2-0 -o $(CLI_DEMO_DIR)/kcol.json; \
	$$PRED create SpinGlass --graph 0-1,1-2 -o $(CLI_DEMO_DIR)/sg.json; \
	$$PRED create MaxCut --graph 0-1,1-2,2-0 -o $(CLI_DEMO_DIR)/maxcut.json; \
	$$PRED create MVC --graph 0-1,1-2,2-3 -o $(CLI_DEMO_DIR)/mvc.json; \
	$$PRED create MaximumMatching --graph 0-1,1-2,2-3 -o $(CLI_DEMO_DIR)/matching.json; \
	$$PRED create Factoring --target 15 --m 4 --n 4 -o $(CLI_DEMO_DIR)/factoring.json; \
	$$PRED create Factoring --target 21 --m 3 --n 3 -o $(CLI_DEMO_DIR)/factoring2.json; \
	\
	echo ""; \
	echo "--- 9. evaluate: test configurations ---"; \
	$$PRED evaluate $(CLI_DEMO_DIR)/mis.json --config 1,0,1,0,0; \
	$$PRED evaluate $(CLI_DEMO_DIR)/mis.json --config 1,1,0,0,0; \
	$$PRED evaluate $(CLI_DEMO_DIR)/sat.json --config 0,1,1; \
	$$PRED evaluate $(CLI_DEMO_DIR)/mis.json --config 1,0,1,0,0 -o $(CLI_DEMO_DIR)/eval.json; \
	\
	echo ""; \
	echo "--- 10. solve: direct ILP (auto-reduces to ILP) ---"; \
	$$PRED solve $(CLI_DEMO_DIR)/mis.json; \
	$$PRED solve $(CLI_DEMO_DIR)/mis.json -o $(CLI_DEMO_DIR)/sol_ilp.json; \
	\
	echo ""; \
	echo "--- 11. solve: brute-force ---"; \
	$$PRED solve $(CLI_DEMO_DIR)/mis.json --solver brute-force; \
	\
	echo ""; \
	echo "--- 12. solve: weighted MIS ---"; \
	$$PRED solve $(CLI_DEMO_DIR)/mis_weighted.json; \
	\
	echo ""; \
	echo "--- 13. reduce: MIS → QUBO (auto-discover path) ---"; \
	$$PRED reduce $(CLI_DEMO_DIR)/mis.json --to QUBO -o $(CLI_DEMO_DIR)/bundle_qubo.json; \
	\
	echo ""; \
	echo "--- 14. solve bundle: brute-force on reduced QUBO ---"; \
	$$PRED solve $(CLI_DEMO_DIR)/bundle_qubo.json --solver brute-force; \
	\
	echo ""; \
	echo "--- 15. reduce --via: use explicit path file ---"; \
	$$PRED reduce $(CLI_DEMO_DIR)/mis.json --via $(CLI_DEMO_DIR)/path_mis_qubo.json -o $(CLI_DEMO_DIR)/bundle_via.json; \
	\
	echo ""; \
	echo "--- 16. solve bundle with ILP: MIS → MVC → ILP ---"; \
	$$PRED reduce $(CLI_DEMO_DIR)/mis.json --to MVC -o $(CLI_DEMO_DIR)/bundle_mvc.json; \
	$$PRED solve $(CLI_DEMO_DIR)/bundle_mvc.json --solver ilp; \
	\
	echo ""; \
	echo "--- 17. solve: other problem types ---"; \
	$$PRED solve $(CLI_DEMO_DIR)/sat.json --solver brute-force; \
	$$PRED solve $(CLI_DEMO_DIR)/kcol.json --solver brute-force; \
	$$PRED solve $(CLI_DEMO_DIR)/maxcut.json --solver brute-force; \
	$$PRED solve $(CLI_DEMO_DIR)/mvc.json; \
	\
	echo ""; \
	echo "--- 18. closed-loop: create → reduce → solve → verify ---"; \
	echo "Creating a 6-vertex graph..."; \
	$$PRED create MIS --graph 0-1,1-2,2-3,3-4,4-5,0-5,1-4 -o $(CLI_DEMO_DIR)/big.json; \
	echo "Solving with ILP..."; \
	$$PRED solve $(CLI_DEMO_DIR)/big.json -o $(CLI_DEMO_DIR)/big_sol.json; \
	echo "Reducing to QUBO and solving with brute-force..."; \
	$$PRED reduce $(CLI_DEMO_DIR)/big.json --to QUBO -o $(CLI_DEMO_DIR)/big_qubo.json; \
	$$PRED solve $(CLI_DEMO_DIR)/big_qubo.json --solver brute-force -o $(CLI_DEMO_DIR)/big_qubo_sol.json; \
	echo "Verifying both solutions have the same evaluation..."; \
	ILP_EVAL=$$(jq -r '.evaluation' $(CLI_DEMO_DIR)/big_sol.json); \
	BF_EVAL=$$(jq -r '.evaluation' $(CLI_DEMO_DIR)/big_qubo_sol.json); \
	echo "  ILP solution evaluation:         $$ILP_EVAL"; \
	echo "  Brute-force (via QUBO) evaluation: $$BF_EVAL"; \
	if [ "$$ILP_EVAL" = "$$BF_EVAL" ]; then \
		echo "  ✅ Solutions agree!"; \
	else \
		echo "  ❌ Solutions disagree!" && exit 1; \
	fi; \
	\
	echo ""; \
	echo "--- 19. show with alias and variant slash syntax ---"; \
	$$PRED show MIS/UnitDiskGraph; \
	\
	echo ""; \
	echo "--- 20. completions: generate shell completions ---"; \
	$$PRED completions bash > /dev/null && echo "bash completions: OK"; \
	$$PRED completions zsh > /dev/null && echo "zsh completions:  OK"; \
	$$PRED completions fish > /dev/null && echo "fish completions: OK"; \
	\
	echo ""; \
	echo "=== Demo complete: $$(ls $(CLI_DEMO_DIR)/*.json | wc -l | tr -d ' ') JSON files in $(CLI_DEMO_DIR) ==="
	@echo "=== All 20 steps passed ✅ ==="

# Run project-pipeline: pick a Ready issue, implement, move to In Review
# Usage: make run-pipeline          (picks next Ready issue automatically)
#        make run-pipeline N=97     (processes specific issue)
run-pipeline:
	@if [ -n "$(N)" ]; then \
		PROMPT="/project-pipeline $(N)"; \
	else \
		PROMPT="/project-pipeline"; \
	fi; \
	claude --dangerously-skip-permissions \
		--model opus \
		--verbose \
		--max-turns 500 \
		-p "$$PROMPT" 2>&1 | tee "pipeline-output.log"

# Usage: make run-review          (picks next review-agentic PR automatically)
#        make run-review N=570    (processes specific PR)
run-review:
	@if [ -n "$(N)" ]; then \
		PROMPT="/review-pipeline $(N)"; \
	else \
		PROMPT="/review-pipeline"; \
	fi; \
	claude --dangerously-skip-permissions \
		--model opus \
		--verbose \
		--max-turns 500 \
		-p "$$PROMPT" 2>&1 | tee "review-output.log"

# Request Copilot code review on the current PR
# Requires: gh extension install ChrisCarini/gh-copilot-review
copilot-review:
	@PR=$$(gh pr view --json number --jq .number 2>/dev/null) || { echo "No PR found for current branch"; exit 1; }; \
	echo "Requesting Copilot review on PR #$$PR..."; \
	gh copilot-review $$PR

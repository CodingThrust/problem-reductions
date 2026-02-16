# Makefile for problemreductions

.PHONY: help build test fmt clippy doc mdbook paper examples clean coverage rust-export compare qubo-testdata export-schemas release run-plan diagrams jl-testdata

# Default target
help:
	@echo "Available targets:"
	@echo "  build        - Build the project"
	@echo "  test         - Run all tests"
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
	@echo "  run-plan   - Execute a plan with Claude autorun (latest plan in docs/plans/)"

# Build the project
build:
	cargo build --all-features

# Run all tests (including ignored tests)
test:
	cargo test --all-features -- --include-ignored

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Build mdBook documentation
doc:
	cargo run --example export_graph
	cargo run --example export_schemas
	mdbook build docs
	RUSTDOCFLAGS="--default-theme=dark" cargo doc --all-features --no-deps
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
	cargo run --example export_graph
	cargo run --example export_schemas
	RUSTDOCFLAGS="--default-theme=dark" cargo doc --all-features --no-deps
	mdbook build
	rm -rf book/api
	cp -r target/doc book/api
	@-lsof -ti:3001 | xargs kill 2>/dev/null || true
	@echo "Serving at http://localhost:3001"
	python3 -m http.server 3001 -d book &
	@sleep 1 && (command -v xdg-open >/dev/null && xdg-open http://localhost:3001 || open http://localhost:3001)

# Generate all example JSON files for the paper
REDUCTION_EXAMPLES := $(patsubst examples/%.rs,%,$(wildcard examples/reduction_*.rs))
examples:
	@mkdir -p docs/paper/examples
	@for example in $(REDUCTION_EXAMPLES); do \
		echo "Running $$example..."; \
		cargo run --all-features --example $$example || exit 1; \
	done
	cargo run --all-features --example export_petersen_mapping

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
	cargo llvm-cov --all-features --workspace --html --open

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
	sed -i 's/^version = ".*"/version = "$(V)"/' Cargo.toml
	sed -i 's/^version = ".*"/version = "$(V)"/' problemreductions-macros/Cargo.toml
	sed -i 's/problemreductions-macros = { version = "[^"]*"/problemreductions-macros = { version = "$(V)"/' Cargo.toml
	cargo check
	git add Cargo.toml problemreductions-macros/Cargo.toml
	git commit -m "release: v$(V)"
	git tag -a "v$(V)" -m "Release v$(V)"
	git push origin main --tags
	@echo "v$(V) pushed — CI will publish to crates.io"

# Generate Rust mapping JSON exports for all graphs and modes
GRAPHS := diamond bull house petersen
MODES := unweighted weighted triangular
rust-export:
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
	if [ "$(AGENT_TYPE)" = "claude" ]; then \
		PROCESS="1. Read the plan file$${NL}2. Use /subagent-driven-development to execute tasks$${NL}3. Push: git push origin $$BRANCH$${NL}4. Create a pull request"; \
	else \
		PROCESS="1. Read the plan file$${NL}2. Execute the tasks step by step. For each task, implement and test before moving on.$${NL}3. Push: git push origin $$BRANCH$${NL}4. Create a pull request"; \
	fi; \
	PROMPT="Execute the plan in '$${PLAN_FILE}'."; \
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

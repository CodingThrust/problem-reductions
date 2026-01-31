# Makefile for problemreductions

.PHONY: help build test fmt clippy doc mdbook paper clean coverage julia-export rust-export compare

# Default target
help:
	@echo "Available targets:"
	@echo "  build        - Build the project"
	@echo "  test         - Run all tests"
	@echo "  fmt          - Format code with rustfmt"
	@echo "  fmt-check    - Check code formatting"
	@echo "  clippy       - Run clippy lints"
	@echo "  doc          - Build mdBook documentation"
	@echo "  mdbook       - Build and serve mdBook (with live reload)"
	@echo "  paper        - Build Typst paper (requires typst)"
	@echo "  coverage     - Generate coverage report (requires cargo-llvm-cov)"
	@echo "  clean        - Clean build artifacts"
	@echo "  check        - Quick check (fmt + clippy + test)"
	@echo "  julia-export - Generate Julia mapping JSON exports"
	@echo "  rust-export  - Generate Rust mapping JSON exports"
	@echo "  compare      - Generate and compare Julia/Rust mapping exports"

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
	mdbook build docs

# Build and serve mdBook with live reload
mdbook:
	mdbook serve docs --open

# Build Typst paper
paper:
	cargo run --example export_graph
	cd docs/paper && typst compile reductions.typ reductions.pdf

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

# Generate Julia mapping JSON exports (requires Julia with UnitDiskMapping)
julia-export:
	cd tests/julia && julia --project=. dump_bull_mapping.jl

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

# Generate both Julia and Rust exports and show comparison
compare: julia-export rust-export
	@echo ""
	@echo "=== Julia vs Rust Comparison ==="
	@for graph in $(GRAPHS); do \
		echo ""; \
		echo "=== $$graph ==="; \
		echo "-- unweighted --"; \
		echo "Julia: $$(jq '{nodes: .num_grid_nodes, overhead: .mis_overhead, tape: .num_tape_entries}' tests/julia/$${graph}_unweighted_trace.json)"; \
		echo "Rust:  $$(jq '{nodes: .stages[3].num_nodes, overhead: .total_overhead, tape: ((.crossing_tape | length) + (.simplifier_tape | length))}' tests/julia/$${graph}_rust_unweighted.json)"; \
		echo "-- weighted --"; \
		echo "Julia: $$(jq '{nodes: .num_grid_nodes, overhead: .mis_overhead, tape: .num_tape_entries}' tests/julia/$${graph}_weighted_trace.json)"; \
		echo "Rust:  $$(jq '{nodes: .stages[3].num_nodes, overhead: .total_overhead, tape: ((.crossing_tape | length) + (.simplifier_tape | length))}' tests/julia/$${graph}_rust_weighted.json)"; \
		echo "-- triangular --"; \
		echo "Julia: $$(jq '{nodes: .num_grid_nodes, overhead: .mis_overhead, tape: .num_tape_entries}' tests/julia/$${graph}_triangular_trace.json)"; \
		echo "Rust:  $$(jq '{nodes: .stages[3].num_nodes, overhead: .total_overhead, tape: ((.crossing_tape | length) + (.simplifier_tape | length))}' tests/julia/$${graph}_rust_triangular.json)"; \
	done

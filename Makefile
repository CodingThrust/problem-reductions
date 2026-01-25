# Makefile for problemreductions

.PHONY: help build test fmt clippy doc mdbook paper clean coverage

# Default target
help:
	@echo "Available targets:"
	@echo "  build      - Build the project"
	@echo "  test       - Run all tests"
	@echo "  fmt        - Format code with rustfmt"
	@echo "  fmt-check  - Check code formatting"
	@echo "  clippy     - Run clippy lints"
	@echo "  doc        - Build mdBook documentation"
	@echo "  mdbook     - Build and serve mdBook (with live reload)"
	@echo "  paper      - Build Typst paper (requires typst)"
	@echo "  coverage   - Generate coverage report (requires cargo-llvm-cov)"
	@echo "  clean      - Clean build artifacts"
	@echo "  check      - Quick check (fmt + clippy + test)"

# Build the project
build:
	cargo build --all-features

# Run all tests
test:
	cargo test --all-features

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

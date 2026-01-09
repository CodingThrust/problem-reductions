# Makefile for problemreductions

.PHONY: help build test fmt clippy doc clean coverage

# Default target
help:
	@echo "Available targets:"
	@echo "  build      - Build the project"
	@echo "  test       - Run all tests"
	@echo "  fmt        - Format code with rustfmt"
	@echo "  fmt-check  - Check code formatting"
	@echo "  clippy     - Run clippy lints"
	@echo "  doc        - Build and open documentation"
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

# Build and open documentation
doc:
	cargo doc --all-features --no-deps --open

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

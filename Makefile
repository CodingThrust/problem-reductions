# Makefile for problemreductions

.PHONY: help build test bench mcp-test fmt clippy doc mdbook website paper paper-data clean coverage rust-export compare qubo-testdata export-schemas release diagrams jl-testdata cli cli-demo copilot-review papers papers-lookup papers-download papers-scihub papers-status papers-push papers-pull papers-index

TEST_FEATURES := example-db

# Cross-platform sed in-place: macOS needs -i '', Linux needs -i
SED_I := sed -i$(shell if [ "$$(uname)" = "Darwin" ]; then echo " ''"; fi)

# Default target
help:
	@echo "Available targets:"
	@echo "  build        - Build the project"
	@echo "  test         - Run all tests"
	@echo "  bench        - Run solver benchmarks"
	@echo "  mcp-test     - Run MCP server tests"
	@echo "  fmt          - Format code with rustfmt"
	@echo "  fmt-check    - Check code formatting"
	@echo "  clippy       - Run clippy lints"
	@echo "  doc          - Build mdBook documentation"
	@echo "  diagrams     - Generate SVG diagrams from Typst (light + dark)"
	@echo "  mdbook       - Build and serve mdBook (with live reload)"
	@echo "  website      - Build the research website and documentation"
	@echo "  paper        - Generate example data and build the Typst paper (requires typst)"
	@echo "  coverage     - Generate coverage report (requires cargo-llvm-cov)"
	@echo "  clean        - Clean build artifacts"
	@echo "  check        - Quick check (fmt + clippy + test)"
	@echo "  rust-export  - Generate Rust mapping JSON exports"
	@echo "  compare      - Generate and compare Rust mapping exports"
	@echo "  export-schemas - Export problem schemas to JSON"
	@echo "  qubo-testdata - Regenerate QUBO test data (requires uv)"
	@echo "  jl-testdata  - Regenerate Julia parity test data (requires julia)"
	@echo "  release V=x.y.z - Tag and push a new release (triggers CI publish)"
	@echo "  cli          - Build the pred CLI tool"
	@echo "  cli-demo     - Run closed-loop CLI demo (build + exercise all commands)"
	@echo "  copilot-review - Request Copilot code review on current PR"
	@echo ""
	@echo "  papers         - Full paper fetch: lookup + download + scihub"
	@echo "  papers-lookup  - Lookup arxiv/OA URLs for references.bib entries"
	@echo "  papers-download - Download available free PDFs"
	@echo "  papers-scihub  - Fetch remaining papers via Sci-Hub"
	@echo "  papers-status  - Show paper collection stats"

# Build the project
build:
	cargo build

# Run all workspace tests (including ignored tests)
test:
	cargo test --features "$(TEST_FEATURES)" --workspace -- --include-ignored

# Compile Criterion only when benchmarks are requested
bench:
	cargo bench --features benchmarks

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
	cargo clippy --workspace --all-targets --all-features -- -D warnings

node_modules/elkjs/package.json: package.json package-lock.json
	npm ci

# Example data read by docs/paper/reductions.typ, which the PDF and the website details compile.
paper-data:
	cargo run --features "$(TEST_FEATURES)" --example export_examples
	cargo run --features "$(TEST_FEATURES)" --example export_petersen_mapping

# Build mdBook documentation
doc: node_modules/elkjs/package.json paper-data
	cargo run --example export_graph
	node scripts/generate_reduction_graph_layout.js
	cargo run --example export_schemas
	cargo build -p problemreductions-cli --bin pred
	bash scripts/generate_doc_snippets.sh target/debug/pred
	mdbook build
	RUSTDOCFLAGS="--default-theme=dark" cargo doc --no-deps
	rm -rf book/api
	cp -r target/doc book/api
	python3 scripts/build_website.py

# Build the product website with fresh atlas data; API/PDF builds remain in doc/paper.
website: node_modules/elkjs/package.json paper-data
	cargo run --example export_graph
	node scripts/generate_reduction_graph_layout.js
	cargo run --example export_schemas
	cargo build -p problemreductions-cli --bin pred
	bash scripts/generate_doc_snippets.sh target/debug/pred
	mdbook build
	python3 scripts/build_website.py

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
mdbook: node_modules/elkjs/package.json paper-data
	@echo "Exporting graph..."
	@cargo run --example export_graph 2>&1 | tail -1
	@echo "Generating graph layout..."
	@node scripts/generate_reduction_graph_layout.js 2>&1 | tail -1
	@echo "Exporting schemas..."
	@cargo run --example export_schemas 2>&1 | tail -1
	@echo "Generating CLI doc snippets..."
	@cargo build -p problemreductions-cli --bin pred
	@bash scripts/generate_doc_snippets.sh target/debug/pred
	@echo "Building API docs..."
	@RUSTDOCFLAGS="--default-theme=dark" cargo doc --no-deps 2>&1 | tail -1
	@echo "Building mdBook..."
	@mdbook build
	rm -rf book/api
	cp -r target/doc book/api
	@python3 scripts/build_website.py
	@-lsof -ti:3001 | xargs kill 2>/dev/null || true
	@echo "Serving at http://localhost:3001"
	python3 -m http.server 3001 -d book &
	@sleep 1 && (command -v xdg-open >/dev/null && xdg-open http://localhost:3001 || open http://localhost:3001)


# Export problem schemas to JSON
export-schemas:
	cargo run --example export_schemas

# Build Typst paper (generates example data on demand)
paper: paper-data
	cargo run --features "$(TEST_FEATURES)" --example export_graph
	cargo run --features "$(TEST_FEATURES)" --example export_schemas
	typst compile --root . docs/paper/reductions.typ docs/paper/reductions.pdf

# Generate coverage report (requires: cargo install cargo-llvm-cov)
coverage:
	@command -v cargo-llvm-cov >/dev/null 2>&1 || { echo "Installing cargo-llvm-cov..."; cargo install cargo-llvm-cov; }
	cargo llvm-cov --workspace --html --open

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
	@test "$$(git branch --show-current)" = main || { echo "release: must be on main"; exit 1; }
	@test -z "$$(git status --porcelain)" || { echo "release: working tree not clean"; exit 1; }
	@git fetch -q origin main && test "$$(git rev-parse HEAD)" = "$$(git rev-parse origin/main)" || { echo "release: main is not up to date with origin/main"; exit 1; }
	$(MAKE) check
	@echo "Releasing v$(V)..."
	$(SED_I) 's/^version = ".*"/version = "$(V)"/' Cargo.toml
	$(SED_I) 's/^version = ".*"/version = "$(V)"/' problemreductions-macros/Cargo.toml
	$(SED_I) 's/^version = ".*"/version = "$(V)"/' problemreductions-cli/Cargo.toml
	$(SED_I) 's/problemreductions-macros = { version = "[^"]*"/problemreductions-macros = { version = "$(V)"/' Cargo.toml
	$(SED_I) 's/problemreductions = { version = "[^"]*"/problemreductions = { version = "$(V)"/' problemreductions-cli/Cargo.toml
	cargo check
	git add Cargo.toml problemreductions-macros/Cargo.toml problemreductions-cli/Cargo.toml
	git commit -m "release: v$(V)"
	git tag -a "v$(V)" -m "Release v$(V)"
	git push origin main --tags
	@echo "v$(V) pushed — CI will publish to crates.io"

# Build and install the pred CLI tool (without MCP for fast builds)
cli:
	cargo install --path problemreductions-cli

# Build and install the pred CLI tool with MCP server support
mcp:
	cargo install --path problemreductions-cli --features mcp

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
		julia=$$(jq -c '{nodes: .num_grid_nodes, overhead: .mis_overhead, tape: (.tape | length)}' tests/data/$${graph}_unweighted_trace.json); \
		rust=$$(jq -c '{nodes: .stages[3].num_nodes, overhead: .total_overhead, tape: ((.crossing_tape | length) + (.simplifier_tape | length))}' tests/julia/$${graph}_rust_unweighted.json); \
		echo "Julia: $$julia"; echo "Rust:  $$rust"; test "$$julia" = "$$rust" || exit 1; \
		echo "-- weighted --"; \
		julia=$$(jq -c '{nodes: .num_grid_nodes, overhead: .mis_overhead, tape: (.tape | length)}' tests/data/$${graph}_weighted_trace.json); \
		rust=$$(jq -c '{nodes: .stages[3].num_nodes, overhead: .total_overhead, tape: ((.crossing_tape | length) + (.simplifier_tape | length))}' tests/julia/$${graph}_rust_weighted.json); \
		echo "Julia: $$julia"; echo "Rust:  $$rust"; test "$$julia" = "$$rust" || exit 1; \
		echo "-- triangular --"; \
		julia=$$(jq -c '{nodes: .num_grid_nodes, overhead: .mis_overhead, tape: (.tape | length)}' tests/data/$${graph}_triangular_trace.json); \
		rust=$$(jq -c '{nodes: .stages[3].num_nodes, overhead: .total_overhead, tape: ((.crossing_tape | length) + (.simplifier_tape | length))}' tests/julia/$${graph}_rust_triangular.json); \
		echo "Julia: $$julia"; echo "Rust:  $$rust"; test "$$julia" = "$$rust" || exit 1; \
	done

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
	echo "--- 5. path: symbolic path enumeration ---"; \
	$$PRED path MIS QUBO; \
	$$PRED path Factoring SpinGlass; \
	echo "--- 5b. explicitly choose one route from the path set ---"; \
	$$PRED path MIS QUBO -o $(CLI_DEMO_DIR)/paths_mis_qubo.json; \
	jq -e 'first(.paths[] | select(([.path[0].from.name] + [.path[].to.name]) == ["MaximumIndependentSet", "MaximumIndependentSet", "MaximumSetPacking", "MaximumSetPacking", "QUBO"]))' $(CLI_DEMO_DIR)/paths_mis_qubo.json > $(CLI_DEMO_DIR)/path_mis_qubo.json; \
	\
	echo ""; \
	echo "--- 7. export-graph: full reduction graph ---"; \
	$$PRED export-graph -o $(CLI_DEMO_DIR)/graph.json; \
	\
	echo ""; \
	echo "--- 8. create: build problem instances ---"; \
	$$PRED create MIS --graph 0-1,1-2,2-3,3-4,4-0 -o $(CLI_DEMO_DIR)/mis.json; \
	$$PRED create MaximumIndependentSet/SimpleGraph/i64 --graph 0-1,1-2,2-3 --weights 2,1,3,1 -o $(CLI_DEMO_DIR)/mis_weighted.json; \
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
	echo "--- 13. reduce: MIS → QUBO along the explicitly chosen route ---"; \
	$$PRED reduce $(CLI_DEMO_DIR)/mis.json --via $(CLI_DEMO_DIR)/path_mis_qubo.json -o $(CLI_DEMO_DIR)/bundle_qubo.json; \
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
	$$PRED path MIS MVC -o $(CLI_DEMO_DIR)/paths_mis_mvc.json; \
	jq -e 'first(.paths[] | select(([.path[0].from.name] + [.path[].to.name]) == ["MaximumIndependentSet", "MaximumIndependentSet", "MinimumVertexCover"]))' $(CLI_DEMO_DIR)/paths_mis_mvc.json > $(CLI_DEMO_DIR)/path_mis_mvc.json; \
	$$PRED reduce $(CLI_DEMO_DIR)/mis.json --via $(CLI_DEMO_DIR)/path_mis_mvc.json -o $(CLI_DEMO_DIR)/bundle_mvc.json; \
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
	$$PRED reduce $(CLI_DEMO_DIR)/big.json --via $(CLI_DEMO_DIR)/path_mis_qubo.json -o $(CLI_DEMO_DIR)/big_qubo.json; \
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

# Request Copilot code review on the current PR
# Requires: gh extension install ChrisCarini/gh-copilot-review
copilot-review:
	@PR=$$(gh pr view --json number --jq .number 2>/dev/null) || { echo "No PR found for current branch"; exit 1; }; \
	echo "Requesting Copilot review on PR #$$PR..."; \
	gh copilot-review $$PR

# ── Research paper management ──────────────────────────────────────
# Downloads referenced papers to docs/research/raw/ (gitignored).
# Manifest in docs/research/manifest.json tracks URLs and sources.

# Full pipeline: lookup URLs, download free PDFs, then try Sci-Hub for the rest
papers: papers-lookup papers-download papers-scihub papers-status

# Step 1: Find arxiv/OA URLs via Semantic Scholar + arxiv APIs
papers-lookup:
	python3 scripts/fetch_papers.py lookup

# Step 2: Download papers with known free URLs
papers-download:
	python3 scripts/fetch_papers.py download

# Step 3: Fetch remaining papers (with DOIs) via Sci-Hub
papers-scihub:
	python3 scripts/fetch_papers.py scihub

# Step 4: Push PDFs to shared remote (requires rclone + PAPERS_REMOTE env var)
papers-push:
	python3 scripts/fetch_papers.py push

# Pull PDFs from shared remote (collaborator setup)
papers-pull:
	python3 scripts/fetch_papers.py pull

# Regenerate docs/research/index.md from references.bib + reductions.typ
papers-index:
	python3 scripts/gen_paper_index.py

# Show current collection stats
papers-status:
	python3 scripts/fetch_papers.py status

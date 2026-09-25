# Development Dependencies

Static tool list for problemreductions maintainers. Update it by hand when the Makefile, CI
workflows, or `Cargo.toml` start needing a new tool.

## Core (build, test, docs, paper)

| Tool | Check | Install (macOS) | Install (Linux, Debian/Ubuntu) | Needed by |
|------|-------|-----------------|--------------------------------|-----------|
| git | `git --version` | `xcode-select --install` | `sudo apt install git` | everything |
| C/C++ toolchain | `cc --version && c++ --version` | `xcode-select --install` | `sudo apt install build-essential` | `highs` (HiGHS built from source) |
| cmake | `cmake --version` | `brew install cmake` | `sudo apt install cmake` | `highs` build |
| libclang | `ldconfig -p \| grep libclang` (Linux) | `xcode-select --install` | `sudo apt install libclang-dev` | `highs-sys` bindgen |
| rust (stable) | `rustc --version` | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` | same | build, test |
| clippy, rustfmt | `cargo clippy --version && rustfmt --version` | `rustup component add clippy rustfmt` | same | `make check` |
| make | `make --version` | `xcode-select --install` | `sudo apt install make` | all targets |
| python3 | `python3 --version` | `brew install python` | `sudo apt install python3` | `scripts/`, papers targets |
| uv | `uv --version` | `curl -LsSf https://astral.sh/uv/install.sh \| sh` | same | `make qubo-testdata`, script tests |
| jq | `jq --version` | `brew install jq` | `sudo apt install jq` | `make cli-demo`, `make compare` |
| node + npm (22) | `node --version && npm --version` | `brew install node@22` | NodeSource or `nvm install 22` | `make doc` / `mdbook` / `website` (`npm ci`) |
| mdbook | `mdbook --version` | `cargo install mdbook` | same | `make doc`, `make mdbook` |
| typst | `typst --version` | `brew install typst` | `cargo install --locked typst-cli` | `make paper`, `make diagrams` |
| cargo-llvm-cov | `cargo llvm-cov --version` | `cargo install cargo-llvm-cov && rustup component add llvm-tools-preview` | same | `make coverage` |

## Contributor workflow

| Tool | Check | Install (macOS) | Install (Linux) | Needed by |
|------|-------|-----------------|-----------------|-----------|
| gh | `gh --version` | `brew install gh` | `sudo apt install gh` | issues, PRs, `propose` |
| pred, pred-sym | `pred --version && pred-sym --help` | `make cli` | same | `propose`, `find-solver`, `find-problem` |

## Optional

| Tool | Check | Install (macOS) | Install (Linux) | Needed by |
|------|-------|-----------------|-----------------|-----------|
| julia | `julia --version` | `brew install julia` | `curl -fsSL https://install.julialang.org \| sh` | `make jl-testdata` |
| rclone | `rclone version` | `brew install rclone` | `sudo apt install rclone` | `make papers-push` / `papers-pull` |
| claude | `claude --version` | `npm install -g @anthropic-ai/claude-code` | same | running repo skills |

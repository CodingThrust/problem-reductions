# Development Dependencies

Auto-generated tool checklist for problemreductions maintainers.
Rescan by running `/dev-setup` and choosing "rescan".

## Core Tools (build, test, docs)

| Tool | Check Command | Install (macOS) | Install (Linux) | Purpose |
|------|--------------|-----------------|-----------------|---------|
| git | `git --version` | preinstalled | `sudo apt install git` | Version control |
| rust | `rustc --version` | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` | same | Compiler and toolchain |
| clippy | `cargo clippy --version` | `rustup component add clippy` | same | Linting |
| rustfmt | `rustfmt --version` | `rustup component add rustfmt` | same | Code formatting |
| llvm-tools-preview | `rustup component list --installed \| grep llvm-tools` | `rustup component add llvm-tools-preview` | same | Required by cargo-llvm-cov |
| make | `make --version` | preinstalled | `sudo apt install make` | Build orchestration |
| python3 | `python3 --version` | `brew install python` | `sudo apt install python3` | Scripts and data generation |
| uv | `uv --version` | `curl -LsSf https://astral.sh/uv/install.sh \| sh` | same | Python package manager |
| jq | `jq --version` | `brew install jq` | `sudo apt install jq` | JSON processing (cli-demo, compare) |
| mdbook | `mdbook --version` | `cargo install mdbook` | same | Documentation site |
| typst | `typst --version` | `brew install typst` | `cargo install typst-cli` | Paper and diagrams |
| cargo-llvm-cov | `cargo llvm-cov --version` | `cargo install cargo-llvm-cov` | same | Coverage reports |

## Skill Tools (AI-assisted pipeline)

| Tool | Check Command | Install (macOS) | Install (Linux) | Purpose |
|------|--------------|-----------------|-----------------|---------|
| gh | `gh --version` | `brew install gh` | `sudo apt install gh` | GitHub CLI |
| gh-copilot-review | `gh copilot-review --help` | `gh extension install ChrisCarini/gh-copilot-review` | same | Copilot PR reviews |
| claude | `claude --version` | `npm install -g @anthropic-ai/claude-code` | same | AI-assisted pipeline |
| pred | `pred --version` | `cargo install --path problemreductions-cli` | same | Project CLI (check-issue, check-rule-redundancy) |

## Optional Tools

| Tool | Check Command | Install (macOS) | Install (Linux) | Purpose |
|------|--------------|-----------------|-----------------|---------|
| julia | `julia --version` | `brew install julia` | `sudo apt install julia` | Julia parity tests |

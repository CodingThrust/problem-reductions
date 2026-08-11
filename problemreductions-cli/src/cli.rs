use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

pub use crate::create_args::CreateArgs;

#[derive(Parser)]
#[command(
    name = "pred",
    about = "Explore NP-hard problem reductions",
    version,
    after_help = "\
Typical workflow:
  pred create MIS --graph 0-1,1-2,2-3 -o problem.json
  pred solve problem.json
  pred evaluate problem.json --config 1,0,1,0

Piping (use - to read from stdin):
  pred create MIS --graph 0-1,1-2 | pred solve -                    # when an ILP reduction path exists
  pred create StringToStringCorrection --source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2 | pred solve - --solver brute-force
  pred create MIS --graph 0-1,1-2 | pred evaluate - --config 1,0,1
  pred create MIS --graph 0-1,1-2 | pred reduce - --via route.json

JSON output (any command):
  pred list --json                 # JSON to stdout
  pred show MIS --json | jq '.'   # pipe to jq

Use `pred <command> --help` for detailed usage of each command.
Use `pred list` to see all available problem types.

Enable tab completion:
  eval \"$(pred completions)\"     # add to ~/.bashrc or ~/.zshrc"
)]
pub struct Cli {
    /// Output file path (implies JSON output)
    #[arg(long, short, global = true)]
    pub output: Option<PathBuf>,

    /// Suppress informational messages on stderr
    #[arg(long, short, global = true)]
    pub quiet: bool,

    /// Output JSON to stdout instead of human-readable text
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Browse registered problem types (or reduction rules with --rules)
    #[command(after_help = "\
Examples:
  pred list                   # show catalog summary and categories
  pred list matching          # search names and aliases
  pred list --category graph  # list graph problems
  pred list --all             # list every problem compactly
  pred list --rules --all     # list every reduction rule
  pred list -o problems.json  # save as JSON")]
    List {
        /// Case-insensitive substring to search in names and aliases
        query: Option<String>,

        /// List reduction rules instead of problem types
        #[arg(long)]
        rules: bool,

        /// Restrict problems to a model category such as graph, set, or scheduling
        #[arg(long, conflicts_with = "rules")]
        category: Option<String>,

        /// List the complete catalog instead of the summary
        #[arg(long)]
        all: bool,

        /// Include per-variant complexity, rule counts, or rule size contracts
        #[arg(long)]
        verbose: bool,
    },

    /// Show details for a problem type or variant (fields, reductions, complexity)
    #[command(after_help = "\
Examples:
  pred show MIS                   # all variants for MIS
  pred show MIS/UnitDiskGraph     # specific variant
  pred show MIS/UnitDiskGraph/i32 # fully qualified variant
  pred show KSAT/K3               # KSatisfiability with K=3

Use `pred list` to see all available problem types and variants.")]
    Show {
        /// Problem name or variant (e.g., MIS, MIS/UnitDiskGraph, KSAT/K3)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        problem: String,
    },

    /// Explore problems that reduce TO this one (incoming neighbors)
    #[command(after_help = "\
Examples:
  pred to MIS              # what reduces to MIS? (1 hop)
  pred to MIS --hops 2     # 2-hop incoming neighbors
  pred to MIS -o out.json  # save as JSON

Use `pred from <problem>` for outgoing neighbors (what this reduces to).")]
    To {
        /// Problem name or alias (e.g., MIS, QUBO, MIS/UnitDiskGraph)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        problem: String,
        /// Number of hops to explore
        #[arg(long, default_value = "1")]
        hops: usize,
    },

    /// Explore problems this reduces to, starting FROM it (outgoing neighbors)
    #[command(after_help = "\
Examples:
  pred from MIS              # what does MIS reduce to? (1 hop)
  pred from MIS --hops 2     # 2-hop outgoing neighbors
  pred from MIS -o out.json  # save as JSON

Use `pred to <problem>` for incoming neighbors (what reduces to this).")]
    From {
        /// Problem name or alias (e.g., MIS, QUBO, MIS/UnitDiskGraph)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        problem: String,
        /// Number of hops to explore
        #[arg(long, default_value = "1")]
        hops: usize,
    },

    /// Find reduction paths between two problems
    #[command(after_help = "\
Examples:
  pred path MIS QUBO                              # inspect reduction paths
  pred path MIS Clique mis.json                   # execute paths on an instance
  pred path MIS QUBO --max-paths 50              # increase the output cap
  pred path MIS QUBO -o paths.json               # save the path set

Use `pred list` to see available problems.")]
    Path {
        /// Source problem (e.g., MIS, MIS/UnitDiskGraph)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        source: String,
        /// Target problem (e.g., QUBO)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        target: String,
        /// Maximum paths to return
        #[arg(long, default_value_t = 20)]
        max_paths: usize,
        /// Source problem instance JSON. When present, execute every returned path and measure each constructed problem.
        instance: Option<std::path::PathBuf>,
    },

    /// Export the reduction graph to JSON
    #[command(after_help = "\
Examples:
  pred export-graph                           # print to stdout
  pred export-graph -o reduction_graph.json   # save to file")]
    ExportGraph,

    /// Create a problem instance and save as JSON
    Create(Box<CreateArgs>),
    /// Evaluate a configuration against a problem instance JSON file
    Evaluate(EvaluateArgs),
    /// Reduce a problem instance to a target type
    Reduce(ReduceArgs),
    /// Inspect a problem JSON or reduction bundle
    #[command(after_help = "\
Examples:
  pred inspect problem.json
  pred inspect bundle.json
  pred create MIS --graph 0-1,1-2 | pred inspect -")]
    Inspect(InspectArgs),
    /// Solve a problem instance
    Solve(SolveArgs),
    /// Extract a source-space solution from a reduction bundle and a target-space config
    #[command(after_help = "\
Examples:
  pred extract bundle.json --config 1,0,1,0
  pred extract bundle.json --config 1,0,1,0 -o source.json
  cat bundle.json | pred extract - --config 1,0,1,0

Use this when an external solver has solved the bundle's target problem
(e.g. a QUBO sampler, a neutral-atom platform, a QAOA runtime) and you want
the corresponding solution in the original source problem space without
having to shell back into `pred solve`.

Input: a reduction bundle JSON (from `pred reduce`). Use - to read from stdin.
--config is the target-space configuration (comma-separated, e.g. 1,0,1,0).")]
    Extract(ExtractArgs),
    /// Start MCP (Model Context Protocol) server for AI assistant integration
    #[cfg(feature = "mcp")]
    #[command(after_help = "\
Start a stdio-based MCP server that exposes problem reduction tools
to any MCP-compatible AI assistant.

Configuration:

  Claude Code / Claude Desktop (.mcp.json or ~/.claude/mcp.json):
    { \"mcpServers\": { \"problemreductions\": {
        \"command\": \"pred\", \"args\": [\"mcp\"] } } }

  Cursor (.cursor/mcp.json):
    { \"mcpServers\": { \"problemreductions\": {
        \"command\": \"pred\", \"args\": [\"mcp\"] } } }

  Windsurf (~/.codeium/windsurf/mcp_config.json):
    { \"mcpServers\": { \"problemreductions\": {
        \"command\": \"pred\", \"args\": [\"mcp\"] } } }

  OpenCode (opencode.json):
    { \"mcp\": { \"problemreductions\": {
        \"type\": \"local\", \"command\": [\"pred\", \"mcp\"] } } }

Test with MCP Inspector:
  npx @modelcontextprotocol/inspector pred mcp")]
    Mcp,
    /// Print shell completions to stdout (auto-detects shell)
    #[command(after_help = "\
Setup: add one line to your shell rc file:

  # bash (~/.bashrc)
  eval \"$(pred completions bash)\"

  # zsh (~/.zshrc)
  eval \"$(pred completions zsh)\"

  # fish (~/.config/fish/config.fish)
  pred completions fish | source")]
    Completions {
        /// Shell type (bash, zsh, fish, etc.). Auto-detected if omitted.
        shell: Option<clap_complete::Shell>,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ExampleSide {
    Source,
    Target,
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred solve problem.json                        # deterministic registered backend or fallback
  pred solve problem.json --solver brute-force   # brute-force (exhaustive search)
  pred solve problem.json --solver ilp           # require the registered fixed ILP pipeline
  pred solve reduced.json                        # solve a reduction bundle
  pred solve reduced.json -o solution.json       # save result to file
  pred create MIS --graph 0-1,1-2 | pred solve - # read from stdin
  pred create GroupingBySwapping --string \"0,1,2,0,1,2\" --bound 5 | pred solve - --solver brute-force
  pred create StringToStringCorrection --source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2 | pred solve - --solver brute-force
  pred create TwoDimensionalConsecutiveSets --alphabet-size 6 --subsets \"0,1,2;3,4,5;1,3;2,4;0,5\" | pred solve - --solver brute-force
  pred solve problem.json --timeout 10           # abort after 10 seconds

Typical workflow:
  pred create MIS --graph 0-1,1-2,2-3 -o problem.json
  pred solve problem.json

Solve via explicit reduction:
  pred reduce problem.json --via route.json -o reduced.json
  pred solve reduced.json

Input: a problem JSON from `pred create`, or a reduction bundle from `pred reduce`.
When given a bundle, the target is solved and the solution is mapped back to the source.
By default, solve deterministically selects the exact variant's registered native
backend, then its fixed ILP pipeline, and otherwise brute force. `--solver ilp`
requires a registered ILP pipeline; it never searches the reduction graph.
ILP problems are solved with HiGHS.")]
pub struct SolveArgs {
    /// Problem JSON file (from `pred create`) or reduction bundle (from `pred reduce`). Use - for stdin.
    pub input: PathBuf,
    /// Solver override: ilp or brute-force. Omit for deterministic default dispatch.
    #[arg(long)]
    pub solver: Option<String>,
    /// Timeout in seconds (0 = no limit)
    #[arg(long, default_value = "0")]
    pub timeout: u64,
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred reduce problem.json --via path.json -o reduced.json
  pred create MIS --graph 0-1,1-2 | pred reduce - --via path.json  # read from stdin

Input: a problem JSON from `pred create`. Use - to read from stdin.
The --via file must be one explicit entry selected by the caller from `pred path` output.
Output is a reduction bundle with source, target, and path.
Use `pred solve reduced.json` to solve and map the solution back.")]
pub struct ReduceArgs {
    /// Problem JSON file (from `pred create`). Use - for stdin.
    pub input: PathBuf,
    /// Explicit reduction route selected from a path-set entry.
    #[arg(long, required = true)]
    pub via: PathBuf,
}

#[derive(clap::Args)]
pub struct ExtractArgs {
    /// Reduction bundle JSON (from `pred reduce`). Use - for stdin.
    pub input: PathBuf,
    /// Target-space configuration to map back (comma-separated, e.g. 1,0,1,0)
    #[arg(long)]
    pub config: String,
}

#[derive(clap::Args)]
pub struct InspectArgs {
    /// Problem JSON file or reduction bundle. Use - for stdin.
    pub input: PathBuf,
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred evaluate problem.json --config 1,0,1,0
  pred evaluate problem.json --config 1,0,1,0 -o result.json
  pred create MIS --graph 0-1,1-2 | pred evaluate - --config 1,0,1  # read from stdin

Input: a problem JSON from `pred create`. Use - to read from stdin.")]
pub struct EvaluateArgs {
    /// Problem JSON file (from `pred create`). Use - for stdin.
    pub input: PathBuf,
    /// Configuration to evaluate (comma-separated, e.g., 1,0,1,0)
    #[arg(long)]
    pub config: String,
}

/// Print the after_help text for a subcommand on parse error.
///
/// Only matches the first line of the error message. Without this,
/// bare `pred` (no subcommand) would match "pred solve" in the
/// top-level workflow examples and incorrectly append the solve
/// subcommand's help text.
pub fn print_subcommand_help_hint(error_msg: &str) {
    let first_line = error_msg.lines().next().unwrap_or("");
    let subcmds = [
        ("pred solve", "solve"),
        ("pred reduce", "reduce"),
        ("pred extract", "extract"),
        ("pred create", "create"),
        ("pred evaluate", "evaluate"),
        ("pred inspect", "inspect"),
        ("pred path", "path"),
        ("pred show", "show"),
        ("pred to", "to"),
        ("pred from", "from"),
        ("pred export-graph", "export-graph"),
    ];
    let cmd = Cli::command();
    for (pattern, name) in subcmds {
        if first_line.contains(pattern) {
            if let Some(sub) = cmd.find_subcommand(name) {
                if let Some(help) = sub.get_after_help() {
                    eprintln!("\n{help}");
                }
            }
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{error::ErrorKind, Parser};

    #[test]
    fn dynamic_create_parser_uses_bounded_stack() {
        std::thread::Builder::new()
            .stack_size(1024 * 1024)
            .spawn(|| {
                assert_eq!(
                    Cli::try_parse_from(["pred", "--version"])
                        .err()
                        .expect("version exits")
                        .kind(),
                    ErrorKind::DisplayVersion
                );
                Cli::try_parse_from(["pred", "list"]).expect("list parses");
                Cli::try_parse_from(["pred", "create", "MIS", "--graph", "0-1"])
                    .expect("MIS parses");
                Cli::try_parse_from([
                    "pred",
                    "create",
                    "SAT",
                    "--num-vars",
                    "2",
                    "--clauses",
                    "1,2",
                ])
                .expect("SAT parses");
                assert_eq!(
                    Cli::try_parse_from(["pred", "create", "MIS", "--clauses", "1,2"])
                        .err()
                        .expect("unrelated flag is rejected")
                        .kind(),
                    ErrorKind::UnknownArgument
                );
                assert_eq!(
                    Cli::try_parse_from([
                        "pred", "create", "KClique", "--graph", "0-1", "--k", "nope"
                    ])
                    .err()
                    .expect("invalid numeric value is rejected by Clap")
                    .kind(),
                    ErrorKind::ValueValidation
                );
                Cli::try_parse_from([
                    "pred",
                    "create",
                    "ShortestCommonSupersequence",
                    "--strings",
                    "0,1;1,2",
                ])
                .expect("derived SCS fields are not required as inputs");
                assert_eq!(
                    Cli::try_parse_from([
                        "pred",
                        "create",
                        "LCS",
                        "--strings",
                        "0,1;1,2",
                        "--max-length",
                        "99",
                    ])
                    .err()
                    .expect("derived LCS max_length is not exposed")
                    .kind(),
                    ErrorKind::UnknownArgument
                );
                Cli::try_parse_from([
                    "pred",
                    "create",
                    "ThreePartition",
                    "--sizes",
                    "1,1,1",
                    "--bound",
                    "18446744073709551615",
                ])
                .expect("u64 construction inputs are typed by schema, not by flag name");
                Cli::try_parse_from([
                    "pred",
                    "create",
                    "MinimumCodeGenerationOneRegister",
                    "--edges",
                    "0>1",
                ])
                .expect("ordinary edges fields keep their schema-derived name");
                crate::create_args::with_static_completion_schema(|| {
                    Cli::command().debug_assert();
                });
            })
            .expect("spawn parser thread")
            .join()
            .expect("parser thread completes");
    }

    #[test]
    fn dynamic_create_parser_preserves_problem_and_variant_aliases() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "3SAT",
            "--num-vars",
            "3",
            "--clauses",
            "1,2,3",
        ])
        .expect("variant alias parses");
        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };
        assert_eq!(args.problem.as_deref(), Some("KSatisfiability/K3"));
        assert_eq!(args.raw("num-vars"), Some("3"));
    }

    #[test]
    fn dynamic_create_parser_builds_every_registered_subcommand() {
        Cli::command().debug_assert();
    }
}

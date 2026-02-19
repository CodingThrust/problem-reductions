use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "pred",
    about = "Explore NP-hard problem reductions",
    version,
    after_help = "\
Typical workflow:
  pred create MIS --edges 0-1,1-2,2-3 -o problem.json
  pred solve problem.json
  pred evaluate problem.json --config 1,0,1,0

Piping (use - to read from stdin):
  pred create MIS --edges 0-1,1-2 | pred solve -
  pred create MIS --edges 0-1,1-2 | pred evaluate - --config 1,0,1
  pred create MIS --edges 0-1,1-2 | pred reduce - --to QUBO

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

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all registered problem types
    #[command(after_help = "\
Examples:
  pred list                   # print to terminal
  pred list -o problems.json  # save as JSON")]
    List,

    /// Show details for a problem type (variants, fields, reductions)
    #[command(after_help = "\
Examples:
  pred show MIS                   # using alias
  pred show MaximumIndependentSet # full name
  pred show MIS/UnitDiskGraph     # specific graph variant

Use `pred list` to see all available problem types and aliases.
Use `pred to MIS --hops 2` to explore outgoing neighbors.
Use `pred from QUBO --hops 1` to explore incoming neighbors.")]
    Show {
        /// Problem name or alias (e.g., MIS, QUBO, MIS/UnitDiskGraph)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        problem: String,
    },

    /// Explore outgoing neighbors in the reduction graph (problems this reduces TO)
    #[command(after_help = "\
Examples:
  pred to MIS              # 1-hop outgoing neighbors
  pred to MIS --hops 2     # 2-hop outgoing neighbors
  pred to MIS -o out.json  # save as JSON

Use `pred from <problem>` for incoming neighbors.")]
    To {
        /// Problem name or alias (e.g., MIS, QUBO, MIS/UnitDiskGraph)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        problem: String,
        /// Number of hops to explore [default: 1]
        #[arg(long, default_value = "1")]
        hops: usize,
    },

    /// Explore incoming neighbors in the reduction graph (problems that reduce FROM this)
    #[command(after_help = "\
Examples:
  pred from QUBO              # 1-hop incoming neighbors
  pred from QUBO --hops 2     # 2-hop incoming neighbors
  pred from QUBO -o in.json   # save as JSON

Use `pred to <problem>` for outgoing neighbors.")]
    From {
        /// Problem name or alias (e.g., MIS, QUBO, MIS/UnitDiskGraph)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        problem: String,
        /// Number of hops to explore [default: 1]
        #[arg(long, default_value = "1")]
        hops: usize,
    },

    /// Find the cheapest reduction path between two problems
    #[command(after_help = "\
Examples:
  pred path MIS QUBO                              # cheapest path
  pred path MIS QUBO --all                        # all paths
  pred path MIS QUBO -o path.json                 # save for `pred reduce --via`
  pred path MIS QUBO --all -o paths/              # save all paths to a folder
  pred path MIS QUBO --cost minimize:num_variables

Use `pred list` to see available problems.")]
    Path {
        /// Source problem (e.g., MIS, MIS/UnitDiskGraph)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        source: String,
        /// Target problem (e.g., QUBO)
        #[arg(value_parser = crate::problem_name::ProblemNameParser)]
        target: String,
        /// Cost function [default: minimize-steps]
        #[arg(long, default_value = "minimize-steps")]
        cost: String,
        /// Show all paths instead of just the cheapest
        #[arg(long)]
        all: bool,
    },

    /// Export the reduction graph to JSON
    #[command(after_help = "\
Examples:
  pred export-graph                           # print to stdout
  pred export-graph -o reduction_graph.json   # save to file")]
    ExportGraph,

    /// Create a problem instance and save as JSON
    Create(CreateArgs),
    /// Evaluate a configuration against a problem instance JSON file
    Evaluate(EvaluateArgs),
    /// Reduce a problem instance to a target type
    Reduce(ReduceArgs),
    /// Inspect a problem JSON or reduction bundle
    #[command(after_help = "\
Examples:
  pred inspect problem.json
  pred inspect bundle.json
  pred create MIS --edges 0-1,1-2 | pred inspect -")]
    Inspect(InspectArgs),
    /// Solve a problem instance
    Solve(SolveArgs),
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

#[derive(clap::Args)]
#[command(after_help = "\
Options by problem type:
  Graph problems (MIS, MVC, MaxCut, MaxClique, ...):
    --edges       Edge list, e.g., 0-1,1-2,2-3 [required]
    --weights     Vertex weights, e.g., 2,1,3,1 [default: all 1s]
  SAT problems (SAT, 3SAT, KSAT):
    --num-vars    Number of variables [required]
    --clauses     Semicolon-separated clauses, e.g., \"1,2;-1,3\" [required]
  QUBO:
    --matrix      Semicolon-separated rows, e.g., \"1,0.5;0.5,2\" [required]
  KColoring:
    --edges       Edge list [required]
    --k           Number of colors [required]

Random generation (graph-based problems only):
  --random        Generate a random Erdos-Renyi graph instance
  --num-vertices  Number of vertices [required with --random]
  --edge-prob     Edge probability (0.0 to 1.0) [default: 0.5]
  --seed          Random seed for reproducibility

Examples:
  pred create MIS --edges 0-1,1-2,2-3 -o problem.json
  pred create MIS --edges 0-1,1-2 --weights 2,1,3 -o weighted.json
  pred create SAT --num-vars 3 --clauses \"1,2;-1,3\" -o sat.json
  pred create QUBO --matrix \"1,0.5;0.5,2\" -o qubo.json
  pred create KColoring --k 3 --edges 0-1,1-2,2-0 -o kcol.json
  pred create MIS --random --num-vertices 10 --edge-prob 0.3
  pred create MIS --random --num-vertices 10 --seed 42 -o big.json

Output (`-o`) uses the standard problem JSON format:
  {\"type\": \"...\", \"variant\": {...}, \"data\": {...}}")]
pub struct CreateArgs {
    /// Problem type (e.g., MIS, QUBO, SAT)
    #[arg(value_parser = crate::problem_name::ProblemNameParser)]
    pub problem: String,
    /// Edges for graph problems (e.g., 0-1,1-2,2-3)
    #[arg(long)]
    pub edges: Option<String>,
    /// Vertex weights (e.g., 1,1,1,1) [default: all 1s]
    #[arg(long)]
    pub weights: Option<String>,
    /// Clauses for SAT problems (semicolon-separated, e.g., "1,2;-1,3")
    #[arg(long)]
    pub clauses: Option<String>,
    /// Number of variables (for SAT/KSAT)
    #[arg(long)]
    pub num_vars: Option<usize>,
    /// Matrix for QUBO (semicolon-separated rows, e.g., "1,0.5;0.5,2")
    #[arg(long)]
    pub matrix: Option<String>,
    /// Number of colors for KColoring
    #[arg(long)]
    pub k: Option<usize>,
    /// Generate a random instance (graph-based problems only)
    #[arg(long)]
    pub random: bool,
    /// Number of vertices for random graph generation
    #[arg(long)]
    pub num_vertices: Option<usize>,
    /// Edge probability for random graph generation (0.0 to 1.0) [default: 0.5]
    #[arg(long)]
    pub edge_prob: Option<f64>,
    /// Random seed for reproducibility
    #[arg(long)]
    pub seed: Option<u64>,
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred solve problem.json                        # ILP solver (default, auto-reduces to ILP)
  pred solve problem.json --solver brute-force   # brute-force (exhaustive search)
  pred solve reduced.json                        # solve a reduction bundle
  pred solve reduced.json -o solution.json       # save result to file
  pred create MIS --edges 0-1,1-2 | pred solve - # read from stdin

Typical workflow:
  pred create MIS --edges 0-1,1-2,2-3 -o problem.json
  pred solve problem.json

Solve via explicit reduction:
  pred reduce problem.json --to QUBO -o reduced.json
  pred solve reduced.json

Input: a problem JSON from `pred create`, or a reduction bundle from `pred reduce`.
When given a bundle, the target is solved and the solution is mapped back to the source.
The ILP solver auto-reduces non-ILP problems before solving.

ILP backend (default: HiGHS). To use a different backend:
  cargo install problemreductions-cli --features coin-cbc
  cargo install problemreductions-cli --features scip
  cargo install problemreductions-cli --no-default-features --features clarabel")]
pub struct SolveArgs {
    /// Problem JSON file (from `pred create`) or reduction bundle (from `pred reduce`). Use - for stdin.
    pub input: PathBuf,
    /// Solver: ilp (default) or brute-force
    #[arg(long, default_value = "ilp")]
    pub solver: String,
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred reduce problem.json --to QUBO -o reduced.json
  pred reduce problem.json --to ILP -o reduced.json
  pred reduce problem.json --via path.json -o reduced.json
  pred create MIS --edges 0-1,1-2 | pred reduce - --to QUBO  # read from stdin

Input: a problem JSON from `pred create`. Use - to read from stdin.
The --via path file is from `pred path <SRC> <DST> -o path.json`.
When --via is given, --to is inferred from the path file.
Output is a reduction bundle with source, target, and path.
Use `pred solve reduced.json` to solve and map the solution back.")]
pub struct ReduceArgs {
    /// Problem JSON file (from `pred create`). Use - for stdin.
    pub input: PathBuf,
    /// Target problem type (e.g., QUBO, SpinGlass). Inferred from --via if omitted.
    #[arg(long, value_parser = crate::problem_name::ProblemNameParser)]
    pub to: Option<String>,
    /// Reduction route file (from `pred path ... -o`)
    #[arg(long)]
    pub via: Option<PathBuf>,
    /// Output raw JSON to stdout instead of human-readable summary
    #[arg(long)]
    pub json: bool,
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
  pred create MIS --edges 0-1,1-2 | pred evaluate - --config 1,0,1  # read from stdin

Input: a problem JSON from `pred create`. Use - to read from stdin.")]
pub struct EvaluateArgs {
    /// Problem JSON file (from `pred create`). Use - for stdin.
    pub input: PathBuf,
    /// Configuration to evaluate (comma-separated, e.g., 1,0,1,0)
    #[arg(long)]
    pub config: String,
}

/// Print the after_help text for a subcommand on parse error.
pub fn print_subcommand_help_hint(error_msg: &str) {
    let subcmds = [
        ("pred solve", "solve"),
        ("pred reduce", "reduce"),
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
        if error_msg.contains(pattern) {
            if let Some(sub) = cmd.find_subcommand(name) {
                if let Some(help) = sub.get_after_help() {
                    eprintln!("\n{help}");
                }
            }
            return;
        }
    }
}

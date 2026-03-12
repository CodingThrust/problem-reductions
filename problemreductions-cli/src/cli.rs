use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;

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
  pred create MIS --graph 0-1,1-2 | pred solve -
  pred create MIS --graph 0-1,1-2 | pred evaluate - --config 1,0,1
  pred create MIS --graph 0-1,1-2 | pred reduce - --to QUBO

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
Use `pred to MIS --hops 2` to explore what reduces to MIS.
Use `pred from QUBO --hops 1` to explore what QUBO reduces to.")]
    Show {
        /// Problem name or alias (e.g., MIS, QUBO, MIS/UnitDiskGraph)
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

#[derive(clap::Args)]
#[command(after_help = "\
TIP: Run `pred create <PROBLEM>` (no other flags) to see problem-specific help.
     Not every flag applies to every problem — the above list shows ALL flags.

Flags by problem type:
  MIS, MVC, MaxClique, MinDomSet  --graph, --weights
  MaxCut, MaxMatching, TSP        --graph, --edge-weights
  MaximalIS                       --graph, --weights
  SAT, 3SAT/KSAT                  --num-vars, --clauses [--k]
  QUBO                            --matrix
  SpinGlass                       --graph, --couplings, --fields
  KColoring                       --graph, --k
  Factoring                       --target, --m, --n
  BinPacking                      --sizes, --capacity
  PaintShop                       --sequence
  MaximumSetPacking               --sets [--weights]
  MinimumSetCovering              --universe, --sets [--weights]
  X3C (ExactCoverBy3Sets)         --universe, --sets (3 elements each)
  BicliqueCover                   --left, --right, --biedges, --k
  BMF                             --matrix (0/1), --rank
  CVP                             --basis, --target-vec [--bounds]
  ILP, CircuitSAT                 (via reduction only)

Geometry graph variants (use slash notation, e.g., MIS/KingsSubgraph):
  KingsSubgraph, TriangularSubgraph   --positions (integer x,y pairs)
  UnitDiskGraph                        --positions (float x,y pairs) [--radius]

Random generation:
  --random --num-vertices N [--edge-prob 0.5] [--seed 42]

Examples:
  pred create MIS --graph 0-1,1-2,2-3 --weights 1,1,1
  pred create SAT --num-vars 3 --clauses \"1,2;-1,3\"
  pred create QUBO --matrix \"1,0.5;0.5,2\"
  pred create MIS/KingsSubgraph --positions \"0,0;1,0;1,1;0,1\"
  pred create MIS/UnitDiskGraph --positions \"0,0;1,0;0.5,0.8\" --radius 1.5
  pred create MIS --random --num-vertices 10 --edge-prob 0.3")]
pub struct CreateArgs {
    /// Problem type (e.g., MIS, QUBO, SAT)
    #[arg(value_parser = crate::problem_name::ProblemNameParser)]
    pub problem: String,
    /// Graph edge list (e.g., 0-1,1-2,2-3)
    #[arg(long)]
    pub graph: Option<String>,
    /// Vertex weights (e.g., 1,1,1,1) [default: all 1s]
    #[arg(long)]
    pub weights: Option<String>,
    /// Edge weights (e.g., 2,3,1) [default: all 1s]
    #[arg(long)]
    pub edge_weights: Option<String>,
    /// Pairwise couplings J_ij for SpinGlass (e.g., 1,-1,1) [default: all 1s]
    #[arg(long)]
    pub couplings: Option<String>,
    /// On-site fields h_i for SpinGlass (e.g., 0,0,1) [default: all 0s]
    #[arg(long)]
    pub fields: Option<String>,
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
    /// Target number to factor (for Factoring)
    #[arg(long)]
    pub target: Option<u64>,
    /// Bits for first factor (for Factoring)
    #[arg(long)]
    pub m: Option<usize>,
    /// Bits for second factor (for Factoring)
    #[arg(long)]
    pub n: Option<usize>,
    /// Vertex positions for geometry-based graphs (semicolon-separated x,y pairs, e.g., "0,0;1,0;1,1")
    #[arg(long)]
    pub positions: Option<String>,
    /// Radius for UnitDiskGraph [default: 1.0]
    #[arg(long)]
    pub radius: Option<f64>,
    /// Item sizes for BinPacking (comma-separated, e.g., "3,3,2,2")
    #[arg(long)]
    pub sizes: Option<String>,
    /// Bin capacity for BinPacking
    #[arg(long)]
    pub capacity: Option<String>,
    /// Car paint sequence for PaintShop (comma-separated, each label appears exactly twice, e.g., "a,b,a,c,c,b")
    #[arg(long)]
    pub sequence: Option<String>,
    /// Sets for SetPacking/SetCovering (semicolon-separated, e.g., "0,1;1,2;0,2")
    #[arg(long)]
    pub sets: Option<String>,
    /// Universe size for MinimumSetCovering
    #[arg(long)]
    pub universe: Option<usize>,
    /// Bipartite graph edges for BicliqueCover (e.g., "0-0,0-1,1-2" for left-right pairs)
    #[arg(long)]
    pub biedges: Option<String>,
    /// Left partition size for BicliqueCover
    #[arg(long)]
    pub left: Option<usize>,
    /// Right partition size for BicliqueCover
    #[arg(long)]
    pub right: Option<usize>,
    /// Rank for BMF
    #[arg(long)]
    pub rank: Option<usize>,
    /// Lattice basis for CVP (semicolon-separated column vectors, e.g., "1,0;0,1")
    #[arg(long)]
    pub basis: Option<String>,
    /// Target vector for CVP (comma-separated, e.g., "0.5,0.5")
    #[arg(long)]
    pub target_vec: Option<String>,
    /// Variable bounds for CVP as "lower,upper" (e.g., "-10,10") [default: -10,10]
    #[arg(long, allow_hyphen_values = true)]
    pub bounds: Option<String>,
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred solve problem.json                        # ILP solver (default, auto-reduces to ILP)
  pred solve problem.json --solver brute-force   # brute-force (exhaustive search)
  pred solve reduced.json                        # solve a reduction bundle
  pred solve reduced.json -o solution.json       # save result to file
  pred create MIS --graph 0-1,1-2 | pred solve - # read from stdin
  pred solve problem.json --timeout 10           # abort after 10 seconds

Typical workflow:
  pred create MIS --graph 0-1,1-2,2-3 -o problem.json
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
    /// Timeout in seconds (0 = no limit)
    #[arg(long, default_value = "0")]
    pub timeout: u64,
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred reduce problem.json --to QUBO -o reduced.json
  pred reduce problem.json --to ILP -o reduced.json
  pred reduce problem.json --via path.json -o reduced.json
  pred create MIS --graph 0-1,1-2 | pred reduce - --to QUBO  # read from stdin

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

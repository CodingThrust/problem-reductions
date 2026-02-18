use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pred", about = "Explore NP-hard problem reductions", version)]
pub struct Cli {
    /// Output file path (implies JSON output)
    #[arg(long, short, global = true)]
    pub output: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all registered problem types
    List,
    /// Show details for a problem type (variants, fields, reductions)
    #[command(after_help = "Example: pred show MIS")]
    Show {
        /// Problem name (e.g., MIS, QUBO, MIS/UnitDiskGraph)
        problem: String,
    },
    /// Find the cheapest reduction path between two problems
    #[command(after_help = "Example: pred path MIS QUBO\n        pred path MIS QUBO --all")]
    Path {
        /// Source problem (e.g., MIS, MIS/UnitDiskGraph)
        source: String,
        /// Target problem (e.g., QUBO)
        target: String,
        /// Cost function: "minimize-steps" (default) or "minimize:<field>"
        #[arg(long, default_value = "minimize-steps")]
        cost: String,
        /// Show all paths instead of just the cheapest
        #[arg(long)]
        all: bool,
    },
    /// Export the reduction graph to JSON
    #[command(after_help = "Example: pred export-graph reduction_graph.json")]
    ExportGraph {
        /// Output file path
        output: PathBuf,
    },
    /// Create a problem instance and save as JSON
    Create(CreateArgs),
    /// Evaluate a configuration against a problem instance JSON file
    Evaluate(EvaluateArgs),
    /// Reduce a problem instance to a target type
    Reduce(ReduceArgs),
    /// Solve a problem instance (not yet implemented)
    Solve(SolveArgs),
}

#[derive(clap::Args)]
#[command(after_help = "\
Examples:
  pred create MIS --edges 0-1,1-2,2-3 -o problem.json
  pred create SAT --num-vars 3 --clauses \"1,2;-1,3\" -o sat.json
  pred create QUBO --matrix \"1,0.5;0.5,2\" -o qubo.json")]
pub struct CreateArgs {
    /// Problem type (e.g., MIS, QUBO, SAT)
    pub problem: String,
    /// Edges for graph problems (e.g., 0-1,1-2,2-3)
    #[arg(long)]
    pub edges: Option<String>,
    /// Weights (e.g., 1,1,1,1)
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
}

#[derive(clap::Args)]
pub struct SolveArgs {
    /// Path to a JSON problem file
    pub input: Option<PathBuf>,
    /// Problem type for inline construction (e.g., MIS)
    #[arg(long)]
    pub problem: Option<String>,
    /// Edges for inline graph problems (e.g., 0-1,1-2,2-0)
    #[arg(long)]
    pub edges: Option<String>,
    /// Weights for inline problems (e.g., 1,1,1)
    #[arg(long)]
    pub weights: Option<String>,
    /// Target problem to reduce to before solving
    #[arg(long)]
    pub via: Option<String>,
    /// Solver to use
    #[arg(long, default_value = "brute-force")]
    pub solver: String,
}

#[derive(clap::Args)]
#[command(after_help = "Example: pred reduce problem.json --to QUBO -o reduced.json")]
pub struct ReduceArgs {
    /// Path to a problem JSON file (created via `pred create`)
    pub input: PathBuf,
    /// Target problem type (e.g., QUBO, SpinGlass)
    #[arg(long)]
    pub to: String,
}

#[derive(clap::Args)]
#[command(after_help = "Example: pred evaluate problem.json --config 1,0,1,0")]
pub struct EvaluateArgs {
    /// Path to a problem JSON file (created via `pred create`)
    pub input: PathBuf,
    /// Configuration to evaluate (comma-separated, e.g., 1,0,1)
    #[arg(long)]
    pub config: String,
}

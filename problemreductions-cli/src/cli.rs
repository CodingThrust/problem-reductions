use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pred", about = "Explore NP-hard problem reductions", version)]
pub struct Cli {
    /// Output as JSON (saved to file)
    #[arg(long, global = true)]
    pub json: bool,

    /// Output file path (used with --json)
    #[arg(long, short, global = true)]
    pub output: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Explore the reduction graph
    Graph {
        #[command(subcommand)]
        command: GraphCommands,
    },
    /// Solve a problem instance via reduction
    Solve(SolveArgs),
    /// Reduce a problem to a target type
    Reduce(ReduceArgs),
    /// Evaluate a configuration against a problem
    Evaluate(EvaluateArgs),
    /// Show the JSON schema for a problem type
    Schema(SchemaArgs),
}

#[derive(Subcommand)]
pub enum GraphCommands {
    /// List all registered problem types
    List,
    /// Show details for a problem type
    Show {
        /// Problem name (e.g., MIS, QUBO, MIS/UnitDiskGraph)
        problem: String,
    },
    /// Find the cheapest reduction path
    Path {
        /// Source problem (e.g., MIS, MIS/UnitDiskGraph)
        source: String,
        /// Target problem (e.g., QUBO)
        target: String,
        /// Cost function: "minimize-steps" (default) or "minimize:<field>"
        #[arg(long, default_value = "minimize-steps")]
        cost: String,
    },
    /// Export the reduction graph to JSON
    Export {
        /// Output file path (default: reduction_graph.json)
        #[arg(default_value = "reduction_graph.json")]
        output: PathBuf,
    },
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
pub struct ReduceArgs {
    /// Path to a JSON problem file
    pub input: PathBuf,
    /// Target problem type
    #[arg(long)]
    pub to: String,
}

#[derive(clap::Args)]
pub struct EvaluateArgs {
    /// Path to a JSON problem file
    pub input: PathBuf,
    /// Configuration to evaluate (comma-separated, e.g., 1,0,1)
    #[arg(long)]
    pub config: String,
}

#[derive(clap::Args)]
pub struct SchemaArgs {
    /// Problem name (e.g., MIS, QUBO)
    pub problem: String,
}

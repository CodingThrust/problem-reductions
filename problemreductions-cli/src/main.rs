mod cli;
mod commands;
mod dispatch;
mod output;
mod problem_name;

use cli::{Cli, Commands};
use clap::Parser;
use output::OutputConfig;

fn main() -> anyhow::Result<()> {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            // Let --help and --version print normally
            if e.kind() == clap::error::ErrorKind::DisplayHelp
                || e.kind() == clap::error::ErrorKind::DisplayVersion
            {
                e.exit();
            }
            let msg = e.to_string();
            eprint!("{e}");
            // Append usage examples based on which subcommand failed
            let hint = if msg.contains("pred solve") {
                Some("\
Examples:
  pred solve problem.json
  pred solve problem.json --solver brute-force
  pred solve reduced.json
Use `pred create` to create a problem, or `pred reduce` to create a bundle.")
            } else if msg.contains("pred reduce") {
                Some("\
Examples:
  pred reduce problem.json --to QUBO -o reduced.json
  pred reduce problem.json --to QUBO --via path.json -o reduced.json
Use `pred create` to create a problem instance first.
Use `pred path <SOURCE> <TARGET> -o path.json` to generate a path file.")
            } else if msg.contains("pred create") {
                Some("Example: pred create MIS --edges 0-1,1-2,2-3 -o problem.json")
            } else if msg.contains("pred evaluate") {
                Some("Example: pred evaluate problem.json --config 1,0,1,0")
            } else if msg.contains("pred path") {
                Some("Example: pred path MIS QUBO")
            } else if msg.contains("pred show") {
                Some("Example: pred show MIS")
            } else if msg.contains("pred export-graph") {
                Some("Example: pred export-graph reduction_graph.json")
            } else {
                None
            };
            if let Some(hint) = hint {
                eprintln!("\n{hint}");
            }
            std::process::exit(e.exit_code());
        }
    };

    let out = OutputConfig {
        output: cli.output,
    };

    match cli.command {
        Commands::List => commands::graph::list(&out),
        Commands::Show { problem } => commands::graph::show(&problem, &out),
        Commands::Path {
            source,
            target,
            cost,
            all,
        } => commands::graph::path(&source, &target, &cost, all, &out),
        Commands::ExportGraph { output } => commands::graph::export(&output),
        Commands::Create(args) => commands::create::create(&args, &out),
        Commands::Solve(args) => commands::solve::solve(&args.input, &args.solver, &out),
        Commands::Reduce(args) => {
            commands::reduce::reduce(&args.input, &args.to, args.via.as_deref(), &out)
        }
        Commands::Evaluate(args) => commands::evaluate::evaluate(&args.input, &args.config, &out),
    }
}

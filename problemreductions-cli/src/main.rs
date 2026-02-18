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
            if msg.contains("pred path") {
                eprintln!("Example: pred path MIS QUBO");
            } else if msg.contains("pred show") {
                eprintln!("Example: pred show MIS");
            } else if msg.contains("pred create") {
                eprintln!("Example: pred create MIS --edges 0-1,1-2,2-3 -o problem.json");
            } else if msg.contains("pred evaluate") {
                eprintln!("Example: pred evaluate problem.json --config 1,0,1,0");
            } else if msg.contains("pred reduce") {
                eprintln!("Example: pred reduce problem.json --to QUBO -o reduced.json");
            } else if msg.contains("pred export-graph") {
                eprintln!("Example: pred export-graph reduction_graph.json");
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
        Commands::Solve(_args) => {
            anyhow::bail!("The 'solve' command is not yet implemented")
        }
        Commands::Reduce(args) => commands::reduce::reduce(&args.input, &args.to, &out),
        Commands::Evaluate(args) => commands::evaluate::evaluate(&args.input, &args.config, &out),
    }
}

mod cli;
mod commands;
mod dispatch;
mod output;
mod problem_name;

use cli::{Cli, Commands, GraphCommands};
use clap::Parser;
use output::OutputConfig;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let out = OutputConfig {
        json: cli.json,
        output: cli.output,
    };

    match cli.command {
        Commands::Graph { command } => match command {
            GraphCommands::List => commands::graph::list(&out),
            GraphCommands::Show { problem } => commands::graph::show(&problem, &out),
            GraphCommands::Path {
                source,
                target,
                cost,
            } => commands::graph::path(&source, &target, &cost, &out),
            GraphCommands::Export { output } => commands::graph::export(&output),
        },
        Commands::Solve(_args) => todo!("solve"),
        Commands::Reduce(args) => commands::reduce::reduce(&args.input, &args.to, &out),
        Commands::Evaluate(args) => commands::evaluate::evaluate(&args.input, &args.config, &out),
        Commands::Schema(args) => commands::schema::schema(&args.problem, &out),
    }
}

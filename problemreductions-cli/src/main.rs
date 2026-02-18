mod cli;
mod commands;
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
            GraphCommands::Show { problem, variants } => {
                commands::graph::show(&problem, variants, &out)
            }
            GraphCommands::Path {
                source,
                target,
                cost,
            } => commands::graph::path(&source, &target, &cost, &out),
            GraphCommands::Export { output } => commands::graph::export(&output),
        },
        Commands::Solve(args) => todo!("solve"),
        Commands::Reduce(args) => todo!("reduce"),
        Commands::Evaluate(args) => todo!("evaluate"),
        Commands::Schema(args) => todo!("schema"),
    }
}

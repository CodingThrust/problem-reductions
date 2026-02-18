mod cli;
mod commands;
mod dispatch;
mod output;
mod problem_name;

use cli::{Cli, Commands};
use clap::{CommandFactory, Parser};
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
            // Show the subcommand's after_help (defined once in cli.rs)
            cli::print_subcommand_help_hint(&msg);
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
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "pred", &mut std::io::stdout());
            Ok(())
        }
    }
}

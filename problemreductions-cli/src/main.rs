mod cli;
mod commands;
mod create_args;
mod dispatch;
#[cfg(feature = "mcp")]
mod mcp;
mod output;
mod problem_name;
#[cfg(test)]
mod test_support;
mod util;

use clap::CommandFactory;
use cli::{Cli, Commands};
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

    // Data-producing commands auto-output JSON when piped
    let auto_json = matches!(
        cli.command,
        Commands::Reduce(_)
            | Commands::Solve(_)
            | Commands::Evaluate(_)
            | Commands::Inspect(_)
            | Commands::Extract(_)
    );

    let out = OutputConfig {
        output: cli.output,
        quiet: cli.quiet,
        json: cli.json,
        auto_json,
    };

    match cli.command {
        Commands::List {
            query,
            rules,
            category,
            all,
            verbose,
        } => {
            if rules {
                commands::graph::list_rules(query.as_deref(), all, verbose, &out)
            } else {
                commands::graph::list(query.as_deref(), category, all, verbose, &out)
            }
        }
        Commands::Show { problem } => commands::graph::show(&problem, &out),
        Commands::To { problem, hops } => commands::graph::neighbors(&problem, hops, "in", &out),
        Commands::From { problem, hops } => commands::graph::neighbors(&problem, hops, "out", &out),
        Commands::Path {
            source,
            target,
            limit,
            unfiltered,
            instance,
        } => commands::graph::path(
            &source,
            &target,
            limit,
            unfiltered,
            instance.as_deref(),
            &out,
        ),
        Commands::ExportGraph => commands::graph::export(&out),
        Commands::Inspect(args) => commands::inspect::inspect(&args.input, &out),
        Commands::Create(args) => commands::create::create(&args, &out),
        Commands::Solve(args) => {
            commands::solve::solve(&args.input, args.solver.as_deref(), args.timeout, &out)
        }
        Commands::Reduce(args) => commands::reduce::reduce(&args.input, &args.via, &out),
        Commands::Evaluate(args) => commands::evaluate::evaluate(&args.input, &args.config, &out),
        Commands::Extract(args) => commands::extract::extract(&args.input, &args.config, &out),
        #[cfg(feature = "mcp")]
        Commands::Mcp => mcp::run(),
        Commands::Completions { shell } => {
            let shell = shell
                .or_else(clap_complete::Shell::from_env)
                .unwrap_or(clap_complete::Shell::Bash);
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "pred", &mut std::io::stdout());
            Ok(())
        }
    }
}

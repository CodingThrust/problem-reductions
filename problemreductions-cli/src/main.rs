use clap::Parser;

#[derive(Parser)]
#[command(name = "pred", about = "Explore NP-hard problem reductions")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Placeholder
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Version => {
            println!("pred {}", env!("CARGO_PKG_VERSION"));
        }
    }
    Ok(())
}

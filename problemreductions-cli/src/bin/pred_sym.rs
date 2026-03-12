use clap::{Parser, Subcommand};
use problemreductions::{big_o_normal_form, canonical_form, Expr, ProblemSize};

#[derive(Parser)]
#[command(
    name = "pred-sym",
    about = "Symbolic expression engine for problemreductions"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and echo an expression
    Parse {
        /// Expression string
        expr: String,
    },
    /// Compute exact canonical form
    Canon {
        /// Expression string
        expr: String,
    },
    /// Compute Big-O normal form
    BigO {
        /// Expression string
        #[arg(name = "expr")]
        expr: String,
    },
    /// Compare two expressions
    Compare {
        /// First expression
        a: String,
        /// Second expression
        b: String,
    },
    /// Evaluate an expression with variable bindings
    Eval {
        /// Expression string
        expr: String,
        /// Variable bindings (e.g., n=10,m=20)
        #[arg(long)]
        vars: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { expr } => {
            let parsed = Expr::parse(&expr);
            println!("{parsed}");
        }
        Commands::Canon { expr } => {
            let parsed = Expr::parse(&expr);
            match canonical_form(&parsed) {
                Ok(result) => println!("{result}"),
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::BigO { expr } => {
            let parsed = Expr::parse(&expr);
            match big_o_normal_form(&parsed) {
                Ok(result) => println!("O({result})"),
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Compare { a, b } => {
            let expr_a = Expr::parse(&a);
            let expr_b = Expr::parse(&b);
            let canon_a = canonical_form(&expr_a);
            let canon_b = canonical_form(&expr_b);
            let big_o_a = big_o_normal_form(&expr_a);
            let big_o_b = big_o_normal_form(&expr_b);

            println!("Expression A: {a}");
            println!("Expression B: {b}");
            if let (Ok(ca), Ok(cb)) = (&canon_a, &canon_b) {
                println!("Canonical A:  {ca}");
                println!("Canonical B:  {cb}");
                println!("Exact equal:  {}", ca == cb);
            }
            if let (Ok(ba), Ok(bb)) = (&big_o_a, &big_o_b) {
                println!("Big-O A:      O({ba})");
                println!("Big-O B:      O({bb})");
                println!("Big-O equal:  {}", ba == bb);
            }
        }
        Commands::Eval { expr, vars } => {
            let parsed = Expr::parse(&expr);
            let bindings: Vec<(&str, usize)> = vars
                .split(',')
                .filter_map(|pair| {
                    let mut parts = pair.splitn(2, '=');
                    let name = parts.next()?.trim();
                    let value: usize = parts.next()?.trim().parse().ok()?;
                    // Leak the name for &'static str compatibility
                    let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
                    Some((leaked, value))
                })
                .collect();
            let size = ProblemSize::new(bindings);
            let result = parsed.eval(&size);

            // Format as integer if it's a whole number
            if (result - result.round()).abs() < 1e-10 {
                println!("{}", result.round() as i64);
            } else {
                println!("{result}");
            }
        }
    }
}

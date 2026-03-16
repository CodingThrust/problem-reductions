use clap::{Parser, Subcommand};
use problemreductions::{big_o_normal_form, canonical_form, Expr, ProblemSize};

#[derive(Parser)]
#[command(
    name = "pred-sym",
    version,
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
        /// Output without O(...) wrapper
        #[arg(long)]
        raw: bool,
    },
    /// Compare two expressions (exits with code 1 if neither exact nor Big-O equal)
    Compare {
        /// First expression
        a: String,
        /// Second expression
        b: String,
    },
    /// Evaluate an expression with variable bindings.
    /// Supported functions: exp (e^x), log (natural log, base e), sqrt
    Eval {
        /// Expression string
        expr: String,
        /// Variable bindings (e.g., n=10,m=20)
        #[arg(long)]
        vars: String,
    },
}

fn parse_expr_or_exit(expr: &str) -> Expr {
    match Expr::try_parse(expr) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Error: failed to parse expression \"{expr}\": {e}");
            std::process::exit(2);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { expr } => {
            let parsed = parse_expr_or_exit(&expr);
            println!("{parsed}");
        }
        Commands::Canon { expr } => {
            let parsed = parse_expr_or_exit(&expr);
            match canonical_form(&parsed) {
                Ok(result) => println!("{result}"),
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::BigO { expr, raw } => {
            let parsed = parse_expr_or_exit(&expr);
            match big_o_normal_form(&parsed) {
                Ok(result) => {
                    if raw {
                        println!("{result}");
                    } else {
                        println!("O({result})");
                    }
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Compare { a, b } => {
            let expr_a = parse_expr_or_exit(&a);
            let expr_b = parse_expr_or_exit(&b);
            let canon_a = canonical_form(&expr_a);
            let canon_b = canonical_form(&expr_b);
            let big_o_a = big_o_normal_form(&expr_a);
            let big_o_b = big_o_normal_form(&expr_b);

            println!("Expression A: {a}");
            println!("Expression B: {b}");
            let mut exact_equal = false;
            let mut big_o_equal = false;
            if let (Ok(ca), Ok(cb)) = (&canon_a, &canon_b) {
                exact_equal = ca == cb;
                println!("Canonical A:  {ca}");
                println!("Canonical B:  {cb}");
                println!("Exact equal:  {exact_equal}");
            }
            if let (Ok(ba), Ok(bb)) = (&big_o_a, &big_o_b) {
                big_o_equal = ba == bb;
                println!("Big-O A:      O({ba})");
                println!("Big-O B:      O({bb})");
                println!("Big-O equal:  {big_o_equal}");
            }
            if !exact_equal && !big_o_equal {
                std::process::exit(1);
            }
        }
        Commands::Eval { expr, vars } => {
            let parsed = parse_expr_or_exit(&expr);
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

            // Check for unbound variables
            let expr_vars = parsed.variables();
            let bound_vars: std::collections::HashSet<&str> =
                bindings.iter().map(|(k, _)| *k).collect();
            let mut unbound: Vec<&str> = expr_vars
                .iter()
                .filter(|v| !bound_vars.contains(*v))
                .copied()
                .collect();
            if !unbound.is_empty() {
                unbound.sort();
                eprintln!(
                    "Error: unbound variable{}: {}",
                    if unbound.len() > 1 { "s" } else { "" },
                    unbound.join(", ")
                );
                std::process::exit(1);
            }

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

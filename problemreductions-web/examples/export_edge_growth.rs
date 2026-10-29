//! Certify every registered reduction's target-parameter growth for the website graph.
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    println!("{}", problemreductions_web::edge_growth(&input)?);
    Ok(())
}

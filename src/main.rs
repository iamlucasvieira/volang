mod eval;
mod parse;
use clap::Parser;
use std::process;

#[derive(Parser)]
#[command(author, version, about = "VOlang interpreter")]
struct Args {
    /// Input file containing expressions to evaluate
    #[arg(help = "Path to the input file")]
    filename: String,
}

fn main() {
    let args = Args::parse();
    
    let input = std::fs::read_to_string(&args.filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", args.filename, err);
        process::exit(1);
    });

    let parse_result = parse::parse_call(&input).unwrap_or_else(|err| {
        eprintln!("Error parsing expression: {}", err);
        process::exit(1);
    });

    let (_, expr) = parse_result;
    eval::eval(expr);
}

mod eval;
mod parse;
use clap::Parser;
use std::{collections::HashMap, process};

#[derive(Parser)]
#[command(author, version, about = "VOlang interpreter")]
struct Args {
    /// Input file containing expressions to evaluate
    #[arg(help = "Path to the input file")]
    filename: String,

    /// Debug mode
    #[arg(short, long, help = "Turn debug mode on")]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let input = std::fs::read_to_string(&args.filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", args.filename, err);
        process::exit(1);
    });

    if let Ok((_, exprs)) = parse::parser(&input) {
        let mut context = HashMap::new();
        if args.debug {
            dbg!(&exprs);
        }

        for e in exprs {
            eval::eval(e, &mut context);
        }
    }
}

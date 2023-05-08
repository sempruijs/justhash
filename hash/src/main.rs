use clap::Parser;
use sha256::digest;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///input to hash
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let input = args.input;
    let output = digest(input);
    println!("{}", output);
}

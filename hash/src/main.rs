use clap::Parser;
use sha256::digest;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///string as input to hash
    #[arg(short, long)]
    string: String,
}

fn main() {
    let args = Args::parse();
    let input = args.string;
    let output = digest(input);
    println!("{}", output);
}

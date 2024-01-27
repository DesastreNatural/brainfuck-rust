mod interpreter;

use clap::Parser;
use std::path::Path;
use std::fs;
#[derive(Parser)]
#[clap(author="https://github.com/DesastreNatural", version="0.1", about="A very simple Brainf*ck implementation in Rust")]
struct Cli {
    path: std::path::PathBuf,
}

use interpreter::interpreter::Interpreter;

fn main() {
    let args = Cli::parse();
    if Path::new(args.path.as_os_str()).exists() {
        println!("path: {:?}", args.path);
        let raw_data = fs::read_to_string(args.path).expect("Unable to read file");
        let allowed_tokens: Vec<char> = ['<','>','+','-','[',']','.',','].to_vec();
        let data: String = raw_data.chars().filter(|&c| allowed_tokens.contains(&c)).collect();
        let mut bf: Interpreter = Interpreter::created(data.chars().collect());
        bf.interpret();
    } else {
        eprintln!("Not a valid file.")
    }
}

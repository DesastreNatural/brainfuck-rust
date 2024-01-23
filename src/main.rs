use clap::Parser;
use std::path::Path;
use std::fs;

#[derive(Parser)]
#[clap(author="https://github.com/DesastreNatural", version="0.1", about="A very simple Brainf*ck implementation in Rust")]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    if Path::new(args.path.as_os_str()).exists() {
        println!("path: {:?}", args.path);
        let data = fs::read_to_string(args.path).expect("Unable to read file");
        println!("{}", data);
    } else {
        eprintln!("Not a valid file.")
    }
}

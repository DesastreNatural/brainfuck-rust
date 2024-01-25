use clap::Parser;
use std::path::Path;
use std::fs;

pub mod interpreter {
    pub struct Interpreter {
        code_source: Vec<char>,
        code_ptr: i32,
        mem_ptr: i32,
        mem_ptr_shift: i32,
        memory: Vec<i32>,
    }

    impl Interpreter {
        pub fn created(code: Vec<char>)-> Interpreter {
            println!("initializing interpreter with: {:?}",code);
            Interpreter {
                code_source: code,
                code_ptr: 0,
                mem_ptr: 0,
                mem_ptr_shift: 0,
                memory: vec![0] 
            }
        }
        pub fn print_memory(&self){ println!("{:?}",&self.memory); }
        pub fn inc_mem_value(&mut self){ 
            self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] += 1;
        }
        pub fn dec_mem_value(&mut self){
            self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] -= 1;
        }
        pub fn inc_mem_ptr(&mut self){
            self.mem_ptr += 1;
            if self.memory.len() == self.mem_ptr as usize  {
                self.memory.push(0);
            }
        }
        pub fn dec_mem_ptr(&mut self){
            self.mem_ptr -= 1;
            if (self.mem_ptr + self.mem_ptr_shift) == -1 {
                self.mem_ptr_shift += 1;
                self.memory.insert(0, 0)
            }
        }
        pub fn output_ptr(){ todo!(); }
        pub fn input_ptr(){ todo!(); }
    }
}

#[derive(Parser)]
#[clap(author="https://github.com/DesastreNatural", version="0.1", about="A very simple Brainf*ck implementation in Rust")]
struct Cli {
    path: std::path::PathBuf,
}

use crate::interpreter::Interpreter;

fn main() {
    let args = Cli::parse();
    if Path::new(args.path.as_os_str()).exists() {
        println!("path: {:?}", args.path);
        let raw_data = fs::read_to_string(args.path).expect("Unable to read file");
        let allowed_tokens: Vec<char> = ['<','>','+','-','[',']','.',','].to_vec();
        let data: String = raw_data.chars().filter(|&c| allowed_tokens.contains(&c)).collect();
        let mut bf: Interpreter = Interpreter::created(data.chars().collect());
        println!("{}", data);
        bf.print_memory(); 
        bf.inc_mem_ptr();
        bf.inc_mem_value();
        bf.inc_mem_ptr();
        bf.inc_mem_value();
        bf.dec_mem_ptr();
        bf.dec_mem_ptr();
        bf.dec_mem_ptr();
        bf.dec_mem_ptr();
        bf.inc_mem_ptr();
        bf.print_memory();
        
    } else {
        eprintln!("Not a valid file.")
    }
}

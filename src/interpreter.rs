pub mod interpreter {


    use getch::Getch;
    use std::process::exit;
    use std::io::{self, Write};
    use log::{debug, error, log_enabled, info, Level};

    pub struct Interpreter {
        code_source: Vec<char>,
        code_ptr: i32,
        mem_ptr: i32,
        mem_ptr_shift: i32,
        memory: Vec<u8>,
        running: bool,
    }

    impl Interpreter {
        pub fn created(code: Vec<char>)-> Interpreter {
            println!("initializing interpreter with: {:?}",code);
            env_logger::init();
            Interpreter {
                code_source: code,
                code_ptr: 0,
                mem_ptr: 0,
                mem_ptr_shift: 0,
                memory: vec![0],
                running: true
            }
        }
        pub fn print_memory(&self){
            println!("{:?}",&self.memory);
        }
        pub fn inc_mem_value(&mut self){
            if self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] == 255 {
                self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] = 0;
            } else {
                self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] += 1;
            }
        }
        pub fn dec_mem_value(&mut self){
            if self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] == 0 {
                self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] = 255;
            } else {
                self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] -= 1;
            }
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
        pub fn output_ptr(&self){
            print!("{}", self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] as char);
            io::stdout().flush().unwrap();
        }
        pub fn input_ptr(&mut self){ 
            let f = Getch::new();
            self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] = f.getch().unwrap();
        }
        pub fn jump_if_zero(&mut self){
            if self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] == 0 {
                let mut parity_point = -1;
                while parity_point != 0 {
                    self.code_ptr += 1;
                    if self.code_ptr == self.code_source.len() as i32 {
                        eprintln!("unmatched [");
                        exit(1);
                    }
                    if self.code_source[self.code_ptr as usize] == '[' {
                        parity_point -= 1
                    } else if self.code_source[self.code_ptr as usize] == ']' {
                        parity_point += 1
                    }
                }
            }
        }
        pub fn jump_back_if_not_zero(&mut self){
            if self.memory[(self.mem_ptr + self.mem_ptr_shift) as usize] != 0 {
                let mut parity_point = -1;
                while parity_point != 0 {
                    self.code_ptr -= 1;
                    if self.code_ptr < 0 {
                        eprintln!("unmatched ]");
                        exit(1);
                    }
                    if self.code_source[self.code_ptr as usize] == ']' {
                        parity_point -= 1
                    } else if self.code_source[self.code_ptr as usize] == '[' {
                        parity_point += 1
                    }
                }
            }
        }
        pub fn eval_ptr(&mut self){
            if self.code_ptr >= self.code_source.len() as i32 {
                self.running = false;
            } else {
                match self.code_source[self.code_ptr as usize] {
                    '<' => self.dec_mem_ptr(),
                    '>' => self.inc_mem_ptr(),
                    '+' => self.inc_mem_value(),
                    '-' => self.dec_mem_value(),
                    '[' => self.jump_if_zero(),
                    ']' => self.jump_back_if_not_zero(),
                    '.' => self.output_ptr(),
                    ',' => self.input_ptr(),
                    _ => todo!()
                }
            }
        }
        pub fn interpret(&mut self){
            while(self.running){
                self.eval_ptr();
                self.code_ptr += 1;
            }
            print!("\n");
        }
    }
}
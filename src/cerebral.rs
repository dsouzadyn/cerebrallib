use std::io;
use std::fmt;
use std::io::Read;
use std::collections::{HashMap, LinkedList};

pub struct CerebralVM {
    code: String,
    data_ptr: usize,
    instruction_ptr: usize,
    memory: [u8; 30000],
    bracs: HashMap<usize, usize>,
}

impl fmt::Debug for CerebralVM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ code: {}, data_ptr: {}, instruction_ptr: {} }}", self.code, self.data_ptr, self.instruction_ptr)
    }
}

impl CerebralVM {
    pub fn new(c: String) -> Self {
        let mut stk = LinkedList::new();
        let mut tbracs:HashMap<usize, usize> = HashMap::new();
        for (i, c) in c.chars().enumerate() {
            match c {
                '[' => stk.push_back(i),
                ']' => {
                    tbracs.insert(i, *stk.back().unwrap());
                    tbracs.insert(stk.pop_back().unwrap(), i);
                },
                _ => {}
            }
        }
        return CerebralVM {
            code: c,
            data_ptr: 0,
            instruction_ptr: 0,
            memory: [0; 30000],
            bracs: tbracs,
        }
    }
    pub fn get_data_ptr(&self) -> usize {
        return self.data_ptr;
    }
    pub fn get_instruction_ptr(&self) -> usize {
        return self.instruction_ptr;
    }
    pub fn inc_data(&mut self) {
        self.memory[self.data_ptr] += 1;
        self.instruction_ptr += 1;
    }
    pub fn dec_data(&mut self) {
        self.memory[self.data_ptr] -= 1;
        self.instruction_ptr += 1;
    }
    pub fn inc_data_ptr(&mut self) {
        self.data_ptr += 1;
        self.instruction_ptr += 1;
    }
    pub fn dec_data_ptr(&mut self) {
        self.data_ptr -= 1;
        self.instruction_ptr += 1;
    }
    pub fn jump_forward(&mut self) {
        if self.memory[self.data_ptr] == 0 {
            self.instruction_ptr = self.bracs[&self.instruction_ptr] + 1;
        } else {
            self.instruction_ptr += 1;
        }
    }
    pub fn jump_backward(&mut self) {
        if self.memory[self.data_ptr] != 0 {
            self.instruction_ptr = self.bracs[&self.instruction_ptr] + 1;
        } else {
            self.instruction_ptr += 1;
        }
    }
    pub fn print_data(&mut self) {
        print!("{}", self.memory[self.data_ptr] as char);
        self.instruction_ptr += 1;
    }
    pub fn read_data(&mut self) {
        self.memory[self.data_ptr] = io::stdin().bytes().next().and_then(|r| r.ok()).map(|b| b as u8).unwrap();
        self.instruction_ptr += 1;
    }
    pub fn execute(&mut self) {
        loop {
            if self.instruction_ptr == self.code.len() { break; }
            match self.code.as_bytes()[self.instruction_ptr] as char {
                '>' => self.inc_data_ptr(),
                '<' => self.dec_data_ptr(),
                '+' => self.inc_data(),
                '-' => self.dec_data(),
                '[' => self.jump_forward(),
                ']' => self.jump_backward(),
                '.' => self.print_data(),
                ',' => self.read_data(),
                _ => self.instruction_ptr += 1
            };
        }
        println!();
    }
}

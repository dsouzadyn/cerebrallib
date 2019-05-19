use std::fmt;
use std::io::{Read, Write};
use std::collections::{HashMap, LinkedList};

pub struct CerebralVM<I: Read, O: Write> {
    code: Vec<u8>,
    input: I,
    output: O,
    data_ptr: usize,
    instruction_ptr: usize,
    memory: [i8; 30000],
    bracs: HashMap<usize, usize>,
}

impl<I: Read, O: Write> fmt::Debug for CerebralVM<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ code: {:?}, data_ptr: {}, instruction_ptr: {} }}", self.code, self.data_ptr, self.instruction_ptr)
    }
}

impl<I: Read, O: Write> CerebralVM<I, O> {
    pub fn new(c: String, inp: I, out: O) -> Self {
        let mut stk = LinkedList::new();
        let mut tbracs:HashMap<usize, usize> = HashMap::new();
        let tc = c.into_bytes();
        for (i, c) in tc.iter().enumerate() {
            match *c as char {
                '['=> stk.push_back(i),
                ']' => {
                    tbracs.insert(i, *stk.back().unwrap());
                    tbracs.insert(stk.pop_back().unwrap(), i);
                },
                _ => {}
            }
        }
        return CerebralVM {
            code: tc,
            input: inp,
            output: out,
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
        self.output.write(&[self.memory[self.data_ptr] as u8]).unwrap();
        self.instruction_ptr += 1;
    }
    pub fn read_data(&mut self) {
        let mut val: u8 = 0;
        self.input.read_exact(std::slice::from_mut(&mut val)).expect("EOF reacted at stdin");
        self.memory[self.data_ptr] = val as i8;
        self.instruction_ptr += 1;
    }
    pub fn execute(&mut self) {
        while self.instruction_ptr < self.code.len() {
            match self.code[self.instruction_ptr] as char {
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

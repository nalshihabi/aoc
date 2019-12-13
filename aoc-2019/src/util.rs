pub use std::fs::File;
pub use std::io::prelude::*;
pub use std::io::BufReader;
pub use std::io;

pub fn read_file(file_name: &str) -> String {
    let file = File::open(file_name).expect(format!("Failed to open file {}", file_name).as_str());
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Failed to read to string!");
    contents
}

pub fn read_lines(input: String) -> Vec<String> {
    let mut lines: Vec<String> = input.split("\n").map(|val| String::from(val)).collect();
    lines.pop();
    lines
}

type Instruction = (i64, i64, i64, i64);
#[derive(Clone)]
pub struct Vm {
    index: usize,
    pub program: Vec<i64>,
    pub running: bool,
    pub display: bool,
    pub debug_display: bool,
    base: usize,
    pub output: Vec<i64>,
}

impl Vm {
    pub fn new(program: Vec<i64>) -> Self {
        let mut the_program = program.clone();
        let mut extra_space: Vec<i64> = vec![0; 1000000];
        the_program.append(&mut extra_space);

        Vm {
            index: 0,
            program: the_program,
            running: true,
            display: false,
            debug_display: false,
            base: 0,
            output: Vec::new(),
        }
    }

    pub fn new_with_file(file_input: &str) -> Self {
        let input: Vec<String> = read_file(file_input)
            .trim()
            .split(",")
            .map(|val| String::from(val))
            .collect();

        let program: Vec<i64> = input
            .iter()
            .map(|val| {
                val.as_str()
                    .parse::<i64>()
                    .expect("String is not a valid i64 value")
            })
            .collect();

        Vm::new(program)
    }

    pub fn clear_output(&mut self) -> Vec<i64> {
        let output = self.output.clone();
        self.output = Vec::new();
        output
    }

    fn parse_instruction(&self, mut num: i64) -> Instruction {
        if self.debug_display {
            if num == 2101 {
                println!("Instruction of 2101 at {}", self.index);
            }
        }

        let de = num % 100;
        num /= 100;
        let c = num % 10;
        num /= 10;
        let b = num % 10;
        num /= 10;
        let a = num % 10;
        (a, b, c, de)
    }

    fn store(&mut self, destination: usize, value: i64) {
        if self.debug_display {
            println!("Storage:: program[{}] = {} => {}",
                destination, self.program[destination], value
            );
        }

        self.program[destination] = value;
    }

    fn set_index(&mut self, new_index: usize) {
        if self.debug_display {
            println!("Set Index:: index = {} => {}",
                self.index, new_index
            );
        }

        self.index = new_index;
    }

    fn inc_index(&mut self, increment: usize) {
        if self.debug_display {
            println!("Set Index:: index = {} => {}",
                self.index, self.index + increment
            );
        }

        self.index += increment;
    }

    fn set_base(&mut self, new_base: usize) {
        if self.debug_display {
            println!("Set Base:: base = {} => {}",
                self.base, new_base
            );
        }

        self.base = new_base;
    }

    fn read_parameters(&self, inst: Instruction) -> (i64, i64, usize) {
        let index = self.index;
        let inp = &self.program;
        let p1: i64;
        let p2: i64;
        let de: usize;

        if inp[index] == 2101 && self.debug_display {
            println!("!!!\n{:?}", inp.get(index..index + 10));
        }

        let p1_index = match inst.2 {
            0 => inp[index + 1] as usize,
            1 => index + 1,
            2 => ((self.base as i64) + inp[index + 1]) as usize,
            _ => {
                println!("Parameter type not matching for p1: {}", inst.2);
                0
            }
        };

        let p2_index = match inst.1 {
            0 => inp[index + 2] as usize,
            1 => index + 2,
            2 => ((self.base as i64) + inp[index + 2]) as usize,
            _ => {
                println!("Parameter type not matching for p2: {}", inst.1);
                0
            }
        };

        let de_index = match inst.0 {
            0 => inp[index + 3] as usize,
            1 => index + 3,
            2 => ((self.base as i64) + inp[index + 3]) as usize,
            _ => {
                println!("Parameter type not matching for de: {}", inst.0);
                0
            }
        };

        p1 = self.program[p1_index];
        p2 = self.program[p2_index];
        de = de_index;

        if self.debug_display {
            println!("Parameters:
  p1({}) {} => {}
  p2({}) {} => {}
  de({}) {} => {}",
                inst.2, p1_index, p1,
                inst.1, p2_index, p2,
                inst.0, de_index, de
            );
        }

        (p1, p2, de)
    }

    fn add(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, de) = self.read_parameters(inst);
        self.store(de, p1 + p2);
        self.inc_index(4);
        None
    }

    fn mul(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, de) = self.read_parameters(inst);
        self.store(de, p1 * p2);
        self.inc_index(4);
        None
    }

    fn read(&mut self, inst: Instruction, input: Option<i64>) -> Option<i64> {
        let p1: i64;
        let de = match inst.2 {
            0 => self.program[self.index + 1] as usize,
            1 => self.index + 1,
            2 => ((self.base as i64) + self.program[self.index + 1]) as usize,
            _ => {
                println!("Parameter type not matching for p1: {}", inst.2);
                0
            }
        };

        if self.debug_display {
            if input.is_some() {
                println!("    input: {}", input.expect("I thought"));
            } else {
                println!("    input: None");
            }
        }

        if !input.is_some() {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                    p1 = input.trim().parse().expect("Not an integer");
                }
                Err(_error) => p1 = 0,
            };
        } else {
            p1 = input.clone().expect("No text input");
        }

        self.store(de, p1);
        self.inc_index(2);
        None
    }

    fn write(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, _p2, _de) = self.read_parameters(inst);
        if self.display {
            if self.debug_display {
                print!("output:");
            }
            println!("{}", p1);
        }

        self.output.push(p1);
        self.inc_index(2);
        Some(p1)
    }

    fn jne(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, _de) = self.read_parameters(inst);
        if p1 != 0 {
            self.set_index(p2 as usize);
        } else {
            self.inc_index(3);
        }

        None
    }

    fn je(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, _de) = self.read_parameters(inst);
        if p1 == 0 {
            self.set_index(p2 as usize);
        } else {
            self.inc_index(3);
        }

        None
    }

    fn cmpl(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, de) = self.read_parameters(inst);

        if p1 < p2 {
            self.store(de, 1);
        } else {
            self.store(de, 0);
        }

        self.inc_index(4);
        None
    }

    fn cmpe(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, de) = self.read_parameters(inst);

        if p1 == p2 {
            self.store(de, 1);
        } else {
            self.store(de, 0);
        }

        self.inc_index(4);
        None
    }

    fn mvb(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, _p2, _de) = self.read_parameters(inst);
        self.set_base(((self.base as i64) + p1) as usize);
         self.inc_index(2);
        None
    }

    pub fn step_until_input(&mut self) {
        while self.running && self.program[self.index] % 10 != 3 {
            self.step(None);
        }
    }

    pub fn step_until_print(&mut self) {
        while self.running && self.program[self.index] % 10 != 4 {
            self.step(None);
        }
    }

    pub fn step(&mut self, input: Option<i64>) -> Option<i64> {
        let inst = self.parse_instruction(self.program[self.index]);
        if self.debug_display {

            println!(
                "\nAt {} for {:?}\n  base: {} | inst: {} | params: {:?}",
                self.index,
                inst,
                self.base,
                self.program[self.index],
                self.program.get(self.index + 1..self.index + 4),
            );
        }

        match inst.3 {
            99 => {
                self.running = false;
                None
            }
            1 => self.add(inst),
            2 => self.mul(inst),
            3 => self.read(inst, input),
            4 => self.write(inst),
            5 => self.jne(inst),
            6 => self.je(inst),
            7 => self.cmpl(inst),
            8 => self.cmpe(inst),
            9 => self.mvb(inst),
            _ => {
                println!("No opcode found for {}", inst.3);
                None
            },
        }
    }

    pub fn run(&mut self) {
        while self.running {
            match self.step(None) {
                Some(value) => {
                    if self.display {
                        println!("{}", value);
                    }
                },
                None => {},
            };
        }
    }
}

pub fn gcd(a: usize, b: usize) -> usize {
    match a % b {
        0 => b,
        _ => gcd(b, a % b)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

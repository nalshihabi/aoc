use crate::util;
use std::io;

fn read_input() -> Vec<i64> {
    let input: Vec<String> = util::read_file("input/day09.txt")
        .trim()
        .split(",")
        .map(|val| String::from(val))
        .collect();

    input
        .iter()
        .map(|val| {
            val.as_str()
                .parse::<i64>()
                .expect("String is not a valid i64 value")
        })
        .collect()
}

type Instruction = (i64, i64, i64, i64);
struct Vm {
    index: usize,
    program: Vec<i64>,
    running: bool,
    display: bool,
    debug_display: bool,
    base: usize,
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
        }
    }

    fn parse_instruction(&self, mut num: i64) -> Instruction {
        let de = num % 100;
        num /= 100;
        let c = num % 10;
        num /= 10;
        let b = num % 10;
        num /= 10;
        let a = num % 10;
        (a, b, c, de)
    }

    fn read_parameters(&self, inst: Instruction) -> (i64, i64, usize) {
        let index = self.index;
        let inp = &self.program;
        let p1: i64;
        let p2: i64;
        let de: usize;

        if inst.2 == 0 {
            p1 = inp[inp[index + 1] as usize];
        } else if inst.2 == 1 {
            p1 = inp[index + 1];
        } else {
            p1 = inp[((self.base as i64) + inp[index + 1]) as usize];
        }

        if inst.1 == 0 {
            p2 = inp[inp[index + 2] as usize];
        } else if inst.1 == 1 {
            p2 = inp[index + 2];
        } else {
            p2 = inp[((self.base as i64) + inp[index + 1]) as usize];
        }

        if inst.0 == 0 {
            de = inp[index + 3] as usize;
        } else if inst.1 == 2 {
            de = inp[inp[index + 3] as usize] as usize;
        } else {
            de = ((self.base as i64) + inp[index + 1]) as usize;
        }

        (p1, p2, de)
    }

    fn step_until_input(&mut self) {
        while self.program[self.index] % 10 != 3 && self.running {
            self.step(None);
        }
    }

    fn step_until_print(&mut self) {
        while self.program[self.index] % 10 != 4 && self.running {
            self.step(None);
        }
    }

    fn add(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, de) = self.read_parameters(inst);
        self.program[de] = p1 + p2;
        self.index += 4;
        None
    }

    fn mul(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, de) = self.read_parameters(inst);
        self.program[de] = p1 * p2;
        self.index += 4;
        None
    }

    fn read(&mut self, inst: Instruction, input: Option<i64>) -> Option<i64> {
        let p1: i64;
        // let de = self.program[self.index + self.base + 1] as usize;
        let (_p1, _p2, de) = self.read_parameters(inst);
        // println!("getting value {} {} {}", _p1, _p2, de);

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

        self.program[de as usize] = p1;
        self.index += 2;
        None
    }

    fn write(&mut self, inst: Instruction) -> Option<i64> {
        // let de = ((self.base as i64) + self.program[self.index + 1]) as usize;
        let (p1, _p2, _de) = self.read_parameters(inst);
        if self.display {
            // println!("---{} {}   {}", de, self.index, self.program[self.index + self.base + 1]);
            // println!("    {:?}", self.program.get(0..25));
            // println!("{}", self.program[p1 as usize]);
            println!("{}", p1);
        }

        self.index += 2;
        // Some(self.program[p1 as usize])
        Some(p1)
    }

    fn jne(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, _de) = self.read_parameters(inst);
        if p1 != 0 {
            self.index = p2 as usize;
        } else {
            self.index += 3;
        }

        None
    }

    fn je(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, _de) = self.read_parameters(inst);
        if p1 == 0 {
            self.index = p2 as usize;
        } else {
            self.index += 3;
        }

        None
    }

    fn cmpl(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, de) = self.read_parameters(inst);

        if p1 < p2 {
            self.program[de] = 1;
        } else {
            self.program[de] = 0;
        }

        self.index += 4;
        None
    }

    fn cmpe(&mut self, inst: Instruction) -> Option<i64> {
        let (p1, p2, de) = self.read_parameters(inst);

        if p1 == p2 {
            self.program[de] = 1;
        } else {
            self.program[de] = 0;
        }

        self.index += 4;
        None
    }

    fn mvb(&mut self) -> Option<i64> {
        self.base = ((self.base as i64)  + self.program[self.index + 1]) as usize;
        self.index += 2;
        None
    }

    fn step(&mut self, input: Option<i64>) -> Option<i64> {
        let inst = self.parse_instruction(self.program[self.index]);
        if self.debug_display {
            println!(
                "\nAt {} for {:?}\n  base: {} params: {:?}\n  Program: {:?}",
                self.index, inst, self.base, self.read_parameters(inst.clone()), self.program.get(0..2500)
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
            9 => self.mvb(),
            _ => { println!("whyyyyyy"); None },
        }
    }

    pub fn run(&mut self) {
        while self.running {
            match self.step(None) {
                Some(value) => {
                    if !self.display {
                        println!("{}", value);
                    }
                },
                None => {},
            };
        }
    }
}


pub fn part1() {
    let inp = read_input();
    let mut vm = Vm::new(inp);
    vm.display = true;
    vm.debug_display = true;

    vm.run();
    println!("part1 done");
}

pub fn part2() {
    let inp = read_input();
    println!("part2 {:?}", inp.len());
}

use crate::util;
use std::io;

fn read_input() -> Vec<i32> {
    let input: Vec<String> = util::read_file("input/day07.txt")
        .trim()
        .split(",")
        .map(|val| String::from(val))
        .collect();

    input
        .iter()
        .map(|val| {
            val.as_str()
                .parse::<i32>()
                .expect("String is not a valid i32 value")
        })
        .collect()
}

type Instruction = (i32, i32, i32, i32);
struct Vm {
    index: usize,
    program: Vec<i32>,
    running: bool,
    display: bool,
    debug_display: bool,
}

impl Vm {
    pub fn new(program: Vec<i32>) -> Self {
        Vm {
            index: 0,
            program: program,
            running: true,
            display: false,
            debug_display: false,
        }
    }

    fn parse_instruction(&self, mut num: i32) -> Instruction {
        let de = num % 100;
        num /= 100;
        let c = num % 10;
        num /= 10;
        let b = num % 10;
        num /= 10;
        let a = num % 10;
        (a, b, c, de)
    }

    fn read_parameters(&self, inst: Instruction) -> (i32, i32, usize) {
        let index = self.index;
        let inp = &self.program;
        let p1: i32;
        let p2: i32;
        let de: usize;

        if inst.2 == 0 {
            p1 = inp[inp[index + 1] as usize];
        } else {
            p1 = inp[index + 1];
        }

        if inst.1 == 0 {
            p2 = inp[inp[index + 2] as usize];
        } else {
            p2 = inp[index + 2];
        }

        if inst.0 == 0 {
            de = inp[index + 3] as usize;
        } else {
            de = inp[inp[index + 3] as usize] as usize;
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

    fn add(&mut self, inst: Instruction) -> Option<i32> {
        let (p1, p2, de) = self.read_parameters(inst);
        self.program[de] = p1 + p2;
        self.index += 4;
        None
    }

    fn mul(&mut self, inst: Instruction) -> Option<i32> {
        let (p1, p2, de) = self.read_parameters(inst);
        self.program[de] = p1 * p2;
        self.index += 4;
        None
    }

    fn read(&mut self, input: Option<i32>) -> Option<i32> {
        let p1: i32;
        let de = self.program[self.index + 1] as usize;

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

        self.program[de] = p1;
        self.index += 2;
        None
    }

    fn write(&mut self) -> Option<i32> {
        let de = self.program[self.index + 1] as usize;
        if self.display {
            println!("{}", self.program[de]);
        }

        self.index += 2;
        Some(self.program[de])
    }

    fn jne(&mut self, inst: Instruction) -> Option<i32> {
        let (p1, p2, _de) = self.read_parameters(inst);
        if p1 != 0 {
            self.index = p2 as usize;
        } else {
            self.index += 3;
        }

        None
    }

    fn je(&mut self, inst: Instruction) -> Option<i32> {
        let (p1, p2, _de) = self.read_parameters(inst);
        if p1 == 0 {
            self.index = p2 as usize;
        } else {
            self.index += 3;
        }

        None
    }

    fn cmpl(&mut self, inst: Instruction) -> Option<i32> {
        let (p1, p2, de) = self.read_parameters(inst);

        if p1 < p2 {
            self.program[de] = 1;
        } else {
            self.program[de] = 0;
        }

        self.index += 4;
        None
    }

    fn cmpe(&mut self, inst: Instruction) -> Option<i32> {
        let (p1, p2, de) = self.read_parameters(inst);

        if p1 == p2 {
            self.program[de] = 1;
        } else {
            self.program[de] = 0;
        }

        self.index += 4;
        None
    }

    fn step(&mut self, input: Option<i32>) -> Option<i32> {
        let inst = self.parse_instruction(self.program[self.index]);
        if self.debug_display {
            println!(
                "At {} for {:?}\n  Program: {:?}",
                self.index, inst, self.program
            );
        }

        match inst.3 {
            99 => {
                self.running = false;
                None
            }
            1 => self.add(inst),
            2 => self.mul(inst),
            3 => self.read(input),
            4 => self.write(),
            5 => self.jne(inst),
            6 => self.je(inst),
            7 => self.cmpl(inst),
            8 => self.cmpe(inst),
            _ => None,
        }
    }
}

fn part1_emulate(items: &Vec<i32>, program: &Vec<i32>, best: &mut Vec<(i32, Vec<i32>)>) {
    let mut output = 0;

    items.iter().for_each(|item| {
        let mut vm = Vm::new(program.clone());
        vm.step_until_input();
        vm.step(Some(*item));
        vm.step_until_input();
        vm.step(Some(output));
        vm.step_until_print();
        let cur_output = vm.step(None);

        if cur_output.is_none() {
            return;
        } else {
            output = cur_output.expect("There should be output here");
        }
    });

    best.push((output, items.clone()));
}

fn part2_emulate(items: &Vec<i32>, program: &Vec<i32>, best: &mut Vec<(i32, Vec<i32>)>) {
    let mut vms: Vec<Vm> = items.iter().map(|_item| Vm::new(program.clone())).collect();
    let mut inputs: Vec<i32> = vec![0; items.len()];
    let mut cur_best = 0;
    let mut first_done = false;

    while !vms
        .iter()
        .fold(false, |one_done, vm| one_done || !vm.running)
    {
        for i in 0..vms.len() {
            if !first_done {
                vms[i].step_until_input();
                vms[i].step(Some(items[i]));
            }
            vms[i].step_until_input();
            vms[i].step(Some(inputs[i]));
            vms[i].step_until_print();

            let cur_output = vms[i].step(None);
            if cur_output.is_none() {
                break;
            } else {
                let output = cur_output.expect("There should be output");
                let next = (i + 1) % inputs.len();
                inputs[next] = output;

                if output > cur_best {
                    cur_best = output;
                }
            }
        }

        first_done = true;
    }
    best.push((cur_best, items.clone()));
}

fn recur(
    loc: usize,
    used: &mut Vec<bool>,
    items: &mut Vec<i32>,
    best: &mut Vec<(i32, Vec<i32>)>,
    values: &Vec<i32>,
    inp: &Vec<i32>,
    part1: bool,
) {
    if loc == items.len() {
        if part1 {
            part1_emulate(items, inp, best);
        } else {
            part2_emulate(items, inp, best);
        }
    } else {
        for i in 0..items.len() {
            if !used[i] {
                used[i] = true;
                items[loc] = values[i];
                recur(loc + 1, used, items, best, values, inp, part1);
                used[i] = false;
            }
        }
    }
}

pub fn part1() {
    let inp = read_input();
    let mut best: Vec<(i32, Vec<i32>)> = Vec::new();
    let mut used = vec![false; 5];
    let mut items = vec![0; 5];
    let values = vec![0, 1, 2, 3, 4];
    recur(0, &mut used, &mut items, &mut best, &values, &inp, true);
    best.sort();
    best.reverse();
    println!("part1 {:?}", best[0]);
}

pub fn part2() {
    let inp = read_input();
    let mut best: Vec<(i32, Vec<i32>)> = Vec::new();
    let mut used = vec![false; 5];
    let mut items = vec![0; 5];
    let values = vec![5, 6, 7, 8, 9];
    recur(0, &mut used, &mut items, &mut best, &values, &inp, false);
    best.sort();
    best.reverse();
    println!("part2 {:?}", best[0]);
}

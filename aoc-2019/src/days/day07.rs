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

struct Vm {
    index: usize,
    program: Vec<i32>,
    output: Vec<i32>,
    running: bool,
}

impl Vm {
    pub fn new(program: Vec<i32>) -> Self {
        Vm {
            index: 0,
            program: program,
            output: Vec::new(),
            running: true,
        }
    }

    fn det_instruction(&self, mut num: i32) -> (i32, i32, i32, i32) {
        let de = num % 100;
        num /= 100;
        let c = num % 10;
        num /= 10;
        let b = num % 10;
        num /= 10;
        let a = num % 10;
        (a, b, c, de)
    }

    fn det_values(&self, inst: (i32, i32, i32, i32)) -> (i32, i32, i32) {
        let index = self.index;
        let inp = &self.program;
        let p1: i32;
        let p2: i32;
        let de: i32;

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
            de = inp[index + 3];
        } else {
            de = inp[inp[index + 3] as usize];
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

    fn step(&mut self, input: Option<i32>) -> Option<i32> {
        let op = self.program[self.index];
        if op == 99 {
            self.running = false;
            return None;
        }

        let inst = self.det_instruction(op);
        let p1: i32;
        let p2: i32;
        let de: i32;

        if inst.3 == 1 {
            let vals = self.det_values(inst);
            p1 = vals.0;
            p2 = vals.1;
            de = vals.2;

            self.program[de as usize] = p1 + p2;
            self.index += 4;
        } else if inst.3 == 2 {
            let vals = self.det_values(inst);
            p1 = vals.0;
            p2 = vals.1;
            de = vals.2;

            self.program[de as usize] = p1 * p2;
            self.index += 4;
        } else if inst.3 == 3 {
            p1 = self.program[self.index + 1];

            if !input.is_some() {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_n) => {
                        de = input
                            .as_str()
                            .trim()
                            .parse::<i32>()
                            .expect("Not an integer");
                    }
                    Err(_error) => de = 0,
                };
            } else {
                de = input.clone().expect("No text input");
            }

            self.program[p1 as usize] = de;
            self.index += 2;
        } else if inst.3 == 4 {
            p1 = self.program[self.index + 1];
            println!("{}", self.program[p1 as usize]);
            self.output.push(self.program[p1 as usize]);
            self.index += 2;
            return Some(self.program[p1 as usize]);
        } else if inst.3 == 5 {
            let vals = self.det_values(inst);
            p1 = vals.0;
            p2 = vals.1;

            if p1 != 0 {
                self.index = p2 as usize;
            } else {
                self.index += 3;
            }
        } else if inst.3 == 6 {
            let vals = self.det_values(inst);
            p1 = vals.0;
            p2 = vals.1;

            if p1 == 0 {
                self.index = p2 as usize;
            } else {
                self.index += 3;
            }
        } else if inst.3 == 7 {
            let vals = self.det_values(inst);
            p1 = vals.0;
            p2 = vals.1;
            de = vals.2;

            if p1 < p2 {
                self.program[de as usize] = 1;
            } else {
                self.program[de as usize] = 0;
            }

            self.index += 4;
        } else if inst.3 == 8 {
            let vals = self.det_values(inst);
            p1 = vals.0;
            p2 = vals.1;
            de = vals.2;

            if p1 == p2 {
                self.program[de as usize] = 1;
            } else {
                self.program[de as usize] = 0;
            }

            self.index += 4;
        }

        None
    }
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
            let mut vm = Vm::new(inp.clone());
            let mut output = 0;
            items.iter().for_each(|item| {
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
        } else {
            let mut vms: Vec<Vm> = items.iter().map(|_item| Vm::new(inp.clone())).collect();

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
                        // first_done = true;
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

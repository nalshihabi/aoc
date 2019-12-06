use std::io;
use crate::util;

fn read_input() -> Vec<i32> {
    let input: Vec<String> = util::read_file("input/day05.txt")
        .trim()
        .split(",")
        .map(|val| { String::from(val) })
        .collect();

    input
        .iter()
        .map(|val| {
            val.as_str().parse::<i32>()
                .expect("String is not a valid i32 value")
        })
        .collect()
}

fn det_instruction(mut num: i32) -> (i32, i32, i32, i32) {
    let de = num % 100;
    num /= 100;
    let c = num % 10;
    num /= 10;
    let b = num % 10;
    num /= 10;
    let a = num % 10;
    (a, b, c, de)
}

fn det_values(index: usize, inst: (i32, i32, i32, i32), inp: &Vec<i32>) -> (i32, i32, i32) {
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

fn run_program(mut inp: Vec<i32>) -> Vec<i32> {
    let mut index = 0;

    loop {
        let op = inp[index];
        if op == 99 {
            break;
        }

        let inst = det_instruction(op);
        let p1: i32;
        let p2: i32;
        let de: i32;

        if inst.3 == 1 {
            let vals = det_values(index, inst, &inp);
            p1 = vals.0;
            p2 = vals.1;
            de = vals.2;

            inp[de as usize] = p1 + p2;
            index += 4;
        } else if inst.3 == 2 {
            let vals = det_values(index, inst, &inp);
            p1 = vals.0;
            p2 = vals.1;
            de = vals.2;

            inp[de as usize] = p1 * p2;
            index += 4;
        } else if inst.3 == 3 {
            p1 = inp[index + 1];

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

            inp[p1 as usize] = de;
            index += 2;
        } else if inst.3 == 4 {
            p1 = inp[index + 1];
            println!("{}", inp[p1 as usize]);
            index += 2;
        } else if inst.3 == 5 {
            let vals = det_values(index, inst, &inp);
            p1 = vals.0;
            p2 = vals.1;

            if p1 != 0 {
                index = p2 as usize;
            } else {
                index += 3;
            }
        } else if inst.3 == 6 {
            let vals = det_values(index, inst, &inp);
            p1 = vals.0;
            p2 = vals.1;

            if p1 == 0 {
                index = p2 as usize;
            } else {
                index += 3;
            }
        } else if inst.3 == 7 {
            let vals = det_values(index, inst, &inp);
            p1 = vals.0;
            p2 = vals.1;
            de = vals.2;

            if p1 < p2 {
                inp[de as usize] = 1;
            } else {
                inp[de as usize] = 0;
            }

            index += 4;
        } else if inst.3 == 8 {
            let vals = det_values(index, inst, &inp);
            p1 = vals.0;
            p2 = vals.1;
            de = vals.2;

            if p1 == p2 {
                inp[de as usize] = 1;
            } else {
                inp[de as usize] = 0;
            }

            index += 4;
        }

    }

    inp
}

pub fn part1() {
    println!("part 1");
    let inp = read_input();
    run_program(inp);
}

pub fn part2() {
    println!("part 2");
    let inp = read_input();
    run_program(inp);
}


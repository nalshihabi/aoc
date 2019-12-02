pub use crate::util;

fn read_input() -> Vec<usize> {
    let input: Vec<String> = util::read_file("input/day02.txt")
        .trim()
        .split(",")
        .map(|val| { String::from(val) })
        .collect();

    input
        .iter()
        .map(|val| {
            val.as_str().parse::<usize>()
                .expect("String is not a valid usize value")
        })
        .collect()
}

fn run_program(mut inp: Vec<usize>, noun: usize, verb: usize) -> Vec<usize> {
    let mut index = 0;
    inp[1] = noun;
    inp[2] = verb;

    loop {
        // println!("{} {:?}", index, inp);
        let op = inp[index];
        if op == 99 {
            break;
        }

        let p1 = inp[index + 1];
        let p2 = inp[index + 2];
        let de = inp[index + 3];

        if op == 1 {
            inp[de] = inp[p1] + inp[p2];
        } else if op == 2 {
            inp[de] = inp[p1] * inp[p2];
        }

        index += 4;
    }

    inp
}

pub fn part1() {
    let val = run_program(read_input(), 12, 2);
    println!("{}", val[0]);
}

pub fn part2() {
    let input = read_input();
    let target = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(input.clone(), noun, verb)[0] == target {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }
}

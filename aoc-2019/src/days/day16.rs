use crate::util;

const PATTERN: &[i32] = &[0, 1, 0, -1];

fn read_input() -> Vec<i32> {
    let inp = util::read_file("input/day16.txt");
    inp
        .trim()
        .chars()
        .map(|ch| -> i32 {
            ch.to_digit(10).unwrap() as i32
        })
        .collect()
}

fn gen_pattern(step: usize, size: usize) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();

    while vec.len() < size + 1 {
        PATTERN
            .iter()
            .for_each(|val| {
                for _ in 0..step + 1 {
                    vec.push(*val);
                }
            });
    }

    vec.get(1..size + 1).unwrap().to_vec()
}

fn _run_phase(input: Vec<i32>, print_it: bool) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    if print_it {
        input
            .iter()
            .for_each(|val| print!("{} ", val));
        println!();
    }

    input
        .iter()
        .enumerate()
        .for_each(|(step, _val)| {
            let pattern = gen_pattern(step, input.len());
            let ans = input
                .iter()
                .zip(pattern.iter())
                .fold(0, |sum, (inp, pat)| {
                    if print_it {
                        print!("{} ", match pat {
                            -1 => String::from("-"),
                            _ => format!("{}", pat),
                        });
                    }

                    sum + inp * pat
                });

            if print_it {
                println!(" = {} => {}      {}", ans, i32::abs(ans) % 10, step);
            }

            output.push(i32::abs(ans) % 10);
        });
    output
}

fn sums_array(input: &Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = input.iter().map(|_| 0).collect();
    input
        .iter()
        .enumerate()
        .for_each(|(index, val)| {
            ans[index] = match index {
                0 => *val,
                _ => *val + ans[index - 1],
            };
        });
    ans
}

fn calc_band(start: usize, input: &Vec<i32>, sums: &Vec<i32>, ans: &mut Vec<i32>, neg: bool) {
    let mut beg = start;
    let mut end = start + 1;
    let mut index = 0;
    let db = start + 1;
    let de = start + 2;

    while beg < input.len() {
        let sum = match end <= input.len() {
            true => sums[end - 1] - sums[beg] + input[beg],
            false => sums[input.len() - 1] - sums[beg] + input[beg],
        };

        ans[index] = match neg {
            true => ans[index] - sum,
            false => ans[index] + sum,
        };

        index += 1;
        beg += db;
        end += de;
        end = match end <= input.len() { true => end, false => input.len(), };
    }
}

fn fast_phase(input: Vec<i32>) -> Vec<i32> {
    let sums = sums_array(&input);
    let mut ans = input.iter().map(|_val| 0).collect();
    // println!("inp {:?}", input);
    // println!("pat {:?}", gen_pattern(0, input.len()));
    gen_pattern(0, input.len())
        .iter()
        .enumerate()
        .step_by(2)
        .for_each(|(index, val)| {
            // println!("\nCalling band {} {}", index, val);
            calc_band(index, &input, &sums, &mut ans, *val == -1);
        });

    ans
        .iter()
        .map(|val| i32::abs(*val) % 10)
        .collect()
}

fn make_big(mut input: Vec<i32>) -> Vec<i32> {
    let orig = input.clone();
    for _ in 0..10000 {
        input.append(&mut orig.clone());
    }

    input
}

fn run_faster(mut input: Vec<i32>) -> Vec<i32> {
    input = make_big(input);
    let index = get_index(&input);

    for _run in 0..100 {
        println!("run {} {} {}", _run, input.len(), index);
        // let ff = input.clone();
        // let ff = fast_phase(ff);
        // input = _run_phase(input, true);
        // input = ff;
        // println!("{:?}\n", input);
        // println!("fast {:?}\n", ff);

        input = fast_phase(input);
    }

    if index + 8 <= input.len() {
        println!("{:?}", input.get(index..index + 8).unwrap());
    } else {
        println!("input {:?}", input);
    }
    input
}

fn get_index(input: &Vec<i32>) -> usize {
    input.get(0..7).unwrap().iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join("").as_str().parse::<usize>().unwrap()
}

pub fn part1() {
    let mut inp = read_input();
    let print_it = false;

    for _i in 0..100 {
        // println!("{} {:?}", _i, inp);
        // let ii = fast_phase(inp.clone());
        inp = _run_phase(inp, print_it);
        if print_it {
            println!("");
        }

        // println!("{} inp {:?}\n", _i, inp);
        // inp = fast_phase(inp);
    }

    println!("p1 {:?}", inp.get(0..std::cmp::min(8, inp.len())).unwrap());
}

pub fn part2() {
    let inp = read_input();
    run_faster(inp);
}

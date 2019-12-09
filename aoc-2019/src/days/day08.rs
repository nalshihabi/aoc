use crate::util;

fn read_input() -> String {
    util::read_file("input/day08.txt")
}

fn parse_input(input: &String) -> Vec<Vec<usize>> {
    println!("{} {} {}", input.len(), input.len() / 6, input.len() / 6 / 25);
    // let ans: Vec<Vec<usize>> = vec![];
    let i = 0;
    let step = input.len() / 6;
    let jstep = input.len() / 6 / 25;

    while i < input.len() {
        let mut j = 0;
        while j < i + step {
            j += jstep;
        }
    }

    vec![vec![]]
}

pub fn part1() {
    let inp = read_input();
    parse_input(&inp);
    // println!("part1 {:?}", inp);
}

pub fn part2() {
    let inp = read_input();
    println!("part2 {:?}", inp.len());
}

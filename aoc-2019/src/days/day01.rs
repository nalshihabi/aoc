pub use crate::util;

fn read_input() -> Vec<i32> {
    let input = util::read_file("input/day01.txt");
    util::read_lines(input)
        .iter()
        .map(|val| {
            val.as_str()
                .parse::<i32>()
                .expect("String is not a valid i32 value")
        })
        .collect()
}

fn calc_needed_simple() -> i32 {
    read_input().iter().fold(0, |sum, val| sum + (val / 3 - 2))
}

fn calc_needed_complex() -> i32 {
    read_input()
        .iter()
        .fold(0, |sum, val| sum + calc_needed_for_gas(val))
}

fn calc_needed_for_gas(mass: &i32) -> i32 {
    let mut gas_need = mass / 3 - 2;
    let mut total = 0;
    while gas_need > 0 {
        total += gas_need;
        gas_need = gas_need / 3 - 2;
    }
    total
}

pub fn part1() {
    println!("{}", calc_needed_simple());
}

pub fn part2() {
    println!("{}", calc_needed_complex());
}

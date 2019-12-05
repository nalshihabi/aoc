use crate::util;

fn read_input() -> Vec<u32> {
    let input: Vec<String> = util::read_file("input/day04.txt")
        .as_str()
        .trim()
        .split("-")
        .map(|val| { String::from(val) })
        .collect();

    input
        .iter()
        .map(|val| {
            val
                .as_str()
                .parse::<u32>()
                .expect("String is not a valid u32 value")
        })
        .collect()
}

fn works(input: u32, rule: bool) -> (bool, bool) {
    let mut works = false;

    let g: String = format!("{}", input);
    let vv: Vec<u32> = g
        .chars()
        .map(|val: char| {
            val
                .to_digit(10)
                .expect("whatever")
        })
        .collect();

    let mut i = 0;
    while i < vv.len() {
        let mut j = i + 1;

        if i > 0 && vv.get(i).unwrap() < vv.get(i - 1).unwrap() {
            return (false, false);
        }

        while j < vv.len() {
            if vv.get(j) != vv.get(i) {
                break;
            }
            j += 1;
        }
        if rule {
            if j - i == 2 {
                works = true;
            }
        } else {
            if j - i >= 2 {
                works = true;
            }
        }
        i = j;
    }

    (true, works)
}

fn run(input: Vec<u32>, rule: bool) -> String {
    let beg = input[0];
    let end = input[1];
    let mut i = beg;
    let mut count = 0;

    loop {
        if i == end + 1 {
            break;
        }

        let v = works(i, rule);
        if v.0 && v.1 {
            count += 1;
        }

        i+= 1;
    }
    format!("something {:?}", count)
}

pub fn part1() {
    let inp = read_input();
    println!("{:?}", run(inp, false));
}

pub fn part2() {
    let inp = read_input();
    println!("{:?}", run(inp, true));
}


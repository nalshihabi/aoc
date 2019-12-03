use std::collections::BTreeMap;

pub use crate::util;

struct Move {
    dir: (i32,i32),
    dis: i32
}

const U: &str = "U";
const R: &str = "R";
const D: &str = "D";
const L: &str = "L";

fn read_moves(line: String) -> Vec<Move> {
    let input: Vec<String> = line
        .as_str()
        .trim()
        .split(",")
        .map(|val| { String::from(val) })
        .collect();

    input
        .iter()
        .map(|val| {
            let dir = find_dir(
                String::from(val.as_str().get(0..1).expect("Nothing"))
            );
            let dis = val
                .as_str()
                .get(1..)
                .expect("Nothing")
                .parse::<i32>()
                .expect("String is not a valid i32 value");
            Move{dir: dir, dis: dis}
        })
        .collect()
}

fn read_input() -> (Vec<Move>, Vec<Move>) {
    let lines = util::read_lines(util::read_file("input/day03.txt"));
    (
        read_moves(lines[0].clone()),
        read_moves(lines[1].clone())
    )
}

fn calc_dist(loc: (i32,i32)) -> i32 {
    i32::abs(loc.0) + i32::abs(loc.1)
}

fn find_dir(dir: String) -> (i32,i32) {
    match dir.as_str() {
        U => (0,1),
        R => (1,0),
        D => (0,-1),
        L => (-1,0),
        _ => (0,0)
    }
}

fn run(snake1: Vec<Move>, snake2: Vec<Move>) -> (i32,i32) {
    let mut x = 0;
    let mut y = 0;
    let mut set: BTreeMap<(i32,i32),i32> = BTreeMap::new();
    let mut dx;
    let mut dy;
    let mut num_steps = 0;
    let mut best = 500000000;
    let mut least_steps = 50000000;

    let mut sn_iter = snake1.iter();
    loop {
        let cur = sn_iter.next();

        match cur {
            Some(val) => {
                let mut num_go = val.dis;
                dx = val.dir.0;
                dy = val.dir.1;

                while num_go > 0 {
                    x += dx;
                    y += dy;
                    num_steps += 1;
                    set.insert((x,y), num_steps);
                    num_go -= 1;
                }
            },
            None => { break }
        };
    }

    let mut sn_iter = snake2.iter();
    x = 0;
    y = 0;
    num_steps = 0;
    loop {
        let cur = sn_iter.next();

        match cur {
            Some(val) => {
                let mut num_go = val.dis;
                dx = val.dir.0;
                dy = val.dir.1;

                while num_go > 0 {
                    x += dx;
                    y += dy;
                    let tuple = (x,y);
                    num_steps += 1;
                    if set.contains_key(&tuple) {
                        if calc_dist(tuple) < best {
                            best = calc_dist(tuple);
                        }

                        if num_steps + set.get(&tuple).expect("uh oh") < least_steps {
                            least_steps = num_steps + set.get(&tuple).expect("uh oh");
                        }
                    }

                    num_go -= 1;
                }
            },
            None => { break }
        };
    }

    (best, least_steps)
}

pub fn part1() {
    let snakes = read_input();
    let v1 = snakes.0;
    let v2 = snakes.1;
    println!("{:?}", run(v1, v2).0);
}

pub fn part2() {
    let snakes = read_input();
    let v1 = snakes.0;
    let v2 = snakes.1;
    println!("{:?}", run(v1, v2).1);
}


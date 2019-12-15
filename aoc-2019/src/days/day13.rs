use crate::util::Vm;
use std::collections::BTreeMap;
use std::cmp::{max, Ordering};

type Loc = (usize, usize);
type Map = BTreeMap<Loc, i64>;
type Grid = Vec<Vec<i64>>;

fn parse_output(output: &Vec<i64>, map: &mut Map) -> Loc {
    let mut largest_row = 0;
    let mut largest_col = 0;
    output
        .iter()
        .enumerate()
        .step_by(3)
        .for_each(|(index, val)| {
            if output[index + 1] < 0 || *val < 0 {
                println!("Score: {}", output[index + 2]);
                return;
            }

            let loc: Loc = (output[index + 1] as usize, *val as usize);
            largest_row = max(largest_row, loc.0);
            largest_col = max(largest_col, loc.1);
            let val = output[index + 2];
            map.insert(loc, val);
        });
    (largest_row, largest_col)
}

fn run0() -> (Map, Loc) {
    let mut vm: Vm = Vm::new_with_file("input/day13.txt");
    let mut map: Map = BTreeMap::new();
    vm.run();

    let size = parse_output(&vm.output, &mut map);
    (map, size)
}

fn make_grid(map: &Map, size: Loc) -> Grid {
    let mut grid: Grid = vec![vec![0; size.1 + 1]; size.0 + 1];
    map
        .keys()
        .for_each(|key| {
            grid[key.0][key.1] = *map.get(key).unwrap();
        });
    grid
}

fn update_grid(map: &Map, grid: &mut Grid) {
    map
        .keys()
        .for_each(|key| {
            grid[key.0][key.1] = *map.get(key).unwrap();
        });
}

fn draw_grid(grid: &Grid) {
    grid
        .iter()
        .for_each(|row| {
            row
                .iter()
                .for_each(|col| {
                    print!("{}", match col {
                        0 => ' ',
                        1 => 'X',
                        2 => 'B',
                        3 => '=',
                        4 => 'O',
                        _ => '?'
                    });
                });
            println!("");
        });
}

fn _find_ball(map: &Map) -> Loc {
    map.keys().fold((0, 0), |cur, next| {
        match map.get(next) {
            Some(4) => *next,
            _ => cur
        }
    })
}

fn _find_paddle(map: &Map) -> Loc {
    map.keys().fold((0, 0), |cur, next| {
        match map.get(next) {
            Some(3) => *next,
            _ => cur
        }
    })
}

fn run_with_input() {
    let (map, size) = run0();
    let mut grid = make_grid(&map, size);
    draw_grid(&grid);

    let mut vm = Vm::new_with_file("input/day13.txt");
    vm.display = false;
    vm.program[0] = 2;
    vm.program[381] = 3;

    while vm.running {
        vm.step_until_input();

        let output = vm.clear_output();
        let mut map: Map = BTreeMap::new();
        parse_output(&output, &mut map);
        // let grid = make_grid(&map, size);
        update_grid(&map, &mut grid);
        draw_grid(&grid);
        let _ball = _find_ball(&map);
        let _paddle = _find_paddle(&map);

        // let next_input = input[ind];
        let next_input = match _ball.1.cmp(&_paddle.1) {
            Ordering::Greater => 1,
            Ordering::Equal => 0,
            Ordering::Less => -1
        };

        println!("Next input: {} | ball {:?} paddle {:?}\n", next_input, _ball, _paddle);

        vm.step(Some(next_input));
    }
}

pub fn part1() {
    let (map, _) = run0();
    let blocks: Vec<&Loc> = map
        .keys()
        .filter(|key| map.get(key) == Some(&2))
        .collect();

    println!("Num blocks {}", blocks.len());
}

pub fn part2() {
    let (map, size) = run0();
    let grid = make_grid(&map, size);
    draw_grid(&grid);

    println!("");
    let mut vm = Vm::new_with_file("input/day13.txt");
    // vm.debug_display = true;
    vm.program[0] = 2;
    // vm.run();

    // vm.display = true;
    // vm.debug_display = true;
    // while vm.running {
    //     vm.step_until_input();

    //     let output = vm.clear_output();
    //     let mut map: Map = BTreeMap::new();
    //     parse_output(&output, &mut map);
    //     // let grid = make_grid(&map, size);
    //     update_grid(&map, &mut grid);
    //     draw_grid(&grid);
    //     let _ball = find_ball(&map);

    //     vm.step(None);
    // }

    run_with_input();

    /*
    let mut memo: BTreeMap<Moment, usize> = BTreeMap::new();
    let mut steps: Vec<i64> = Vec::new();
    let score = recur(&mut vm.clone(), 0, 0, 21, &mut memo, &mut steps);
    println!("score {}", score);
    // */

    // let mut map = BTreeMap::new();
    // let size = parse_output(&vm.output, &mut map);
    // let grid = make_grid(&map, size);
    // draw_grid(&grid);
}

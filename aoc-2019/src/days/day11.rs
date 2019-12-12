use crate::util::Vm;
use std::collections::BTreeSet;

const GRID_SIZE: usize = 100;

type Grid = Vec<Vec<char>>;

struct Bot {
    row: usize,
    col: usize,
    dir: usize
}

impl Bot {
    fn new() -> Self {
        Bot{row: GRID_SIZE / 2, col: GRID_SIZE / 2, dir: 0}
    }

    fn next_spot(&self) -> (usize, usize) {
        match self.dir {
            0 => (self.row - 1, self.col),
            1 => (self.row, self.col + 1),
            2 => (self.row + 1, self.col),
            3 => (self.row, self.col - 1),
            _ => (self.row, self.col)
        }
    }

    fn move_forward(&mut self, grid: &mut Grid, color: usize, dir: usize) {
        self.turn(dir);
        let (row, col) = self.next_spot();
        grid[self.row][self.col] = match color {
            1 => '#',
            _ => '.'
        };
        self.row = row;
        self.col = col;
    }

    fn turn(&mut self, dir: usize) {
        match (self.dir, dir) {
            (0, 0) => self.dir = 3,
            (0, 1) => self.dir = 1,
            (1, 0) => self.dir = 0,
            (1, 1) => self.dir = 2,
            (2, 0) => self.dir = 1,
            (2, 1) => self.dir = 3,
            (3, 0) => self.dir = 2,
            (3, 1) => self.dir = 0,
            (_, _) => {},
        };
    }
}

fn draw_grid(grid: &Grid) {
    grid
        .iter()
        .for_each(|vec: &Vec<char>| {
            vec.iter().for_each(|val| print!("{}", val));
            println!("");
        });
}

fn run1(vm: &mut Vm, start_white: bool) -> (Grid, BTreeSet<(usize, usize)>) {
    let mut grid: Grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE];
    let mut bot = Bot::new();
    let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();

    if start_white {
        grid[bot.row][bot.col] = '#';
    }

    while vm.running {
        // println!("");
        // draw_grid(&grid);
        vm.step_until_input();
        let input: i64 = match grid[bot.row][bot.col] {
            '.' => 0,
            _ => 1
        };

        vm.step(Some(input));
        vm.step_until_print();
        let color: i64 = vm.step(None).expect("Should get output for color");
        vm.step_until_print();
        let dir: i64 = vm.step(None).expect("Should get output for direction");
        set.insert((bot.row, bot.col));
        bot.move_forward(&mut grid, color as usize, dir as usize);
        vm.step_until_input();
    }

    // draw_grid(&grid);
    (grid, set)
}

pub fn part1() {
    let mut vm = Vm::new_with_file("input/day11.txt");
    let (grid, set) = run1(&mut vm, false);
    draw_grid(&grid);
    println!("part1: {}", set.len());
}

pub fn part2() {
    let mut vm = Vm::new_with_file("input/day11.txt");
    let (grid, _set) = run1(&mut vm, true);
    draw_grid(&grid);
}

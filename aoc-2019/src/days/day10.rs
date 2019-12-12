use crate::util;
// use std::collections::BTreeMap;
// use std::cmp;
// 
// #[derive(Debug)]
// struct Asteroid {
//     row: usize,
//     col: usize,
// }
// 
// struct Line {
//     m1: i64,
//     m2: i64,
//     b: i64,
// }
// 
// impl Line {
//     fn new(m1: i64, m2: i64, b: i64) -> Self {
//         Line{m1: 0, m2: 0, b: 0}
//     }
// }
// 
// impl PartialOrd for Line {
//     fn partial_cmp(&self, other: &Line) -> Option<cmp::Ordering> {
//         Some(other.cmp(self))
//     }
// }
// 
// impl PartialEq for Line {
//     fn eq(&self, other: &Line) -> bool {
//         self.m1 == other.m1 && self.m2 == other.m2 && self.b == other.b
//     }
// }
// impl Eq for Line {}
// 
// impl Ord for Line {
//     fn cmp(&self, other: &Line) -> cmp::Ordering {
//         if self.m1 == other.m1 && self.m2 == other.m2 {
//             self.b.cmp(&other.b)
//         } else if self.m1 == other.m1 {
//             self.m2.cmp(&other.m2)
//         } else {
//             self.m1.cmp(&other.m1)
//         }
//     }
// }
// 
// impl Asteroid {
//     fn new(row: usize, col: usize) -> Self {
//         Asteroid{
//             row: row,
//             col: col
//         }
//     }
// }
// 
// impl PartialOrd for Asteroid {
//     fn partial_cmp(&self, other: &Asteroid) -> Option<cmp::Ordering> {
//         Some(other.cmp(self))
//     }
// }
// 
// impl PartialEq for Asteroid {
//     fn eq(&self, other: &Asteroid) -> bool {
//         self.row == other.row && self.col == other.col
//     }
// }
// impl Eq for Asteroid {}
// 
// impl Ord for Asteroid {
//     fn cmp(&self, other: &Asteroid) -> cmp::Ordering {
//         if self.row == other.row {
//             self.col.cmp(&other.col)
//         } else{
//             self.row.cmp(&other.row)
//         }
//     }
// }
// 
// fn calc_line(a1: Asteroid, a2: Asteroid) -> Line {
//     let m1: i64 = (a2.row - a1.row) as i64;
//     let m2: i64 = (a2.col - a2.col) as i64;
//     let b: i64;
// 
//     if m1 == 0 || m2 == 0 {
//         b = 0;
//     } else {
//         // let b: i64 = (a1.row as f64) - (m1 as f64 / m2 as f64) * (a1.col as f64) as i64;
//         b = 0;
//     }
// 
//     Line::new(m1, m2, b)
// }
// 
fn read_input() -> Vec<String> {
    util::read_lines(util::read_file("input/day10.txt"))
}

// fn parse_input(v: &Vec<String>) -> (Vec<Vec<char>>, Vec<Asteroid>) {
//     let mut map: Vec<Vec<char>> = vec![];
//     let mut asteroids: Vec<Asteroid> = vec![];
// 
//     for (row, line) in v.iter().enumerate() {
//         let chars: Vec<char> = line.chars().collect();
//         for (col, ch) in chars.iter().enumerate() {
//             if *ch == '#' {
//                 asteroids.push(Asteroid::new(row, col));
//             }
//         }
//         map.push(chars);
//     }
// 
//     (map, asteroids)
// }

// fn find_visible(cur: &Asteroid, alist: &Vec<Asteroid>) -> usize {
//     let mut map: BTreeMap<Line, Vec<Asteroid>> = BTreeMap::new();
//     let mut count = 0;
// 
// 
// 
//     count
// }

pub fn part1() {
    let inp = read_input();
    // let inp = parse_input(&read_input());
    println!("part1 {:?}", inp);
}

pub fn part2() {
    let inp = read_input();
    println!("part2 {:?}", inp.len());
}

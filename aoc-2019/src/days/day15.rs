use crate::util::Vm;
use std::collections::{BTreeMap, VecDeque};
use std::cmp::max;

type Loc = (i64, i64);
type Map = BTreeMap<Loc, char>;
type CMap = BTreeMap<Loc, usize>;

const LIMIT: i64 = 25;

fn explore(vm: &mut Vm, loc: Loc, map: &mut Map, found: &mut bool) {
    let dr = [-1, 1, 0, 0];
    let dc = [0, 0, -1, 1];
    if i64::abs(loc.0) == LIMIT || i64::abs(loc.1) == LIMIT {
        return;
    }

    dr
        .iter()
        .zip(dc.iter())
        .enumerate()
        .for_each(|(index, (cr, cc))| {
            let (nr, nc) = (cr + loc.0, cc + loc.1);
            if map.contains_key(&(nr + 0, nc + 0)) {
                return;
            }

            let mut nvm = vm.clone();
            nvm.step_until_input();
            nvm.step(Some((index + 1) as i64));
            nvm.step_until_print();
            let ans = nvm.step(None);
            match ans {
                Some(a) => {
                    match a {
                        0 => {
                            map.insert((nr, nc), '#');
                        },
                        1 => {
                            map.insert((nr + 0, nc + 0), '.');
                            explore(&mut nvm, (nr, nc), map, found);
                        },
                        2 => {
                            map.insert((nr + 0, nc + 0), 'X');
                            *found = true;
                            return;
                        },
                        _ => { println!("uhoh"); },
                    };
                },
                None => println!("What?????"),
            }
        });
}

fn draw_grid(map: &Map) {
    for i in -LIMIT..LIMIT {
        for j in -LIMIT..LIMIT {
            let vv = (i, j);
            if map.contains_key(&vv) {
                print!("{}", map.get(&vv).unwrap());
            } else {
                print!("{}", '#');
            }
        }
        println!("");
    }
}

fn dfs(loc: Loc, map: &mut Map, been: &mut CMap, count: usize, mark: bool) -> usize {
    let dr = [-1, 1, 0, 0];
    let dc = [0, 0, -1, 1];
    been.insert(loc.clone(), count);
    if mark {
        map.insert(loc.clone(), 'O');
    }

    dr
        .iter()
        .zip(dc.iter())
        .enumerate()
        .fold(0, |cur, (_index, (cr, cc))| -> usize {
            let np = (cr + loc.0, cc + loc.1);
            if been.contains_key(&np) {
                if mark {
                    if *been.get(&np).unwrap() <= count + 1 {
                        return cur + 0;
                    }
                } else {
                    return cur;
                }
            }

            if map.contains_key(&np) {
                match *map.get(&np).unwrap() {
                    'X' => return cur + 1,
                    '#' => return cur + 0,
                    'O' => {
                        if *been.get(&np).unwrap() <= count + 1 {
                            return cur + 0;
                        }
                    },
                    _ => {},
                };
            } else {
                return cur;
            }

            let dd = dfs(np, map, been, count + 1, mark);
            match dd {
                0 => cur,
                _ => cur + dd + 1,
            }
        })
}

fn _bfs(loc: Loc, map: &mut Map, mark: bool) -> CMap {
    let mut q: VecDeque<Loc> = VecDeque::new();
    let mut been: CMap = BTreeMap::new();
    q.push_back(loc);

    let dr = [-1, 1, 0, 0];
    let dc = [0, 0, -1, 1];
    been.insert(loc.clone(), 0);

    while q.len() > 0 {
        let cur = q.pop_front().unwrap();
        let count = been.get(&cur).unwrap().clone();
        if mark {
            map.insert(cur.clone(), 'O');
            draw_grid(&map);
        }

        dr
            .iter()
            .zip(dc.iter())
            .for_each(|(cr, cc)| {
                let nl = (cur.0 + cr, cur.1 + cc);

                if been.contains_key(&nl) {
                    return;
                }

                if map.contains_key(&nl) {
                    match map.get(&nl).unwrap() {
                        '#' => return,
                        _ => {},
                    };
                } else {
                    return;
                }

                been.insert(nl.clone(), count + 1);
                q.push_back(nl);
            });
    }

    been
}

fn get_map() -> Map {
    let mut vm = Vm::new_with_file("input/day15.txt");
    let mut map: Map = BTreeMap::new();
    explore(&mut vm, (0, 0), &mut map, &mut false);
    map
}

fn find_ox(map: &mut Map) -> (usize, Loc) {
    let mut seen: CMap = BTreeMap::new();
    let ans = dfs((0, 0), map, &mut seen, 0, false);
    let loc: Option<Loc> = map
        .keys()
        .fold(None, |cur, key| {
            match map.get(key).unwrap() {
                'X' => Some(key.clone()),
                _ => cur,
            }
        });
    (ans, loc.unwrap())
}

pub fn part1() {
    let mut map = get_map();
    draw_grid(&map);
    let mut seen: CMap = BTreeMap::new();
    let ans = dfs((0, 0), &mut map, &mut seen, 0, false);
    println!("{}", ans);
}

pub fn part2() {
    let mut map = get_map();
    let (_, loc) = find_ox(&mut map);
    let mut seen: CMap = BTreeMap::new();
    dfs(loc, &mut map, &mut seen, 0, true);
    // let seen = _bfs(loc, &mut map, true);

    let ans = seen
        .keys()
        .fold(0, |cur, key| {
            max(*seen.get(key).unwrap(), cur)
        });
    draw_grid(&map);
    println!("ans {}", ans);
}

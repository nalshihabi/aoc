use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use crate::util;

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub par: Option<String>,
    pub children: Vec<String>
}

fn read_input() -> (BTreeMap<String, Object>, String) {
    let mut map: BTreeMap<String, Object> = BTreeMap::new();
    let mut par: BTreeSet<String> = BTreeSet::new();
    let mut chi: BTreeSet<String> = BTreeSet::new();
    util::read_lines(util::read_file("input/day06.txt"))
        .iter()
        .for_each(|val: &String| {
            let strings: Vec<&str> = val.as_str().split(")").collect();
            let o1 = String::from(*(strings.get(0).expect("nothing here")));
            let o2 = String::from(*(strings.get(1).expect("nothing here")));

            let parent: &mut Object;
            if !map.contains_key(&o1) {
                map.insert(
                    o1.clone(),
                    Object{name: o1.clone(), par: None, children: Vec::new()}
                );
            }

            if !map.contains_key(&o2) {
                map.insert(
                    o2.clone(),
                    Object{name: o2.clone(), par: Some(o1.clone()), children: Vec::new()}
                );
            }

            parent = map.get_mut(&o1).expect("uh oh");

            parent.children.push(o2.clone());

            chi.insert(o2.clone());
            map.get_mut(&o2).expect("shouldn't").par = Some(o1.clone());
            if par.contains(&o2) {
                par.remove(&o2);
            }

            if !chi.contains(&o1) {
                par.insert(o1.clone());
            }
        });

    let parents: Vec<String> = par.iter().map(|val| val).cloned().collect();
    (
        map,
        parents[0].clone()
    )
}

fn run(inp: &BTreeMap<String, Object>, cur: String, sum: u32, req: bool, target: Option<String>) -> (u32, bool) {
    let mut found = false;
    (
        inp
            .get(&cur)
            .expect("Nothing?")
            .children
            .iter()
            .fold(sum, | acc, next | {
                if req {
                    if *next == target.clone().expect("nothing") {
                        found = true;
                        sum + 1
                    } else {
                        let val = run(inp, next.clone(), sum + 1, req, target.clone());
                        if val.1 {
                            found = true;
                            val.0
                        } else {
                            acc + 0
                        }
                    }
                } else {
                    acc + run(inp, next.clone(), sum + 1, req, target.clone()).0
                }
            }),
        found
    )
}

fn heritage(inp: &BTreeMap<String, Object>, start: String) -> Vec<String> {
    let mut her: Vec<String> = Vec::new();
    let mut cur: &Object = inp.get(&start).expect("err 2");
    loop {
        if cur.par.is_none() {
            break;
        }
        her.push(cur.name.clone());
        let new = cur.par.clone().expect("thing");
        cur = inp.get(&new).expect("err");
    }

    her
}

pub fn lca(inp: &BTreeMap<String, Object>, a: String, b: String) -> u32 {
    let her1 = heritage(inp, a.clone());
    let her2 = heritage(inp, b.clone());
    let mut least: String = String::from("");

    for i in her1 {
        if her2.contains(&i) {
            least = i.clone();
            break;
        }
    }

    let v1 = run(inp, least.clone(), 0, true, Some(a.clone()));
    let v2 = run(inp, least.clone(), 0, true, Some(b.clone()));
    v1.0 + v2.0 - 2
}

pub fn part1() {
    let inp = read_input();
    println!("part 1: {}", run(&(inp.0), inp.1, 0, false, None).0);
}

pub fn part2() {
    let inp = read_input();
    println!("part2: {}", lca(&(inp.0), String::from("YOU"), String::from("SAN")));
}


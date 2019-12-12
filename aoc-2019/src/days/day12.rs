use crate::util;
use crate::util::lcm;
use std::cmp::Ordering;
use std::iter;

type Velocity = (i64, i64, i64);

#[derive(Debug, Clone)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64
}

impl Moon {
    fn new(string: String) -> Self {
        let equals: Vec<_> = string.match_indices('=').collect();
        let commas: Vec<_> = string.match_indices(',').collect();
        let inp = string.as_str();
        Moon{
            x: string.get(equals[0].0+1..commas[0].0)
                .expect("Non string")
                .parse()
                .expect("Not a number"),
            y: string.get(equals[1].0+1..commas[1].0)
                .expect("Non string")
                .parse()
                .expect("Not a number"),
            z: string.get(equals[2].0+1..inp.len()-1)
                .expect("Non string")
                .parse()
                .expect("Not a number"),
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    fn calc_gravity(&self, velocity: Velocity, other: &Moon) -> Velocity {
        let (x, y, z) = (self.x, self.y, self.z);
        let (mut vx, mut vy, mut vz) = velocity;
        let (ox, oy, oz) = (other.x, other.y, other.z);

        if x < ox {
            vx += 1;
        } else  if x > ox {
            vx -= 1;
        }

        if y < oy {
            vy += 1;
        } else  if y > oy {
            vy -= 1;
        }

        if z < oz {
            vz += 1;
        } else  if z > oz {
            vz -= 1;
        }

        (vx, vy, vz)
    }

    fn apply_velocity(&mut self, velocity: Velocity) {
        self.vx = velocity.0;
        self.vy = velocity.1;
        self.vz = velocity.2;

        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;

        // println!("    Getting myself -- {:?} {:?}", self, velocity);
    }

    fn calc_energy(&self) -> i64 {
        (self.x.abs() + self.y.abs() + self.z.abs())
            * (self.vx.abs() + self.vy.abs() + self.vz.abs())
    }
}

fn read_input() -> Vec<Moon> {
    util::read_lines(util::read_file("input/day12.txt"))
        .iter()
        .map(|string| -> Moon {
            Moon::new(string.clone())
        })
        .collect()
}

fn update_moons(moons: &mut Vec<Moon>) {
    let new_velocities: Vec<Velocity> = moons
        .iter()
        .map(|moon| -> Velocity {
            let velocity = (moon.vx, moon.vy, moon.vz);
            moons
                .iter()
                .fold(velocity, |cur_velocity, omoon| -> Velocity {
                    moon.calc_gravity(cur_velocity, omoon)
                })
        })
        .collect();

    moons
        .iter_mut()
        .zip(new_velocities)
        .for_each(|(moon, velocity)| moon.apply_velocity(velocity));
}

fn calc_velocity(pos: &Vec<i64>, vel: &Vec<i64>) -> Vec<i64> {
    vel
        .iter()
        .zip(pos.iter())
        .map(|(v, p)| -> i64 {
            v + pos.iter().fold(0, |sum, pe| {
                sum + match pe.cmp(p) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                }
            })
        })
        .collect()
}

fn find_cycle(mut pos: Vec<i64>, mut vel: Vec<i64>) -> usize {
    let init = (pos.clone(), vel.clone());
    iter::repeat(()).take_while(|_| {
        vel = calc_velocity(&pos, &vel);
        pos = pos
            .iter_mut()
            .zip(vel.iter())
            .map(|(p, v)| {
                *p + *v
            })
            .collect();
        (pos.clone(), vel.clone()) != init
    }).count() + 1
}

pub fn part1() {
    let mut moons = read_input();
    let debug = false;

    for i in 0..1000 {
        update_moons(&mut moons);

        if debug {
            println!("\niteration {}", i);
            moons
                .iter()
                .for_each(|moon| println!("Moon {:?}", moon));

            println!("total energy {}",
                moons.iter().fold(0, |sum, moon| sum + moon.calc_energy())
            );
        }
    }

    let total_energy = moons.iter().fold(0, |sum, moon| sum + moon.calc_energy());
    println!("Total energy {}", total_energy);
}

pub fn part2() {
    let moons = read_input();
    let xpos: Vec<i64> = moons.iter().map(|moon| -> i64 { moon.x }).collect();
    let xvel: Vec<i64> = moons.iter().map(|moon| -> i64 { moon.vx }).collect();
    let ypos: Vec<i64> = moons.iter().map(|moon| -> i64 { moon.y }).collect();
    let yvel: Vec<i64> = moons.iter().map(|moon| -> i64 { moon.vy }).collect();
    let zpos: Vec<i64> = moons.iter().map(|moon| -> i64 { moon.z }).collect();
    let zvel: Vec<i64> = moons.iter().map(|moon| -> i64 { moon.vz }).collect();

    let xcycle = find_cycle(xpos, xvel);
    let ycycle = find_cycle(ypos, yvel);
    let zcycle = find_cycle(zpos, zvel);

    println!("Cycles {} {} {} lcm = {}",
        xcycle,
        ycycle,
        zcycle,
        lcm(xcycle, lcm(ycycle, zcycle))
    );
}

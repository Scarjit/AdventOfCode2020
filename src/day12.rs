use crate::day12::Action::{E, F, L, N, R, S, W};
use std::str::FromStr;

type Program = Vec<Action>;

#[derive(Debug, Clone)]
pub struct Point2D {
    x: isize,
    y: isize,
}

impl Point2D {
    pub fn calc_manhattan(&self, other: Point2D) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_at(1) {
            ("N", v) => N(v.parse().unwrap()),
            ("S", v) => S(v.parse().unwrap()),
            ("E", v) => E(v.parse().unwrap()),
            ("W", v) => W(v.parse().unwrap()),
            ("L", v) => L(v.parse().unwrap()),
            ("R", v) => R(v.parse().unwrap()),
            ("F", v) => F(v.parse().unwrap()),
            _ => {
                unreachable!()
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Ship {
    current_position: Point2D,

    /// N: 0
    /// E: 1
    /// S: 2
    /// W: 3
    facing_direction: u8,
    waypoint: Point2D,
}

impl Ship {
    pub fn step_once_p1(&mut self, action: &Action) {
        match action {
            N(v) => {
                self.current_position.y += v;
            }
            S(v) => {
                self.current_position.y -= v;
            }
            E(v) => {
                self.current_position.x += v;
            }
            W(v) => {
                self.current_position.x -= v;
            }
            L(v) => {
                self.facing_direction = self.facing_direction.wrapping_sub((v / 90) as u8) % 4;
            }
            R(v) => {
                self.facing_direction = (self.facing_direction + (v / 90) as u8) % 4;
            }
            F(v) => match self.facing_direction {
                0 => self.step_once_p1(&Action::N(*v)),
                1 => self.step_once_p1(&Action::E(*v)),
                2 => self.step_once_p1(&Action::S(*v)),
                3 => self.step_once_p1(&Action::W(*v)),
                _ => {}
            },
        }
    }

    pub fn step_once_p2(&mut self, action: &Action) {
        match action {
            N(v) => {
                self.waypoint.y += v;
            }
            S(v) => {
                self.waypoint.y -= v;
            }
            E(v) => {
                self.waypoint.x += v;
            }
            W(v) => {
                self.waypoint.x -= v;
            }
            L(v) => match v {
                90 | 180 | 270 => {
                    let n = (v / 90isize) as usize;
                    for i in 0..n {
                        let current_y = self.waypoint.y;
                        self.waypoint.y = self.waypoint.x;
                        self.waypoint.x = -current_y;
                    }
                }
                _ => {
                    unreachable!()
                }
            },
            R(v) => match v {
                90 | 180 | 270 => {
                    let n = (v / 90isize) as usize;
                    for i in 0..n {
                        let current_x = self.waypoint.x;
                        self.waypoint.x = self.waypoint.y;
                        self.waypoint.y = -current_x;
                    }
                }
                _ => {
                    unreachable!()
                }
            },
            F(v) => {
                self.current_position.x += v * self.waypoint.x;
                self.current_position.y += v * self.waypoint.y;
            }
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> (Ship, Program) {
    (
        Ship {
            current_position: Point2D { x: 0, y: 0 },
            facing_direction: 1,
            waypoint: Point2D { x: 10, y: 1 },
        },
        input
            .lines()
            .map(|l| Action::from_str(l).unwrap())
            .collect(),
    )
}

#[aoc(day12, part1)]
pub fn solve_part_1(input: &(Ship, Program)) -> usize {
    let mut ship = input.0.clone();
    for action in &input.1 {
        ship.step_once_p1(action);
    }
    ship.current_position.calc_manhattan(Point2D { x: 0, y: 0 })
}

#[aoc(day12, part2)]
pub fn solve_part_2(input: &(Ship, Program)) -> usize {
    let mut ship = input.0.clone();
    for action in &input.1 {
        ship.step_once_p2(action);
    }
    ship.current_position.calc_manhattan(Point2D { x: 0, y: 0 })
}

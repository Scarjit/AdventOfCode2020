use crate::day3::Square::Tree;
use smallvec::{SmallVec, smallvec};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Square {
    Tree,
    Field,
}
impl Square {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Field,
            '#' => Self::Tree,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Slope {
    right: usize,
    down: usize,
}

#[derive(Debug)]
struct Forest {
    squares: Vec<Vec<Square>>,
}

impl Forest {
    pub fn square_at(&self, x: usize, y: usize) -> Option<Square> {
        self.squares
            .get(y)
            .map(|row| row.get(x % row.len()))
            .map(|o_sqr| o_sqr.copied())
            .flatten()
    }
    pub fn travel_slope(&self, slope: Slope) -> Vec<Square> {
        let (mut x, mut y) = (0, 0);
        let mut path = Vec::with_capacity(324);
        while let Some(square) = self.square_at(x, y) {
            path.push(square);
            x += slope.right;
            y += slope.down;
        }
        path
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Forest {
    Forest {
        squares: input
            .lines()
            .map(|l| l.chars().map(|c| Square::from_char(c)).collect())
            .collect(),
    }
}

#[aoc(day3, part1)]
fn solve_part_1(input: &Forest) -> usize {
    input
        .travel_slope(Slope { right: 3, down: 1 })
        .iter()
        .filter(|sqr| sqr == &&Tree)
        .count()
}


#[aoc(day3, part2)]
fn solve_part_2(input: &Forest) -> usize{
    let slopes = vec![
        Slope{right:1,down:1},
        Slope{right:3,down:1},
        Slope{right:5,down:1},
        Slope{right:7,down:1},
        Slope{right:1,down:2},
    ];

    slopes
        .iter()
        .map(|slope| {
            input.travel_slope(*slope)
                .iter()
                .filter(|sqr| sqr == &&Tree)
                .count()
        }).product()
}
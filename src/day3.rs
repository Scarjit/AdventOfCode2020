#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Square {
    Field = 0,
    Tree = 1,
}
impl Square {
    #[inline]
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
    #[inline]
    pub fn square_at(&self, x: usize, y: usize) -> Option<Square> {
        self.squares
            .get(y)
            .map(|row| row.get(x % row.len()))
            .map(|o_sqr| o_sqr.copied())
            .flatten()
    }
    #[inline]
    pub fn travel_slope(&self, slope: Slope) -> usize {
        let (mut x, mut y) = (0, 0);
        let mut n = 0;
        while let Some(square) = self.square_at(x, y) {
            n += square as usize;
            x += slope.right;
            y += slope.down;
        }
        return n;
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
    input.travel_slope(Slope{right: 3, down: 1})
}


#[aoc(day3, part2)]
fn solve_part_2(input: &Forest) -> usize{
    let slopes = [
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
        }).product()
}
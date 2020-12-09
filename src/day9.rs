use itertools;
use itertools::Itertools;

const PREAMBLE_LENGTH: usize = 25;

fn find_x_set(mut numbers: Vec<usize>, search_val: usize) -> Option<(usize, usize)> {
    //println!("Search set: {:?}", numbers);
    //println!("Search val: {:?}", search_val);
    let (mut l_value, mut r_value) = (0, numbers.len() - 1);
    numbers.sort_unstable();
    while l_value < r_value {
        match (numbers[l_value], numbers[r_value]) {
            (a, b) if a + b == search_val => return Some((a, b)),
            (a, b) if a + b < search_val => l_value += 1,
            _ => r_value -= 1,
        }
    }
    None
}

#[derive(Debug, Clone)]
pub struct XMASData {
    data: Vec<usize>,
}

impl XMASData {
    fn check_has_unused_num(&mut self, current_i: usize, current_d: usize) -> bool {
        let min_off = current_i - PREAMBLE_LENGTH;
        let search_range = &self.data[min_off..current_i];

        match find_x_set(search_range.to_vec(), current_d) {
            None => return false,
            Some(v) => v,
        };
        true
    }

    pub fn check_seq_validity(&mut self) -> usize {
        let mut i = PREAMBLE_LENGTH;
        for d in self.data.clone().iter().skip(25) {
            if !self.check_has_unused_num(i, *d) {
                return *d;
            }

            i += 1;
        }
        0
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> XMASData {
    #[cfg(target_arch = "x86")]
    panic!("x86 is not supported !");

    XMASData {
        data: input.lines().map(|l| l.parse().unwrap()).collect(),
    }
}

#[aoc(day9, part1)]
pub fn solve_part_1(input: &XMASData) -> usize {
    let mut input_clone = input.clone();
    input_clone.check_seq_validity()
}

#[aoc(day9, part2)]
pub fn solve_part_2(input: &XMASData) -> isize {
    let mut input_clone = input.clone();
    let x = input_clone.check_seq_validity();
    0
}

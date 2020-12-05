use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct BoardingPass {
    pub row: usize,
    pub seat: usize,
    pub id: usize,
}

impl BoardingPass {
    pub fn from_str(s: &str) -> Self {
        let row = Self::decode_slice(&s[..7], 127);
        let seat = Self::decode_slice(&s[7..], 7);
        let id = row * 8 + seat;
        Self { row, seat, id }
    }

    fn decode_slice(slice: &str, max: usize) -> usize {
        slice.chars().enumerate().fold(max, |curr, (i, c)| match c {
            'F' | 'L' => curr - 2usize.pow((slice.len() - i - 1) as u32),
            'B' | 'R' => curr,
            _ => panic!("Invalid BoardingPass"),
        })
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<BoardingPass> {
    input.lines().map(|l| BoardingPass::from_str(l)).collect()
}

#[aoc(day5, part1)]
fn solve_part_1(input: &Vec<BoardingPass>) -> usize {
    input.iter().map(|bpass| bpass.id).max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part_2(input: &Vec<BoardingPass>) -> usize {
    let mut ids = input.iter().map(|bpass| bpass.id).collect::<Vec<usize>>();

    ids.sort_unstable();
    let min_id = *ids.first().unwrap();
    let max_id = *ids.last().unwrap();
    for id in min_id..max_id {
        if !ids.contains(&id) {
            return id;
        }
    }
    panic!()
}

#[aoc(day5, part2, using_hashset)]
fn solve_part_2_hashset(input: &Vec<BoardingPass>) -> usize {
    let mut ids = input.iter().map(|bpass| bpass.id).collect::<Vec<usize>>();

    ids.sort_unstable();
    let min_id = *ids.first().unwrap();
    let max_id = *ids.last().unwrap();
    let idset: HashSet<usize> = HashSet::from_iter(ids);
    for id in min_id..max_id {
        if !idset.contains(&id) {
            return id;
        }
    }
    panic!()
}

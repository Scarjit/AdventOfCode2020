use rayon::prelude::*;
use stackvec::TryCollect;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Instructions {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl FromStr for Instructions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split(' ').try_collect::<[&str; 2]>().unwrap();
        Ok(match splits[0] {
            "acc" => Self::ACC(splits[1].parse().unwrap()),
            "jmp" => Self::JMP(splits[1].parse().unwrap()),
            "nop" => Self::NOP(splits[1].parse().unwrap()),
            _ => {
                panic!()
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    instructions: Vec<Instructions>,
    current_pointer: usize,
    accumulator: isize,
    trace: HashMap<usize, Instructions>,
}

impl Program {
    pub fn step_once(&mut self) {
        let current_instruction = &self.instructions[self.current_pointer];
        match current_instruction {
            Instructions::ACC(i) => {
                self.accumulator += i;
                self.current_pointer += 1;
            }
            Instructions::JMP(i) => {
                let next_pointer: isize = self.current_pointer as isize + i;
                self.current_pointer = next_pointer as usize;
            }
            Instructions::NOP(_) => {
                self.current_pointer += 1;
            }
        }
    }

    //Returns true if program terminates
    pub fn run_program(&mut self) -> bool {
        loop {
            if self.current_pointer >= self.instructions.len() {
                return true;
            }
            if self.trace.contains_key(&self.current_pointer) {
                return false;
            }
            self.trace.insert(
                self.current_pointer,
                self.instructions[self.current_pointer].clone(),
            );
            self.step_once();
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Program {
    let instructions = input
        .lines()
        .map(|l| Instructions::from_str(l).unwrap())
        .collect::<Vec<Instructions>>();
    let len = &instructions.len();
    Program {
        instructions,
        current_pointer: 0,
        accumulator: 0,
        trace: HashMap::with_capacity(*len),
    }
}

#[aoc(day8, part1)]
pub fn solve_part_1(input: &Program) -> isize {
    let mut prg = input.clone();
    prg.run_program();
    prg.accumulator
}

#[aoc(day8, part2)]
pub fn solve_part_2(input: &Program) -> isize {
    let mut prg = input.clone();
    prg.run_program();

    for t in &prg.trace {
        let new_instruction = match t.1 {
            Instructions::JMP(i) => Instructions::NOP(*i),
            Instructions::NOP(i) => Instructions::JMP(*i),
            _ => continue,
        };

        let mut p_clone = prg.clone();
        p_clone.trace.clear();
        p_clone.current_pointer = 0;
        p_clone.accumulator = 0;
        p_clone.instructions[*t.0] = new_instruction;

        if p_clone.run_program() {
            return p_clone.accumulator;
        }
    }
    panic!()
}

#[aoc(day8, part2, rayon)]
pub fn solve_part_2_rayon(input: &Program) -> isize {
    let mut prg = input.clone();
    prg.run_program();

    let x = prg
        .trace
        .iter()
        .filter(|f| match f.1 {
            Instructions::ACC(_) => false,
            _ => true,
        })
        .collect::<Vec<(&usize, &Instructions)>>()
        .into_par_iter()
        .filter_map(|t| {
            let new_instruction = match t.1 {
                Instructions::JMP(i) => Instructions::NOP(*i),
                Instructions::NOP(i) => Instructions::JMP(*i),
                _ => {
                    panic!()
                }
            };

            let mut p_clone = prg.clone();
            p_clone.trace.clear();
            p_clone.current_pointer = 0;
            p_clone.accumulator = 0;
            p_clone.instructions[*t.0] = new_instruction;

            if p_clone.run_program() {
                Some(p_clone)
            } else {
                None
            }
        })
        .find_any(|_| true)
        .unwrap();

    x.accumulator
}

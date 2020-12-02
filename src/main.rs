use crate::day1::Day1;
use crate::day2::Day2;

mod day1;
mod day2;

trait RunnableDay {
    fn run(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub(crate) fn read_input(day: i32) -> Result<Vec<String>, std::io::Error> {
    Ok(std::fs::read_to_string(format!(
        "/home/scarjit/CLionProjects/adventofcode/src/day{}/input",
        day
    ))?
    .split('\n')
    .map(String::from)
    .collect())
}

fn main() {
    let day1 = Day1::default();
    day1.run().expect("Day1 failed to run");

    let day2 = Day2::default();
    day2.run().expect("Day 2 failed to run");
}

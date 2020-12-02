use crate::day1::Day1;
mod day1;

trait RunnableDay{
    fn run(&self) -> Result<(),std::io::Error>{
        Ok(())
    }
}

pub(crate) fn read_input(day: i32) -> Result<Vec<String>,std::io::Error> {
    Ok(std::fs::read_to_string("/home/scarjit/CLionProjects/adventofcode/src/day1/input")?.split('\n').map(
        |c| String::from(c)
    ).collect())
}

fn main() {
    let day1 = Day1::default();
    day1.run().expect("Day1 failed to run");
}

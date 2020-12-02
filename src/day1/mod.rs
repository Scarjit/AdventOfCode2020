use crate::{read_input, RunnableDay};
use std::time::Instant;

#[derive(Default, Debug)]
pub(crate) struct Day1 {}

impl RunnableDay for Day1 {
    fn run(&self) -> Result<(), std::io::Error> {
        let input = read_input(1)?;
        let mut numbers = input
            .iter()
            .filter(|p| !p.is_empty())
            .map(|c| c.parse().expect("Couldn't parse to integer"))
            .collect::<Vec<i32>>();

        //Part 1
        for i in 0..numbers.len() {
            let n_current = numbers[i];
            for i2 in i..numbers.len() {
                let n_current_2 = numbers[i2];
                let nx = n_current + n_current_2;
                if nx == 2020 {
                    println!(
                        "{} + {} -> 2020 | {}",
                        n_current,
                        n_current_2,
                        n_current * n_current_2
                    );
                    break;
                }
            }
        }

        let now = Instant::now();
        //Part 2
        for i in 0..numbers.len() {
            let n_current = numbers[i];
            for i2 in i..numbers.len() {
                let n_current_2 = numbers[i2];
                let nx = n_current + n_current_2;
                if nx <= 2020 {
                    for i3 in i2..numbers.len() {
                        let n_current_3 = numbers[i3];
                        let ixx = nx + n_current_3;
                        if ixx == 2020 {
                            println!(
                                "{} + {} + {} -> 2020 | {}",
                                n_current,
                                n_current_2,
                                n_current_3,
                                n_current * n_current_2 * n_current_3
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

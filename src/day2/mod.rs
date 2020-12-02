use crate::{read_input, RunnableDay};

#[derive(Default, Debug)]
pub(crate) struct Day2 {}

#[derive(Debug)]
struct Password {
    password: String,
    policy_char: char,
    policy_min: u32,
    policy_max: u32,
}
impl Password {
    fn validate_part_1(&self) -> bool {
        let mut chars_found = 0;
        for c in self.password.chars() {
            if c == self.policy_char {
                chars_found += 1;
            }
        }
        if chars_found >= self.policy_min && chars_found <= self.policy_max {
            return true;
        }

        false
    }

    fn validate_part_2(&self) -> bool {
        let char_vec = self.password.chars().collect::<Vec<char>>();
        let mut found = 0;
        match char_vec.get((self.policy_min - 1) as usize) {
            Some(v) => {
                if v == &self.policy_char {
                    found += 1;
                }
            }
            _ => {
                return false;
            }
        }
        match char_vec.get((self.policy_max - 1) as usize) {
            Some(v) => {
                if v == &self.policy_char {
                    found += 1;
                }
            }
            _ => {
                return false;
            }
        }

        if found == 1 {
            return true;
        }

        false
    }
}

impl RunnableDay for Day2 {
    fn run(&self) -> Result<(), std::io::Error> {
        let input = read_input(2)?;

        let passwords = input
            .iter()
            .map(|x| {
                let splits = x.split(':').collect::<Vec<&str>>();
                let password = splits.get(1).unwrap().trim();
                let policy_splits = splits.get(0).unwrap().split(' ').collect::<Vec<&str>>();
                let policy_char = policy_splits.get(1).unwrap();
                let policy_amount_splits = policy_splits
                    .get(0)
                    .unwrap()
                    .split('-')
                    .collect::<Vec<&str>>();
                let min = policy_amount_splits.get(0).unwrap();
                let max = policy_amount_splits.get(1).unwrap();

                Password {
                    password: password.to_string(),
                    policy_char: policy_char.chars().next().unwrap(),
                    policy_min: min.parse().unwrap(),
                    policy_max: max.parse().unwrap(),
                }
            })
            .collect::<Vec<Password>>();

        let valid_passwords_part_1 = passwords
            .iter()
            .filter(|p| p.validate_part_1())
            .collect::<Vec<&Password>>();

        println!(
            "Number of valid passwords (Part 1): {}",
            valid_passwords_part_1.len()
        );

        let valid_passwords_part_2 = passwords
            .iter()
            .filter(|p| p.validate_part_2())
            .collect::<Vec<&Password>>();

        println!(
            "Number of valid passwords (Part 2): {}",
            valid_passwords_part_2.len()
        );

        Ok(())
    }
}

#[derive(Debug)]
pub struct Password {
    pub password: String,
    pub policy_char: char,
    pub policy_min: u32,
    pub policy_max: u32,
}
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .split('\n')
        .filter_map(|f| {
            return if f.is_empty() {
                None
            } else {
                let splits = f.split(':').collect::<Vec<&str>>();

                let password = splits.get(1).unwrap().trim();
                let policy_splits = splits.get(0).unwrap().split(' ').collect::<Vec<&str>>();
                let policy_char = policy_splits.get(1).unwrap();
                let mut policy_amount_splits = policy_splits
                    .get(0)
                    .unwrap()
                    .split('-');
                let min = policy_amount_splits.next().unwrap();
                let max = policy_amount_splits.next().unwrap();

                Some(Password {
                    password: password.to_string(),
                    policy_char: policy_char.chars().next().unwrap(),
                    policy_min: min.parse().unwrap(),
                    policy_max: max.parse().unwrap(),
                })
            };
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|p| {
            let mut chars_found = 0;
            for c in p.password.chars() {
                if c == p.policy_char {
                    chars_found += 1;
                }
            }
            if chars_found >= p.policy_min && chars_found <= p.policy_max {
                return true;
            }

            false
        })
        .collect::<Vec<&Password>>()
        .len()
}

#[aoc(day2, part2)]
pub fn solve_part2(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|p| {
            let char_vec = p.password.chars().collect::<Vec<char>>();
            let mut found = 0;
            match char_vec.get((p.policy_min - 1) as usize) {
                Some(v) => {
                    if v == &p.policy_char {
                        found += 1;
                    }
                }
                _ => {
                    return false;
                }
            }
            match char_vec.get((p.policy_max - 1) as usize) {
                Some(v) => {
                    if v == &p.policy_char {
                        found += 1;
                    }
                }
                _ => {
                    return false;
                }
            }

            found == 1
        })
        .collect::<Vec<&Password>>()
        .len()
}

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct CustomsForm {
    pub group_size: usize,
    pub answers: HashMap<char, usize>,
}

impl FromStr for CustomsForm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            group_size: s.lines().count(),
            answers: s
                .chars()
                .filter(|c| c.is_alphabetic())
                .fold(HashMap::new(), |mut map, c| {
                    if let Some(cnt) = map.get_mut(&c) {
                        *cnt += 1;
                    } else {
                        map.insert(c, 1);
                    }
                    map
                }),
        })
    }
}

impl CustomsForm {
    #[inline]
    pub fn answered_by_any(&self) -> usize {
        self.answers.len()
    }
    #[inline]
    pub fn answered_by_all(&self) -> usize {
        self.answers
            .values()
            .filter(|v| v == &&self.group_size)
            .count()
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<CustomsForm> {
    input
        .split("\n\n")
        .map(|l| l.parse::<CustomsForm>().unwrap())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part_1(input: &Vec<CustomsForm>) -> usize {
    input.iter().map(|cform| cform.answered_by_any()).sum()
}

#[aoc(day6, part2)]
pub fn solve_part_2(input: &Vec<CustomsForm>) -> usize {
    input.iter().map(|cform| cform.answered_by_all()).sum()
}

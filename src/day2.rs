type Password = String;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct PassPolicy {
    pub policy_char: char,
    pub policy_min: usize,
    pub policy_max: usize,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(PassPolicy, Password)> {
    input
        .lines()
        .map(|line| {
            let splits = line
                .split(|c| c == ' ' || c == '-' || c == ':')
                .collect::<Vec<&str>>();
            let pass_policy = PassPolicy {
                policy_min: splits.get(0).unwrap().parse().unwrap(),
                policy_max: splits.get(1).unwrap().parse().unwrap(),
                policy_char: splits.get(2).unwrap().parse().unwrap(),
            };
            (pass_policy, splits.last().unwrap().to_string())
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1_alt(inp: &[(PassPolicy, Password)]) -> usize {
    inp.iter()
        .filter(|(pol, pass)| {
            let n = pass.chars().filter(|c| *c == pol.policy_char).count();
            n >= pol.policy_min && n <= pol.policy_max
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(passwords: &[(PassPolicy, Password)]) -> usize {
    passwords
        .iter()
        .filter(|(pol, pass)| {
            let chars = pass.chars().collect::<Vec<char>>();
            let is_first_set = chars.get(pol.policy_min - 1).unwrap() == &pol.policy_char;
            let is_second_set = chars.get(pol.policy_max - 1).unwrap() == &pol.policy_char;
            is_first_set ^ is_second_set
        })
        .count()
}

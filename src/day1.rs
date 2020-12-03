#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

fn find_2020_set(mut numbers: Vec<u32>) -> Option<(u32, u32)> {
    let (mut l_value, mut r_value) = (0, numbers.len() - 1);
    numbers.sort();
    while l_value < r_value {
        match (numbers[l_value], numbers[r_value]) {
            (a, b) if a + b == 2020 => return Some((a, b)),
            (a, b) if a + b < 2020 => l_value += 1,
            _ => r_value -= 1,
        }
    }
    None
}

#[aoc(day1, part1)]
pub fn solve_part_1(numbers: &[u32]) -> u32 {
    let (a, b) = find_2020_set(numbers.to_vec()).unwrap();
    a * b
}

#[aoc(day1, part2)]
pub fn solve_part2(numbers: &[u32]) -> u32 {
    let max_v = 2020 - numbers.iter().min().unwrap();
    for i in 0..numbers.len() {
        let n_current = numbers[i];
        for i2 in i..numbers.len() {
            let n_current_2 = numbers[i2];
            let nx = n_current + n_current_2;
            if nx <= max_v {
                for n_current_3 in numbers.iter().skip(i2) {
                    if nx + n_current_3 == 2020 {
                        return n_current * n_current_2 * n_current_3;
                    }
                }
            }
        }
    }
    panic!()
}

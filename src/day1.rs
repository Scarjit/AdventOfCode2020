#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(numbers: &[u32]) -> u32 {
    for i in 0..numbers.len() {
        let n_current = numbers[i];
        for n_current_2 in numbers.iter().skip(i) {
            if n_current + n_current_2 == 2020 {
                return n_current * n_current_2;
            }
        }
    }
    panic!()
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
            } else {
                continue;
            }
        }
    }
    panic!()
}



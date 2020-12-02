#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split('\n')
        .filter(|p| !p.is_empty())
        .map(|c| c.parse().expect("Couldn't parse number"))
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(numbers: &[i32]) -> i32 {
    for i in 0..numbers.len() {
        let n_current = numbers[i];
        for n_current_2 in numbers.iter().skip(i) {
            let nx = n_current + n_current_2;
            if nx == 2020 {
                return n_current * n_current_2;
            }
        }
    }
    panic!()
}

#[aoc(day1, part2)]
pub fn solve_part2(numbers: &[i32]) -> i32 {
    for i in 0..numbers.len() {
        let n_current = numbers[i];
        for i2 in i..numbers.len() {
            let n_current_2 = numbers[i2];
            let nx = n_current + n_current_2;
            if nx <= 2020 {
                for n_current_3 in numbers.iter().skip(i2) {
                    let ixx = nx + n_current_3;
                    if ixx == 2020 {
                        return n_current * n_current_2 * n_current_3;
                    }
                }
            }
        }
    }
    panic!()
}

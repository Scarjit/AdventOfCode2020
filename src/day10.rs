#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut adapters = input
        .lines()
        .map(|l|{
            l.parse().unwrap()
        }).collect::<Vec<usize>>();
    adapters.push(0);
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap()+3);
    adapters
}

#[aoc(day10, part1)]
pub fn solve_part_1(input: &[usize]) -> usize {
    let diffs = input.windows(2).map(|w| w[1]-w[0]).collect::<Vec<usize>>();
    let c1 = diffs.iter().filter(|x| x == &&1).count();
    let c3 = diffs.iter().filter(|x| x == &&3).count();

    c1*c3
}

#[aoc(day10, part2)]
pub fn solve_part_2(input: &[usize]) -> usize {
    0
}

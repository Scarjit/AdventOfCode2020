#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let adapters = input
        .lines()
        .map(|l|{
            l.parse().unwrap()
        }).collect::<Vec<usize>>();
    adapters
}

#[aoc(day10, part1)]
pub fn solve_part_1(input: &[usize]) -> usize {
    let mut adapters:Vec<usize> = input.to_vec();
    adapters.push(0);
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap()+3);

    let diffs = adapters.windows(2).map(|w| w[1]-w[0]).collect::<Vec<usize>>();
    let c1 = diffs.iter().filter(|x| x == &&1).count();
    let c3 = diffs.iter().filter(|x| x == &&3).count();

    c1*c3
}

#[aoc(day10, part2)]
pub fn solve_part_2(input: &[usize]) -> usize {
    let mut adapters:Vec<usize> = input.to_vec();
    adapters.sort_unstable();

    let mut v = Vec::with_capacity(adapters.len()+1);
    v.push((0, 1usize));

    for adapter in adapters {
        let sum = v.iter().rev().take(3)
            .take_while(|(c,_)| {
                c+3 >= adapter
            })
            .map(|(_,s)|s).sum();
        v.push((adapter, sum));
    }

    v.last().unwrap().1
}

pub mod day5;

fn main() {
    let input = include_str!("../input/2020/day5.txt");
    let gen = day5::input_generator(input);

    for _ in 0..2_000_000i64 {
        let val = day5::solve_part_2_hashset(&gen);
        assert_eq!(val, 657);
    }
}
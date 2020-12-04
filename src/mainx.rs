pub mod day4;

fn main() {
    let input = include_str!("../input/2020/day4.txt");
    let gen = day4::input_generator(input);

    for _ in 0..2_000_000i64 {
        let val = day4::solve_part_2(&gen);
        assert_eq!(val, 160);
    }
}
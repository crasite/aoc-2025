use aoc_2025::day1;

fn main() {
    let input = include_str!("./day1_input.txt");
    let result = day1::solve1(input);
    println!("day 1 part 1: {result}");
    let result = day1::solve2(input);
    println!("day 1 part 2: {result}");
}

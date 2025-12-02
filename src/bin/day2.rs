use aoc_2025::day2;

fn main() {
    let input = include_str!("./day2_input.txt").trim();
    let result = day2::solve1(input);
    println!("day 2 part 1: {result}");
    let result = day2::solve2(input);
    println!("day 2 part 2: {result}");
}

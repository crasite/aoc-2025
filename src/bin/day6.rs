use aoc_2025::day6;

fn main() {
    let day = "6";
    let input = include_str!("./day6_input.txt").trim();
    let result = day6::solve1(input);
    println!("day {day} part 1: {result}");
    let result = day6::solve2(input);
    println!("day {day} part 2: {result}");
}

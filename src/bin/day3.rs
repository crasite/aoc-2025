use aoc_2025::day3;

fn main() {
    let day = "3";
    let input = include_str!("./day3_input.txt").trim();
    let result = day3::solve1(input);
    println!("day {day} part 1: {result}");
    let result = day3::solve2(input);
    println!("day {day} part 2: {result}");
}

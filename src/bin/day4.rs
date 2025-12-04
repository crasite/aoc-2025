use aoc_2025::day4;

fn main() {
    let day = "4";
    let input = include_str!("./day4_input.txt").trim();
    let result = day4::solve1(input);
    println!("day {day} part 1: {result}");
    let result = day4::solve2(input);
    println!("day {day} part 2: {result}");
}

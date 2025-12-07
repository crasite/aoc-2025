use aoc_2025::day7;

fn main() {
    let day = "7";
    let input = include_str!("./day7_input.txt");
    let result = day7::solve1(input);
    println!("day {day} part 1: {result}");
    let result = day7::solve2(input);
    println!("day {day} part 2: {result}");
}

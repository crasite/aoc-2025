use aoc_2025::day5;

fn main() {
    let day = "5";
    let input = include_str!("./day5_input.txt");
    let result = day5::solve1(input);
    println!("day {day} part 1: {result}");
    let result = day5::solve2(input);
    println!("day {day} part 2: {result}");
}

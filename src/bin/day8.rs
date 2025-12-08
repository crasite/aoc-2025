use aoc_2025::day8;

fn main() {
    let day = "8";
    let input = include_str!("./day8_input.txt");
    let result = day8::solve1(input, 1000);
    println!("day {day} part 1: {result}");
    let result = day8::solve2(input);
    println!("day {day} part 2: {result}");
}

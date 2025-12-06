use aoc_2025::day1;

fn main() {
    let day = "2";
    let input = include_str!("./day1_input.txt");
    let result = day1::solve1(input);
    println!("day {day} part 1: {result}");
    let result = day1::solve2(input);
    println!("day {day} part 2: {result}");
}

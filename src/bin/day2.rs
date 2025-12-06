use aoc_2025::day2;

fn main() {
    let day = "2";
    let input = include_str!("./day2_input.txt");
    let result = day2::solve1(input);
    println!("day {day} part 1: {result}");
    let result = day2::solve2(input);
    println!("day {day} part 2: {result}");
}

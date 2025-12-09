use std::time::Instant;

use aoc_2025::day9;

fn main() {
    let day = "9";
    let input = include_str!("./day9_input.txt");
    let now = Instant::now();
    let result = day9::solve1(input);
    println!("day {day} part 1: {result}");
    println!("solved in {} ms", now.elapsed().as_millis());
    let now = Instant::now();
    let result = day9::solve2(input);
    println!("day {day} part 2: {result}");
    println!("solved in {} ms", now.elapsed().as_millis());
}

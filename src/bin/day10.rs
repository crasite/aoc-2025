use std::time::Instant;

use aoc_2025::day10::{solve1, solve2};
const INPUT: &str = include_str!("./day10_input.txt");
const DAY: &str = "10";

fn main() {
    let now = Instant::now();
    let result = solve1(INPUT);
    println!("day {DAY} part 1: {result}");
    println!("solved in {} ms", now.elapsed().as_millis());
    // let now = Instant::now();
    // let result = solve2(INPUT);
    // println!("day {DAY} part 2: {result}");
    // println!("solved in {} ms", now.elapsed().as_millis());
}

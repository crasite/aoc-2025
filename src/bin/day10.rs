use std::{fs::File, time::Instant};

use aoc_2025::day10::{solve1, solve2};
const INPUT: &str = include_str!("./day10_input.txt");
const DAY: &str = "10";

fn main() {
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(600)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();
    let now = Instant::now();
    let result = solve1(INPUT);
    println!("day {DAY} part 1: {result}");
    println!("solved in {} ms", now.elapsed().as_millis());
    let now = Instant::now();
    let result = solve2(INPUT);
    println!("day {DAY} part 2: {result}");
    println!("solved in {} ms", now.elapsed().as_millis());
    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").unwrap();
        let mut options = pprof::flamegraph::Options::default();
        options.image_width = Some(2500);
        report.flamegraph_with_options(file, &mut options).unwrap();
    } else {
        println!("no report?");
    };
}

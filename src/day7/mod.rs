use std::collections::{HashMap, HashSet};

fn get_starting_loc(line: &str) -> usize {
    line.chars().position(|c| c == 'S').unwrap()
}

pub fn solve1(input: &str) -> usize {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut beam_list = HashSet::new();
    beam_list.insert(get_starting_loc(first_line));
    let mut total_split = 0;
    for line in lines {
        let mut split_list = vec![];
        for (idx, c) in line.chars().enumerate() {
            if c == '^' && beam_list.contains(&idx) {
                total_split += 1;
                split_list.push(idx)
            }
        }
        for split_idx in split_list {
            assert!(beam_list.remove(&split_idx));
            beam_list.insert(split_idx - 1);
            beam_list.insert(split_idx + 1);
        }
    }
    total_split
}

pub fn solve2(input: &str) -> usize {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let current_loc = get_starting_loc(first_line);
    let rest_line: Vec<&str> = lines.collect();
    //key in form of (current_loc,current_depth)
    let mut cache = HashMap::new();
    simulate(&rest_line, current_loc, 0, &mut cache)
}

fn simulate(
    line_list: &[&str],
    current_loc: usize,
    current_depth: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(cached_result) = cache.get(&(current_loc, current_depth)) {
        return *cached_result;
    }
    if current_depth >= line_list.len() {
        return 1;
    }
    let this_line = line_list[current_depth];
    if this_line.chars().nth(current_loc).unwrap() == '^' {
        let result = simulate(line_list, current_loc - 1, current_depth + 1, cache)
            + simulate(line_list, current_loc + 1, current_depth + 1, cache);
        cache.insert((current_loc, current_depth), result);
        result
    } else {
        let result = simulate(line_list, current_loc, current_depth + 1, cache);
        cache.insert((current_loc, current_depth), result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_get_starting_loc() {
        let input = ".......S.......";
        let expected = 7;
        assert_eq!(get_starting_loc(input), expected);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(TEST_INPUT), 21);
    }

    #[test]
    fn test_solve2_sub() {
        let input = r".......S.......
...............
.......^.......
...............
......^.^......
...............";

        assert_eq!(solve2(input), 4);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(TEST_INPUT), 40);
    }
}

fn get_highest_joltage(line: &str) -> usize {
    let jolt_list: Vec<usize> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let highest = jolt_list.iter().rev().skip(1).max().unwrap();
    let start_idx = jolt_list.iter().position(|v| v == highest).unwrap();
    let next_highest = jolt_list.iter().skip(start_idx + 1).max().unwrap();

    highest * 10 + next_highest
}

fn get_highest_joltage_huge(line: &str) -> usize {
    let jolt_list: Vec<usize> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut n = 12;
    let mut final_list = vec![];
    let mut current_skip_idx = 0;
    while n > 0 {
        let highest = jolt_list
            .iter()
            .skip(current_skip_idx)
            .rev()
            .skip(n - 1)
            .max()
            .unwrap();
        current_skip_idx = jolt_list
            .iter()
            .skip(current_skip_idx)
            .position(|v| v == highest)
            .unwrap()
            + 1
            + current_skip_idx;
        n -= 1;
        final_list.push(highest);
    }
    let mut result = 0;
    for (idx, n) in final_list.iter().enumerate() {
        result += *n * 10_usize.pow((11 - idx) as u32);
    }
    result
}

pub fn solve1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        total += get_highest_joltage(line);
    }
    total
}

pub fn solve2(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        total += get_highest_joltage_huge(line);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_joltage() {
        assert_eq!(get_highest_joltage("987654321111111"), 98);
        assert_eq!(get_highest_joltage("811111111111119"), 89);
        assert_eq!(get_highest_joltage("234234234234278"), 78);
        assert_eq!(get_highest_joltage("818181911112111"), 92);
    }
    #[test]
    fn test_highest_joltage_huge() {
        assert_eq!(get_highest_joltage_huge("987654321111111"), 987654321111);
        assert_eq!(get_highest_joltage_huge("811111111111119"), 811111111119);
        assert_eq!(get_highest_joltage_huge("234234234234278"), 434234234278);
        assert_eq!(get_highest_joltage_huge("818181911112111"), 888911112111);
    }

    #[test]
    fn test_solve1() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!(solve1(input), 357);
    }
}

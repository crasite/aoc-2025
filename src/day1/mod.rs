pub fn solve1(input: &str) -> usize {
    let lines = input.lines();
    let mut current = 50;
    let mut total_zero = 0;
    for line in lines {
        let (s, rest) = line.split_at(1);
        let symbol = if s.chars().next().unwrap() == 'L' {
            -1
        } else if s.chars().next().unwrap() == 'R' {
            1
        } else {
            panic!("line not possible")
        };
        let total: isize = rest.parse().unwrap();
        let movement = symbol * total;
        current += movement;
        if current < 0 {
            current += 100;
        }
        current %= 100;
        if current == 0 {
            total_zero += 1;
        }
    }
    total_zero
}
pub fn solve2(input: &str) -> isize {
    let lines = input.lines();
    let mut current = 50;
    let mut total_zero = 0;
    for line in lines {
        let (s, rest) = line.split_at(1);
        let symbol = if s.starts_with("L") {
            -1
        } else if s.starts_with("R") {
            1
        } else {
            panic!("line not possible")
        };
        let mut total: isize = rest.parse().unwrap();
        total_zero += total / 100;
        total %= 100;
        let movement = symbol * total;
        current += movement;
        if current < 0 {
            if current - movement == 0 {
                total_zero -= 1;
            }
            total_zero += 1;
            current += 100;
        }
        if current > 100 {
            total_zero += 1;
        }
        current %= 100;
        if current == 0 {
            total_zero += 1;
        }
    }
    total_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        let expected = 3;
        assert_eq!(solve1(input), expected)
    }

    #[test]
    fn test_part2() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        let expected = 6;
        assert_eq!(solve2(input), expected)
    }
}


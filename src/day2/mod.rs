use winnow::{
    Parser, Result,
    ascii::{dec_uint, multispace0},
    combinator::{separated, terminated},
};
struct Range {
    from: usize,
    to: usize,
}

fn parse_input(input: &mut &str) -> Result<Vec<Range>> {
    terminated(separated(1.., parse_range, ','), multispace0).parse_next(input)
}

fn parse_range(input: &mut &str) -> Result<Range> {
    let start: usize = dec_uint(input)?;
    "-".parse_next(input)?;
    let end: usize = dec_uint(input)?;
    Ok(Range {
        from: start,
        to: end,
    })
}

fn is_invalid(input: usize) -> bool {
    let input_str = input.to_string();
    if input_str.len() % 2 == 1 {
        return false;
    }
    let (front, back) = input_str.split_at(input_str.len() / 2);
    front == back
}

fn is_invalid_2(input: usize) -> bool {
    let input_str = input.to_string();

    for i in 1..=input_str.len() / 2 {
        if !input_str.len().is_multiple_of(i) {
            continue;
        }
        let slice = split_char(&input_str, i);
        if slice.iter().all(|v| *v == slice[0]) {
            return true;
        }
    }
    false
}

fn split_char(input: &str, size: usize) -> Vec<&str> {
    let mut result = vec![];
    let mut current = input;
    loop {
        let (start, rest) = current.split_at(size);
        result.push(start);
        if rest.is_empty() {
            break;
        }
        current = rest;
    }
    result
}

pub fn solve1(input: &str) -> usize {
    let mut result = 0;
    let ranges = parse_input.parse(input).unwrap();
    for range in ranges {
        for i in range.from..=range.to {
            if is_invalid(i) {
                result += i;
            }
        }
    }
    result
}

pub fn solve2(input: &str) -> usize {
    let mut result = 0;
    let ranges = parse_input.parse(input).unwrap();
    for range in ranges {
        for i in range.from..=range.to {
            if is_invalid_2(i) {
                result += i;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        let input = "222220-222224";
        let expected = Range {
            from: 222220,
            to: 222224,
        };
        let range = parse_range.parse(input).unwrap();
        assert_eq!(expected.from, range.from);
        assert_eq!(expected.to, range.to);
    }

    #[test]
    fn test_parse_vec_range() {
        let input = "11-22,95-115,998-1012";
        let output = parse_input.parse(input).unwrap();
        assert_eq!(3, output.len());
        assert_eq!(998, output[2].from);
    }

    #[test]
    fn test_is_invalid() {
        assert!(!is_invalid(101));
        assert!(is_invalid(2121));
        assert!(is_invalid(1010));
    }

    #[test]
    fn test_is_invalid_2() {
        assert!(!is_invalid_2(101));
        assert!(is_invalid_2(2121));
        assert!(is_invalid_2(1010));
        assert!(is_invalid_2(565656));
        assert!(is_invalid_2(824824824));
        assert!(is_invalid_2(1188511885));
    }

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let expected = 1227775554;
        assert_eq!(solve1(input), expected);
    }

    #[test]
    fn test_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let expected = 4174379265;
        assert_eq!(solve2(input), expected);
    }
}

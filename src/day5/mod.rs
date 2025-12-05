use std::collections::HashSet;

use winnow::{Parser, Result, ascii::dec_uint};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn is_in_range(&self, n: usize) -> bool {
        n >= self.from && n <= self.to
    }

    fn total_elem(&self) -> usize {
        self.to + 1 - self.from
    }

    fn join_range(&mut self, other: &Range) -> bool {
        let mut updated = false;
        if self.from <= other.to && self.to >= other.from {
            let new_to = self.to.max(other.to);
            let new_from = self.from.min(other.from);
            if self.to != new_to {
                self.to = new_to;
                updated = true
            }
            if self.from != new_from {
                self.from = new_from;
                updated = true
            }
        }
        updated
    }
}

pub fn solve1(input: &str) -> usize {
    let mut fresh_list = vec![];
    let mut line_iter = input.lines();
    let mut total_fresh = 0;
    for line in line_iter.by_ref() {
        if line.is_empty() {
            break;
        }
        fresh_list.push(parse_range.parse(line).unwrap());
    }
    for line in line_iter {
        let to_check = dec_uint::<_, usize, ()>.parse(line).unwrap();
        if fresh_list.iter().any(|r| r.is_in_range(to_check)) {
            total_fresh += 1;
        }
    }
    total_fresh
}

pub fn solve2(input: &str) -> usize {
    let mut fresh_list = vec![];
    let mut line_iter = input.lines();
    for line in line_iter.by_ref() {
        if line.is_empty() {
            break;
        }
        let range = parse_range.parse(line).unwrap();
        fresh_list.push(range);
    }
    let mut unique_range_set: HashSet<Range, std::hash::RandomState> =
        HashSet::from_iter(fresh_list);
    loop {
        let mut updated = false;
        let cloned_list: HashSet<Range, _> = Clone::clone(&unique_range_set);
        let mut new_range = HashSet::new();
        for range in unique_range_set.iter() {
            let mut range = range.clone();
            for cloned_range in cloned_list.iter() {
                let is_updated = range.join_range(cloned_range);
                updated = updated || is_updated;
            }
            new_range.insert(range);
        }
        if !updated {
            break;
        }
        unique_range_set = new_range;
    }
    let mut total = 0;
    for r in unique_range_set {
        total += r.total_elem();
    }
    total
}

fn parse_range(input: &mut &str) -> Result<Range> {
    let from = dec_uint.parse_next(input)?;
    "-".parse_next(input)?;
    let to = dec_uint.parse_next(input)?;
    Ok(Range { from, to })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        let input = "10-14";
        let result = parse_range.parse(input).unwrap();
        assert_eq!(result.from, 10);
        assert_eq!(result.to, 14);
    }

    #[test]
    fn test_join_range() {
        let mut range1 = Range { from: 10, to: 14 };
        let mut range2 = Range { from: 14, to: 18 };
        range1.join_range(&range2);
        range2.join_range(&range1);
        assert_eq!(range1.from, 10);
        assert_eq!(range1.to, 18);
        assert_eq!(range2.from, 10);
        assert_eq!(range2.to, 18);
    }

    #[test]
    fn test_solve1() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

        assert_eq!(solve1(input), 3)
    }

    #[test]
    fn test_solve2() {
        let input = r#"3-5
10-14
16-20
10-14
12-18
19-21
5-10
11-13
23-30
24-24

1
5
8
11
17
32"#;

        assert_eq!(solve2(input), 27)
    }
}

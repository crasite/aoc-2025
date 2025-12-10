use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use winnow::{Parser, ascii::dec_uint};

fn parse_red_tile(line: &mut &str) -> winnow::Result<(usize, usize)> {
    let x = dec_uint.parse_next(line)?;
    ",".parse_next(line)?;
    let y = dec_uint.parse_next(line)?;
    Ok((x, y))
}

fn rect_size(a: &(usize, usize), b: &(usize, usize)) -> usize {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

fn between(n: usize, min: usize, max: usize) -> bool {
    n > min && n < max
}

fn is_valid(a: &(usize, usize), b: &(usize, usize), list: &[(usize, usize)]) -> bool {
    let (min_x, min_y) = (a.0.min(b.0), a.1.min(b.1));
    let (max_x, max_y) = (a.0.max(b.0), a.1.max(b.1));

    for (&(x1, y1), &(x2, y2)) in list.iter().circular_tuple_windows() {
        // Vertical line crossing through the rectangle
        if x1 == x2 && between(x1, min_x, max_x) && y1.max(y2) > min_y && y1.min(y2) < max_y {
            return false;
        }
        // Horizontal line crossing through the rectangle
        if y1 == y2 && between(y1, min_y, max_y) && x1.max(x2) > min_x && x1.min(x2) < max_x {
            return false;
        }
    }
    true
}

pub fn solve1(input: &str) -> usize {
    let red_tile_list: Vec<_> = input
        .lines()
        .map(|line| parse_red_tile.parse(line).unwrap())
        .collect();

    red_tile_list
        .iter()
        .tuple_combinations()
        .map(|(a, b)| rect_size(a, b))
        .max()
        .unwrap_or(0)
}

pub fn solve2(input: &str) -> usize {
    let red_tile_list: Vec<_> = input
        .lines()
        .map(|line| parse_red_tile.parse(line).unwrap())
        .collect();

    let nonpar: Vec<_> = red_tile_list.iter().tuple_combinations().collect();
    nonpar
        .par_iter()
        .filter(|(a, b)| is_valid(a, b, &red_tile_list))
        .map(|(a, b)| rect_size(a, b))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let expected = 50;
        assert_eq!(solve1(input), expected);
    }

    #[test]
    fn test_solve2_1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let expected = 24;
        assert_eq!(solve2(input), expected);
    }

    #[test]
    fn test_solve2_2() {
        let input = "3,2
17,2
17,13
13,13
13,11
15,11
15,8
11,8
11,15
18,15
18,17
4,17
4,12
6,12
6,5
3,5";
        let expected = 66;
        assert_eq!(solve2(input), expected);
    }

    #[test]
    fn test_solve2_3() {
        let input = "4,2
13,2
13,4
8,4
8,6
11,6
11,10
4,10";
        let expected = 40;
        assert_eq!(solve2(input), expected);
    }
}

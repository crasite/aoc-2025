use crate::grid::Grid2D;

#[derive(PartialEq, Debug)]
enum Tile {
    Ground,
    Paper,
}

fn input_to_map(input: &str) -> Grid2D<Tile> {
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize;
    let char_list = input.lines().flat_map(|l| l.chars());
    let mut map = vec![];
    for c in char_list {
        match c {
            '.' => map.push(Tile::Ground),
            '@' => map.push(Tile::Paper),
            _ => unreachable!("what input is that!!!"),
        }
    }
    Grid2D { width, height, map }
}

pub fn solve1(input: &str) -> usize {
    let grid_map = input_to_map(input);
    let mut total = 0;
    for x in 0..grid_map.width {
        for y in 0..grid_map.height {
            if grid_map
                .get_at_index(x, y)
                .is_none_or(|v| *v == Tile::Ground)
            {
                continue;
            }
            let adjacent_papers = grid_map
                .adjacent(x, y)
                .iter()
                .flatten()
                .filter(|v| ***v == Tile::Paper)
                .count();
            if adjacent_papers < 4 {
                total += 1
            }
        }
    }
    total
}

pub fn solve2(input: &str) -> usize {
    let mut grid_map = input_to_map(input);
    let mut total = 0;
    loop {
        let mut updated = false;
        for x in 0..grid_map.width {
            for y in 0..grid_map.height {
                if grid_map
                    .get_at_index(x, y)
                    .is_none_or(|v| *v == Tile::Ground)
                {
                    continue;
                }
                let adjacent_papers = grid_map
                    .adjacent(x, y)
                    .iter()
                    .flatten()
                    .filter(|v| ***v == Tile::Paper)
                    .count();
                if adjacent_papers < 4 {
                    updated = true;
                    total += 1;

                    grid_map.replace_at_index(x, y, Tile::Ground);
                }
            }
        }
        if !updated {
            break;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let map = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
        assert_eq!(solve1(map), 13)
    }

    #[test]
    fn test_solve2() {
        let map = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
        assert_eq!(solve2(map), 43)
    }
}

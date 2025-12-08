use std::{
    collections::{BTreeMap, HashSet},
    ops::Sub,
};

use itertools::Itertools;
use winnow::{Parser, Result, ascii::dec_uint};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}
impl Coord {
    fn distance(&self, other: &Coord) -> f64 {
        let xdiff = self.x.sub(other.x) as f64;
        let ydiff = self.y.sub(other.y) as f64;
        let zdiff = self.z.sub(other.z) as f64;
        (xdiff.powi(2) + ydiff.powi(2) + zdiff.powi(2)).sqrt()
    }
}

fn parse_line(input: &mut &str) -> Result<Coord> {
    let x: u32 = dec_uint.parse_next(input)?;
    ','.parse_next(input)?;
    let y: u32 = dec_uint.parse_next(input)?;
    ','.parse_next(input)?;
    let z: u32 = dec_uint.parse_next(input)?;
    Ok(Coord {
        x: x as i64,
        y: y as i64,
        z: z as i64,
    })
}

pub fn solve1(input: &str, connection: usize) -> usize {
    let mut coord_list = vec![];
    let mut junction_set: Vec<HashSet<Coord>> = vec![];
    for line in input.lines() {
        coord_list.push(parse_line.parse(line).unwrap());
    }

    let mut min_dist_set: BTreeMap<usize, [Coord; 2]> = BTreeMap::new();
    for (i, coord) in coord_list.iter().enumerate() {
        for other in coord_list.iter().skip(i + 1) {
            let distance = (coord.distance(other) * 1000000.0) as usize;
            if min_dist_set.len() < connection {
                min_dist_set.insert(distance, [coord.clone(), other.clone()]);
            } else {
                let last = min_dist_set.pop_last().unwrap();
                if last.0 > distance {
                    min_dist_set.insert(distance, [coord.clone(), other.clone()]);
                } else {
                    min_dist_set.insert(last.0, last.1);
                }
            }
        }
    }
    for (_, [a, b]) in min_dist_set {
        if !junction_set.iter().any(|set| set.contains(&a)) {
            if let Some(set) = junction_set.iter_mut().find(|c| c.contains(&b)) {
                set.insert(a);
            } else {
                let set = HashSet::from_iter([a, b]);
                junction_set.push(set);
            }
        } else if !junction_set.iter().any(|set| set.contains(&b)) {
            let set = junction_set.iter_mut().find(|c| c.contains(&a)).unwrap();
            set.insert(b);
        } else {
            let a_idx = junction_set
                .iter()
                .position(|set| set.contains(&a))
                .unwrap();
            let main_set = junction_set.swap_remove(a_idx);
            if main_set.contains(&b) {
                junction_set.push(main_set);
            } else {
                let b_idx = junction_set
                    .iter()
                    .position(|set| set.contains(&b))
                    .unwrap();
                let sub_set = junction_set.swap_remove(b_idx);
                let new_set = main_set.union(&sub_set).cloned().collect();
                junction_set.push(new_set);
            }
        }
    }

    let mut junction_size = junction_set.into_iter().map(|v| v.len()).sorted_unstable();
    junction_size.next_back().unwrap()
        * junction_size.next_back().unwrap()
        * junction_size.next_back().unwrap()
}

pub fn solve2(input: &str) -> usize {
    let mut coord_list = vec![];
    let mut junction_set: Vec<HashSet<Coord>> = vec![];
    for line in input.lines() {
        coord_list.push(parse_line.parse(line).unwrap());
    }

    let mut min_dist_set: BTreeMap<usize, [Coord; 2]> = BTreeMap::new();
    for (i, coord) in coord_list.iter().enumerate() {
        for other in coord_list.iter().skip(i + 1) {
            let distance = (coord.distance(other) * 1000000.0) as usize;
            min_dist_set.insert(distance, [coord.clone(), other.clone()]);
        }
    }
    for (_, [a, b]) in min_dist_set {
        let ax = a.x;
        let bx = b.x;
        if !junction_set.iter().any(|set| set.contains(&a)) {
            if let Some(set) = junction_set.iter_mut().find(|c| c.contains(&b)) {
                set.insert(a);
            } else {
                let set = HashSet::from_iter([a, b]);
                junction_set.push(set);
            }
        } else if !junction_set.iter().any(|set| set.contains(&b)) {
            let set = junction_set.iter_mut().find(|c| c.contains(&a)).unwrap();
            set.insert(b);
        } else {
            let a_idx = junction_set
                .iter()
                .position(|set| set.contains(&a))
                .unwrap();
            let main_set = junction_set.swap_remove(a_idx);
            if main_set.contains(&b) {
                junction_set.push(main_set);
            } else {
                let b_idx = junction_set
                    .iter()
                    .position(|set| set.contains(&b))
                    .unwrap();
                let sub_set = junction_set.swap_remove(b_idx);
                let new_set = main_set.union(&sub_set).cloned().collect();
                junction_set.push(new_set);
            }
        }
        if junction_set.len() == 1 && junction_set[0].len() == coord_list.len() {
            return (ax * bx) as usize;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "162,817,812";
        let expected = Coord {
            x: 162,
            y: 817,
            z: 812,
        };
        assert_eq!(parse_line.parse(input).unwrap(), expected);
    }

    #[test]
    fn test_part1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(solve1(input, 10), 40);
    }

    #[test]
    fn test_part2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(solve2(input), 25272);
    }
}

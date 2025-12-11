use std::collections::HashMap;

use winnow::{Parser, combinator::separated, token::take};

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    output_list: Vec<&'a str>,
}

fn parse_node<'a>(input: &mut &'a str) -> winnow::Result<Node<'a>> {
    let name = take(3usize).parse_next(input)?;
    ": ".parse_next(input)?;
    let output_list: Vec<&str> = separated(1.., take(3usize), " ").parse_next(input)?;
    Ok(Node { name, output_list })
}

fn path_finding(node_list: &[Node], from: &str, target: &str) -> usize {
    let node = node_list.iter().find(|n| n.name == from).unwrap();
    if node.output_list.contains(&target) {
        return 1;
    }
    node.output_list
        .iter()
        .map(|v| path_finding(node_list, v, target))
        .sum()
}

fn path_finding_mandatory_visit<'a>(
    node_list: &[Node<'a>],
    from: &'a str,
    target: &str,
    mandatory_visit: &[&'a str],
    non_functioning_node: &mut HashMap<(&'a str, Vec<&'a str>), usize>,
) -> usize {
    if let Some(cache_result) = non_functioning_node.get(&(from, mandatory_visit.to_vec())) {
        return *cache_result;
    }
    let mandatory_visit: Vec<_> = mandatory_visit
        .iter()
        .filter(|visit| **visit != from)
        .cloned()
        .collect();
    let node = node_list.iter().find(|n| n.name == from).unwrap();
    if node.output_list.contains(&target) {
        if mandatory_visit.is_empty() {
            return 1;
        } else {
            non_functioning_node.insert((from, mandatory_visit.to_vec()), 0);
            return 0;
        }
    }
    let rs = node
        .output_list
        .iter()
        .map(|v| {
            path_finding_mandatory_visit(
                node_list,
                v,
                target,
                &mandatory_visit,
                non_functioning_node,
            )
        })
        .sum();
    non_functioning_node.insert((from, mandatory_visit.to_vec()), rs);
    rs
}

pub fn solve1(input: &str) -> usize {
    let node_list: Vec<_> = input
        .lines()
        .map(|v| parse_node.parse(v).unwrap())
        .collect();
    path_finding(&node_list, "you", "out")
}

pub fn solve2(input: &str) -> usize {
    let mut cache = HashMap::new();
    let mandatory = vec!["fft", "dac"];
    let node_list: Vec<_> = input
        .lines()
        .map(|v| parse_node.parse(v).unwrap())
        .collect();

    path_finding_mandatory_visit(&node_list, "svr", "out", &mandatory, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(TEST_INPUT), 5);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(solve2(TEST_INPUT_2), 2);
    }
}

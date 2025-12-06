use itertools::Itertools;
use winnow::{
    Parser, Result,
    ascii::{dec_uint, multispace1},
    combinator::{alt, separated},
    stream::AsChar,
};

#[derive(Clone, PartialEq, Debug)]
enum Operator {
    Plus,
    Mult,
}

fn parse_number_row(input: &mut &str) -> Result<Vec<usize>> {
    separated(1.., dec_uint::<_, usize, _>, multispace1).parse_next(input)
}

fn parse_operator_row(input: &mut &str) -> Result<Vec<Operator>> {
    separated(1.., parse_operator, multispace1).parse_next(input)
}

fn parse_operator(input: &mut &str) -> Result<Operator> {
    alt((("+".value(Operator::Plus)), ("*".value(Operator::Mult)))).parse_next(input)
}

pub fn solve1(input: &str) -> usize {
    let input_row_count = input.lines().count() - 1;
    let mut lines = input.lines();
    let mut number_rows = vec![];
    for _ in 0..input_row_count {
        let line = lines.next().unwrap();
        number_rows.push(parse_number_row.parse(line.trim()).unwrap());
    }
    let operator_row = parse_operator_row
        .parse(lines.next().unwrap().trim())
        .unwrap();
    let mut rs = 0;
    for (i, op) in operator_row.into_iter().enumerate() {
        match op {
            Operator::Plus => {
                let mut col_result = 0;
                for num_row in number_rows.iter() {
                    col_result += num_row[i]
                }
                rs += col_result;
            }
            Operator::Mult => {
                let mut col_result = 1;
                for num_row in number_rows.iter() {
                    col_result *= num_row[i]
                }
                rs += col_result;
            }
        }
    }
    rs
}

pub fn solve2(input: &str) -> usize {
    let mut rs = 0;
    let mut lines = input.lines();
    let mut operator_row = parse_operator_row
        .parse(lines.next_back().unwrap().trim())
        .unwrap()
        .into_iter();
    let lines: Vec<&str> = lines.collect();
    let mut operand_list: Vec<Vec<usize>> = vec![];
    let mut n = 0;
    for _ in 0..lines.len() + 10 {
        operand_list.push(vec![]);
    }
    for idx in 0..lines[0].len() {
        let mut empty = true;
        for line in lines.iter() {
            let c = line.chars().nth(idx).unwrap();
            if c.is_space() {
                continue;
            }
            let digit = c.to_digit(10).unwrap() as usize;
            operand_list[n].push(digit);
            empty = false;
        }
        n += 1;
        if empty || idx + 1 == lines[0].len() {
            n = 0;
            let parsed_operand_list = operand_list
                .iter()
                .map(|num_arr| num_arr.iter().map(|v| v.to_string()).join(""))
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<usize>().unwrap());

            let op = operator_row.next().unwrap();
            match op {
                Operator::Plus => {
                    let mut col_result = 0;
                    for num_row in parsed_operand_list {
                        col_result += num_row
                    }
                    rs += col_result;
                }
                Operator::Mult => {
                    let mut col_result = 1;
                    for num_row in parsed_operand_list {
                        col_result *= num_row
                    }
                    rs += col_result;
                }
            }

            operand_list.iter_mut().for_each(|v| v.clear());
        }
    }
    rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_row() {
        let input = r#"123 328  51 64"#;
        let expected = vec![123, 328, 51, 64];
        assert_eq!(parse_number_row.parse(input).unwrap(), expected);
    }

    #[test]
    fn test_parse_operator_row() {
        let input = r#"*   +   *   +"#;
        let expected = vec![
            Operator::Mult,
            Operator::Plus,
            Operator::Mult,
            Operator::Plus,
        ];
        assert_eq!(parse_operator_row.parse(input).unwrap(), expected);
    }

    #[test]
    fn test_part_1() {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;
        let expected = 4277556;
        assert_eq!(solve1(input), expected);
    }

    #[test]
    fn test_part_2() {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;
        let expected = 3263827;
        assert_eq!(solve2(input), expected);
    }
}

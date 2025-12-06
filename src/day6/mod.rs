use winnow::{
    Parser, Result,
    ascii::{dec_uint, multispace1},
    combinator::{alt, separated},
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
}

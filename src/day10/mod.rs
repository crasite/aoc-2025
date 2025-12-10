use winnow::{
    ModalResult, Parser,
    ascii::{dec_int, dec_uint},
    combinator::{alt, repeat_till, separated},
    token::rest,
};

struct InstructionLine {
    target: Vec<isize>,
    button_list: Vec<Vec<isize>>,
    joltage_requirement: Vec<isize>,
}

fn parse_target_command(input: &mut &str) -> winnow::Result<isize> {
    alt(('.'.value(-1), '#'.value(1))).parse_next(input)
}

fn parse_target(input: &mut &str) -> winnow::Result<Vec<isize>> {
    "[".parse_next(input)?;
    let result = repeat_till(1.., parse_target_command, "]").parse_next(input)?;
    Ok(result.0)
}

fn parse_button(length: usize) -> impl FnMut(&mut &str) -> winnow::Result<Vec<isize>> {
    move |input: &mut &str| {
        let mut updated_list = vec![1; length];
        "(".parse_next(input)?;
        let dec_list: Vec<usize> =
            separated(1.., dec_uint::<_, usize, _>, ",").parse_next(input)?;
        ") ".parse_next(input)?;
        for i in dec_list {
            updated_list[i] = -1;
        }
        Ok(updated_list)
    }
}

fn parse_instruction_line(line: &mut &str) -> winnow::Result<InstructionLine> {
    let target = parse_target.parse_next(line)?;
    let line_length = target.len();
    " ".parse_next(line)?;
    let button_list: Vec<Vec<isize>> = repeat_till(1.., parse_button(line_length), "{")
        .parse_next(line)?
        .0;
    let joltage_requirement: Vec<isize> =
        separated(1.., dec_int::<_, isize, _>, ",").parse_next(line)?;
    rest.parse_next(line)?;
    Ok(InstructionLine {
        target,
        button_list,
        joltage_requirement,
    })
}

pub fn solve1(input: &str) -> usize {
    for line in input.lines() {
        parse_instruction_line.parse(line).unwrap();
    }
    todo!()
}

pub fn solve2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(TEST_INPUT), 1);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(solve2(TEST_INPUT), 2);
    }
}

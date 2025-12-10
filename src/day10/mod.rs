use good_lp::*;
use itertools::Itertools;
use winnow::{
    Parser,
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

fn solve_line_part2(line: &InstructionLine) -> usize {
    // Step 1: Create the problem
    let mut vars = ProblemVariables::new();
    let num_buttons = line.button_list.len();

    // Step 2: Create one variable for each button
    let button_presses: Vec<Variable> = (0..num_buttons)
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();
    let exp: Expression = button_presses.iter().sum();

    // Step 4: Build constraints - one for each counter
    let num_counters = line.joltage_requirement.len();
    let mut constraints = Vec::with_capacity(num_counters);

    for counter_idx in 0..num_counters {
        // Build constraint for this counter
        let target_value = line.joltage_requirement[counter_idx] as f64;

        // Sum up button presses that affect this counter
        let mut counter_expression = Expression::from(0);
        for (button_idx, btn) in line.button_list.iter().enumerate() {
            if btn[counter_idx] == -1 {
                counter_expression += button_presses[button_idx];
            }
        }

        // Create constraint: counter_expression == target_value
        constraints.push(constraint!(counter_expression == target_value));
    }

    // Step 5: Build and solve the problem

    let mut problem = vars.minimise(&exp).using(coin_cbc).with_all(constraints);
    problem.set_parameter("loglevel", "0");
    let solution = problem.solve().unwrap();

    solution.eval(exp) as usize
}

fn solve_line(line: &InstructionLine) -> usize {
    for current_count in 1..line.target.len() {
        for combi in line.button_list.iter().combinations(current_count) {
            let combi_result = combi.iter().fold(vec![-1; line.target.len()], |acc, n| {
                acc.iter().zip_eq(*n).map(|v| v.0 * v.1).collect()
            });
            if combi_result == line.target {
                return current_count;
            }
        }
    }
    unreachable!("imposible to turn on the light");
}

pub fn solve1(input: &str) -> usize {
    let mut instruction_list = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        instruction_list.push(parse_instruction_line.parse(line).unwrap());
    }
    let mut rs = 0;
    for instruction in instruction_list {
        rs += solve_line(&instruction);
    }
    rs
}

pub fn solve2(input: &str) -> usize {
    input
        .lines()
        .map(|line| parse_instruction_line.parse(line).unwrap())
        .map(|v| solve_line_part2(&v))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(TEST_INPUT), 7);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(solve2(TEST_INPUT), 34);
    }
}

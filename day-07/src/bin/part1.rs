use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

const OPERATORS: [Operator; 2] = [Operator::Add, Operator::Multiply];

fn process(input: &str) -> String {
    let (_input, equations) = parse(input).expect("should parse");

    equations
        .par_iter()
        .filter_map(|equation| {
            let total = equation.0;
            let operands = &equation.1;
            let num_operators = operands.len() - 1;

            println!("total={total} num_operators={num_operators}");

            (0..num_operators)
                .map(|_| OPERATORS)
                .multi_cartesian_product()
                .inspect(|operators| println!("operators={operators:?}"))
                .any(|operators| {
                    // dbg!(&operators);

                    let mut operators_iter = operators.iter();

                    operands.iter().copied().reduce(|acc, operand| {
                        let next = operators_iter.next().unwrap();

                        // dbg!(&acc, &next, &operand);
                        match next {
                            Operator::Add => acc + operand,
                            Operator::Multiply => acc * operand,
                        }
                    }) == Some(total)
                })
                .then_some(total)
        })
        .sum::<u64>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(tag(" "), complete::u64),
        ),
    )(input)
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );

        assert_eq!(result, "3749");
    }
}

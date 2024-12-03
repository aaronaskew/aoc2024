use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_input, instructions) = parse(input).unwrap();
    dbg!(_input, &instructions);

    let mut total = 0;
    let mut enabled = true;

    for instruction in instructions {
        match instruction {
            Instruction::Mul(a, b) => {
                if enabled {
                    total += a * b
                }
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
        dbg!(instruction, enabled, total);
    }

    total.to_string()
}

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = many1(
        many_till(anychar, nom::branch::alt((instr_mul, instr_do, instr_dont)))
            .map(|(_discard, instruction)| instruction),
    )(input)?;
    Ok((input, instructions))
}

fn instr_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, (a, b)) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(a, b)))
}

fn instr_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}
fn instr_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
",
        );

        assert_eq!(result, "48");
    }
}

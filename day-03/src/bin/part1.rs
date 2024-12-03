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
    let (_input, mults) = parse(input).unwrap();
    dbg!(_input, &mults);

    mults
        .iter()
        .map(|mult| mult.a * mult.b)
        .sum::<u32>()
        .to_string()
}

#[derive(Debug)]
struct Mult {
    a: u32,
    b: u32,
}

fn parse(input: &str) -> IResult<&str, Vec<Mult>> {
    let (input, mults) = many1(many_till(anychar, mult).map(|(_discard, mult)| mult))(input)?;
    Ok((input, mults))
}

fn mult(input: &str) -> IResult<&str, Mult> {
    let (input, _) = tag("mul")(input)?;
    let (input, (a, b)) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Mult { a, b }))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
",
        );

        assert_eq!(result, "161");
    }
}

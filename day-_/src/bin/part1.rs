fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );

        assert_eq!(result, "11");
    }
}

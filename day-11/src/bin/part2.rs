use cached::proc_macro::cached;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let stones = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    stones
        .iter()
        .map(|stone| count(*stone, 75))
        .sum::<u64>()
        .to_string()
}

#[cached]
fn count(stone: u64, steps: u64) -> u64 {
    if steps == 0 {
        return 1;
    } else if stone == 0 {
        return count(1, steps - 1);
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let (left, right) = stone_str.split_at(stone_str.len() / 2);
            return count(left.parse().unwrap(), steps - 1)
                + count(right.parse().unwrap(), steps - 1);
        } else {
            return count(stone * 2024, steps - 1);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process("125 17");

        assert_eq!(result, "55312");
    }
}

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

struct Lists {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Lists {
    fn smililarity(&self) -> u32 {
        let mut similarity = 0;

        for i in 0..self.left.len() {
            let value = self.left[i];
            let count = self
                .right
                .iter()
                .filter(|r_value| **r_value == value)
                .count();

            similarity += value * count as u32;
        }

        similarity
    }
}

fn process(input: &str) -> String {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let nums = line.split_ascii_whitespace().collect::<Vec<_>>();
        list1.push(nums[0].parse::<u32>().unwrap());
        list2.push(nums[1].parse::<u32>().unwrap());
    }

    let lists = Lists {
        left: list1,
        right: list2,
    };

    lists.smililarity().to_string()
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

        assert_eq!(result, "31");
    }
}

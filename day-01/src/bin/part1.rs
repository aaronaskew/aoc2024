fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let nums = line.split_ascii_whitespace().collect::<Vec<_>>();
        list1.push(nums[0].parse::<u32>().unwrap());
        list2.push(nums[1].parse::<u32>().unwrap());
    }

    dbg!(&list1);
    dbg!(&list2);

    list1.sort();
    list2.sort();

    dbg!(&list1);
    dbg!(&list2);

    let mut sum: u32 = 0;

    for i in 0..list1.len() {
        let distance = list1[i].abs_diff(list2[i]);
        sum += distance;
    }

    sum.to_string()
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

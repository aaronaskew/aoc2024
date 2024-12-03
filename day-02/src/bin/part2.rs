use itertools::Itertools;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

enum Direction {
    Increase,
    Decrease,
}

type Report = Vec<i32>;

fn check_safety(report: &Report) -> Safety {
    let mut direction = None;

    for (a, b) in report.iter().tuple_windows() {
        let diff = a.abs_diff(*b);

        if !(1..=3).contains(&diff) {
            return Safety::Unsafe;
        }

        match (a - b).signum() {
            -1 => match direction {
                None => direction = Some(Direction::Decrease),
                Some(Direction::Decrease) => {}
                Some(Direction::Increase) => return Safety::Unsafe,
            },
            1 => match direction {
                None => direction = Some(Direction::Increase),
                Some(Direction::Increase) => {}
                Some(Direction::Decrease) => return Safety::Unsafe,
            },
            _ => return Safety::Unsafe,
        }
    }

    Safety::Safe
}

fn process(input: &str) -> String {
    let reports: Vec<Report> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    reports
        .iter()
        .map(|report| {
            if check_safety(report) == Safety::Unsafe {
                for index in 0..report.len() {
                    let mut new_report = report.clone();
                    new_report.remove(index);
                    if check_safety(&new_report) == Safety::Safe {
                        return Safety::Safe;
                    } else {
                        continue;
                    }
                }
            } else {
                return Safety::Safe;
            }

            Safety::Unsafe
        })
        .filter(|safety| safety == &Safety::Safe)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );

        assert_eq!(result, "4");
    }
}

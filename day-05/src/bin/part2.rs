use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    if let Ok((_input, (rules, updates))) = parse(input) {
        updates
            .iter()
            .filter(|update| {
                !update.iter().all(|page| {
                    let mut page_rules_iter = rules.iter().filter(|rule| rule.before == *page);
                    let page_idx = update.iter().position(|p| p == page).unwrap();
                    page_rules_iter.all(|page_rule| {
                        if let Some(after_page_idx) =
                            update.iter().position(|p| p == &page_rule.after)
                        {
                            page_idx < after_page_idx
                        } else {
                            true
                        }
                    })
                })
            })
            // incorrectly ordered
            .inspect(|update| {
                dbg!(update);
            })
            .map(|update| {
                let mut update = update.clone();

                update.sort_by(|a, b| {
                    if rules
                        .iter()
                        .filter(|rule| rule.before == *a && rule.after == *b)
                        .count()
                        > 0
                    {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });

                update
            })
            .inspect(|update| {
                dbg!(update);
            })
            .map(|update| {
                assert!(update.len() % 2 != 0);
                let middle_idx = update.len() / 2;
                update[middle_idx]
            })
            .sum::<u32>()
            .to_string()
    } else {
        panic!()
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Vec<u32>>)> {
    let (input, rules) = separated_list1(newline, rule)(input)?;
    dbg!(&input);
    let (input, updates) = preceded(
        many1(newline),
        separated_list1(newline, separated_list1(tag(","), complete::u32)),
    )(input)?;
    Ok((input, (rules, updates)))
}

fn rule(input: &str) -> IResult<&str, Rule> {
    map(
        separated_pair(complete::u32, tag("|"), complete::u32),
        |(before, after)| Rule { before, after },
    )(input)
}

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );

        assert_eq!(result, "123");
    }
}

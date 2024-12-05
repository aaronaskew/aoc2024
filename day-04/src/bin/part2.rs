use std::collections::HashMap;

use glam::IVec2;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

const DIRECTIONS: [[IVec2; 2]; 4] = [
    // northeast
    [IVec2::new(-1, 1), IVec2::new(1, -1)],
    // northwest
    [IVec2::new(1, 1), IVec2::new(-1, -1)],
    // southeast
    [IVec2::new(-1, -1), IVec2::new(1, 1)],
    // southwest
    [IVec2::new(1, -1), IVec2::new(-1, 1)],
];

fn process(input: &str) -> String {
    let grid: HashMap<IVec2, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect();

    grid.iter()
        .filter(|(_pos, c)| **c == 'A')
        .map(|(pos, _c)| {
            DIRECTIONS
                .iter()
                .map(|direction| {
                    grid.get(&(pos + direction[0])) == Some(&'M')
                        && grid.get(&(pos + direction[1])) == Some(&'S')
                })
                .filter(|found| *found)
                .count()
        })
        .filter(|count| *count == 2)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );

        assert_eq!(result, "9");
    }
}

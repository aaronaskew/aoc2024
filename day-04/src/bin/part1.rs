use std::collections::HashMap;

use glam::IVec2;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

const DIRECTIONS: [[IVec2; 3]; 8] = [
    // east
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
    // west
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
    // north
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
    // south
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
    // northeast
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
    // northwest
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)],
    // southeast
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
    // southwest
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
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
        .filter(|(_pos, c)| **c == 'X')
        .map(|(pos, _c)| {
            DIRECTIONS
                .iter()
                .map(|direction| {
                    grid.get(&(pos + direction[0])) == Some(&'M')
                        && grid.get(&(pos + direction[1])) == Some(&'A')
                        && grid.get(&(pos + direction[2])) == Some(&'S')
                })
                .filter(|found| *found)
                .count()
        })
        .sum::<usize>()
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

        assert_eq!(result, "18");
    }
}

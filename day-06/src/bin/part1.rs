use std::collections::HashMap;

use glam::IVec2;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut grid: HashMap<IVec2, Cell> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, ch)| {
                (
                    IVec2::new(col as i32, row as i32),
                    match ch {
                        '.' => Cell::Empty,
                        '#' => Cell::Obstacle,
                        '^' => Cell::Visited,
                        _ => panic!("wrong cell type"),
                    },
                )
            })
        })
        .collect();

    let mut guard: (IVec2, Direction) = (
        { *grid.iter().find(|(_, v)| **v == Cell::Visited).unwrap().0 },
        Direction::North,
    );

    dbg!(&guard);

    loop {
        let next_cell = guard.0
            + match guard.1 {
                Direction::North => IVec2::new(0, -1),
                Direction::East => IVec2::new(1, 0),
                Direction::South => IVec2::new(0, 1),
                Direction::West => IVec2::new(-1, 0),
            };

        dbg!(&next_cell);

        if let Some(cell) = grid.get_mut(&next_cell) {
            match cell {
                Cell::Obstacle => {
                    //rotate 90°
                    guard.1 = match guard.1 {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    }
                }
                Cell::Empty | Cell::Visited => {
                    guard.0 = next_cell;
                    *cell = Cell::Visited;
                }
            }
        } else {
            break;
        }
    }

    grid.iter()
        .filter(|(_, v)| **v == Cell::Visited)
        .count()
        .to_string()
}

#[derive(Debug, PartialEq)]
enum Cell {
    Empty,
    Obstacle,
    Visited,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );

        assert_eq!(result, "41");
    }
}

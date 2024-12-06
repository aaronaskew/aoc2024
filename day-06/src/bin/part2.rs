use std::collections::{HashMap, HashSet};

use glam::IVec2;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let init_grid: HashMap<IVec2, Cell> = input
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

    let init_guard: (IVec2, Direction) = (
        {
            *init_grid
                .iter()
                .find(|(_, v)| **v == Cell::Visited)
                .unwrap()
                .0
        },
        Direction::North,
    );

    dbg!(&init_guard);

    init_grid
        .par_iter()
        .filter(|(k, _)| detect_loop(init_grid.clone(), init_guard, **k))
        .count()
        .to_string()
}

fn detect_loop(
    init_grid: HashMap<IVec2, Cell>,
    init_guard: (IVec2, Direction),
    obstacle_pos: IVec2,
) -> bool {
    let mut grid = init_grid.clone();
    let mut guard = init_guard;

    if let Some(cell) = grid.get_mut(&obstacle_pos) {
        if matches!(*cell, Cell::Visited | Cell::Obstacle) {
            return false;
        } else {
            *cell = Cell::Obstacle
        }
    }

    let mut visited: HashSet<(IVec2, Direction)> = HashSet::new();

    visited.insert(guard);

    loop {
        let next_cell = guard.0
            + match guard.1 {
                Direction::North => IVec2::new(0, -1),
                Direction::East => IVec2::new(1, 0),
                Direction::South => IVec2::new(0, 1),
                Direction::West => IVec2::new(-1, 0),
            };

        // dbg!(&next_cell);

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

            if !visited.insert(guard) {
                return true;
            } else {
                // println!("inserted {guard:?}");
            }
        } else {
            break;
        }
    }

    false
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Obstacle,
    Visited,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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

        assert_eq!(result, "6");
    }
}

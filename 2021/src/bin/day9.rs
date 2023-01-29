use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use itertools::Itertools;
use utils::line_to_char_vec;

struct Grid {
    contents: Vec<Vec<i32>>,
    row_size: usize,
    column_size: usize,
}

impl Grid {
    fn new(contents: Vec<Vec<i32>>) -> Self {
        let row_size = contents.len();
        let column_size = contents[0].len();
        Grid {
            contents,
            row_size,
            column_size,
        }
    }

    fn immediate_neighbour_coords(&self, row: i32, col: i32) -> HashSet<(usize, usize)> {
        let mut neighbours: HashSet<(usize, usize)> = HashSet::new();
        for &(row_offset, col_offset) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_row: i32 = row + row_offset;
            let new_col: i32 = col + col_offset;
            if new_row >= 0
                && new_row < self.row_size as i32
                && new_col >= 0
                && new_col < self.column_size as i32
            {
                neighbours.insert((new_row as usize, new_col as usize));
            }
        }
        neighbours
    }

    fn find_low_points(&self) -> Vec<i32> {
        let mut low_points = Vec::new();
        for row in 0..self.row_size {
            for col in 0..self.column_size {
                let neighbours = self.immediate_neighbour_coords(row as i32, col as i32);
                let mut low = true;
                for (r, c) in neighbours {
                    if self.contents[row][col] >= self.contents[r][c] {
                        low = false;
                        break;
                    }
                }
                if low {
                    low_points.push(self.contents[row][col]);
                }
            }
        }
        low_points
    }

    fn find_basins(&self) -> Vec<usize> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut basin_sizes: Vec<usize> = Vec::new();

        for row_num in 0..self.row_size {
            for col_num in 0..self.column_size {
                if !visited.contains(&(row_num, col_num)) && self.contents[row_num][col_num] != 9 {
                    let mut basin_size = 0;
                    let mut de_queue = VecDeque::new();
                    de_queue.push_back((row_num, col_num));
                    while let Some((row, col)) = de_queue.pop_front() {
                        if visited.contains(&(row, col)) {
                            continue;
                        }
                        visited.insert((row, col));
                        basin_size += 1;
                        let neighbours = self.immediate_neighbour_coords(row as i32, col as i32);
                        for (r, c) in neighbours {
                            if self.contents[r][c] != 9 {
                                de_queue.push_back((r, c));
                            }
                        }
                    }
                    basin_sizes.push(basin_size);
                }
            }
        }
        basin_sizes
    }
}

fn main() -> Result<()> {
    let contents = line_to_char_vec("inputs/day9.txt", |c| {
        char::to_digit(c, 10).map(|x| x as i32)
    })?;
    let grid = Grid::new(contents);
    let low_points = grid
        .find_low_points()
        .iter()
        .map(|point| point + 1)
        .sum::<i32>();
    println!("Part 1: {}", low_points);
    let basin_sizes = grid.find_basins();
    println!(
        "Part 2: {}",
        basin_sizes.iter().sorted().rev().take(3).product::<usize>()
    );
    Ok(())
}

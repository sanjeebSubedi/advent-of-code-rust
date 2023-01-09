#[allow(unused_variables)]
use std::{
    fs,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Board {
    sets: Vec<Vec<i32>>,
}

impl Board {
    fn new(horiz_sets: Vec<Vec<i32>>) -> Self {
        let mut vertical_sets: Vec<Vec<i32>> = Vec::new();
        for i in 0..horiz_sets[0].len() {
            vertical_sets.push(horiz_sets.iter().map(|v| v[i]).collect());
        }
        vertical_sets.extend(horiz_sets.to_owned());
        Board {
            sets: vertical_sets,
        }
    }

    fn check_bingo(&self) -> bool {
        self.sets.contains(&vec![])
    }

    fn remove_marked_number(&mut self, marked_number: i32) {
        for set in self.sets.iter_mut() {
            set.retain(|&x| x != marked_number)
        }
    }
}

fn get_winners(
    mut all_boards: Vec<Board>,
    marker: Vec<i32>,
    get_bingo_position: i32,
) -> Option<i32> {
    let mut bingo_position = 1;
    for number in marker {
        let mut to_remove: Vec<usize> = vec![];
        for (idx, board) in all_boards.iter_mut().enumerate() {
            board.remove_marked_number(number);
            if board.check_bingo() {
                let unmarked_sum: i32 = board.sets.iter().flatten().fold(0, |acc, x| acc + x) / 2;
                to_remove.push(idx);
                if bingo_position == get_bingo_position {
                    return Some(unmarked_sum * number);
                }
                bingo_position += 1;
            }
        }
        to_remove.sort_by(|a, b| b.cmp(a));
        for index in to_remove {
            all_boards.remove(index);
        }
    }
    None
}

fn main() -> Result<()> {
    let mut file_content: Vec<String> = BufReader::new(fs::File::open("inputs/day4.txt")?)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line != "")
        .collect();
    let marker = file_content.remove(0);
    let marker: Vec<i32> = marker
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let file_content: Vec<Vec<i32>> = file_content
        .iter_mut()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect()
        })
        .collect_vec();
    let mut all_boards: Vec<Board> = vec![];
    for board in file_content.windows(5).step_by(5) {
        all_boards.push(Board::new(board.to_owned()));
    }
    println!(
        "Part 1: {:?}",
        get_winners(all_boards.clone(), marker.clone(), 1).unwrap()
    );
    let boards_len = all_boards.len();
    println!(
        "Part 2: {:?}",
        get_winners(all_boards, marker, boards_len as i32).unwrap()
    );
    Ok(())
}

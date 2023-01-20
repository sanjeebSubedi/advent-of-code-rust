use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

fn decode_signal(input_signals: &Vec<HashSet<char>>) -> Result<Vec<HashSet<char>>> {
    let mut decoded: Vec<HashSet<char>> = vec![HashSet::new(); 10];
    let unique_sets = |input_signals: &Vec<HashSet<char>>, num: usize| {
        input_signals
            .iter()
            .find(|signal| signal.len() == num)
            .unwrap()
            .to_owned()
    };
    for num in vec![2, 3, 4, 7] {
        match num {
            2 => decoded[1] = unique_sets(&input_signals, 2),
            3 => decoded[7] = unique_sets(&input_signals, 3),
            4 => decoded[4] = unique_sets(&input_signals, 4),
            7 => decoded[8] = unique_sets(&input_signals, 7),
            _ => {}
        }
    }
    let seven_union_four: HashSet<char> = decoded[7].union(&decoded[4]).cloned().collect();
    for input_signal in input_signals {
        match input_signal.len() {
            6 => {
                if input_signal.is_superset(&seven_union_four) {
                    decoded[9] = input_signal.to_owned();
                } else if input_signal.is_superset(&decoded[1]) {
                    decoded[0] = input_signal.to_owned();
                } else {
                    decoded[6] = input_signal.to_owned();
                }
            }
            5 => {
                if input_signal.symmetric_difference(&seven_union_four).count() == 4 {
                    decoded[2] = input_signal.to_owned();
                } else if input_signal.is_superset(&decoded[1]) {
                    decoded[3] = input_signal.to_owned();
                } else {
                    decoded[5] = input_signal.to_owned();
                }
            }
            _ => {}
        }
    }
    Ok(decoded)
}

fn main() -> Result<()> {
    let contents: Vec<Vec<String>> = BufReader::new(fs::File::open("inputs/day8.txt")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split("|")
                .map(|sequence| sequence.trim())
                .flat_map(|seq| seq.split_whitespace())
                .map(|word| word.to_string())
                .collect()
        })
        .collect();
    let count = contents
        .iter()
        .flat_map(|entry| entry.iter().skip(10))
        .filter(|word| [2, 4, 3, 7].contains(&word.len()))
        .count() as u32;
    println!("Part 1: {}", count);

    let all_signals: Vec<Vec<HashSet<char>>> = contents
        .iter()
        .map(|inner_vec| {
            inner_vec
                .iter()
                .map(|signal| signal.chars().collect::<HashSet<char>>())
                .collect()
        })
        .collect();
    let mut outer_sum: i32 = 0;
    for (i, inputs) in all_signals.iter().enumerate() {
        let decoded_map = decode_signal(&inputs[..10].to_vec())?;
        let mut inner_sum = String::from("");
        for output in &all_signals[i][10..] {
            for (i, input) in decoded_map.iter().enumerate() {
                if input.symmetric_difference(&output).count() == 0 {
                    inner_sum.push_str(&i.to_string());
                }
            }
        }
        outer_sum += i32::from_str_radix(&inner_sum, 10)?;
    }
    println!("Part 2: {}", outer_sum);
    Ok(())
}

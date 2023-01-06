use std::{
    fs,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::Result;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(BufReader::new(fs::File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse::<T>().ok())
        .collect())
}

pub fn line_to_char_vec<T, F>(path: &str, func: F) -> Result<Vec<Vec<T>>>
where
    T: FromStr,
    F: Fn(char) -> Option<T>,
{
    Ok(BufReader::new(fs::File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.chars().map(|c| func(c).unwrap()).collect())
        .collect())
}

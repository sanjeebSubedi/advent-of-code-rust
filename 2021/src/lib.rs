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

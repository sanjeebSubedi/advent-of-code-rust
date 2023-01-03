use anyhow::Result;
use itertools::Itertools;
use utils::read_one_per_line;

fn sliding_window(window_size: usize) -> Result<usize> {
    Ok(read_one_per_line::<u32>("inputs/day1.txt")?
        .windows(window_size)
        .filter(|win| win[0] < win[window_size - 1])
        .collect_vec()
        .len())
}

fn main() -> Result<()> {
    println!("Part 1 answer: {}", sliding_window(2)?);
    println!("Part 2 answer: {}", sliding_window(4)?);
    Ok(())
}

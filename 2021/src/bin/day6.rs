use anyhow::Result;

fn fish_number_on_day(contents: Vec<u32>, days: usize) -> usize {
    let mut count_vector = vec![0; 9];
    for fish in contents {
        count_vector[fish.to_owned() as usize] += 1;
    }
    for _ in 0..days {
        let new_fish = count_vector.remove(0);
        count_vector.push(new_fish);
        count_vector[6] += count_vector[8];
    }
    count_vector.iter().sum()
}

fn main() -> Result<()> {
    let contents: Vec<u32> = include_str!("../../inputs/day6.txt")
        .trim()
        .split(',')
        .map(|c| c.parse::<u32>().expect("Error converting char to u32!"))
        .collect();
    println!("Part 1: {}", fish_number_on_day(contents.clone(), 80));
    println!("Part 2: {}", fish_number_on_day(contents, 256));
    Ok(())
}

fn main() {
    let mut contents: Vec<i32> = include_str!("../../inputs/day7.txt")
        .trim()
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    contents.sort();
    let median: i32 = contents[contents.len() / 2] as i32;
    let mean: i32 = contents.iter().sum::<i32>() / contents.iter().count() as i32;
    let fuel_used_1: i32 = contents.iter().map(|&pos| (pos - median).abs()).sum();
    let fuel_used_2: i32 = contents
        .iter()
        .map(|&pos| (pos - mean).abs() * ((pos - mean).abs() + 1))
        .sum::<i32>()
        / 2;
    println!("Part 1: {}", fuel_used_1);
    println!("Part 2: {}", fuel_used_2);
}

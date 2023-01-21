use anyhow::Result;

fn main() -> Result<()> {
    let contents = include_str!("../../inputs/day9.txt");
    let line_break = contents.find('\n').unwrap();
    let contents: Vec<i32> = contents
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i32))
        .collect();
    let mut part1_answer = 0;

    for (i, point) in contents.iter().enumerate() {
        let (left, right, up, down) = (
            contents.get(i.wrapping_sub(1)).unwrap_or(&10),
            contents.get(i + 1).unwrap_or(&10),
            contents.get(i.wrapping_sub(line_break)).unwrap_or(&10),
            contents.get(i + line_break).unwrap_or(&10),
        );
        if point < left && point < right && point < up && point < down {
            part1_answer += point + 1;
        }
    }
    println!("Part 1: {}", part1_answer);
    Ok(())
}

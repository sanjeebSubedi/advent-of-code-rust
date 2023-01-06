use anyhow::Result;
use std::str::FromStr;

enum Direction {
    Forward(i32),
    Upward(i32),
    Downward(i32),
}

struct Location {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Location {
    fn new() -> Self {
        Location {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn calculate_result(self, part: char) {
        println!("Part {}: {}", part, self.horizontal * self.depth)
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((command, distance)) = s.split_once(' ') {
            let distance = distance.parse()?;
            Ok(match command {
                "forward" => Direction::Forward(distance),
                "down" => Direction::Downward(distance),
                "up" => Direction::Upward(distance),
                _ => panic!("Invalid direction"),
            })
        } else {
            Err(anyhow::format_err!(
                "Couldnot separated command and distance!"
            ))
        }
    }
}

fn main() -> Result<()> {
    Ok::<(), anyhow::Error>(
        utils::read_one_per_line::<Direction>("inputs/day2.txt")?
            .iter()
            .fold(Location::new(), |location, direction| match direction {
                Direction::Forward(distance) => Location {
                    horizontal: location.horizontal + distance,
                    ..location
                },
                Direction::Upward(distance) => Location {
                    depth: location.depth - distance,
                    ..location
                },
                Direction::Downward(distance) => Location {
                    depth: location.depth + distance,
                    ..location
                },
            })
            .calculate_result('1'),
    )?;

    Ok::<(), anyhow::Error>(
        utils::read_one_per_line::<Direction>("inputs/day2.txt")?
            .iter()
            .fold(Location::new(), |location, direction| match direction {
                Direction::Forward(distance) => Location {
                    horizontal: location.horizontal + distance,
                    depth: location.depth + (location.aim * distance),
                    ..location
                },
                Direction::Upward(distance) => Location {
                    aim: location.aim - distance,
                    ..location
                },
                Direction::Downward(distance) => Location {
                    aim: location.aim + distance,
                    ..location
                },
            })
            .calculate_result('2'),
    )
}

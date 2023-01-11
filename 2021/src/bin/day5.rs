use anyhow::Result;
use itertools::Itertools;
use std::cmp::Ordering;

struct Diagram {
    matrix: Vec<Vec<i32>>,
}

impl Diagram {
    fn new(dimension: usize) -> Self {
        Diagram {
            matrix: vec![vec![0; dimension]; dimension],
        }
    }

    fn increase_index(&mut self, row: usize, col: usize) {
        self.matrix[col][row] += 1;
    }

    fn add_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<()> {
        match (x1.cmp(&x2), y1.cmp(&y2)) {
            (Ordering::Less, Ordering::Equal) => {
                for idx in x1..=x2 {
                    self.increase_index(idx.try_into()?, y2.try_into()?);
                }
            }
            (Ordering::Greater, Ordering::Equal) => {
                for idx in x2..=x1 {
                    self.increase_index(idx.try_into()?, y2.try_into()?);
                }
            }
            (Ordering::Equal, Ordering::Less) => {
                for idx in y1..=y2 {
                    self.increase_index(x1.try_into()?, idx.try_into()?);
                }
            }
            (Ordering::Equal, Ordering::Greater) => {
                for idx in y2..=y1 {
                    self.increase_index(x1.try_into()?, idx.try_into()?);
                }
            }
            (_, _) => {}
        };
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut contents: Vec<Vec<String>> = utils::read_one_per_line::<String>("inputs/day5.txt")?
        .iter()
        .map(|line| line.split("->").map(|x| x.trim().to_owned()).collect())
        .collect();
    let contents: Vec<Vec<Vec<i32>>> = contents
        .iter_mut()
        .map(|v| {
            v.iter_mut()
                .map(|s| {
                    s.split(",")
                        .collect_vec()
                        .iter_mut()
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    let max_num = contents.iter().flatten().flatten().max();
    let mut d = Diagram::new(max_num.unwrap().to_owned() as usize + 1);
    for content in contents {
        d.add_line(content[0][0], content[0][1], content[1][0], content[1][1])?;
    }
    let greater_than_2 = d
        .matrix
        .iter()
        .flatten()
        .fold(0, |acc, x| if x > &1 { acc + 1 } else { acc });
    println!("Part 1: {}", greater_than_2);
    Ok(())
}

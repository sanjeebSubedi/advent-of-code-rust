use anyhow::Result;
use itertools::Itertools;

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

    fn greater_than_two(&self) -> i32 {
        self.matrix
            .iter()
            .flatten()
            .fold(0, |acc, x| if x > &1 { acc + 1 } else { acc })
    }

    fn diagonally_add(
        &mut self,
        x1: i32,
        x2: i32,
        y1: i32,
        y2: i32,
        rev_x: bool,
        rev_y: bool,
    ) -> Result<()> {
        let iter_x: Vec<i32> = if rev_x {
            (x1..=x2).into_iter().rev().collect()
        } else {
            (x1..=x2).into_iter().collect()
        };
        let iter_y: Vec<i32> = if rev_y {
            (y1..=y2).into_iter().rev().collect()
        } else {
            (y1..=y2).into_iter().collect()
        };
        for (x, y) in iter_x.into_iter().zip(iter_y.into_iter()) {
            self.increase_index(x.try_into()?, y.try_into()?);
        }
        Ok(())
    }

    fn add_line(&mut self, point: &Vec<i32>, part2: bool) -> Result<()> {
        if point[0] == point[2] && point[1] == point[3] {
            return Ok(());
        }
        if point[0] == point[2] || point[1] == point[3] {
            for idx_x in point[0].min(point[2])..=point[0].max(point[2]) {
                for idx_y in point[1].min(point[3])..=point[1].max(point[3]) {
                    self.increase_index(idx_x.try_into()?, idx_y.try_into()?);
                }
            }
        }
        if part2 && (point[0] - point[2]).abs() == (point[1] - point[3]).abs() {
            self.diagonally_add(
                point[0].min(point[2]),
                point[0].max(point[2]),
                point[1].min(point[3]),
                point[1].max(point[3]),
                point[0] > point[2],
                point[1] > point[3],
            )?
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut contents: Vec<Vec<String>> = utils::read_one_per_line::<String>("inputs/day5.txt")?
        .iter()
        .map(|line| line.split("->").map(|x| x.trim().to_owned()).collect())
        .collect();
    let contents: Vec<Vec<i32>> = contents
        .iter_mut()
        .map(|v| {
            v.iter_mut()
                .map(|s| {
                    s.split(",")
                        .collect_vec()
                        .iter_mut()
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .flatten()
                .collect()
        })
        .collect();
    let max_num = contents.iter().flatten().max();
    let mut d1 = Diagram::new(max_num.unwrap().to_owned() as usize + 1);
    for point in &contents {
        d1.add_line(point, false)?;
    }
    let mut d2 = Diagram::new(max_num.unwrap().to_owned() as usize + 1);
    for point in &contents {
        d2.add_line(point, true)?;
    }
    println!("Part 1: {}", d1.greater_than_two());
    println!("Part 2: {}", d2.greater_than_two());
    Ok(())
}

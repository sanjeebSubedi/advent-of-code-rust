use anyhow::Result;

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
        self.matrix.iter().flatten().filter(|x| **x > 1).count() as i32
    }

    fn diagonally_add(&mut self, point: &Vec<i32>) -> Result<()> {
        let iter_x: Vec<i32> = if point[0] > point[2] {
            (point[2]..=point[0]).rev().collect()
        } else {
            (point[0]..=point[2]).collect()
        };
        let iter_y: Vec<i32> = if point[1] > point[3] {
            (point[3]..=point[1]).rev().collect()
        } else {
            (point[1]..=point[3]).collect()
        };
        for (x, y) in iter_x.into_iter().zip(iter_y.into_iter()) {
            self.increase_index(x.try_into()?, y.try_into()?);
        }
        Ok(())
    }

    fn add_line(&mut self, point: &Vec<i32>, add_diagonally: bool) -> Result<()> {
        if !add_diagonally && (point[0] == point[2] || point[1] == point[3]) {
            for idx_x in point[0].min(point[2])..=point[0].max(point[2]) {
                for idx_y in point[1].min(point[3])..=point[1].max(point[3]) {
                    self.increase_index(idx_x.try_into()?, idx_y.try_into()?);
                }
            }
        }
        if add_diagonally && (point[0] - point[2]).abs() == (point[1] - point[3]).abs() {
            self.diagonally_add(point)?
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let contents: Vec<Vec<i32>> = utils::read_one_per_line::<String>("inputs/day5.txt")?
        .iter()
        .map(|line| {
            line.split("->")
                .map(|x| x.trim())
                .flat_map(|s| s.split(","))
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let max_num = contents.iter().flatten().max().unwrap().to_owned();
    let mut diagram = Diagram::new(max_num as usize + 1);
    for point in &contents {
        diagram.add_line(point, false)?;
    }
    println!("Part 1: {}", diagram.greater_than_two());
    for point in &contents {
        diagram.add_line(point, true)?;
    }
    println!("Part 2: {}", diagram.greater_than_two());
    Ok(())
}

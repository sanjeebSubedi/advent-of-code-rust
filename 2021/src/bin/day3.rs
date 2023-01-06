use anyhow::{anyhow, Result};
use itertools::Itertools;

enum RatingType {
    OxygenGenerator,
    CO2Scrubber,
}

fn common_bit(bit_vectors: &Vec<Vec<u32>>) -> Result<(i32, i32)> {
    let mut bit_string: String = String::from("");
    for (index, _) in bit_vectors.first().unwrap().iter().enumerate() {
        let count = bit_vectors
            .iter()
            .fold(0, |acc, x| if x[index] == 0 { acc - 1 } else { acc + 1 });
        bit_string.push_str(if count < 0 { "0" } else { "1" });
    }
    let epsilon_string: String = bit_string
        .replace("0", "x")
        .replace("1", "0")
        .replace("x", "1");
    let gamma_rate = i32::from_str_radix(&bit_string, 2)
        .map_err(|_| anyhow!("Error while parsing gamma rate from binary string"))?;
    let epsilon_rate = i32::from_str_radix(&epsilon_string, 2)
        .map_err(|_| anyhow!("Error while parsing gamma rate from binary string"))?;
    Ok((gamma_rate, epsilon_rate))
}

fn get_rating(bit_vectors: &mut Vec<Vec<u32>>, rating_type: RatingType) -> Result<i32> {
    let mut index: usize = 0;
    while bit_vectors.len() != 1 {
        let count = bit_vectors
            .iter()
            .fold(0, |acc, x| if x[index] == 0 { acc - 1 } else { acc + 1 });
        match (count >= 0, &rating_type) {
            (true, RatingType::OxygenGenerator) => bit_vectors.retain(|x| x[index] == 1),
            (false, RatingType::OxygenGenerator) => bit_vectors.retain(|x| x[index] == 0),
            (true, RatingType::CO2Scrubber) => bit_vectors.retain(|x| x[index] == 0),
            (false, RatingType::CO2Scrubber) => bit_vectors.retain(|x| x[index] == 1),
        }
        index += 1;
    }
    let rating: String = bit_vectors[0].iter().map(|x| x.to_string()).join("");
    i32::from_str_radix(&rating, 2)
        .map_err(|_| anyhow!("Error while parsing integer from binary string"))
}

fn main() -> Result<()> {
    let mut char_vec = utils::line_to_char_vec("inputs/day3.txt", |c| char::to_digit(c, 2))?;
    let (gamma_rate, epsilon_rate) = common_bit(&char_vec)?;
    let oxygen_rating = get_rating(&mut char_vec.clone(), RatingType::OxygenGenerator)?;
    let co2_rating = get_rating(&mut char_vec, RatingType::CO2Scrubber)?;
    println!("Part 1: {}", gamma_rate * epsilon_rate);
    println!("Part 2: {}", oxygen_rating * co2_rating);
    Ok(())
}

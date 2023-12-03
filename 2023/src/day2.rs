use std::path::Path;

use fancy_regex::Regex;

use crate::utils::input_file_to_string_vec;

#[derive(Debug)]
struct Cube {
    colour: String,
    amount: u32,
}

pub fn part1(input: &Path) -> u32 {
    let lines = input_file_to_string_vec(input);
    let mut lines = lines.iter().enumerate();

    let mut total: u32 = 0;

    let re = Regex::new(r"(\d+)\s(blue|red|green)").unwrap();

    while let Some((game_number, line)) = lines.next() {
        let result = re
            .captures_iter(line)
            .map(|m| m.unwrap().get(0).expect("bad").as_str())
            .map(|m| {
                let temp: Vec<&str> = m.split_ascii_whitespace().collect();
                return Cube {
                    colour: String::from(temp.get(1).unwrap().to_owned()),
                    amount: temp.get(0).unwrap().parse().unwrap(),
                };
            })
            .filter(|cube| match cube.colour.as_str() {
                "red" => cube.amount > 12,
                "green" => cube.amount > 13,
                "blue" => cube.amount > 14,
                &_ => false,
            })
            .collect::<Vec<Cube>>();

        if result.len() == 0 {
            total += (game_number + 1) as u32
        }
    }

    return total;
}

pub fn part2(input: &Path) -> u32 {
    let lines = input_file_to_string_vec(input);
    let mut lines = lines.iter();

    let mut total: u32 = 0;

    let re = Regex::new(r"(\d+)\s(blue|red|green)").unwrap();

    while let Some(line) = lines.next() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        re.captures_iter(line)
            .map(|m| m.unwrap().get(0).expect("bad").as_str())
            .map(|m| {
                let temp: Vec<&str> = m.split_ascii_whitespace().collect();
                return Cube {
                    colour: String::from(temp.get(1).unwrap().to_owned()),
                    amount: temp.get(0).unwrap().parse().unwrap(),
                };
            })
            .for_each(|cube| match cube.colour.as_str() {
                "red" => {
                    if cube.amount > max_red {
                        max_red = cube.amount
                    }
                }
                "green" => {
                    if cube.amount > max_green {
                        max_green = cube.amount
                    }
                }
                "blue" => {
                    if cube.amount > max_blue {
                        max_blue = cube.amount
                    }
                }
                &_ => (),
            });

        total += (max_red * max_green * max_blue) as u32
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_passes() {
        assert!(part1(Path::new("./data/day2/test.txt")) == 8)
    }

    #[test]
    fn part2_passes() {
        assert!(part2(Path::new("./data/day2/test.txt")) == 2286)
    }
}

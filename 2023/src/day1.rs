use fancy_regex::Regex;
use std::{collections::HashMap, path::Path};

use crate::utils::input_file_to_string_vec;

pub fn part1(input: &Path) -> u32 {
    let lines = input_file_to_string_vec(input);

    let mut lines = lines.iter();

    let mut line_sum: Vec<String> = Vec::new();

    let re = Regex::new(r"(?=(\d))").unwrap();

    while let Some(line) = lines.next() {
        let result = re
            .captures_iter(line)
            .map(|m| m.unwrap().get(1).expect("Bad").as_str())
            .collect::<Vec<&str>>();

        let mut sum = String::from("");
        sum.push_str(result.first().unwrap());
        sum.push_str(result.last().unwrap());

        line_sum.push(sum)
    }

    return line_sum
        .into_iter()
        .fold(0, |acc, e| acc + e.parse::<u32>().unwrap());
}

pub fn part2(input: &Path) -> u32 {
    let lines = input_file_to_string_vec(input);

    let mut lines = lines.iter();

    let mut line_sum: Vec<String> = Vec::new();

    let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|\d))").unwrap();

    let text_to_digit: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let is_digit = Regex::new(r"^(\d)$").unwrap();

    while let Some(line) = lines.next() {
        let result = re
            .captures_iter(line)
            .map(|m| m.unwrap().get(1).expect("Bad").as_str())
            .map(|r| match is_digit.is_match(r) {
                Ok(res) => {
                    if res {
                        return r;
                    }

                    return text_to_digit.get(r).unwrap();
                }
                Err(_) => panic!("Error checking if digit"),
            })
            .collect::<Vec<&str>>();

        let mut sum = String::from("");
        sum.push_str(result.first().unwrap());
        sum.push_str(result.last().unwrap());

        line_sum.push(sum)
    }

    return line_sum
        .into_iter()
        .fold(0, |acc, e| acc + e.parse::<u32>().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_passes() {
        assert!(part1(Path::new("./data/day1/test.txt")) == 142)
    }

    #[test]
    fn part2_passes() {
        assert!(part2(Path::new("./data/day1/test.txt")) == 281)
    }
}

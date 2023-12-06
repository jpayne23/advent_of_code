use std::path::Path;

use fancy_regex::Regex;

use crate::utils::input_file_to_string_vec;

fn build_races(input: Vec<String>) -> Vec<(u32, u32)> {
    let mut lines = input.iter();
    let re = Regex::new(r"\d+").unwrap();
    re.captures_iter(lines.next().unwrap().as_str())
        .map(|m| -> u32 {
            m.unwrap()
                .get(0)
                .expect("bad")
                .as_str()
                .parse::<u32>()
                .unwrap()
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .zip(
            re.captures_iter(lines.next().unwrap().as_str())
                .map(|m| -> u32 {
                    m.unwrap()
                        .get(0)
                        .expect("bad")
                        .as_str()
                        .parse::<u32>()
                        .unwrap()
                })
                .collect::<Vec<u32>>(),
        )
        .collect::<Vec<(u32, u32)>>()
}

fn build_races_2(input: Vec<String>) -> Vec<(u64, u64)> {
    let mut lines = input.iter();
    let re = Regex::new(r"(?<=:)(\s*\d+)+").unwrap();
    re.captures_iter(lines.next().unwrap().as_str())
        .map(|m| -> u64 {
            m.unwrap()
                .get(0)
                .expect("bad")
                .as_str()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>()
        .into_iter()
        .zip(
            re.captures_iter(lines.next().unwrap().as_str())
                .map(|m| -> u64 {
                    m.unwrap()
                        .get(0)
                        .expect("bad")
                        .as_str()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap()
                })
                .collect::<Vec<u64>>(),
        )
        .collect::<Vec<(u64, u64)>>()
}

pub fn part1(input: &Path) -> u32 {
    let lines = input_file_to_string_vec(input);

    let races = build_races(lines);
    let races = races.iter();

    return races.fold(1, |acc, (time, distance)| {
        let mut win_counter = 0;
        for t in 1..*time {
            if t * (time - t) > *distance {
                win_counter += 1;
            }
        }

        println!("win_counter {}", win_counter);
        if win_counter > 0 {
            return acc * win_counter;
        }
        return acc;
    });
}

pub fn part2(input: &Path) -> u64 {
    let lines = input_file_to_string_vec(input);

    let races = build_races_2(lines);
    let races = races.iter();

    return races.fold(1, |acc, (time, distance)| {
        let mut win_counter = 0;
        for t in 1..*time {
            if t * (time - t) > *distance {
                win_counter += 1;
            }
        }

        println!("win_counter {}", win_counter);
        if win_counter > 0 {
            return acc * win_counter;
        }
        return acc;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_passes() {
        assert!(part1(Path::new("./data/day6/test.txt")) == 288)
    }

    #[test]
    fn part2_passes() {
        assert!(part2(Path::new("./data/day6/test.txt")) == 71503)
    }
}

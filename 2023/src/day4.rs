use std::{collections::HashSet, path::Path};

use fancy_regex::Regex;

use crate::utils::input_file_to_string_vec;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    played_numbers: HashSet<u32>,
}

impl Card {
    pub fn get_score(&self) -> i32 {
        let intersection = self.winning_numbers.intersection(&self.played_numbers);

        let wins = intersection.count() as i32;

        if wins - 1 >= 0 {
            return (2 as i32).pow((wins - 1) as u32);
        }

        return 0;
    }

    pub fn get_score_2(&self) -> i32 {
        let intersection = self.winning_numbers.intersection(&self.played_numbers);

        let wins = intersection.count() as i32;

        return wins;
    }
}

pub fn part1(input: &Path) -> u32 {
    let lines = input_file_to_string_vec(input);

    let mut lines = lines.iter();

    let mut total: u32 = 0;

    let re = Regex::new(r"\d+").unwrap();

    while let Some(line) = lines.next() {
        let x: Vec<&str> = line
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split("|")
            .collect();

        let winning_numbers: Vec<u32> = re
            .captures_iter(x.get(0).unwrap())
            .map(|m| {
                m.unwrap()
                    .get(0)
                    .expect("bad")
                    .as_str()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect();

        let played_numbers: Vec<u32> = re
            .captures_iter(x.get(1).unwrap())
            .map(|m| {
                m.unwrap()
                    .get(0)
                    .expect("bad")
                    .as_str()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect();

        let card = Card {
            winning_numbers: HashSet::from_iter(winning_numbers.iter().cloned()),
            played_numbers: HashSet::from_iter(played_numbers.iter().cloned()),
        };

        total += card.get_score() as u32;
    }

    return total;
}

pub fn part2(input: &Path) -> u32 {
    let lines = input_file_to_string_vec(input);

    let mut lines = lines.iter();

    let re = Regex::new(r"\d+").unwrap();

    let mut original_cards = vec![];
    let mut cards_to_play: Vec<u32> = vec![];

    while let Some(line) = lines.next() {
        let x: Vec<&str> = line.split(":").collect::<Vec<&str>>();

        let game_id = re
            .captures(x.get(0).unwrap())
            .unwrap()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        let numbers: Vec<&str> = x.get(1).unwrap().split("|").collect();
        let winning_numbers: Vec<u32> = re
            .captures_iter(numbers.get(0).unwrap())
            .map(|m| {
                m.unwrap()
                    .get(0)
                    .expect("bad")
                    .as_str()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect();

        let played_numbers: Vec<u32> = re
            .captures_iter(numbers.get(1).unwrap())
            .map(|m| {
                m.unwrap()
                    .get(0)
                    .expect("bad")
                    .as_str()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect();

        let card = Card {
            winning_numbers: HashSet::from_iter(winning_numbers.iter().cloned()),
            played_numbers: HashSet::from_iter(played_numbers.iter().cloned()),
        };

        original_cards.push(card);
        cards_to_play.push(game_id);
    }

    let mut i = 0;

    while i < cards_to_play.len() {
        let current_card_id = *cards_to_play.get(i).unwrap() as usize;

        let score = original_cards
            .get(current_card_id - 1)
            .unwrap()
            .get_score_2();

        if score > 0 {
            for s in 1..(score + 1) {
                match original_cards.get(((current_card_id - 1) + s as usize) as usize) {
                    Some(_) => cards_to_play.push((current_card_id + s as usize) as u32),
                    None => continue,
                }
            }
        }

        i += 1;
    }

    return cards_to_play.len() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_passes() {
        assert!(part1(Path::new("./data/day4/test.txt")) == 13)
    }

    #[test]
    fn part2_passes() {
        assert!(part2(Path::new("./data/day4/test.txt")) == 30)
    }
}

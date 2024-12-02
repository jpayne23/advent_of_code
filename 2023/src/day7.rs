use std::{collections::HashMap, path::Path};

use fancy_regex::Regex;

use crate::utils::input_file_to_string_vec;

#[derive(Clone, Debug)]
struct Hand {
    cards: String,
    bid: u32,
}

impl Hand {
    pub fn get_rank(&self) -> usize {
        // 5 of a kind
        let mut re = Regex::new(r"([\d,T,J,Q,K,A])\1{4}").unwrap();

        if re.is_match(&self.cards).unwrap() {
            return 6;
        }

        // 4 of a kind
        re = Regex::new(r"([\d,T,J,Q,K,A])\1{3}").unwrap();
        if re.is_match(&self.cards).unwrap() {
            return 5;
        }

        // Full house
        re = Regex::new(r"(([\d,T,J,Q,K,A])\2{2}([\d,T,J,Q,K,A])\3{1}|([\d,T,J,Q,K,A])\4{1}([\d,T,J,Q,K,A])\5{2})").unwrap();
        if re.is_match(&self.cards).unwrap() {
            return 4;
        }

        // 3 of a kind
        re = Regex::new(r"([\d,T,J,Q,K,A])\1{2}").unwrap();
        if re.is_match(&self.cards).unwrap() {
            return 3;
        }

        // 2 pair
        re = Regex::new(r"([\d,T,J,Q,K,A])\1{1}\d?([\d,T,J,Q,K,A])\2{1}").unwrap();
        if re.is_match(&self.cards).unwrap() {
            return 2;
        }

        // 1 pair ([\d,T,J,Q,K,A])\1{1}
        re = Regex::new(r"([\d,T,J,Q,K,A])\1{1}").unwrap();
        if re.is_match(&self.cards).unwrap() {
            return 1;
        }

        // 1 of a kind - default
        return 0;
    }
}

pub fn part1(input: &Path) -> u8 {
    let lines = input_file_to_string_vec(input);
    let mut lines = lines.iter();

    let card_rank = HashMap::from([
        ("A", 12),
        ("K", 11),
        ("Q", 10),
        ("J", 9),
        ("T", 8),
        ("9", 7),
        ("8", 6),
        ("7", 5),
        ("6", 4),
        ("5", 3),
        ("4", 2),
        ("3", 1),
        ("2", 0),
    ]);
    let mut ranking: Vec<Vec<Hand>> = vec![vec![]; 7];

    while let Some(line) = lines.next() {
        // Get hand and bid
        let re_parse_hand = Regex::new(r"([\d,T,J,Q,K,A]+) (\d+)").unwrap();

        let captures = re_parse_hand.captures(line).unwrap().unwrap();

        let hand = Hand {
            cards: captures.get(1).unwrap().as_str().to_string(),
            bid: captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        };

        ranking[hand.get_rank()].push(hand);
    }

    

    println!("ranking {:?}", ranking);
    return 0;
}

pub fn part2(input: &Path) -> u8 {
    let lines = input_file_to_string_vec(input);

    println!("Day 7, part 2 executing with lines {:?}", lines);
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_passes() {
        assert!(part1(Path::new("./data/day7/test.txt")) == 1)
    }

    #[test]
    fn part2_passes() {
        assert!(part2(Path::new("./data/day7/test.txt")) == 1)
    }
}

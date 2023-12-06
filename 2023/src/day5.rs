use std::{ops::Range, path::Path};

use fancy_regex::Regex;

use crate::utils::input_file_to_string;

fn map_function(input: i64, destination_start: i64, source_start: i64, range_length: i64) -> i64 {
    let modifier: i64 = destination_start as i64 - source_start as i64;
    let range_start = source_start.clone();
    let range_end = source_start + range_length;

    if (range_start..=range_end).contains(&input) {
        println!("input = {}, modifier = {}", input, modifier);
        return (input as i64 + modifier) as i64;
    }

    return input;
}

pub fn part1(input: &Path) -> i64 {
    let contents = input_file_to_string(input);

    // get seeds
    let re_seeds = Regex::new(r"(?<=seeds:\s)(\d+\s?)*\d+").unwrap();
    let seeds = re_seeds
        .captures(contents.as_str())
        .unwrap()
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();

    println!("seeds = {:?}", seeds);

    // map functions
    let re_map_functions = Regex::new(r"(?<=map:\n)(\d+\s?)*\d+").unwrap();
    let maps = re_map_functions
        .captures_iter(contents.as_str())
        .map(|m| m.unwrap().get(0).expect("bad").as_str())
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|l| {
            l.lines()
                .map(|a| a.split_ascii_whitespace().collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        })
        .collect::<Vec<Vec<Vec<&str>>>>();

    let mut running;

    let mut result = vec![];

    for s in seeds.iter() {
        running = s.parse::<i64>().unwrap();
        for map in maps.iter() {
            let mut temp;
            for function in map.iter() {
                temp = map_function(
                    running,
                    function.get(0).unwrap().parse::<i64>().unwrap(),
                    function.get(1).unwrap().parse::<i64>().unwrap(),
                    function.get(2).unwrap().parse::<i64>().unwrap(),
                );
                if temp != running {
                    running = temp;
                    break;
                }
            }
        }
        result.push(running);
    }

    result.sort();
    result.reverse();

    return result.pop().unwrap();
}

use lazy_regex::regex;
use rangemap::RangeMap;

pub trait Puzzle {
    fn day(&self) -> u8;
    fn solve_part_1(&self) -> String;
    fn solve_part_2(&self) -> String;
}

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn day(&self) -> u8 {
        5
    }

    fn solve_part_1(&self) -> String {
        let mut seeds = Vec::new();
        let seed_line = self.input.lines().next().unwrap();
        for m in regex!(r"\d+").find_iter(seed_line) {
            let seed = m.as_str().parse::<i64>().unwrap();
            seeds.push(seed..seed + 1);
        }
        for map in self.parse_maps() {
            seeds = map_range(&mut seeds, &map);
        }
        seeds
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut seeds = Vec::new();
        let seed_line = self.input.lines().next().unwrap();
        for cap in regex!(r"(\d+) (\d+)").captures_iter(seed_line) {
            let start = cap[1].parse::<i64>().unwrap();
            let length = cap[2].parse::<i64>().unwrap();
            seeds.push(start..start + length);
        }
        for map in self.parse_maps() {
            seeds = map_range(&mut seeds, &map);
        }
        seeds
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap()
            .to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_maps(&self) -> Vec<RangeMap<i64, i64>> {
        let mut maps = Vec::new();
        for block in self.input.split("\n\n").skip(1) {
            let mut map = RangeMap::new();
            for line in block.lines().skip(1) {
                let mut parts = line.split_whitespace();
                let dst = parts.next().unwrap().parse::<i64>().unwrap();
                let src = parts.next().unwrap().parse::<i64>().unwrap();
                let length = parts.next().unwrap().parse::<i64>().unwrap();
                map.insert(src..src + length, dst - src);
            }
            maps.push(map);
        }
        maps
    }
}

fn map_range(inputs: &mut Vec<Range<i64>>, map: &RangeMap<i64, i64>) -> Vec<Range<i64>> {
    let mut output = Vec::new();
    while let Some(input) = inputs.pop() {
        if map.overlaps(&input) {
            for (range, offset) in map.overlapping(&input) {
                let start = std::cmp::max(input.start, range.start);
                let end = std::cmp::min(input.end, range.end);
                output.push(start + offset..end + offset);
                if input.start < start {
                    inputs.push(input.start..start);
                }
                if end < input.end {
                    inputs.push(end..input.end);
                }
            }
        } else {
            output.push(input);
        }
    }
    output
}

pub fn part2(input: &Path) {
    let contents = input_file_to_string(input);
    let puzzle = Day::create(&contents);
    println!("Result {:?}", puzzle.solve_part_2())
}

// pub fn part2(input: &Path) -> i64 {
//     let contents = input_file_to_string(input);

//     // get seeds
//     let re_seeds = Regex::new(r"(?<=seeds:\s)(\d+\s?)*\d+").unwrap();
//     let seeds = re_seeds
//         .captures(contents.as_str())
//         .unwrap()
//         .unwrap()
//         .get(0)
//         .unwrap()
//         .as_str()
//         .split_ascii_whitespace()
//         .collect::<Vec<&str>>();

//     // map functions
//     let re_map_functions = Regex::new(r"(?<=map:\n)(\d+\s?)*\d+").unwrap();
//     let maps = re_map_functions
//         .captures_iter(contents.as_str())
//         .map(|m| m.unwrap().get(0).expect("bad").as_str())
//         .collect::<Vec<&str>>()
//         .into_iter()
//         .map(|l| {
//             l.lines()
//                 .map(|a| a.split_ascii_whitespace().collect::<Vec<&str>>())
//                 .collect::<Vec<Vec<&str>>>()
//         })
//         .collect::<Vec<Vec<Vec<&str>>>>();

//     let mut data = vec![];
//     // find range that works to get lowest path through functions
//     for (i, map) in maps.iter().rev().enumerate() {
//         let first = map.first().unwrap();
//         // (destination, source, range)
//         let mut first_path: (i64, i64, i64) = (
//             first.get(0 as usize).unwrap().parse::<i64>().unwrap(),
//             first.get(1 as usize).unwrap().parse::<i64>().unwrap(),
//             first.get(2 as usize).unwrap().parse::<i64>().unwrap(),
//         );

//         println!("data at {} = {:?}", i, data);
//         let target_range = match data.last() {
//             Some(a) => a,
//             None => &(0..0),
//         };

//         let mut new_range = 0..0;
//         let mut is_update_range: bool = false;

//         for function in map.iter() {
//             let destination_start = function.get(0).unwrap().parse::<i64>().unwrap();
//             let source_start = function.get(1).unwrap().parse::<i64>().unwrap();
//             let function_range = function.get(2).unwrap().parse::<i64>().unwrap();
//             let destination_end = destination_start + function_range;
//             let source_end = source_start + function_range;
//             let modifier = destination_start - source_start;

//             println!(
//                 "destination_start = {}, destination_end = {}, source_start = {}, source_end = {}, modifier = {}",
//                 destination_start, destination_end, source_start, source_end, modifier
//             );

//             if i == 0 {
//                 if destination_start < first_path.0 {
//                     println!("First update");

//                     new_range = source_start..source_end;
//                     first_path = (destination_start, source_start, function_range);
//                     is_update_range = true;
//                 }

//                 // if destination_start < (new_range.start + modifier) {
//                 //     println!("Another update");
//                 //     new_range = source_start..source_end;
//                 //     is_update_range = true;
//                 // }
//             } else {
//                 // when we have some overlap of the range and a destination
//                 // we need to then ensure we have the source range
//                 if target_range.contains(&destination_start)
//                     && target_range.contains(&destination_end)
//                 {
//                     println!("Fully in target_range = {:?}, destination_start = {:?}, destination_end = {:?}", target_range, destination_start, destination_end);
//                     new_range = destination_start..destination_end;
//                     is_update_range = true;
//                 }

//                 if target_range.contains(&destination_start) {
//                     println!("destination_start in target_range = {:?}, destination_start = {:?}, destination_end = {:?}", target_range, destination_start, destination_end);
//                     new_range = destination_start..target_range.end;
//                     is_update_range = true;
//                 }

//                 if target_range.contains(&destination_end) {
//                     println!("destination_end in target_range = {:?}, destination_start = {:?}, destination_end = {:?}", target_range, destination_start, destination_end);
//                     new_range = target_range.start..(destination_end - target_range.start);
//                     is_update_range = true;
//                 }

//                 if target_range.start > (destination_start) && target_range.end < destination_end {
//                     println!("destination range consumes target range in target_range = {:?}, destination_start = {:?}, destination_end = {:?}", target_range, destination_start, destination_end);
//                     new_range = (target_range.start - modifier)..(target_range.end - modifier);
//                     is_update_range = true;
//                 }
//             }
//         }

//         if is_update_range {
//             data.push(new_range);
//         }
//     }

//     println!("data {:?}", data);

//     let seed_ranges: Vec<std::ops::Range<i64>> = seeds
//         .chunks(2)
//         .map(|s| -> std::ops::Range<i64> {
//             s.get(0).unwrap().parse::<i64>().unwrap()
//                 ..(s.get(0).unwrap().parse::<i64>().unwrap()
//                     + s.get(1).unwrap().parse::<i64>().unwrap())
//         })
//         .collect();

//     println!("seed_ranges {:?}", seed_ranges);

//     let start_range = data.last().unwrap();
//     for i in start_range.start..start_range.end {
//         for r in seed_ranges.iter() {
//             if r.contains(&i) {
//                 return i;
//             }
//         }
//     }

//     return 0;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_passes() {
        assert!(part1(Path::new("./data/day5/test.txt")) == 35)
    }

    // #[test]
    // fn part2_passes() {
    //     assert!(part2(Path::new("./data/day5/test.txt")) == 1)
    // }
}

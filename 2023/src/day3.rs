use core::panic;
use std::path::Path;

use fancy_regex::Regex;

use crate::utils::input_file_to_string_vec;

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(&mut self, input: Vec<Vec<char>>) {
        self.rows = input;
    }

    pub fn get_by_row_column(&self, row: i32, column: i32) -> Option<&char> {
        return match self.rows.get(row as usize) {
            Some(r) => r.get(column as usize),
            None => None,
        };
    }

    pub fn get_all_surrounding_symbol_x_y(&self, row: u32, column: u32) -> Vec<(u32, u32)> {
        let mut result: Vec<(u32, u32)> = vec![];
        let re_is_not_symbol = Regex::new(r"[\d\.\n]").unwrap();

        let surrounding_cells: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        surrounding_cells
            .iter()
            .for_each(|(row_modifier, column_modifier)| {
                let search_row: i32 = row as i32 + row_modifier;
                let search_column: i32 = column as i32 + column_modifier;
                let current = match self.get_by_row_column(search_row, search_column) {
                    Some(c) => c.to_string(),
                    None => String::new(),
                };

                if !current.is_empty()
                    && match re_is_not_symbol.is_match(current.as_str()) {
                        Ok(r) => !r,
                        Err(_) => false,
                    }
                {
                    println!(
                        "Found surrounding symbol {} at row {}, column {}",
                        current, search_row, search_column
                    );
                    result.push((search_row as u32, search_column as u32))
                }
            });

        return result;
    }

    pub fn get_all_surrounding_digits_x_y(&self, row: u32, column: u32) -> Vec<(u32, u32)> {
        let mut result: Vec<(u32, u32)> = vec![];
        let re_is_digit = Regex::new(r"\d").unwrap();

        let surrounding_cells: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        surrounding_cells
            .iter()
            .for_each(|(row_modifier, column_modifier)| {
                let search_row: i32 = row as i32 + row_modifier;
                let search_column: i32 = column as i32 + column_modifier;
                let current = match self.get_by_row_column(search_row, search_column) {
                    Some(c) => c.to_string(),
                    None => String::new(),
                };

                if !current.is_empty()
                    && match re_is_digit.is_match(current.as_str()) {
                        Ok(r) => r,
                        Err(_) => false,
                    }
                {
                    // println!(
                    //     "Found surrounding digit {} at row {}, column {}",
                    //     current, search_row, search_column
                    // );
                    result.push((search_row as u32, search_column as u32))
                }
            });

        return result;
    }

    pub fn get_full_number(&self, row: u32, column: u32) -> u32 {
        let re_full_digit = Regex::new(r"\d+").unwrap();

        let row_line: String = match self.rows.get(row as usize) {
            Some(l) => l.iter().collect(),
            None => String::new(),
        };

        let mut matches = re_full_digit.captures_iter(row_line.as_str());

        while let Some(m) = matches.next() {
            let info = match m {
                Ok(res) => GridNumber {
                    value: res.get(0).unwrap().as_str().parse::<u32>().unwrap(),
                    start: res.get(0).unwrap().start() as u32,
                    end: res.get(0).unwrap().end() as u32,
                },
                Err(_) => panic!("Something went wrong"),
            };

            if column >= info.start && column <= info.end {
                return info.value;
            }
        }

        return 1;
    }
}

#[derive(Debug)]
struct GridNumber {
    value: u32,
    start: u32,
    end: u32,
}

pub fn part1(input: &Path) -> u32 {
    let lines = input_file_to_string_vec(input);
    let lines = lines.iter();

    let mut total: u32 = 0;

    let mut grid = Grid { rows: vec![] };

    grid.new(
        lines
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect(),
    );

    let mut grid_iter = grid.rows.clone().into_iter().enumerate();

    let re_is_number = Regex::new(r"\d+").unwrap();

    while let Some((row_number, row)) = grid_iter.next() {
        let row_line: String = row.iter().collect();

        let mut matches = re_is_number.captures_iter(&row_line);

        while let Some(m) = matches.next() {
            let info = match m {
                Ok(res) => GridNumber {
                    value: res.get(0).unwrap().as_str().parse::<u32>().unwrap(),
                    start: res.get(0).unwrap().start() as u32,
                    end: res.get(0).unwrap().end() as u32,
                },
                Err(_) => panic!("Something went wrong"),
            };

            let mut surrounding_symbols = false;

            for n in info.start..info.end {
                let temp = grid.get_all_surrounding_symbol_x_y(row_number as u32, n);
                if temp.len() > 0 {
                    surrounding_symbols = true;
                }
            }

            if surrounding_symbols {
                println!("Adding number {} to total", info.value);
                total += info.value;
            }
        }
    }
    println!("Total {}", total);
    return total;
}

#[derive(Debug)]
struct GridSymbol {
    start: u32,
    end: u32,
}

// 86853991
pub fn part2(input: &Path) -> u32 {
    let lines = input_file_to_string_vec(input);
    let lines = lines.iter();

    let mut total: u32 = 0;

    let mut grid = Grid { rows: vec![] };

    grid.new(
        lines
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect(),
    );

    let mut grid_iter = grid.rows.clone().into_iter().enumerate();

    let re_is_number = Regex::new(r"\*").unwrap();

    while let Some((row_number, row)) = grid_iter.next() {
        println!("Row = {}", row_number);
        let row_line: String = row.iter().collect();

        let mut matches = re_is_number.captures_iter(&row_line);

        while let Some(m) = matches.next() {
            let info = match m {
                Ok(res) => GridSymbol {
                    start: res.get(0).unwrap().start() as u32,
                    end: res.get(0).unwrap().end() as u32,
                },
                Err(_) => panic!("Something went wrong"),
            };

            // println!("Symbol found at {:?}", info);

            for n in info.start..info.end {
                let temp = grid.get_all_surrounding_digits_x_y(row_number as u32, n);
                // if temp.len() == 2 {
                //     let mut local_total: u32 = 1;
                //     let mut full_numbers: Vec<u32> = vec![];
                //     for (x, y) in temp {
                //         let full_number = grid.get_full_number(x, y);
                //         if !full_numbers.contains(&full_number) {
                //             println!("Full number = {}", full_number);
                //             local_total = local_total * full_number;
                //             full_numbers.push(full_number);
                //         }
                //     }

                //     // println!("Local total {}", local_total);

                //     total += local_total;
                // }
                // let mut local_total: u32 = 1;
                let mut full_numbers: Vec<u32> = vec![];
                for (x, y) in temp {
                    let full_number = grid.get_full_number(x, y);
                    if !full_numbers.contains(&full_number) {
                        full_numbers.push(full_number);
                    }
                }

                if full_numbers.len() == 2 {
                    total += full_numbers.iter().fold(1, |acc, e| acc * e);
                }
            }
        }
    }
    println!("Total {}", total);
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_passes() {
        assert!(part1(Path::new("./data/day3/test.txt")) == 4361)
    }

    #[test]
    fn part2_passes() {
        assert!(part2(Path::new("./data/day3/test.txt")) == 467835)
    }
}

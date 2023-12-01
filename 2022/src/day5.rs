use std::fs;
use std::path::Path;

// [C]         [S] [H]
// [F] [B]     [C] [S]     [W]
// [B] [W]     [W] [M] [S] [B]
// [L] [H] [G] [L] [P] [F] [Q]
// [D] [P] [J] [F] [T] [G] [M] [T]
// [P] [G] [B] [N] [L] [W] [P] [W] [R]
// [Z] [V] [W] [J] [J] [C] [T] [S] [C]
// [S] [N] [F] [G] [W] [B] [H] [F] [N]
//  1   2   3   4   5   6   7   8   9

// [
//     ['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
//     ['N', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
//     ['F', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
//     ['G', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
//     ['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
//     ['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
//     ['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
//     ['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
//     ['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
//     ['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
// ]

// Parse "move 1 from 2 to 1" to amount, origin, destination
#[derive(Debug)]
struct Command {
    amount: u32,
    origin: u32,
    destination: u32,
}

struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn update_start_state(&mut self, input: Vec<Vec<char>>) {
        for (i, value) in input.iter().enumerate() {
            if self.stacks.len() <= i {
                self.stacks.insert(i, Vec::new());
            }
            self.stacks[i].append(&mut value.to_owned());
        }
    }

    fn finalise_start_state(&mut self) {
        for i in 0..self.stacks.len() {
            self.stacks[i].reverse()
        }
    }

    fn process_command(&mut self, command: Command) {
        for _i in 0..command.amount {
            self.stacks
                .get(command.origin as usize)
                .expect("Stack does not exist");
        }
    }
}

fn parse_start_state_line(line: &String) -> Vec<Vec<char>> {
    let chars = line.chars().collect::<Vec<char>>();
    let chunks = chars.chunks(4);

    let mut result: Vec<Vec<char>> = Vec::new();

    for chunk in chunks {
        if chunk[1].is_alphabetic() {
            result.push(vec![chunk[1]]);
            continue;
        }

        result.push(vec![]);
    }

    result
}

fn parse_command_line(line: &String) -> Command {
    let values: Vec<u32> = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).expect("That wasn't a number"))
        .collect();

    let result = Command {
        amount: values[0],
        origin: values[1],
        destination: values[2],
    };

    result
}

fn input_file_to_iter(input_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(Path::new(input_path)).expect("Couldn't read file");
    return contents.split("\n").map(|el| el.to_string()).collect();
}

pub fn part1(input_path: &str) {
    let lines = input_file_to_iter(input_path);

    let mut state = State { stacks: Vec::new() };

    let mut is_parsing_start_state = true;

    for line in lines {
        if line.is_empty() {
            state.finalise_start_state();
            is_parsing_start_state = false;
            continue;
        }

        if is_parsing_start_state {
            let parsed_line = parse_start_state_line(&line);
            state.update_start_state(parsed_line);
            continue;
        }

        parse_command_line(&line);
    }

    println!("End state {:?}", state.stacks);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn returns_sum_of_top_3_totals() {
//         assert!(part1("data/day5/test.txt") == 45000)
//     }
// }

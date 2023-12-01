use std::fs;
use std::path::Path;

fn input_file_to_iter(input_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(Path::new(input_path)).expect("Couldn't read file");
    return contents.split("\n").map(|el| el.to_string()).collect();
}

struct Command {
    action: String,
    value: i32,
}

#[derive(Debug)]
struct Signal {
    readings: Vec<i32>,
}

impl Signal {
    fn get_signal_strength(&self, position: i32) -> i32 {
        let result: i32 = match self.readings.get((position - 1) as usize) {
            Some(num) => {
                println!("position - 1, {:?}", position - 1);
                println!("num {:?}", num);
                return num * position;
            }
            None => 0,
        };

        result
    }
}

fn parse_line(line: &String) -> Command {
    let split_input: Vec<&str> = line.split(" ").collect();

    let value = match split_input.get(1) {
        Some(x) => x.parse::<i32>().expect("Not a number"),
        None => 0,
    };

    return Command {
        action: split_input[0].to_string(),
        value: value,
    };
}

pub fn part1(input_path: &str, signal_reading_positions: Vec<i32>) {
    let lines = input_file_to_iter(input_path);

    let mut lines_iter = lines.iter();

    let mut signal = Signal {
        readings: Vec::new(),
    };

    while let Some(line) = lines_iter.next() {
        let command = parse_line(line);

        let latest_value = match signal.readings.last() {
            Some(num) => num.to_owned(),
            None => 1,
        };

        if command.action.as_str() == "noop" {
            signal.readings.push(latest_value);
            continue;
        }

        // addx
        signal.readings.push(latest_value);
        signal.readings.push(latest_value + command.value);
    }

    let mut total: i32 = 0;

    for position in signal_reading_positions {
        let signal_strength = signal.get_signal_strength(position);
        println!("signal_strength {:?}", signal_strength);
        total += signal_strength;
    }

    println!("total {:?}", total);
}

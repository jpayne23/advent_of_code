use std::fs;
use std::num::IntErrorKind;
use std::path::Path;

pub fn part2(input_path: &str) -> i32 {
    let contents = fs::read_to_string(Path::new(input_path)).expect("Couldn't read file");

    let lines = contents.split("\n");
    let mut current_elf_total = 0;
    let mut current_line: i32;
    let mut totals: Vec<i32> = Vec::new();

    for line in lines {
        current_line = match line.trim().parse() {
            Ok(num) => num,
            Err(err) => match err.kind() {
                IntErrorKind::Empty => {
                    println!("Moving onto new elf, current elf held {current_elf_total}");

                    totals.push(current_elf_total);
                    current_elf_total = 0;
                    continue;
                }
                _ => {
                    println!("What happened??? {err}");
                    break;
                }
            },
        };

        current_elf_total += current_line;
    }

    println!("Reached end of input, last elf held {current_elf_total}");

    totals.push(current_elf_total);

    totals.sort();

    let winners: Vec<&i32> = totals.iter().rev().take(3).rev().collect();

    println!("Top 3 values: {:?}", winners);

    let total: i32 = winners.into_iter().sum();

    println!("Total: {total}");
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_sum_of_top_3_totals() {
        assert!(part2("data/day1/test.txt") == 45000)
    }
}

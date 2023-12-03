use std::fs::File;
use std::io::prelude::*;
use std::{fs, path::Path};

pub fn input_file_to_string_vec(input_path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(input_path).expect("Couldn't read file");
    return contents.lines().map(|el| el.to_string()).collect();
}

pub fn new_aoc_day(day: &u8) -> std::io::Result<()> {
    let mut file = File::create(format!("./src/day{day}.rs"))?;

    let test_input_path = format!("./data/day{}/test.txt", day);

    file.write_all(
        format!(
            "use std::path::Path;

use crate::utils::input_file_to_string_vec;

pub fn part1(input: &Path) -> u8 {{
    let lines = input_file_to_string_vec(input);

    println!(\"Day {day}, part 1 executing with lines {{:?}}\", lines);
    return 0;
}}

pub fn part2(input: &Path) -> u8 {{
    let lines = input_file_to_string_vec(input);

    println!(\"Day {day}, part 2 executing with lines {{:?}}\", lines);
    return 0;
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn part1_passes() {{
        assert!(part1(Path::new(\"{test_input_path}\")) == 1)
    }}

    #[test]
    fn part2_passes() {{
        assert!(part2(Path::new(\"{test_input_path}\")) == 1)
    }}
}}
    
    "
        )
        .as_bytes(),
    )?;
    Ok(())
}

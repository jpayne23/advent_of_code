mod day1;
mod day10;
mod day5;

fn main() {
    day1::part2("data/day1/input.txt");

    day5::part1("data/day5/test.txt");

    day10::part1("data/day10/test.txt", [20, 60, 100, 140, 180, 220].to_vec());
}

mod days;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Day 1 Part 1: {}", days::one::part1(read_input("1", "1")));
    println!("Day 1 Part 2: {}", days::one::part2(read_input("1", "1")));
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_input(day: &str, part: &str) -> io::Lines<io::BufReader<File>> {
    let file =
        File::open(format!("input/day{}/part_{}.txt", day, part)).expect("Error opening file");
    io::BufReader::new(file).lines()
}

mod days;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    //Day 1 Part 1: 805731
    //Day 1 Part 2: 192684960
    println!("Day 1 Part 1: {}", days::one::part1(read_input("1")));
    println!("Day 1 Part 2: {}", days::one::part2(read_input("1")));
    // Day 2 Part 1: 666
    // Day 2 Part 2: 670
    println!("Day 2 Part 1: {}", days::two::part1(read_input("2")));
    println!("Day 2 Part 2: {}", days::two::part2(read_input("2")));
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_input(day: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(format!("input/day{}.txt", day)).expect("Error opening file");
    io::BufReader::new(file).lines()
}

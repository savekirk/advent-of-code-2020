mod days;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // println!("Day 1 Part 1: {}", days::one::part1(read_input("1")));
    // println!("Day 1 Part 2: {}", days::one::part2(read_input("1")));
    // println!("Day 2 Part 1: {}", days::two::part1(read_input("2")));
    // println!("Day 2 Part 2: {}", days::two::part2(read_input("2")));
    // println!("Day 3 Part 1 test: {}", days::three::part1(read_input("3test")));
    // println!("Day 3 Part 1: {}", days::three::part1(read_input("3")));
    // println!("Day 3 Part 2 test: {}", days::three::part2(read_input("3test")));
    println!("Day 4 Part 1: {}", days::four::part1(read_input("4")));
    println!("Day 4 Part 1 test: {}", days::four::part1(read_input("4test")));
    println!("Day 4 Part 2 test: {}", days::four::part2(read_input("4test")));
    println!("Day 4 Part 2 : {}", days::four::part2(read_input("4")));
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_input(day: &str) -> Vec<String> {
    let file = File::open(format!("input/day{}.txt", day)).expect("Error opening file");
    io::BufReader::new(file).lines().map(|l| l.unwrap()).collect::<Vec<String>>()
}

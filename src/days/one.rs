//   --- Day 1: Report Repair ---

// After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

// The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

// To save your vacation, you need to get all fifty stars by December 25th.

// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

// Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

/// let input = vec![1721, 979, 366, 299, 675, 1456];
/// let result = part1()
pub fn part1(lines: Vec<String>) -> i32 {
    // Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

    // For example, suppose your expense report contained the following:

    // 1721
    // 979
    // 366
    // 299
    // 675
    // 1456

    // In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

    // Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?

    let mut smaller = Vec::new();
    let mut larger = Vec::new();
    let half = 1010;
    let mut half_count = 0;
    let mut result = 0;
    for line in lines {
        let num: i32 = line.parse().expect("Not a number");

        match half % num {
            0 => half_count = half_count + 1,
            1010 => larger.push(num),
            _ => smaller.push(num),
        }
    }

    // short circuit if we have double 1010
    if half_count == 2 {
        result = half * half;
    } else {
        for ln in &larger {
            if let Some(sn) = &smaller.iter().find(|n| *n + ln == 2020) {
                result = *sn * ln;
                break;
            }
        }
    }

    result
}

pub fn part2(lines: Vec<String>) -> i32 {
    //   --- Part Two ---

    // The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

    // Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

    // In your expense report, what is the product of the three entries that sum to 2020?
    let mut smaller = Vec::new();
    let mut larger = Vec::new();
    let mid = 674;
    let mut result = 0;
    for line in lines {
        let num: i32 = line.parse().expect("Not a number");

        if num >= mid {
            larger.push(num);
        } else {
            smaller.push(num);
        }
    }

    for ln in &larger {
        for sn1 in &smaller {
            if let Some(sn2) = &smaller.iter().find(|n| *n + sn1 + ln == 2020) {
                result = *sn2 * sn1 * ln;
                break;
            }
        }
    }

    result
}

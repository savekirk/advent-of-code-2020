use std::collections::HashMap;

pub fn part1(lines: Vec<String>) -> usize {
    answers(lines).iter().fold(0,  |ta, ga| ta + ga.0.len())
}

pub fn part2(lines: Vec<String>) -> usize {
    let mut total_answers = 0;
    for answer in answers(lines).iter() {
        let group_size = answer.1;
        total_answers += answer.0.iter().fold(0, |c, (_, ac)| {
            if group_size == *ac {
                c + 1
            } else {
                c
            }
        });
    }

    total_answers
}

fn answers(lines: Vec<String>) -> Vec<(HashMap<char, u8>, u8)> {
    let mut group_answers = Vec::new();
    let mut current_group_answer: HashMap<char, u8> = HashMap::new();
    let mut group_count = 0;
    for line in lines {
        if !line.is_empty() {
            group_count += 1;
            for c in line.chars() {
                let count = current_group_answer.entry(c).or_insert(0);
                *count += 1;
            }
        } else { 
            group_answers.push((current_group_answer, group_count));
            current_group_answer = HashMap::new();
            group_count = 0;
        }
    }

    group_answers.push((current_group_answer, group_count));
    group_answers
}
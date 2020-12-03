pub fn part1(lines: Vec<String>) -> usize {
    count_trees(lines.clone(), 3, 1)
}

pub fn part2(lines: Vec<String>) -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|slopes| count_trees(lines.clone(), slopes.0, slopes.1))
        .product()
}

fn count_trees(lines: Vec<String>, right: i32, down: i32) -> usize {
    let mut trees = 0;
    let mut position: i32 = right;
    let mut lines = lines.iter().step_by(down as usize);
    lines.next();
    for line in lines {
        let mut pattern = line.trim().to_string();
        let pattern_count = pattern.len() as i32;
        if pattern_count <= position {
            let mut num_of_repeats = (f64::from(position) / f64::from(pattern_count)).ceil();
            if pattern_count * num_of_repeats as i32 == position {
                num_of_repeats = num_of_repeats + 1.0;
            }
            pattern = pattern.repeat(num_of_repeats as usize);
        }

        if let Some('#') = pattern.chars().nth(position as usize) {
            trees = trees + 1;
        }

        position = position + right;
    }

    trees
}

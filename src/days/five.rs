pub fn part1(lines: Vec<String>) -> usize {
    *seat_ids(lines).iter().max().unwrap() as usize
}

pub fn part2(lines: Vec<String>) -> usize {
    let seat_ids = seat_ids(lines);
    let back = *seat_ids.iter().max().unwrap() - 1;
    let front = *seat_ids.iter().min().unwrap() + 1;
    let mut your_seat_id: usize = 0;
    for id in front..back {
        if !seat_ids.contains(&id) {
            if seat_ids.contains(&(id + 1)) && seat_ids.contains(&(id - 1)) {
                your_seat_id = id as usize;
                break;
            }
        }
    }

    your_seat_id
}

fn seat_ids(lines: Vec<String>) -> Vec<u32> {
    let mut seat_ids: Vec<u32> = Vec::new();
    for line in lines {
        let position = line.chars().fold(
            Range {
                row: (0, 127),
                col: (0, 7),
            },
            |r, c| new_range(c, r),
        );
        let si = position.row.0 as u32 * 8u32 + position.col.0 as u32;
        seat_ids.push(si);
    }
    seat_ids
}

fn new_range(char: char, Range { row, col }: Range) -> Range {
    match char {
        'F' => Range {
            row: (row.0, row.1 - mid(row)),
            col,
        },
        'B' => Range {
            row: (row.0 + mid(row), row.1),
            col,
        },
        'L' => Range {
            row,
            col: (col.0, col.1 - mid(col)),
        },
        'R' => Range {
            row,
            col: (col.0 + mid(col), col.1),
        },
        _ => Range { row, col },
    }
}

fn mid((l, u): (u8, u8)) -> u8 {
    ((u + 1) - l) / 2
}

struct Range {
    row: (u8, u8),
    col: (u8, u8),
}

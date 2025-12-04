use std::fs;

fn main() {
    println!("Part 1: {}", part_1("input.txt"));

    println!("Part 2: {}", part_2("input.txt"));
}

fn part_1(file: &str) -> i32 {
    let contents = fs::read_to_string(file)
        .expect("Could not open file.");
    let grid: Vec<Vec<char>> = contents.lines()
        .map(|line| 
            line.chars()
                .collect())
        .collect();
    let y_len = grid.len();
    let x_len = grid[0].len();
    let mut total = 0;
    
    for y in 0..y_len {
        for x in 0..x_len {
            // If this grid spot has a roll of paper
            if grid[y][x] == '@' {
                // Check adjacent locations
                let mut adjacent = 0;
                for y_offset in -1..=1 {
                    for x_offset in -1..=1 {
                        let test_row = grid.get((y as isize +y_offset) as usize);
                        match test_row {
                            Some(row) => {
                                let test_char = row.get((x as isize + x_offset) as usize);
                                match test_char {
                                    Some(ch) => {
                                        if *ch == '@' {
                                            adjacent += 1;
                                        }
                                    },
                                    None => {
                                        continue;
                                    },
                                }
                            },
                            None => {
                                continue;
                            },
                        }

                    }
                }
                if adjacent < 5 {
                    total += 1;
                }
                
            }
        }
    }

    return total;
}

fn part_2(file: &str) -> i32 {
    let contents = fs::read_to_string(file)
        .expect("Could not open file.");
    let grid: Vec<Vec<char>> = contents.lines()
        .map(|line| 
            line.chars()
                .collect())
        .collect();
    let (total, marked_grid) = recursive_removal(grid);
    return total;
}

fn recursive_removal(grid: Vec<Vec<char>>) -> (i32, Vec<Vec<char>>) {
    let mut marked_grid = grid.clone();
    let mut total = 0;
    let y_len = grid.len();
    let x_len = grid[0].len();

    for y in 0..y_len {
        for x in 0..x_len {
            // If this grid spot has a roll of paper
            if grid[y][x] == '@' {
                // Check adjacent locations
                let mut adjacent = 0;
                for y_offset in -1..=1 {
                    for x_offset in -1..=1 {
                        let test_row = grid.get((y as isize +y_offset) as usize);
                        match test_row {
                            Some(row) => {
                                let test_char = row.get((x as isize + x_offset) as usize);
                                match test_char {
                                    Some(ch) => {
                                        if *ch == '@' {
                                            adjacent += 1;
                                        }
                                    },
                                    None => {
                                        continue;
                                    },
                                }
                            },
                            None => {
                                continue;
                            },
                        }

                    }
                }
                if adjacent < 5 {
                    total += 1;
                    marked_grid[y][x] = 'x';
                }
                
            }
        }
    }
    
    if total == 0 {
        return (total, marked_grid);
    } else {
        let (step_total, step_grid) = recursive_removal(marked_grid);
        return (total+step_total, step_grid);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("test_input.txt"), 13);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("test_input.txt"), 43);
    }
}

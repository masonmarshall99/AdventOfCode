fn main() {
    println!("Part 1 Splits: {}", part_1("input.txt"));
    println!("Part 2 Timelines: {}", part_2("input.txt"));
}

fn part_1(file: &str) -> i32 {
    let contents = std::fs::read_to_string(file)
        .expect("Could not read from file.");

    let mut lasers: Vec<usize> = Vec::new();
    let mut splits = 0;

    for line in contents.lines() {
        let mut new_lasers: Vec<usize> = Vec::new(); 
        for i in 0..line.len() {
            if &line[i..i+1] == "^" && lasers.contains(&i) {
                if i-1 > 0 {
                    new_lasers.push(i-1);
                }
                if i+1 < line.len() {
                    new_lasers.push(i+1)
                }
                splits += 1;
            }
            else if &line[i..i+1] == "S" || lasers.contains(&i) {
                new_lasers.push(i);
            }
        }
        lasers = new_lasers;
    }

    return splits;
}

fn part_2(file: &str) -> i64 {
    let contents = std::fs::read_to_string(file)
        .expect("Could not read from file.");

    let grid: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect())
        .collect();
    let starting_indices: Vec<usize> = (0..grid[0].len())
        .filter(|&x| grid[0][x] == 'S')
        .collect();
    let mut pascall_grid: Vec<Vec<i64>> = vec![vec![0; grid[0].len()]; grid.len()];
    for i in starting_indices {
        pascall_grid[0][i] = 1;
    }

    for y in 1..grid.len() {
        for x in 0..grid[0].len() {
            pascall_grid[y][x] += pascall_grid[y-1][x];
            if grid[y][x] == '^' {
               pascall_grid[y][x] = 0;
               pascall_grid[y][x-1] += pascall_grid[y-1][x];
               pascall_grid[y][x+1] += pascall_grid[y-1][x];
            }
        }
    }

    return pascall_grid.last().unwrap().into_iter().sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("test_input.txt"), 21);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("test_input.txt"), 40);
    }
}

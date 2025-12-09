use std::fs;

fn main() {
    println!("Part 1 Total: {}", part_1("input.txt"));

    println!("Part 2 Total: {}", part_2("input.txt"));
}

fn part_1(file: &str) -> i64 {
    let contents = fs::read_to_string(file)
        .expect("Could not read file.");

    let mut columns: Vec<Vec<&str>> = Vec::new();
    for line in contents.lines() {
        let entries: Vec<&str> = line.split_whitespace().collect();
        for i in 0..entries.len() {
            if columns.get(i) == None {
                columns.push(Vec::new())
            }

            let col = columns.get_mut(i).unwrap();
            col.push(entries[i]);
        }
    }

    let mut total = 0;
    for col in columns {
        let mut col_total = 0;
        if *col.last().unwrap() == "+" {
            for i in 0..col.len()-1 {
                col_total += col[i].parse::<i64>().expect("Number could not be parsed.");
            }
        }
        else {
            col_total = col[0].parse().expect("Number could not be parsed.");
            for i in 1..col.len()-1 {
                col_total *= col[i].parse::<i64>().expect("Number could not be parsed.");
            }
        }
        total += col_total;
    }

    return total;
}

fn part_2(file: &str) -> i64 {
    let contents = fs::read_to_string(file)
        .expect("Could not read file.");
    let mut lines = contents.lines();
    let ops: Vec<&str> = lines.next_back().unwrap().split_whitespace().collect();
    let remainder: Vec<&str> = lines.collect();
    
    let mut nums: Vec<Vec<i64>> = Vec::new();
    let mut nums_index = 0;
    let cols = remainder[0].len();
    let rows = remainder.len();

    for x in 0..cols {
        let mut num = String::from("");
        for y in 0..rows {
            num.push(remainder[y][x..x+1].chars().next().unwrap());
        }
        num = num.trim().to_string();

        if nums.get(nums_index) == None {
            nums.push(Vec::new());
        }

        if num == "" {
            nums_index += 1;
        }
        else {
            nums[nums_index].push(num.parse().expect("Number could not be parsed."));
        }
    }
    
    let mut total = 0;
    for i in 0..nums.len() {
        let mut sub_total = 0;
        if ops[i] == "+" {
            for num in nums[i].clone() {
                sub_total += num;
            }
        }
        else {
            sub_total = nums[i][0];
            for j in 1..nums[i].len() {
                sub_total *= nums[i][j];
            }
        }
        total += sub_total;
    }

    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("test_input.txt"), 4277556)
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("test_input.txt"), 3263827)
    }
}

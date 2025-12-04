use std::fs;

fn main() {
    println!("{}", part_1("input.txt"));

    println!("{}", part_2("input.txt"));
}

fn part_1(file: &str) -> i32 {
    let contents = fs::read_to_string(file)
        .expect(&format!("{file} could not be read."));
    let mut sum = 0;
    for line in contents.lines() {
        let len = line.len();
        let mut max = 0;
        for first in 0..len-1 {
            let first_char = &line[first..first+1];
            for second in first+1..len {
                let second_char = &line[second..second+1];
                
                let char_combo = &format!("{first_char}{second_char}");
                let num: i32 = str::parse(&char_combo)
                    .expect(&format!("{char_combo} could not be parsed as i32."));
                if num > max {
                    max = num;
                }
            }
        }
        sum += max;
    }

    return sum;
}

fn part_2(file: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("test_input.txt"), 357);
    }

    #[test]
    fn part_2_test() {
    }
}
